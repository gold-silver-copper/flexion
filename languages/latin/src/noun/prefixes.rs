//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 61 prefixes and 109 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("ab"),
            separable: 0,
            inseparable: 131,
            words: 135,
        },
        Prefix {
            letters: Cow::Borrowed("ad"),
            separable: 0,
            inseparable: 176,
            words: 180,
        },
        Prefix {
            letters: Cow::Borrowed("ag"),
            separable: 0,
            inseparable: 47,
            words: 48,
        },
        Prefix {
            letters: Cow::Borrowed("am"),
            separable: 0,
            inseparable: 121,
            words: 121,
        },
        Prefix {
            letters: Cow::Borrowed("an"),
            separable: 0,
            inseparable: 167,
            words: 173,
        },
        Prefix {
            letters: Cow::Borrowed("ante"),
            separable: 0,
            inseparable: 21,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("apo"),
            separable: 0,
            inseparable: 31,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("ar"),
            separable: 0,
            inseparable: 163,
            words: 168,
        },
        Prefix {
            letters: Cow::Borrowed("archi"),
            separable: 0,
            inseparable: 30,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("as"),
            separable: 0,
            inseparable: 113,
            words: 119,
        },
        Prefix {
            letters: Cow::Borrowed("at"),
            separable: 0,
            inseparable: 49,
            words: 50,
        },
        Prefix {
            letters: Cow::Borrowed("au"),
            separable: 0,
            inseparable: 83,
            words: 85,
        },
        Prefix {
            letters: Cow::Borrowed("auto"),
            separable: 0,
            inseparable: 14,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("ba"),
            separable: 0,
            inseparable: 148,
            words: 153,
        },
        Prefix {
            letters: Cow::Borrowed("bene"),
            separable: 0,
            inseparable: 11,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("bi"),
            separable: 0,
            inseparable: 59,
            words: 60,
        },
        Prefix {
            letters: Cow::Borrowed("ca"),
            separable: 0,
            inseparable: 493,
            words: 506,
        },
        Prefix {
            letters: Cow::Borrowed("cap"),
            separable: 0,
            inseparable: 67,
            words: 69,
        },
        Prefix {
            letters: Cow::Borrowed("circum"),
            separable: 0,
            inseparable: 28,
            words: 29,
        },
        Prefix {
            letters: Cow::Borrowed("cl"),
            separable: 0,
            inseparable: 86,
            words: 86,
        },
        Prefix {
            letters: Cow::Borrowed("co"),
            separable: 0,
            inseparable: 241,
            words: 244,
        },
        Prefix {
            letters: Cow::Borrowed("com"),
            separable: 0,
            inseparable: 140,
            words: 141,
        },
        Prefix {
            letters: Cow::Borrowed("con"),
            separable: 0,
            inseparable: 402,
            words: 407,
        },
        Prefix {
            letters: Cow::Borrowed("cor"),
            separable: 0,
            inseparable: 85,
            words: 89,
        },
        Prefix {
            letters: Cow::Borrowed("cōn"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("dia"),
            separable: 0,
            inseparable: 29,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("dis"),
            separable: 0,
            inseparable: 79,
            words: 80,
        },
        Prefix {
            letters: Cow::Borrowed("do"),
            separable: 0,
            inseparable: 57,
            words: 58,
        },
        Prefix {
            letters: Cow::Borrowed("domi"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("dē"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("em"),
            separable: 0,
            inseparable: 38,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("epi"),
            separable: 0,
            inseparable: 35,
            words: 35,
        },
        Prefix {
            letters: Cow::Borrowed("ex"),
            separable: 0,
            inseparable: 195,
            words: 197,
        },
        Prefix {
            letters: Cow::Borrowed("flā"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("hippo"),
            separable: 0,
            inseparable: 8,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("im"),
            separable: 0,
            inseparable: 85,
            words: 86,
        },
        Prefix {
            letters: Cow::Borrowed("in"),
            separable: 0,
            inseparable: 396,
            words: 403,
        },
        Prefix {
            letters: Cow::Borrowed("inter"),
            separable: 0,
            inseparable: 61,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("lā"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("male"),
            separable: 0,
            inseparable: 9,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("me"),
            separable: 0,
            inseparable: 187,
            words: 190,
        },
        Prefix {
            letters: Cow::Borrowed("mon"),
            separable: 0,
            inseparable: 36,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("multi"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("ob"),
            separable: 0,
            inseparable: 75,
            words: 76,
        },
        Prefix {
            letters: Cow::Borrowed("para"),
            separable: 0,
            inseparable: 33,
            words: 35,
        },
        Prefix {
            letters: Cow::Borrowed("per"),
            separable: 0,
            inseparable: 115,
            words: 116,
        },
        Prefix {
            letters: Cow::Borrowed("pl"),
            separable: 0,
            inseparable: 94,
            words: 95,
        },
        Prefix {
            letters: Cow::Borrowed("prae"),
            separable: 0,
            inseparable: 111,
            words: 116,
        },
        Prefix {
            letters: Cow::Borrowed("pro"),
            separable: 0,
            inseparable: 205,
            words: 209,
        },
        Prefix {
            letters: Cow::Borrowed("prō"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("pseudo"),
            separable: 0,
            inseparable: 7,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("re"),
            separable: 0,
            inseparable: 234,
            words: 239,
        },
        Prefix {
            letters: Cow::Borrowed("sub"),
            separable: 0,
            inseparable: 80,
            words: 81,
        },
        Prefix {
            letters: Cow::Borrowed("sug"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("super"),
            separable: 0,
            inseparable: 26,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("sē"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("sēmi"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("tri"),
            separable: 0,
            inseparable: 81,
            words: 81,
        },
        Prefix {
            letters: Cow::Borrowed("trā"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("trāns"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("īn"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("abacus"), Separation::Neither),
        (Cow::Borrowed("abneptis"), Separation::Neither),
        (Cow::Borrowed("abolla"), Separation::Neither),
        (Cow::Borrowed("absus"), Separation::Neither),
        (Cow::Borrowed("addictus"), Separation::Neither),
        (Cow::Borrowed("adsecula"), Separation::Neither),
        (Cow::Borrowed("advena"), Separation::Neither),
        (Cow::Borrowed("adventus"), Separation::Neither),
        (Cow::Borrowed("agora"), Separation::Neither),
        (Cow::Borrowed("ancala"), Separation::Neither),
        (Cow::Borrowed("anchora"), Separation::Neither),
        (Cow::Borrowed("anculus"), Separation::Neither),
        (Cow::Borrowed("angelus"), Separation::Neither),
        (Cow::Borrowed("antebrachium"), Separation::Neither),
        (Cow::Borrowed("anthrax"), Separation::Neither),
        (Cow::Borrowed("anthus"), Separation::Neither),
        (Cow::Borrowed("arbustum"), Separation::Neither),
        (Cow::Borrowed("arcera"), Separation::Neither),
        (Cow::Borrowed("architector"), Separation::Neither),
        (Cow::Borrowed("ardea"), Separation::Neither),
        (Cow::Borrowed("armus"), Separation::Neither),
        (Cow::Borrowed("artus"), Separation::Neither),
        (Cow::Borrowed("ascopa"), Separation::Neither),
        (Cow::Borrowed("asina"), Separation::Neither),
        (Cow::Borrowed("asinusca"), Separation::Neither),
        (Cow::Borrowed("aspectus"), Separation::Neither),
        (Cow::Borrowed("assecula"), Separation::Neither),
        (Cow::Borrowed("astus"), Separation::Neither),
        (Cow::Borrowed("atrusca"), Separation::Neither),
        (Cow::Borrowed("auricula"), Separation::Neither),
        (Cow::Borrowed("ausus"), Separation::Neither),
        (Cow::Borrowed("baculus"), Separation::Neither),
        (Cow::Borrowed("baiulus"), Separation::Neither),
        (Cow::Borrowed("balana"), Separation::Neither),
        (Cow::Borrowed("batalus"), Separation::Neither),
        (Cow::Borrowed("batus"), Separation::Neither),
        (Cow::Borrowed("benedictus"), Separation::Neither),
        (Cow::Borrowed("bilis"), Separation::Neither),
        (Cow::Borrowed("caesor"), Separation::Neither),
        (Cow::Borrowed("camus"), Separation::Neither),
        (Cow::Borrowed("capsus"), Separation::Neither),
        (Cow::Borrowed("captus"), Separation::Neither),
        (Cow::Borrowed("capus"), Separation::Neither),
        (Cow::Borrowed("carectum"), Separation::Neither),
        (Cow::Borrowed("carex"), Separation::Neither),
        (Cow::Borrowed("carica"), Separation::Neither),
        (Cow::Borrowed("caros"), Separation::Neither),
        (Cow::Borrowed("casus"), Separation::Neither),
        (Cow::Borrowed("catellus"), Separation::Neither),
        (Cow::Borrowed("catus"), Separation::Neither),
        (Cow::Borrowed("caurus"), Separation::Neither),
        (Cow::Borrowed("causia"), Separation::Neither),
        (Cow::Borrowed("cavilla"), Separation::Neither),
        (Cow::Borrowed("circumscriptor"), Separation::Neither),
        (Cow::Borrowed("columen"), Separation::Neither),
        (Cow::Borrowed("compotus"), Separation::Neither),
        (Cow::Borrowed("condama"), Separation::Neither),
        (Cow::Borrowed("conductus"), Separation::Neither),
        (Cow::Borrowed("contus"), Separation::Neither),
        (Cow::Borrowed("convena"), Separation::Neither),
        (Cow::Borrowed("conventus"), Separation::Neither),
        (Cow::Borrowed("cordolium"), Separation::Neither),
        (Cow::Borrowed("corpus"), Separation::Neither),
        (Cow::Borrowed("corrus"), Separation::Neither),
        (Cow::Borrowed("corsus"), Separation::Neither),
        (Cow::Borrowed("corus"), Separation::Neither),
        (Cow::Borrowed("cotinus"), Separation::Neither),
        (Cow::Borrowed("diaconus"), Separation::Neither),
        (Cow::Borrowed("dialectica"), Separation::Neither),
        (Cow::Borrowed("disculus"), Separation::Neither),
        (Cow::Borrowed("domus"), Separation::Neither),
        (Cow::Borrowed("emarcus"), Separation::Neither),
        (Cow::Borrowed("excetra"), Separation::Neither),
        (Cow::Borrowed("exilium"), Separation::Neither),
        (Cow::Borrowed("hippomanes"), Separation::Neither),
        (Cow::Borrowed("impes"), Separation::Neither),
        (Cow::Borrowed("incestus"), Separation::Neither),
        (Cow::Borrowed("indium"), Separation::Neither),
        (Cow::Borrowed("inferiae"), Separation::Neither),
        (Cow::Borrowed("inlectus"), Separation::Neither),
        (Cow::Borrowed("inlusor"), Separation::Neither),
        (Cow::Borrowed("inpes"), Separation::Neither),
        (Cow::Borrowed("interventus"), Separation::Neither),
        (Cow::Borrowed("investimentum"), Separation::Neither),
        (Cow::Borrowed("maleficus"), Separation::Neither),
        (Cow::Borrowed("medium"), Separation::Neither),
        (Cow::Borrowed("metellus"), Separation::Neither),
        (Cow::Borrowed("metus"), Separation::Neither),
        (Cow::Borrowed("obolus"), Separation::Neither),
        (Cow::Borrowed("paramus"), Separation::Neither),
        (Cow::Borrowed("paratus"), Separation::Neither),
        (Cow::Borrowed("persica"), Separation::Neither),
        (Cow::Borrowed("planus"), Separation::Neither),
        (Cow::Borrowed("praedium"), Separation::Neither),
        (Cow::Borrowed("praedulcia"), Separation::Neither),
        (Cow::Borrowed("praepes"), Separation::Neither),
        (Cow::Borrowed("praepositus"), Separation::Neither),
        (Cow::Borrowed("praescriptum"), Separation::Neither),
        (Cow::Borrowed("promus"), Separation::Neither),
        (Cow::Borrowed("proneptis"), Separation::Neither),
        (Cow::Borrowed("prothesis"), Separation::Neither),
        (Cow::Borrowed("proventus"), Separation::Neither),
        (Cow::Borrowed("pseudochristus"), Separation::Neither),
        (Cow::Borrowed("rebellis"), Separation::Neither),
        (Cow::Borrowed("remus"), Separation::Neither),
        (Cow::Borrowed("repugnantia"), Separation::Neither),
        (Cow::Borrowed("respublica"), Separation::Neither),
        (Cow::Borrowed("retina"), Separation::Neither),
        (Cow::Borrowed("subulcus"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // NOM_SG
        Before, // NOM_PL
        Before, // GEN_SG
        Before, // GEN_PL
        Before, // DAT_SG
        Before, // DAT_PL
        Before, // ACC_SG
        Before, // ACC_PL
        Before, // ABL_SG
        Before, // ABL_PL
        Before, // VOC_SG
        Before, // VOC_PL
        Before, // NOUN_LOCATIVE_SG
        Before, // NOUN_LOCATIVE_PL
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // NOM_SG
        Cow::Borrowed(""), // NOM_PL
        Cow::Borrowed(""), // GEN_SG
        Cow::Borrowed(""), // GEN_PL
        Cow::Borrowed(""), // DAT_SG
        Cow::Borrowed(""), // DAT_PL
        Cow::Borrowed(""), // ACC_SG
        Cow::Borrowed(""), // ACC_PL
        Cow::Borrowed(""), // ABL_SG
        Cow::Borrowed(""), // ABL_PL
        Cow::Borrowed(""), // VOC_SG
        Cow::Borrowed(""), // VOC_PL
        Cow::Borrowed(""), // NOUN_LOCATIVE_SG
        Cow::Borrowed(""), // NOUN_LOCATIVE_PL
    ]),
};
