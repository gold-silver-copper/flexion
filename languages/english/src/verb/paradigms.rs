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
    /// Like `ike`: 9,547 words.
    Ike,
    /// Like `eak`: 6,603 words.
    Eak,
    /// Like `pass`: 934 words.
    Pass,
    /// Like `try`: 867 words.
    Try,
    /// Like `drop`: 208 words.
    Drop,
    /// Like `spot`: 200 words.
    Spot,
    /// Like `bag`: 173 words.
    Bag,
    /// Like `hear`: 136 words.
    Hear,
    /// Like `club`: 119 words.
    Club,
    /// Like `run`: 111 words.
    Run,
    /// Like `slam`: 102 words.
    Slam,
    /// Like `bed`: 70 words.
    Bed,
    /// Like `cur`: 59 words.
    Cur,
    /// Like `til`: 50 words.
    Til,
    /// Like `nic`: 48 words.
    Nic,
    /// Like `cast`: 48 words.
    Cast,
    /// Like `die`: 44 words.
    Die,
    /// Like `set`: 37 words.
    Set,
    /// Like `sink`: 23 words.
    Sink,
    /// Like `bend`: 21 words.
    Bend,
    /// Like `bind`: 21 words.
    Bind,
    /// Like `fix`: 19 words.
    Fix,
    /// Like `forbear`: 18 words.
    Forbear,
    /// Like `dig`: 18 words.
    Dig,
    /// Like `show`: 17 words.
    Show,
    /// Like `eed`: 17 words.
    Eed,
    /// Like `take`: 16 words.
    Take,
    /// Like `mislead`: 16 words.
    Mislead,
    /// Like `daylight`: 16 words.
    Daylight,
    /// Like `ould`: 16 words.
    Ould,
    /// Like `keep`: 14 words.
    Keep,
    /// Like `tgo`: 13 words.
    Tgo,
    /// Like `bespeak`: 13 words.
    Bespeak,
    /// Like `eue`: 12 words.
    Eue,
    /// Like `rise`: 12 words.
    Rise,
    /// Like `fic`: 11 words.
    Fic,
    /// Like `octopus`: 11 words.
    Octopus,
    /// Like `quiz`: 10 words.
    Quiz,
    /// Like `lay`: 10 words.
    Lay,
    /// Like `sit`: 10 words.
    Sit,
    /// Like `rdo`: 10 words.
    Rdo,
    /// Like `make`: 10 words.
    Make,
    /// Like `rid`: 9 words.
    Rid,
    /// Like `blow`: 8 words.
    Blow,
    /// Like `beat`: 8 words.
    Beat,
    /// Like `chef`: 8 words.
    Chef,
    /// Like `oot`: 8 words.
    Oot,
    /// Like `wear`: 7 words.
    Wear,
    /// Like `stand`: 7 words.
    Stand,
    /// Like `draw`: 7 words.
    Draw,
    /// Like `hold`: 7 words.
    Hold,
    /// Like `give`: 6 words.
    Give,
    /// Like `think`: 6 words.
    Think,
    /// Like `ride`: 6 words.
    Ride,
    /// Like `spin`: 6 words.
    Spin,
    /// Like `shiv`: 6 words.
    Shiv,
    /// Like `larn`: 6 words.
    Larn,
    /// Like `bite`: 5 words.
    Bite,
    /// Like `dive`: 5 words.
    Dive,
    /// Like `belay`: 5 words.
    Belay,
    /// Like `write`: 5 words.
    Write,
    /// Like `shine`: 5 words.
    Shine,
    /// Like `restrike`: 5 words.
    Restrike,
    /// Like `weave`: 4 words.
    Weave,
    /// Like `fall`: 4 words.
    Fall,
    /// Like `dwell`: 4 words.
    Dwell,
    /// Like `get`: 4 words.
    Get,
    /// Like `seek`: 4 words.
    Seek,
    /// Like `swim`: 4 words.
    Swim,
    /// Like `gild`: 4 words.
    Gild,
    /// Like `rede`: 4 words.
    Rede,
    /// Like `ell`: 4 words.
    Ell,
    /// Like `eave`: 4 words.
    Eave,
    /// Like `ive`: 4 words.
    Ive,
    /// Like `olley`: 4 words.
    Olley,
    /// Like `rdie`: 4 words.
    Rdie,
    /// Like `gas`: 3 words.
    Gas,
    /// Like `see`: 3 words.
    See,
    /// Like `lose`: 3 words.
    Lose,
    /// Like `wake`: 3 words.
    Wake,
    /// Like `hang`: 3 words.
    Hang,
    /// Like `backslide`: 3 words.
    Backslide,
    /// Like `tell`: 3 words.
    Tell,
    /// Like `wind`: 3 words.
    Wind,
    /// Like `melt`: 3 words.
    Melt,
    /// Like `aai`: 3 words.
    Aai,
    /// Like `must`: 3 words.
    Must,
    /// Like `light`: 3 words.
    Light,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 88] = [
        Conjugation::Ike,
        Conjugation::Eak,
        Conjugation::Pass,
        Conjugation::Try,
        Conjugation::Drop,
        Conjugation::Spot,
        Conjugation::Bag,
        Conjugation::Hear,
        Conjugation::Club,
        Conjugation::Run,
        Conjugation::Slam,
        Conjugation::Bed,
        Conjugation::Cur,
        Conjugation::Til,
        Conjugation::Nic,
        Conjugation::Cast,
        Conjugation::Die,
        Conjugation::Set,
        Conjugation::Sink,
        Conjugation::Bend,
        Conjugation::Bind,
        Conjugation::Fix,
        Conjugation::Forbear,
        Conjugation::Dig,
        Conjugation::Show,
        Conjugation::Eed,
        Conjugation::Take,
        Conjugation::Mislead,
        Conjugation::Daylight,
        Conjugation::Ould,
        Conjugation::Keep,
        Conjugation::Tgo,
        Conjugation::Bespeak,
        Conjugation::Eue,
        Conjugation::Rise,
        Conjugation::Fic,
        Conjugation::Octopus,
        Conjugation::Quiz,
        Conjugation::Lay,
        Conjugation::Sit,
        Conjugation::Rdo,
        Conjugation::Make,
        Conjugation::Rid,
        Conjugation::Blow,
        Conjugation::Beat,
        Conjugation::Chef,
        Conjugation::Oot,
        Conjugation::Wear,
        Conjugation::Stand,
        Conjugation::Draw,
        Conjugation::Hold,
        Conjugation::Give,
        Conjugation::Think,
        Conjugation::Ride,
        Conjugation::Spin,
        Conjugation::Shiv,
        Conjugation::Larn,
        Conjugation::Bite,
        Conjugation::Dive,
        Conjugation::Belay,
        Conjugation::Write,
        Conjugation::Shine,
        Conjugation::Restrike,
        Conjugation::Weave,
        Conjugation::Fall,
        Conjugation::Dwell,
        Conjugation::Get,
        Conjugation::Seek,
        Conjugation::Swim,
        Conjugation::Gild,
        Conjugation::Rede,
        Conjugation::Ell,
        Conjugation::Eave,
        Conjugation::Ive,
        Conjugation::Olley,
        Conjugation::Rdie,
        Conjugation::Gas,
        Conjugation::See,
        Conjugation::Lose,
        Conjugation::Wake,
        Conjugation::Hang,
        Conjugation::Backslide,
        Conjugation::Tell,
        Conjugation::Wind,
        Conjugation::Melt,
        Conjugation::Aai,
        Conjugation::Must,
        Conjugation::Light,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 88] = [
    // Ike: 9,547 words (eagle, name, abegge, able, bone, google).
    conjugation!("ike", 9547, "~e";
        verb: "~e", "~es", "~ed", "~ing", "~ed";
    ),
    // Eak: 6,603 words (word, book, pond, crow, october, day).
    conjugation!("eak", 6603, "~";
        verb: "~", "~s", "~ed", "~ing", "~ed";
    ),
    // Pass: 934 words (lens, zero, tax, sms, affix, pi).
    conjugation!("pass", 934, "~";
        verb: "~", "~es", "~ed", "~ing", "~ed";
    ),
    // Try: 867 words (sky, nullify, baby, ruby, cry, fly).
    conjugation!("try", 867, "~y";
        verb: "~y", "~ies", "~ied", "~ying", "~ied";
    ),
    // Drop: 208 words (swap, swop, crap, map, pip, bap).
    conjugation!("drop", 208, "~";
        verb: "~", "~s", "~ped", "~ping", "~ped";
    ),
    // Spot: 200 words (cat, abet, abut, het, bit, bot).
    conjugation!("spot", 200, "~";
        verb: "~", "~s", "~ted", "~ting", "~ted";
    ),
    // Bag: 173 words (big, gag, drug, twig, beg, leg).
    conjugation!("bag", 173, "~";
        verb: "~", "~s", "~ged", "~ging", "~ged";
    ),
    // Hear: 136 words (free, pie, thee, toe, tree, dye).
    conjugation!("hear", 136, "~";
        verb: "~", "~s", "~d", "~ing", "~d";
    ),
    // Club: 119 words (ab, lob, dub, bib, stab, pub).
    conjugation!("club", 119, "~";
        verb: "~", "~s", "~bed", "~bing", "~bed";
    ),
    // Run: 111 words (van, in, sun, man, win, thin).
    conjugation!("run", 111, "~";
        verb: "~", "~s", "~ned", "~ning", "~ned";
    ),
    // Slam: 102 words (cum, um, stem, plum, om, bam).
    conjugation!("slam", 102, "~";
        verb: "~", "~s", "~med", "~ming", "~med";
    ),
    // Bed: 70 words (cod, red, squid, god, bad, bed).
    conjugation!("bed", 70, "~";
        verb: "~", "~s", "~ded", "~ding", "~ded";
    ),
    // Cur: 59 words (star, war, er, far, stir, sir).
    conjugation!("cur", 59, "~";
        verb: "~", "~s", "~red", "~ring", "~red";
    ),
    // Til: 50 words (gel, avel, pal, wel, yel, skol).
    conjugation!("til", 50, "~";
        verb: "~", "~s", "~led", "~ling", "~led";
    ),
    // Nic: 48 words (grok, music, yak, kek, logic, bok).
    conjugation!("nic", 48, "~";
        verb: "~", "~s", "~ked", "~king", "~ked";
    ),
    // Cast: 48 words (cast, read, burst, cost, hurt, thrust).
    conjugation!("cast", 48, "~";
        verb: "~", "~s", "~", "~ing", "~";
    ),
    // Die: 44 words (die, lie, tie, sie, vie, hie).
    conjugation!("die", 44, "~~e";
        verb: "~~e", "~~es", "~~ed", "~y~ng", "~~ed";
    ),
    // Set: 37 words (yet, bet, cut, hit, knit, let).
    conjugation!("set", 37, "~";
        verb: "~", "~s", "~", "~ting", "~";
    ),
    // Sink: 23 words (shrink, drink, sing, begin, stink, rin).
    conjugation!("sink", 23, "~i~";
        verb: "~i~", "~i~s", "~a~", "~i~ing", "~u~";
    ),
    // Bend: 21 words (send, bend, build, lend, spend, rend).
    conjugation!("bend", 21, "~d";
        verb: "~d", "~ds", "~t", "~ding", "~t";
    ),
    // Bind: 21 words (bind, fight, find, rewind, unwind, upwind).
    conjugation!("bind", 21, "~i~";
        verb: "~i~", "~i~s", "~ou~", "~i~ing", "~ou~";
    ),
    // Fix: 19 words (deal, born, gar, fix, daydream, uplean).
    conjugation!("fix", 19, "~";
        verb: "~", "~s", "~t|~ed", "~ing", "~t|~ed";
    ),
    // Forbear: 18 words (upbear, forebear, forbear, overbear, rebear, awreak).
    conjugation!("forbear", 18, "~ea~";
        verb: "~ea~", "~ea~s", "~o~e", "~ea~ing", "~o~ne";
    ),
    // Dig: 18 words (dig, slink, resting, unstick, bestick, restick).
    conjugation!("dig", 18, "~i~";
        verb: "~i~", "~i~s", "~u~", "~i~ing", "~u~";
    ),
    // Show: 17 words (shew, mow, sew, show, sow, hew).
    conjugation!("show", 17, "~";
        verb: "~", "~s", "~ed", "~ing", "~n";
    ),
    // Eed: 17 words (feed, meet, xbreed, xfeed, breed, eed).
    conjugation!("eed", 17, "~e~";
        verb: "~e~", "~e~s", "~~", "~e~ing", "~~";
    ),
    // Take: 16 words (take, shake, retake, eate, wayfare, drawe).
    conjugation!("take", 16, "~a~e";
        verb: "~a~e", "~a~es", "~oo~", "~a~ing", "~a~en";
    ),
    // Mislead: 16 words (mislead, uplead, underlead, inlead, outlead, forthlead).
    conjugation!("mislead", 16, "~a~";
        verb: "~a~", "~a~s", "~~", "~a~ing", "~~";
    ),
    // Daylight: 16 words (relight, daylight, backlight, limelight, greenlight, downlight).
    conjugation!("daylight", 16, "~gh~";
        verb: "~gh~", "~gh~s", "~gh~ed|~~", "~gh~ing", "~gh~ed|~~";
    ),
    // Ould: 16 words (may, can, got, will, mote, darf).
    conjugation!("ould", 16, "~";
        verb: "~", "~", "~ed", "~ing", "~ed";
    ),
    // Keep: 14 words (creep, feel, keep, kneel, sleep, sweep).
    conjugation!("keep", 14, "~e~";
        verb: "~e~", "~e~s", "~~t", "~e~ing", "~~t";
    ),
    // Tgo: 13 words (forego, togo, bego, umbego, atgo, ongo).
    conjugation!("tgo", 13, "~go";
        verb: "~go", "~goes", "~went", "~going", "~gone";
    ),
    // Bespeak: 13 words (misspeak, bespeak, jailbreak, unspeak, tiebreak, respeak).
    conjugation!("bespeak", 13, "~ea~";
        verb: "~ea~", "~ea~s", "~o~e", "~ea~ing", "~o~en";
    ),
    // Eue: 12 words (blue, true, horseshoe, coyote, meme, dude).
    conjugation!("eue", 12, "~e";
        verb: "~e", "~es", "~ed", "~eing|~ing", "~ed";
    ),
    // Rise: 12 words (rise, thrive, belive, rize, outrive, bedrive).
    conjugation!("rise", 12, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~i~en";
    ),
    // Fic: 11 words (sic, floc, tic, vac, proc, spec).
    conjugation!("fic", 11, "~";
        verb: "~", "~s", "~ced", "~cing", "~ced";
    ),
    // Octopus: 11 words (octopus, nonplus, dis, pus, res, sus).
    conjugation!("octopus", 11, "~";
        verb: "~", "~ses", "~sed", "~sing", "~sed";
    ),
    // Quiz: 10 words (quiz, spaz, whiz, buz, wiz, diz).
    conjugation!("quiz", 10, "~";
        verb: "~", "~zes", "~zed", "~zing", "~zed";
    ),
    // Lay: 10 words (say, lay, pay, waylay, yeasay, ysay).
    conjugation!("lay", 10, "~y";
        verb: "~y", "~ys", "~id", "~ying", "~id";
    ),
    // Sit: 10 words (shit, sit, bespit, puppysit, outspit, misspit).
    conjugation!("sit", 10, "~i~";
        verb: "~i~", "~i~s", "~a~", "~i~ting", "~a~";
    ),
    // Rdo: 10 words (do, go, redo, undo, bedo, tdo).
    conjugation!("rdo", 10, "~o";
        verb: "~o", "~oes", "~id", "~oing", "~one";
    ),
    // Make: 10 words (make, demake, kingmake, lawmake, mythmake, chmake).
    conjugation!("make", 10, "~ke";
        verb: "~ke", "~kes", "~de", "~king", "~de";
    ),
    // Rid: 9 words (bid, rid, shed, wed, clad, shid).
    conjugation!("rid", 9, "~";
        verb: "~", "~s", "~", "~ding", "~";
    ),
    // Blow: 8 words (know, blow, grow, throw, forthrow, aknow).
    conjugation!("blow", 8, "~o~";
        verb: "~o~", "~o~s", "~e~", "~o~ing", "~o~n";
    ),
    // Beat: 8 words (be, beat, eat, tyek, unbe, burnbeat).
    conjugation!("beat", 8, "~";
        verb: "~", "~s", "~", "~ing", "~en";
    ),
    // Chef: 8 words (ref, chef, coif, stuf, bauf, smof).
    conjugation!("chef", 8, "~";
        verb: "~", "~s", "~fed", "~fing", "~fed";
    ),
    // Oot: 8 words (reshoot, unshoot, misshoot, shoot, oot, oubleshoot).
    conjugation!("oot", 8, "~o~";
        verb: "~o~", "~o~s", "~~", "~o~ing", "~~";
    ),
    // Wear: 7 words (abear, tear, swear, wear, retear, toshear).
    conjugation!("wear", 7, "~ea~";
        verb: "~ea~", "~ea~s", "~o~e", "~ea~ing", "~o~n";
    ),
    // Stand: 7 words (stand, understand, tand, nerstand, ainstand, nuhstand).
    conjugation!("stand", 7, "~an~";
        verb: "~an~", "~an~s", "~oo~", "~an~ing", "~oo~";
    ),
    // Draw: 7 words (draw, redraw, bedraw, adraw, edraw, nedraw).
    conjugation!("draw", 7, "~a~";
        verb: "~a~", "~a~s", "~e~", "~a~ing", "~a~n";
    ),
    // Hold: 7 words (hold, unthrow, anglehold, holde, eathhold, withhold).
    conjugation!("hold", 7, "~o~";
        verb: "~o~", "~o~s", "~e~", "~o~ing", "~e~";
    ),
    // Give: 6 words (give, yive, giue, anksgive, forgive, egive).
    conjugation!("give", 6, "~i~e";
        verb: "~i~e", "~i~es", "~a~e", "~i~ing", "~i~en";
    ),
    // Think: 6 words (think, rethink, forthink, imethink, ethink, bethink).
    conjugation!("think", 6, "~ink";
        verb: "~ink", "~inks", "~ought", "~inking", "~ought";
    ),
    // Ride: 6 words (ride, hagride, overbide, ghostride, stride, atride).
    conjugation!("ride", 6, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~i~den";
    ),
    // Spin: 6 words (spin, respin, unspin, bespin, despin, misspin).
    conjugation!("spin", 6, "~i~";
        verb: "~i~", "~i~s", "~u~", "~i~ning", "~u~";
    ),
    // Shiv: 6 words (shiv, rev, nav, chiv, scav, prov).
    conjugation!("shiv", 6, "~";
        verb: "~", "~s", "~ved", "~ving", "~ved";
    ),
    // Larn: 6 words (de, dee, myek, deek, larn, ott).
    conjugation!("larn", 6, "~";
        verb: "~", "~s", "~ed", "~in", "~ed";
    ),
    // Bite: 5 words (bite, ostbite, hite, istbite, izbite).
    conjugation!("bite", 5, "~e";
        verb: "~e", "~es", "~", "~ing", "~ten";
    ),
    // Dive: 5 words (dive, rive, gainstrive, unthrive, agedive).
    conjugation!("dive", 5, "~i~e";
        verb: "~i~e", "~i~es", "~i~ed|~o~e", "~i~ing", "~i~ed";
    ),
    // Belay: 5 words (why, belay, daresay, apay, appay).
    conjugation!("belay", 5, "~y";
        verb: "~y", "~ys", "~yed|~id", "~ying", "~yed|~id";
    ),
    // Write: 5 words (ghostwrite, write, rite, smite, ngwrite).
    conjugation!("write", 5, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~i~ten";
    ),
    // Shine: 5 words (overstride, understride, tride, shine, hine).
    conjugation!("shine", 5, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~o~e";
    ),
    // Restrike: 5 words (restrike, misstrike, awestrike, unstrike, strike).
    conjugation!("restrike", 5, "~i~e";
        verb: "~i~e", "~i~es", "~uc~", "~i~ing", "~uc~|~ic~en";
    ),
    // Weave: 4 words (weave, forcleave, tocleave, overheave).
    conjugation!("weave", 4, "~~ave";
        verb: "~~ave", "~~aves", "~ov~", "~~aving", "~ov~n";
    ),
    // Fall: 4 words (fall, atfall, befall, fal).
    conjugation!("fall", 4, "~a~";
        verb: "~a~", "~a~s", "~e~", "~a~ing", "~a~en";
    ),
    // Dwell: 4 words (dwell, misspell, foresmell, bedwell).
    conjugation!("dwell", 4, "~l";
        verb: "~l", "~ls", "~t", "~ling", "~t";
    ),
    // Get: 4 words (forget, get, beget, misget).
    conjugation!("get", 4, "~e~";
        verb: "~e~", "~e~s", "~o~", "~e~ting", "~o~ten";
    ),
    // Seek: 4 words (seek, beseek, reseek, misseek).
    conjugation!("seek", 4, "~eek";
        verb: "~eek", "~eeks", "~ought", "~eeking", "~ought";
    ),
    // Swim: 4 words (swim, nim, benim, wim).
    conjugation!("swim", 4, "~i~";
        verb: "~i~", "~i~s", "~a~", "~i~ming", "~u~";
    ),
    // Gild: 4 words (gird, gild, backbend, inbend).
    conjugation!("gild", 4, "~d";
        verb: "~d", "~ds", "~ded|~t", "~ding", "~ded|~t";
    ),
    // Rede: 4 words (rede, betide, slide, chide).
    conjugation!("rede", 4, "~e";
        verb: "~e", "~es", "~", "~ing", "~";
    ),
    // Ell: 4 words (indwell, ell, spill, smell).
    conjugation!("ell", 4, "~l";
        verb: "~l", "~ls", "~led|~t", "~ling", "~led|~t";
    ),
    // Eave: 4 words (toreave, eave, leave, cleave).
    conjugation!("eave", 4, "~ave";
        verb: "~ave", "~aves", "~ft", "~aving", "~ft";
    ),
    // Ive: 4 words (ide, ise, ive, iue).
    conjugation!("ive", 4, "i~e";
        verb: "i~e", "i~es", "o~e", "i~ing", "i~en";
    ),
    // Olley: 4 words (nkey, gey, tsey, olley).
    conjugation!("olley", 4, "~~y";
        verb: "~~y", "~~ys", "~~yed|~i~d", "~~ying", "~~yed|~i~d";
    ),
    // Rdie: 4 words (eelie, ymie, rdie, oilzie).
    conjugation!("rdie", 4, "~~e";
        verb: "~~e", "~~es", "~~ed", "~~eing|~y~ng", "~~ed";
    ),
    // Gas: 3 words (gas, yes, rgas).
    conjugation!("gas", 3, "~";
        verb: "~", "~es|~ses", "~sed", "~sing", "~sed";
    ),
    // See: 3 words (see, looksee, ghtsee).
    conjugation!("see", 3, "~ee";
        verb: "~ee", "~ees", "~aw", "~eeing", "~een";
    ),
    // Lose: 3 words (lose, kape, beclepe).
    conjugation!("lose", 3, "~e";
        verb: "~e", "~es", "~t", "~ing", "~t";
    ),
    // Wake: 3 words (awake, wake, forcarve).
    conjugation!("wake", 3, "~a~e";
        verb: "~a~e", "~a~es", "~o~e|~a~ed", "~a~ing", "~o~en|~a~ed";
    ),
    // Hang: 3 words (hang, iffhang, aphang).
    conjugation!("hang", 3, "~a~";
        verb: "~a~", "~a~s", "~u~", "~a~ing", "~u~";
    ),
    // Backslide: 3 words (hide, backslide, hyde).
    conjugation!("backslide", 3, "~e";
        verb: "~e", "~es", "~", "~ing", "~den";
    ),
    // Tell: 3 words (sell, tell, luntell).
    conjugation!("tell", 3, "~e~l";
        verb: "~e~l", "~e~ls", "~o~d", "~e~ling", "~o~d";
    ),
    // Wind: 3 words (wind, heatsink, climb).
    conjugation!("wind", 3, "~i~";
        verb: "~i~", "~i~s", "~i~ed", "~i~ing", "~i~ed";
    ),
    // Melt: 3 words (melt, upswell, outswell).
    conjugation!("melt", 3, "~e~";
        verb: "~e~", "~e~s", "~e~ed", "~e~ing", "~e~ed|~o~en";
    ),
    // Aai: 3 words (luigi, aai, bidi).
    conjugation!("aai", 3, "~";
        verb: "~", "~s", "~ed", "~ng", "~ed";
    ),
    // Must: 3 words (must, gaf, wfh).
    conjugation!("must", 3, "~";
        verb: "~", "~", "~", "~", "~";
    ),
    // Light: 3 words (light, ilight, oodlight).
    conjugation!("light", 3, "~gh~";
        verb: "~gh~", "~gh~s", "~~|~gh~ed", "~gh~ing", "~~|~gh~ed";
    ),
];
