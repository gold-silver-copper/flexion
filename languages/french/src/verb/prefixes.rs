//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 107 prefixes and 51 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("ab"),
            separable: 0,
            inseparable: 58,
            words: 58,
        },
        Prefix {
            letters: Cow::Borrowed("abs"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("ac"),
            separable: 0,
            inseparable: 66,
            words: 68,
        },
        Prefix {
            letters: Cow::Borrowed("ad"),
            separable: 0,
            inseparable: 28,
            words: 28,
        },
        Prefix {
            letters: Cow::Borrowed("af"),
            separable: 0,
            inseparable: 45,
            words: 45,
        },
        Prefix {
            letters: Cow::Borrowed("ag"),
            separable: 0,
            inseparable: 23,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("al"),
            separable: 0,
            inseparable: 42,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("an"),
            separable: 0,
            inseparable: 39,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("ap"),
            separable: 0,
            inseparable: 51,
            words: 52,
        },
        Prefix {
            letters: Cow::Borrowed("ar"),
            separable: 0,
            inseparable: 46,
            words: 46,
        },
        Prefix {
            letters: Cow::Borrowed("as"),
            separable: 0,
            inseparable: 44,
            words: 47,
        },
        Prefix {
            letters: Cow::Borrowed("at"),
            separable: 0,
            inseparable: 31,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("auto"),
            separable: 0,
            inseparable: 24,
            words: 24,
        },
        Prefix {
            letters: Cow::Borrowed("ba"),
            separable: 0,
            inseparable: 83,
            words: 83,
        },
        Prefix {
            letters: Cow::Borrowed("bas"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("bi"),
            separable: 0,
            inseparable: 49,
            words: 49,
        },
        Prefix {
            letters: Cow::Borrowed("bo"),
            separable: 0,
            inseparable: 29,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("bou"),
            separable: 0,
            inseparable: 42,
            words: 42,
        },
        Prefix {
            letters: Cow::Borrowed("br"),
            separable: 0,
            inseparable: 61,
            words: 61,
        },
        Prefix {
            letters: Cow::Borrowed("ca"),
            separable: 0,
            inseparable: 101,
            words: 101,
        },
        Prefix {
            letters: Cow::Borrowed("car"),
            separable: 0,
            inseparable: 26,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("ch"),
            separable: 0,
            inseparable: 57,
            words: 57,
        },
        Prefix {
            letters: Cow::Borrowed("cha"),
            separable: 0,
            inseparable: 47,
            words: 47,
        },
        Prefix {
            letters: Cow::Borrowed("cir"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("co"),
            separable: 0,
            inseparable: 100,
            words: 100,
        },
        Prefix {
            letters: Cow::Borrowed("com"),
            separable: 0,
            inseparable: 57,
            words: 57,
        },
        Prefix {
            letters: Cow::Borrowed("con"),
            separable: 0,
            inseparable: 91,
            words: 93,
        },
        Prefix {
            letters: Cow::Borrowed("cont"),
            separable: 0,
            inseparable: 35,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("cor"),
            separable: 0,
            inseparable: 20,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("de"),
            separable: 0,
            inseparable: 12,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("des"),
            separable: 0,
            inseparable: 23,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("di"),
            separable: 0,
            inseparable: 36,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("dif"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("dis"),
            separable: 0,
            inseparable: 49,
            words: 49,
        },
        Prefix {
            letters: Cow::Borrowed("dé"),
            separable: 0,
            inseparable: 436,
            words: 440,
        },
        Prefix {
            letters: Cow::Borrowed("déc"),
            separable: 0,
            inseparable: 140,
            words: 143,
        },
        Prefix {
            letters: Cow::Borrowed("dég"),
            separable: 0,
            inseparable: 50,
            words: 50,
        },
        Prefix {
            letters: Cow::Borrowed("dés"),
            separable: 0,
            inseparable: 159,
            words: 160,
        },
        Prefix {
            letters: Cow::Borrowed("ef"),
            separable: 0,
            inseparable: 16,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("em"),
            separable: 0,
            inseparable: 94,
            words: 94,
        },
        Prefix {
            letters: Cow::Borrowed("en"),
            separable: 0,
            inseparable: 191,
            words: 191,
        },
        Prefix {
            letters: Cow::Borrowed("eng"),
            separable: 0,
            inseparable: 24,
            words: 25,
        },
        Prefix {
            letters: Cow::Borrowed("ent"),
            separable: 0,
            inseparable: 22,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("entr"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("entre"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("es"),
            separable: 0,
            inseparable: 47,
            words: 49,
        },
        Prefix {
            letters: Cow::Borrowed("ex"),
            separable: 0,
            inseparable: 82,
            words: 82,
        },
        Prefix {
            letters: Cow::Borrowed("for"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("fr"),
            separable: 0,
            inseparable: 51,
            words: 51,
        },
        Prefix {
            letters: Cow::Borrowed("géo"),
            separable: 0,
            inseparable: 3,
            words: 3,
        },
        Prefix {
            letters: Cow::Borrowed("homo"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("héli"),
            separable: 0,
            inseparable: 3,
            words: 3,
        },
        Prefix {
            letters: Cow::Borrowed("il"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("im"),
            separable: 0,
            inseparable: 32,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("in"),
            separable: 0,
            inseparable: 121,
            words: 121,
        },
        Prefix {
            letters: Cow::Borrowed("inter"),
            separable: 0,
            inseparable: 26,
            words: 28,
        },
        Prefix {
            letters: Cow::Borrowed("ma"),
            separable: 0,
            inseparable: 90,
            words: 90,
        },
        Prefix {
            letters: Cow::Borrowed("mal"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("manu"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("micro"),
            separable: 0,
            inseparable: 3,
            words: 3,
        },
        Prefix {
            letters: Cow::Borrowed("mono"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("mé"),
            separable: 0,
            inseparable: 37,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("més"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("ob"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("op"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("or"),
            separable: 0,
            inseparable: 16,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("pa"),
            separable: 0,
            inseparable: 53,
            words: 55,
        },
        Prefix {
            letters: Cow::Borrowed("par"),
            separable: 0,
            inseparable: 23,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("para"),
            separable: 0,
            inseparable: 16,
            words: 18,
        },
        Prefix {
            letters: Cow::Borrowed("per"),
            separable: 0,
            inseparable: 32,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("photo"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("pi"),
            separable: 0,
            inseparable: 47,
            words: 47,
        },
        Prefix {
            letters: Cow::Borrowed("poly"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("post"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("pour"),
            separable: 0,
            inseparable: 5,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("pr"),
            separable: 0,
            inseparable: 27,
            words: 27,
        },
        Prefix {
            letters: Cow::Borrowed("pro"),
            separable: 0,
            inseparable: 49,
            words: 50,
        },
        Prefix {
            letters: Cow::Borrowed("pré"),
            separable: 0,
            inseparable: 60,
            words: 63,
        },
        Prefix {
            letters: Cow::Borrowed("ra"),
            separable: 0,
            inseparable: 102,
            words: 103,
        },
        Prefix {
            letters: Cow::Borrowed("radio"),
            separable: 0,
            inseparable: 15,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("rat"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("re"),
            separable: 0,
            inseparable: 325,
            words: 327,
        },
        Prefix {
            letters: Cow::Borrowed("rem"),
            separable: 0,
            inseparable: 56,
            words: 56,
        },
        Prefix {
            letters: Cow::Borrowed("ren"),
            separable: 0,
            inseparable: 43,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("res"),
            separable: 0,
            inseparable: 38,
            words: 38,
        },
        Prefix {
            letters: Cow::Borrowed("ri"),
            separable: 0,
            inseparable: 27,
            words: 27,
        },
        Prefix {
            letters: Cow::Borrowed("ré"),
            separable: 0,
            inseparable: 204,
            words: 209,
        },
        Prefix {
            letters: Cow::Borrowed("rétro"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("si"),
            separable: 0,
            inseparable: 23,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("sol"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("sou"),
            separable: 0,
            inseparable: 31,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("su"),
            separable: 0,
            inseparable: 21,
            words: 21,
        },
        Prefix {
            letters: Cow::Borrowed("sub"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("sup"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("sur"),
            separable: 0,
            inseparable: 76,
            words: 77,
        },
        Prefix {
            letters: Cow::Borrowed("sus"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("ta"),
            separable: 0,
            inseparable: 52,
            words: 53,
        },
        Prefix {
            letters: Cow::Borrowed("ti"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("tour"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("tr"),
            separable: 0,
            inseparable: 48,
            words: 48,
        },
        Prefix {
            letters: Cow::Borrowed("tra"),
            separable: 0,
            inseparable: 27,
            words: 27,
        },
        Prefix {
            letters: Cow::Borrowed("trans"),
            separable: 0,
            inseparable: 33,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("tré"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("télé"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("ver"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("éc"),
            separable: 0,
            inseparable: 62,
            words: 64,
        },
        Prefix {
            letters: Cow::Borrowed("ét"),
            separable: 0,
            inseparable: 40,
            words: 40,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("accroitre"), Separation::Neither),
        (Cow::Borrowed("accroître"), Separation::Neither),
        (Cow::Borrowed("aller"), Separation::Neither),
        (Cow::Borrowed("appeler"), Separation::Neither),
        (Cow::Borrowed("asseoir"), Separation::Neither),
        (Cow::Borrowed("asservir"), Separation::Neither),
        (Cow::Borrowed("assortir"), Separation::Neither),
        (Cow::Borrowed("boire"), Separation::Neither),
        (Cow::Borrowed("connaitre"), Separation::Neither),
        (Cow::Borrowed("connaître"), Separation::Neither),
        (Cow::Borrowed("contredire"), Separation::Neither),
        (Cow::Borrowed("devoir"), Separation::Neither),
        (Cow::Borrowed("débecqueter"), Separation::Neither),
        (Cow::Borrowed("décacheter"), Separation::Neither),
        (Cow::Borrowed("décevoir"), Separation::Neither),
        (Cow::Borrowed("décrire"), Separation::Neither),
        (Cow::Borrowed("dédire"), Separation::Neither),
        (Cow::Borrowed("défaillir"), Separation::Neither),
        (Cow::Borrowed("déficeler"), Separation::Neither),
        (Cow::Borrowed("désensorceler"), Separation::Neither),
        (Cow::Borrowed("engaller"), Separation::Neither),
        (Cow::Borrowed("esposer"), Separation::Neither),
        (Cow::Borrowed("estre"), Separation::Neither),
        (Cow::Borrowed("impartir"), Separation::Neither),
        (Cow::Borrowed("interdire"), Separation::Neither),
        (Cow::Borrowed("interpeler"), Separation::Neither),
        (Cow::Borrowed("médire"), Separation::Neither),
        (Cow::Borrowed("métrer"), Separation::Neither),
        (Cow::Borrowed("paitre"), Separation::Neither),
        (Cow::Borrowed("paraitre"), Separation::Neither),
        (Cow::Borrowed("paraître"), Separation::Neither),
        (Cow::Borrowed("paître"), Separation::Neither),
        (Cow::Borrowed("pourvoir"), Separation::Neither),
        (Cow::Borrowed("promouvoir"), Separation::Neither),
        (Cow::Borrowed("prédire"), Separation::Neither),
        (Cow::Borrowed("prévaloir"), Separation::Neither),
        (Cow::Borrowed("prévoir"), Separation::Neither),
        (Cow::Borrowed("ravoir"), Separation::Neither),
        (Cow::Borrowed("recroitre"), Separation::Neither),
        (Cow::Borrowed("redevoir"), Separation::Neither),
        (Cow::Borrowed("régler"), Separation::Neither),
        (Cow::Borrowed("régner"), Separation::Neither),
        (Cow::Borrowed("répartir"), Separation::Neither),
        (Cow::Borrowed("résoudre"), Separation::Neither),
        (Cow::Borrowed("réépeler"), Separation::Neither),
        (Cow::Borrowed("soudre"), Separation::Neither),
        (Cow::Borrowed("souloir"), Separation::Neither),
        (Cow::Borrowed("surseoir"), Separation::Neither),
        (Cow::Borrowed("taire"), Separation::Neither),
        (Cow::Borrowed("écher"), Separation::Neither),
        (Cow::Borrowed("écrire"), Separation::Neither),
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
        Before, // IMPERATIVE_NOUS
        Before, // IMPERATIVE_VOUS
        Before, // INFINITIVE
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
        Cow::Borrowed(""), // IMPERATIVE_NOUS
        Cow::Borrowed(""), // IMPERATIVE_VOUS
        Cow::Borrowed(""), // INFINITIVE
        Cow::Borrowed(""), // PARTICIPLE_PRES
        Cow::Borrowed(""), // PARTICIPLE_PAST
    ]),
};
