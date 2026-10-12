//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 178 prefixes and 219 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("ab"),
            separable: 0,
            inseparable: 25,
            words: 25,
        },
        Prefix {
            letters: Cow::Borrowed("abb"),
            separable: 0,
            inseparable: 126,
            words: 126,
        },
        Prefix {
            letters: Cow::Borrowed("ac"),
            separable: 0,
            inseparable: 25,
            words: 25,
        },
        Prefix {
            letters: Cow::Borrowed("acc"),
            separable: 0,
            inseparable: 137,
            words: 139,
        },
        Prefix {
            letters: Cow::Borrowed("acca"),
            separable: 0,
            inseparable: 68,
            words: 69,
        },
        Prefix {
            letters: Cow::Borrowed("ad"),
            separable: 0,
            inseparable: 101,
            words: 108,
        },
        Prefix {
            letters: Cow::Borrowed("aero"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("af"),
            separable: 0,
            inseparable: 100,
            words: 103,
        },
        Prefix {
            letters: Cow::Borrowed("ag"),
            separable: 0,
            inseparable: 14,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("agg"),
            separable: 0,
            inseparable: 66,
            words: 67,
        },
        Prefix {
            letters: Cow::Borrowed("aggi"),
            separable: 0,
            inseparable: 15,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("al"),
            separable: 0,
            inseparable: 39,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("all"),
            separable: 0,
            inseparable: 74,
            words: 74,
        },
        Prefix {
            letters: Cow::Borrowed("am"),
            separable: 0,
            inseparable: 25,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("amm"),
            separable: 0,
            inseparable: 51,
            words: 52,
        },
        Prefix {
            letters: Cow::Borrowed("amma"),
            separable: 0,
            inseparable: 38,
            words: 38,
        },
        Prefix {
            letters: Cow::Borrowed("an"),
            separable: 0,
            inseparable: 46,
            words: 47,
        },
        Prefix {
            letters: Cow::Borrowed("ann"),
            separable: 0,
            inseparable: 48,
            words: 49,
        },
        Prefix {
            letters: Cow::Borrowed("anti"),
            separable: 0,
            inseparable: 6,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("ap"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("app"),
            separable: 0,
            inseparable: 121,
            words: 123,
        },
        Prefix {
            letters: Cow::Borrowed("ar"),
            separable: 0,
            inseparable: 121,
            words: 123,
        },
        Prefix {
            letters: Cow::Borrowed("as"),
            separable: 0,
            inseparable: 117,
            words: 122,
        },
        Prefix {
            letters: Cow::Borrowed("at"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("att"),
            separable: 0,
            inseparable: 74,
            words: 75,
        },
        Prefix {
            letters: Cow::Borrowed("auto"),
            separable: 0,
            inseparable: 75,
            words: 75,
        },
        Prefix {
            letters: Cow::Borrowed("av"),
            separable: 0,
            inseparable: 74,
            words: 76,
        },
        Prefix {
            letters: Cow::Borrowed("az"),
            separable: 0,
            inseparable: 21,
            words: 21,
        },
        Prefix {
            letters: Cow::Borrowed("ben"),
            separable: 0,
            inseparable: 9,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("bi"),
            separable: 0,
            inseparable: 42,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("bio"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("br"),
            separable: 0,
            inseparable: 44,
            words: 44,
        },
        Prefix {
            letters: Cow::Borrowed("ca"),
            separable: 0,
            inseparable: 125,
            words: 125,
        },
        Prefix {
            letters: Cow::Borrowed("capi"),
            separable: 0,
            inseparable: 10,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("capo"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("car"),
            separable: 0,
            inseparable: 37,
            words: 38,
        },
        Prefix {
            letters: Cow::Borrowed("chi"),
            separable: 0,
            inseparable: 28,
            words: 28,
        },
        Prefix {
            letters: Cow::Borrowed("cir"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("circon"),
            separable: 0,
            inseparable: 12,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("co"),
            separable: 0,
            inseparable: 72,
            words: 74,
        },
        Prefix {
            letters: Cow::Borrowed("col"),
            separable: 0,
            inseparable: 39,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("com"),
            separable: 0,
            inseparable: 87,
            words: 89,
        },
        Prefix {
            letters: Cow::Borrowed("con"),
            separable: 0,
            inseparable: 167,
            words: 174,
        },
        Prefix {
            letters: Cow::Borrowed("cont"),
            separable: 0,
            inseparable: 30,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("contr"),
            separable: 0,
            inseparable: 22,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("contrap"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("contro"),
            separable: 0,
            inseparable: 19,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("cor"),
            separable: 0,
            inseparable: 38,
            words: 38,
        },
        Prefix {
            letters: Cow::Borrowed("de"),
            separable: 0,
            inseparable: 307,
            words: 314,
        },
        Prefix {
            letters: Cow::Borrowed("di"),
            separable: 0,
            inseparable: 137,
            words: 141,
        },
        Prefix {
            letters: Cow::Borrowed("dif"),
            separable: 0,
            inseparable: 14,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("dis"),
            separable: 0,
            inseparable: 315,
            words: 322,
        },
        Prefix {
            letters: Cow::Borrowed("do"),
            separable: 0,
            inseparable: 32,
            words: 35,
        },
        Prefix {
            letters: Cow::Borrowed("ec"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("ef"),
            separable: 0,
            inseparable: 9,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("equi"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("es"),
            separable: 0,
            inseparable: 124,
            words: 128,
        },
        Prefix {
            letters: Cow::Borrowed("fi"),
            separable: 0,
            inseparable: 50,
            words: 52,
        },
        Prefix {
            letters: Cow::Borrowed("fo"),
            separable: 0,
            inseparable: 49,
            words: 51,
        },
        Prefix {
            letters: Cow::Borrowed("fr"),
            separable: 0,
            inseparable: 40,
            words: 40,
        },
        Prefix {
            letters: Cow::Borrowed("fra"),
            separable: 0,
            inseparable: 25,
            words: 25,
        },
        Prefix {
            letters: Cow::Borrowed("fram"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("gi"),
            separable: 0,
            inseparable: 43,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("gr"),
            separable: 0,
            inseparable: 48,
            words: 48,
        },
        Prefix {
            letters: Cow::Borrowed("il"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("im"),
            separable: 0,
            inseparable: 206,
            words: 207,
        },
        Prefix {
            letters: Cow::Borrowed("imb"),
            separable: 0,
            inseparable: 70,
            words: 70,
        },
        Prefix {
            letters: Cow::Borrowed("imba"),
            separable: 0,
            inseparable: 29,
            words: 29,
        },
        Prefix {
            letters: Cow::Borrowed("in"),
            separable: 0,
            inseparable: 465,
            words: 470,
        },
        Prefix {
            letters: Cow::Borrowed("inca"),
            separable: 0,
            inseparable: 82,
            words: 82,
        },
        Prefix {
            letters: Cow::Borrowed("infi"),
            separable: 0,
            inseparable: 30,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("inn"),
            separable: 0,
            inseparable: 22,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("ins"),
            separable: 0,
            inseparable: 77,
            words: 77,
        },
        Prefix {
            letters: Cow::Borrowed("int"),
            separable: 0,
            inseparable: 86,
            words: 87,
        },
        Prefix {
            letters: Cow::Borrowed("inter"),
            separable: 0,
            inseparable: 50,
            words: 51,
        },
        Prefix {
            letters: Cow::Borrowed("intra"),
            separable: 0,
            inseparable: 15,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("intro"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("ir"),
            separable: 0,
            inseparable: 22,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("la"),
            separable: 0,
            inseparable: 61,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("ma"),
            separable: 0,
            inseparable: 32,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("mal"),
            separable: 0,
            inseparable: 15,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("man"),
            separable: 0,
            inseparable: 31,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("mar"),
            separable: 0,
            inseparable: 32,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("mas"),
            separable: 0,
            inseparable: 21,
            words: 21,
        },
        Prefix {
            letters: Cow::Borrowed("mat"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("mi"),
            separable: 0,
            inseparable: 54,
            words: 55,
        },
        Prefix {
            letters: Cow::Borrowed("micro"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("op"),
            separable: 0,
            inseparable: 16,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("os"),
            separable: 0,
            inseparable: 29,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("ot"),
            separable: 0,
            inseparable: 17,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("pa"),
            separable: 0,
            inseparable: 86,
            words: 87,
        },
        Prefix {
            letters: Cow::Borrowed("par"),
            separable: 0,
            inseparable: 42,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("per"),
            separable: 0,
            inseparable: 63,
            words: 66,
        },
        Prefix {
            letters: Cow::Borrowed("pi"),
            separable: 0,
            inseparable: 93,
            words: 94,
        },
        Prefix {
            letters: Cow::Borrowed("pl"),
            separable: 0,
            inseparable: 17,
            words: 18,
        },
        Prefix {
            letters: Cow::Borrowed("po"),
            separable: 0,
            inseparable: 63,
            words: 65,
        },
        Prefix {
            letters: Cow::Borrowed("pr"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("pre"),
            separable: 0,
            inseparable: 138,
            words: 147,
        },
        Prefix {
            letters: Cow::Borrowed("pro"),
            separable: 0,
            inseparable: 124,
            words: 126,
        },
        Prefix {
            letters: Cow::Borrowed("ra"),
            separable: 0,
            inseparable: 29,
            words: 29,
        },
        Prefix {
            letters: Cow::Borrowed("rab"),
            separable: 0,
            inseparable: 23,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("rac"),
            separable: 0,
            inseparable: 43,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("rad"),
            separable: 0,
            inseparable: 21,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("radio"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("raf"),
            separable: 0,
            inseparable: 23,
            words: 24,
        },
        Prefix {
            letters: Cow::Borrowed("rag"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("ragg"),
            separable: 0,
            inseparable: 33,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("ral"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("ram"),
            separable: 0,
            inseparable: 31,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("rap"),
            separable: 0,
            inseparable: 32,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("ras"),
            separable: 0,
            inseparable: 35,
            words: 35,
        },
        Prefix {
            letters: Cow::Borrowed("rat"),
            separable: 0,
            inseparable: 22,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("rav"),
            separable: 0,
            inseparable: 18,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("re"),
            separable: 0,
            inseparable: 164,
            words: 168,
        },
        Prefix {
            letters: Cow::Borrowed("red"),
            separable: 0,
            inseparable: 14,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("retro"),
            separable: 0,
            inseparable: 4,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("ri"),
            separable: 0,
            inseparable: 1106,
            words: 1122,
        },
        Prefix {
            letters: Cow::Borrowed("ric"),
            separable: 0,
            inseparable: 215,
            words: 215,
        },
        Prefix {
            letters: Cow::Borrowed("rim"),
            separable: 0,
            inseparable: 139,
            words: 141,
        },
        Prefix {
            letters: Cow::Borrowed("rimb"),
            separable: 0,
            inseparable: 44,
            words: 44,
        },
        Prefix {
            letters: Cow::Borrowed("rin"),
            separable: 0,
            inseparable: 279,
            words: 280,
        },
        Prefix {
            letters: Cow::Borrowed("ro"),
            separable: 0,
            inseparable: 39,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("sa"),
            separable: 0,
            inseparable: 75,
            words: 77,
        },
        Prefix {
            letters: Cow::Borrowed("sb"),
            separable: 0,
            inseparable: 103,
            words: 103,
        },
        Prefix {
            letters: Cow::Borrowed("sba"),
            separable: 0,
            inseparable: 73,
            words: 76,
        },
        Prefix {
            letters: Cow::Borrowed("sbi"),
            separable: 0,
            inseparable: 28,
            words: 29,
        },
        Prefix {
            letters: Cow::Borrowed("sc"),
            separable: 0,
            inseparable: 130,
            words: 130,
        },
        Prefix {
            letters: Cow::Borrowed("sca"),
            separable: 0,
            inseparable: 163,
            words: 164,
        },
        Prefix {
            letters: Cow::Borrowed("sci"),
            separable: 0,
            inseparable: 55,
            words: 55,
        },
        Prefix {
            letters: Cow::Borrowed("sco"),
            separable: 0,
            inseparable: 55,
            words: 59,
        },
        Prefix {
            letters: Cow::Borrowed("scom"),
            separable: 0,
            inseparable: 30,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("scon"),
            separable: 0,
            inseparable: 40,
            words: 42,
        },
        Prefix {
            letters: Cow::Borrowed("scor"),
            separable: 0,
            inseparable: 28,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("sd"),
            separable: 0,
            inseparable: 33,
            words: 34,
        },
        Prefix {
            letters: Cow::Borrowed("se"),
            separable: 0,
            inseparable: 83,
            words: 88,
        },
        Prefix {
            letters: Cow::Borrowed("sf"),
            separable: 0,
            inseparable: 108,
            words: 109,
        },
        Prefix {
            letters: Cow::Borrowed("sfi"),
            separable: 0,
            inseparable: 28,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("sg"),
            separable: 0,
            inseparable: 61,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("sgr"),
            separable: 0,
            inseparable: 38,
            words: 38,
        },
        Prefix {
            letters: Cow::Borrowed("sm"),
            separable: 0,
            inseparable: 114,
            words: 114,
        },
        Prefix {
            letters: Cow::Borrowed("so"),
            separable: 0,
            inseparable: 38,
            words: 41,
        },
        Prefix {
            letters: Cow::Borrowed("sob"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("sof"),
            separable: 0,
            inseparable: 16,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("sog"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("sol"),
            separable: 0,
            inseparable: 24,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("som"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("sop"),
            separable: 0,
            inseparable: 10,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("sopr"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("sopra"),
            separable: 0,
            inseparable: 39,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("soprav"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("sor"),
            separable: 0,
            inseparable: 24,
            words: 24,
        },
        Prefix {
            letters: Cow::Borrowed("sot"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("sott"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("sotto"),
            separable: 0,
            inseparable: 25,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("sov"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("sovr"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("sovra"),
            separable: 0,
            inseparable: 29,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("sp"),
            separable: 0,
            inseparable: 265,
            words: 269,
        },
        Prefix {
            letters: Cow::Borrowed("st"),
            separable: 0,
            inseparable: 129,
            words: 130,
        },
        Prefix {
            letters: Cow::Borrowed("str"),
            separable: 0,
            inseparable: 54,
            words: 54,
        },
        Prefix {
            letters: Cow::Borrowed("stra"),
            separable: 0,
            inseparable: 54,
            words: 58,
        },
        Prefix {
            letters: Cow::Borrowed("su"),
            separable: 0,
            inseparable: 35,
            words: 35,
        },
        Prefix {
            letters: Cow::Borrowed("sub"),
            separable: 0,
            inseparable: 18,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("sud"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("super"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("sur"),
            separable: 0,
            inseparable: 10,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("sus"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("tele"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("termo"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("tor"),
            separable: 0,
            inseparable: 20,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("tr"),
            separable: 0,
            inseparable: 28,
            words: 28,
        },
        Prefix {
            letters: Cow::Borrowed("tra"),
            separable: 0,
            inseparable: 81,
            words: 84,
        },
        Prefix {
            letters: Cow::Borrowed("tran"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("trans"),
            separable: 0,
            inseparable: 20,
            words: 24,
        },
        Prefix {
            letters: Cow::Borrowed("tras"),
            separable: 0,
            inseparable: 53,
            words: 56,
        },
        Prefix {
            letters: Cow::Borrowed("tri"),
            separable: 0,
            inseparable: 29,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("vi"),
            separable: 0,
            inseparable: 48,
            words: 49,
        },
        Prefix {
            letters: Cow::Borrowed("video"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("accasciare"), Separation::Neither),
        (Cow::Borrowed("accostare"), Separation::Neither),
        (Cow::Borrowed("accudire"), Separation::Neither),
        (Cow::Borrowed("addarsi"), Separation::Neither),
        (Cow::Borrowed("addire"), Separation::Neither),
        (Cow::Borrowed("addirsi"), Separation::Neither),
        (Cow::Borrowed("adempiere"), Separation::Neither),
        (Cow::Borrowed("adempire"), Separation::Neither),
        (Cow::Borrowed("adire"), Separation::Neither),
        (Cow::Borrowed("adsorbire"), Separation::Neither),
        (Cow::Borrowed("affarsi"), Separation::Neither),
        (Cow::Borrowed("affiggere"), Separation::Neither),
        (Cow::Borrowed("affluire"), Separation::Neither),
        (Cow::Borrowed("aggredire"), Separation::Neither),
        (Cow::Borrowed("agire"), Separation::Neither),
        (Cow::Borrowed("ammostare"), Separation::Neither),
        (Cow::Borrowed("ampiare"), Separation::Neither),
        (Cow::Borrowed("andare"), Separation::Neither),
        (Cow::Borrowed("annare"), Separation::Neither),
        (Cow::Borrowed("antistare"), Separation::Neither),
        (Cow::Borrowed("antivedere"), Separation::Neither),
        (Cow::Borrowed("appannare"), Separation::Neither),
        (Cow::Borrowed("appostare"), Separation::Neither),
        (Cow::Borrowed("ardire"), Separation::Neither),
        (Cow::Borrowed("arrotare"), Separation::Neither),
        (Cow::Borrowed("asservire"), Separation::Neither),
        (Cow::Borrowed("asservirsi"), Separation::Neither),
        (Cow::Borrowed("assolvere"), Separation::Neither),
        (Cow::Borrowed("assorbire"), Separation::Neither),
        (Cow::Borrowed("assortire"), Separation::Neither),
        (Cow::Borrowed("attristare"), Separation::Neither),
        (Cow::Borrowed("avellere"), Separation::Neither),
        (Cow::Borrowed("avere"), Separation::Neither),
        (Cow::Borrowed("bendare"), Separation::Neither),
        (Cow::Borrowed("bendarsi"), Separation::Neither),
        (Cow::Borrowed("bipartire"), Separation::Neither),
        (Cow::Borrowed("capirsi"), Separation::Neither),
        (Cow::Borrowed("cardare"), Separation::Neither),
        (Cow::Borrowed("circondare"), Separation::Neither),
        (Cow::Borrowed("circondarsi"), Separation::Neither),
        (Cow::Borrowed("circonfluire"), Separation::Neither),
        (Cow::Borrowed("comandare"), Separation::Neither),
        (Cow::Borrowed("compartire"), Separation::Neither),
        (Cow::Borrowed("concedere"), Separation::Neither),
        (Cow::Borrowed("condire"), Separation::Neither),
        (Cow::Borrowed("confarsi"), Separation::Neither),
        (Cow::Borrowed("confluire"), Separation::Neither),
        (Cow::Borrowed("connotare"), Separation::Neither),
        (Cow::Borrowed("constare"), Separation::Neither),
        (Cow::Borrowed("contessere"), Separation::Neither),
        (Cow::Borrowed("contradire"), Separation::Neither),
        (Cow::Borrowed("contronotare"), Separation::Neither),
        (Cow::Borrowed("convergere"), Separation::Neither),
        (Cow::Borrowed("copiare"), Separation::Neither),
        (Cow::Borrowed("costare"), Separation::Neither),
        (Cow::Borrowed("decernere"), Separation::Neither),
        (Cow::Borrowed("defiggere"), Separation::Neither),
        (Cow::Borrowed("defluire"), Separation::Neither),
        (Cow::Borrowed("denotare"), Separation::Neither),
        (Cow::Borrowed("desorbire"), Separation::Neither),
        (Cow::Borrowed("destare"), Separation::Neither),
        (Cow::Borrowed("destarsi"), Separation::Neither),
        (Cow::Borrowed("dicere"), Separation::Neither),
        (Cow::Borrowed("dipartirsi"), Separation::Neither),
        (Cow::Borrowed("dipendere"), Separation::Neither),
        (Cow::Borrowed("dirsi"), Separation::Neither),
        (Cow::Borrowed("disabbonare"), Separation::Neither),
        (Cow::Borrowed("disdire"), Separation::Neither),
        (Cow::Borrowed("disdirsi"), Separation::Neither),
        (Cow::Borrowed("disfare"), Separation::Neither),
        (Cow::Borrowed("disfarsi"), Separation::Neither),
        (Cow::Borrowed("dispendere"), Separation::Neither),
        (Cow::Borrowed("dissolvere"), Separation::Neither),
        (Cow::Borrowed("dolere"), Separation::Neither),
        (Cow::Borrowed("dolersi"), Separation::Neither),
        (Cow::Borrowed("dovere"), Separation::Neither),
        (Cow::Borrowed("effluire"), Separation::Neither),
        (Cow::Borrowed("esaudire"), Separation::Neither),
        (Cow::Borrowed("esordire"), Separation::Neither),
        (Cow::Borrowed("espiare"), Separation::Neither),
        (Cow::Borrowed("essere"), Separation::Neither),
        (Cow::Borrowed("fidare"), Separation::Neither),
        (Cow::Borrowed("fidarsi"), Separation::Neither),
        (Cow::Borrowed("fondere"), Separation::Neither),
        (Cow::Borrowed("fondersi"), Separation::Neither),
        (Cow::Borrowed("impartire"), Separation::Neither),
        (Cow::Borrowed("incontrarsi"), Separation::Neither),
        (Cow::Borrowed("indire"), Separation::Neither),
        (Cow::Borrowed("indirigere"), Separation::Neither),
        (Cow::Borrowed("influire"), Separation::Neither),
        (Cow::Borrowed("interdire"), Separation::Neither),
        (Cow::Borrowed("intessere"), Separation::Neither),
        (Cow::Borrowed("intravienire"), Separation::Neither),
        (Cow::Borrowed("invaiare"), Separation::Neither),
        (Cow::Borrowed("lasciare"), Separation::Neither),
        (Cow::Borrowed("maladire"), Separation::Neither),
        (Cow::Borrowed("malandare"), Separation::Neither),
        (Cow::Borrowed("mandare"), Separation::Neither),
        (Cow::Borrowed("mandarsi"), Separation::Neither),
        (Cow::Borrowed("misfare"), Separation::Neither),
        (Cow::Borrowed("oppiare"), Separation::Neither),
        (Cow::Borrowed("osannare"), Separation::Neither),
        (Cow::Borrowed("padire"), Separation::Neither),
        (Cow::Borrowed("parere"), Separation::Neither),
        (Cow::Borrowed("perfare"), Separation::Neither),
        (Cow::Borrowed("perire"), Separation::Neither),
        (Cow::Borrowed("permeare"), Separation::Neither),
        (Cow::Borrowed("pisciare"), Separation::Neither),
        (Cow::Borrowed("plaudire"), Separation::Neither),
        (Cow::Borrowed("postare"), Separation::Neither),
        (Cow::Borrowed("potere"), Separation::Neither),
        (Cow::Borrowed("predare"), Separation::Neither),
        (Cow::Borrowed("predire"), Separation::Neither),
        (Cow::Borrowed("prefare"), Separation::Neither),
        (Cow::Borrowed("prefiggere"), Separation::Neither),
        (Cow::Borrowed("prenotare"), Separation::Neither),
        (Cow::Borrowed("presapere"), Separation::Neither),
        (Cow::Borrowed("prestare"), Separation::Neither),
        (Cow::Borrowed("prestarsi"), Separation::Neither),
        (Cow::Borrowed("pretessere"), Separation::Neither),
        (Cow::Borrowed("procreare"), Separation::Neither),
        (Cow::Borrowed("provedere"), Separation::Neither),
        (Cow::Borrowed("radere"), Separation::Neither),
        (Cow::Borrowed("radersi"), Separation::Neither),
        (Cow::Borrowed("raffare"), Separation::Neither),
        (Cow::Borrowed("rapire"), Separation::Neither),
        (Cow::Borrowed("ravvedersi"), Separation::Neither),
        (Cow::Borrowed("reddire"), Separation::Neither),
        (Cow::Borrowed("redire"), Separation::Neither),
        (Cow::Borrowed("refluire"), Separation::Neither),
        (Cow::Borrowed("resolvere"), Separation::Neither),
        (Cow::Borrowed("restare"), Separation::Neither),
        (Cow::Borrowed("restarsi"), Separation::Neither),
        (Cow::Borrowed("retrocedere"), Separation::Neither),
        (Cow::Borrowed("riabbonare"), Separation::Neither),
        (Cow::Borrowed("riammollire"), Separation::Neither),
        (Cow::Borrowed("riandare"), Separation::Neither),
        (Cow::Borrowed("riavere"), Separation::Neither),
        (Cow::Borrowed("ridare"), Separation::Neither),
        (Cow::Borrowed("ridarsi"), Separation::Neither),
        (Cow::Borrowed("riessere"), Separation::Neither),
        (Cow::Borrowed("rifare"), Separation::Neither),
        (Cow::Borrowed("rifarsi"), Separation::Neither),
        (Cow::Borrowed("rifluire"), Separation::Neither),
        (Cow::Borrowed("rimandare"), Separation::Neither),
        (Cow::Borrowed("rimunire"), Separation::Neither),
        (Cow::Borrowed("rincontrarsi"), Separation::Neither),
        (Cow::Borrowed("ripartorire"), Separation::Neither),
        (Cow::Borrowed("risapere"), Separation::Neither),
        (Cow::Borrowed("risolvere"), Separation::Neither),
        (Cow::Borrowed("ristare"), Separation::Neither),
        (Cow::Borrowed("risviare"), Separation::Neither),
        (Cow::Borrowed("ritessere"), Separation::Neither),
        (Cow::Borrowed("rodare"), Separation::Neither),
        (Cow::Borrowed("rompere"), Separation::Neither),
        (Cow::Borrowed("rompersi"), Separation::Neither),
        (Cow::Borrowed("rotare"), Separation::Neither),
        (Cow::Borrowed("sapere"), Separation::Neither),
        (Cow::Borrowed("sapersi"), Separation::Neither),
        (Cow::Borrowed("sbadire"), Separation::Neither),
        (Cow::Borrowed("sbafare"), Separation::Neither),
        (Cow::Borrowed("sbaudire"), Separation::Neither),
        (Cow::Borrowed("sbisciare"), Separation::Neither),
        (Cow::Borrowed("scafare"), Separation::Neither),
        (Cow::Borrowed("scodare"), Separation::Neither),
        (Cow::Borrowed("scompartire"), Separation::Neither),
        (Cow::Borrowed("scontessere"), Separation::Neither),
        (Cow::Borrowed("scontrarsi"), Separation::Neither),
        (Cow::Borrowed("scorare"), Separation::Neither),
        (Cow::Borrowed("scorarsi"), Separation::Neither),
        (Cow::Borrowed("scordare"), Separation::Neither),
        (Cow::Borrowed("scordarsi"), Separation::Neither),
        (Cow::Borrowed("scosciare"), Separation::Neither),
        (Cow::Borrowed("scostare"), Separation::Neither),
        (Cow::Borrowed("scostarsi"), Separation::Neither),
        (Cow::Borrowed("sdire"), Separation::Neither),
        (Cow::Borrowed("secernere"), Separation::Neither),
        (Cow::Borrowed("sedare"), Separation::Neither),
        (Cow::Borrowed("sedere"), Separation::Neither),
        (Cow::Borrowed("sedersi"), Separation::Neither),
        (Cow::Borrowed("sementire"), Separation::Neither),
        (Cow::Borrowed("sfidare"), Separation::Neither),
        (Cow::Borrowed("sfidarsi"), Separation::Neither),
        (Cow::Borrowed("sfuggire"), Separation::Neither),
        (Cow::Borrowed("sgannare"), Separation::Neither),
        (Cow::Borrowed("sodare"), Separation::Neither),
        (Cow::Borrowed("sofferire"), Separation::Neither),
        (Cow::Borrowed("solere"), Separation::Neither),
        (Cow::Borrowed("solfare"), Separation::Neither),
        (Cow::Borrowed("sonare"), Separation::Neither),
        (Cow::Borrowed("sopire"), Separation::Neither),
        (Cow::Borrowed("soppellire"), Separation::Neither),
        (Cow::Borrowed("soppendere"), Separation::Neither),
        (Cow::Borrowed("sostare"), Separation::Neither),
        (Cow::Borrowed("sottostare"), Separation::Neither),
        (Cow::Borrowed("sovrastare"), Separation::Neither),
        (Cow::Borrowed("spannare"), Separation::Neither),
        (Cow::Borrowed("spergere"), Separation::Neither),
        (Cow::Borrowed("spiacere"), Separation::Neither),
        (Cow::Borrowed("spostare"), Separation::Neither),
        (Cow::Borrowed("staffarsi"), Separation::Neither),
        (Cow::Borrowed("strafare"), Separation::Neither),
        (Cow::Borrowed("strafarsi"), Separation::Neither),
        (Cow::Borrowed("strasapere"), Separation::Neither),
        (Cow::Borrowed("stravedere"), Separation::Neither),
        (Cow::Borrowed("subire"), Separation::Neither),
        (Cow::Borrowed("surfare"), Separation::Neither),
        (Cow::Borrowed("tradire"), Separation::Neither),
        (Cow::Borrowed("tradirsi"), Separation::Neither),
        (Cow::Borrowed("transandare"), Separation::Neither),
        (Cow::Borrowed("transfluire"), Separation::Neither),
        (Cow::Borrowed("transire"), Separation::Neither),
        (Cow::Borrowed("transvedere"), Separation::Neither),
        (Cow::Borrowed("trasandare"), Separation::Neither),
        (Cow::Borrowed("trasparere"), Separation::Neither),
        (Cow::Borrowed("trasvedere"), Separation::Neither),
        (Cow::Borrowed("travedere"), Separation::Neither),
        (Cow::Borrowed("tripartire"), Separation::Neither),
        (Cow::Borrowed("vistare"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // PRES_1_SG
        Before, // PRES_1_PL
        Before, // PRES_2_SG
        Before, // PRES_2_PL
        Before, // PRES_3_SG
        Before, // PRES_3_PL
        Before, // IMPF_1_SG
        Before, // IMPF_1_PL
        Before, // IMPF_2_SG
        Before, // IMPF_2_PL
        Before, // IMPF_3_SG
        Before, // IMPF_3_PL
        Before, // PAST_1_SG
        Before, // PAST_1_PL
        Before, // PAST_2_SG
        Before, // PAST_2_PL
        Before, // PAST_3_SG
        Before, // PAST_3_PL
        Before, // FUT_1_SG
        Before, // FUT_1_PL
        Before, // FUT_2_SG
        Before, // FUT_2_PL
        Before, // FUT_3_SG
        Before, // FUT_3_PL
        Before, // COND_1_SG
        Before, // COND_1_PL
        Before, // COND_2_SG
        Before, // COND_2_PL
        Before, // COND_3_SG
        Before, // COND_3_PL
        Before, // SUBJ_1_SG
        Before, // SUBJ_1_PL
        Before, // SUBJ_2_SG
        Before, // SUBJ_2_PL
        Before, // SUBJ_3_SG
        Before, // SUBJ_3_PL
        Before, // SUBJ_IMPF_1_SG
        Before, // SUBJ_IMPF_1_PL
        Before, // SUBJ_IMPF_2_SG
        Before, // SUBJ_IMPF_2_PL
        Before, // SUBJ_IMPF_3_SG
        Before, // SUBJ_IMPF_3_PL
        Before, // IMPERATIVE_TU
        Before, // IMPERATIVE_LEI
        Before, // IMPERATIVE_NOI
        Before, // IMPERATIVE_VOI
        Before, // IMPERATIVE_LORO
        Before, // NEGATIVE_IMPERATIVE_TU
        Before, // NEGATIVE_IMPERATIVE_LEI
        Before, // NEGATIVE_IMPERATIVE_NOI
        Before, // NEGATIVE_IMPERATIVE_VOI
        Before, // NEGATIVE_IMPERATIVE_LORO
        Before, // INFINITIVE
        Before, // GERUND
        Before, // PARTICIPLE_PRES
        Before, // PARTICIPLE_PAST
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // PRES_1_SG
        Cow::Borrowed(""), // PRES_1_PL
        Cow::Borrowed(""), // PRES_2_SG
        Cow::Borrowed(""), // PRES_2_PL
        Cow::Borrowed(""), // PRES_3_SG
        Cow::Borrowed(""), // PRES_3_PL
        Cow::Borrowed(""), // IMPF_1_SG
        Cow::Borrowed(""), // IMPF_1_PL
        Cow::Borrowed(""), // IMPF_2_SG
        Cow::Borrowed(""), // IMPF_2_PL
        Cow::Borrowed(""), // IMPF_3_SG
        Cow::Borrowed(""), // IMPF_3_PL
        Cow::Borrowed(""), // PAST_1_SG
        Cow::Borrowed(""), // PAST_1_PL
        Cow::Borrowed(""), // PAST_2_SG
        Cow::Borrowed(""), // PAST_2_PL
        Cow::Borrowed(""), // PAST_3_SG
        Cow::Borrowed(""), // PAST_3_PL
        Cow::Borrowed(""), // FUT_1_SG
        Cow::Borrowed(""), // FUT_1_PL
        Cow::Borrowed(""), // FUT_2_SG
        Cow::Borrowed(""), // FUT_2_PL
        Cow::Borrowed(""), // FUT_3_SG
        Cow::Borrowed(""), // FUT_3_PL
        Cow::Borrowed(""), // COND_1_SG
        Cow::Borrowed(""), // COND_1_PL
        Cow::Borrowed(""), // COND_2_SG
        Cow::Borrowed(""), // COND_2_PL
        Cow::Borrowed(""), // COND_3_SG
        Cow::Borrowed(""), // COND_3_PL
        Cow::Borrowed(""), // SUBJ_1_SG
        Cow::Borrowed(""), // SUBJ_1_PL
        Cow::Borrowed(""), // SUBJ_2_SG
        Cow::Borrowed(""), // SUBJ_2_PL
        Cow::Borrowed(""), // SUBJ_3_SG
        Cow::Borrowed(""), // SUBJ_3_PL
        Cow::Borrowed(""), // SUBJ_IMPF_1_SG
        Cow::Borrowed(""), // SUBJ_IMPF_1_PL
        Cow::Borrowed(""), // SUBJ_IMPF_2_SG
        Cow::Borrowed(""), // SUBJ_IMPF_2_PL
        Cow::Borrowed(""), // SUBJ_IMPF_3_SG
        Cow::Borrowed(""), // SUBJ_IMPF_3_PL
        Cow::Borrowed(""), // IMPERATIVE_TU
        Cow::Borrowed(""), // IMPERATIVE_LEI
        Cow::Borrowed(""), // IMPERATIVE_NOI
        Cow::Borrowed(""), // IMPERATIVE_VOI
        Cow::Borrowed(""), // IMPERATIVE_LORO
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_TU
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_LEI
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_NOI
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_VOI
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_LORO
        Cow::Borrowed(""), // INFINITIVE
        Cow::Borrowed(""), // GERUND
        Cow::Borrowed(""), // PARTICIPLE_PRES
        Cow::Borrowed(""), // PARTICIPLE_PAST
    ]),
};
