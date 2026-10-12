//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 102 prefixes and 110 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("ab"),
            separable: 0,
            inseparable: 261,
            words: 263,
        },
        Prefix {
            letters: Cow::Borrowed("abolē"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("abs"),
            separable: 0,
            inseparable: 87,
            words: 88,
        },
        Prefix {
            letters: Cow::Borrowed("ac"),
            separable: 0,
            inseparable: 331,
            words: 331,
        },
        Prefix {
            letters: Cow::Borrowed("ad"),
            separable: 0,
            inseparable: 1007,
            words: 1013,
        },
        Prefix {
            letters: Cow::Borrowed("af"),
            separable: 0,
            inseparable: 83,
            words: 84,
        },
        Prefix {
            letters: Cow::Borrowed("ag"),
            separable: 0,
            inseparable: 74,
            words: 75,
        },
        Prefix {
            letters: Cow::Borrowed("al"),
            separable: 0,
            inseparable: 229,
            words: 231,
        },
        Prefix {
            letters: Cow::Borrowed("ali"),
            separable: 0,
            inseparable: 20,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("am"),
            separable: 0,
            inseparable: 169,
            words: 171,
        },
        Prefix {
            letters: Cow::Borrowed("an"),
            separable: 0,
            inseparable: 200,
            words: 200,
        },
        Prefix {
            letters: Cow::Borrowed("anim"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("ante"),
            separable: 0,
            inseparable: 58,
            words: 58,
        },
        Prefix {
            letters: Cow::Borrowed("ap"),
            separable: 0,
            inseparable: 189,
            words: 189,
        },
        Prefix {
            letters: Cow::Borrowed("aps"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("ar"),
            separable: 0,
            inseparable: 237,
            words: 238,
        },
        Prefix {
            letters: Cow::Borrowed("as"),
            separable: 0,
            inseparable: 232,
            words: 234,
        },
        Prefix {
            letters: Cow::Borrowed("at"),
            separable: 0,
            inseparable: 124,
            words: 124,
        },
        Prefix {
            letters: Cow::Borrowed("au"),
            separable: 0,
            inseparable: 165,
            words: 165,
        },
        Prefix {
            letters: Cow::Borrowed("bene"),
            separable: 0,
            inseparable: 24,
            words: 25,
        },
        Prefix {
            letters: Cow::Borrowed("bi"),
            separable: 0,
            inseparable: 107,
            words: 111,
        },
        Prefix {
            letters: Cow::Borrowed("ca"),
            separable: 0,
            inseparable: 527,
            words: 528,
        },
        Prefix {
            letters: Cow::Borrowed("circum"),
            separable: 0,
            inseparable: 262,
            words: 263,
        },
        Prefix {
            letters: Cow::Borrowed("co"),
            separable: 0,
            inseparable: 226,
            words: 230,
        },
        Prefix {
            letters: Cow::Borrowed("col"),
            separable: 0,
            inseparable: 104,
            words: 104,
        },
        Prefix {
            letters: Cow::Borrowed("com"),
            separable: 0,
            inseparable: 360,
            words: 364,
        },
        Prefix {
            letters: Cow::Borrowed("con"),
            separable: 0,
            inseparable: 1208,
            words: 1212,
        },
        Prefix {
            letters: Cow::Borrowed("cor"),
            separable: 0,
            inseparable: 101,
            words: 103,
        },
        Prefix {
            letters: Cow::Borrowed("cō"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("cōn"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("cōns"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("de"),
            separable: 0,
            inseparable: 1072,
            words: 1078,
        },
        Prefix {
            letters: Cow::Borrowed("di"),
            separable: 0,
            inseparable: 283,
            words: 286,
        },
        Prefix {
            letters: Cow::Borrowed("dif"),
            separable: 0,
            inseparable: 38,
            words: 38,
        },
        Prefix {
            letters: Cow::Borrowed("dis"),
            separable: 0,
            inseparable: 312,
            words: 314,
        },
        Prefix {
            letters: Cow::Borrowed("dē"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("dī"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ef"),
            separable: 0,
            inseparable: 107,
            words: 107,
        },
        Prefix {
            letters: Cow::Borrowed("ex"),
            separable: 0,
            inseparable: 675,
            words: 676,
        },
        Prefix {
            letters: Cow::Borrowed("exs"),
            separable: 0,
            inseparable: 133,
            words: 133,
        },
        Prefix {
            letters: Cow::Borrowed("fr"),
            separable: 0,
            inseparable: 163,
            words: 163,
        },
        Prefix {
            letters: Cow::Borrowed("ge"),
            separable: 0,
            inseparable: 85,
            words: 85,
        },
        Prefix {
            letters: Cow::Borrowed("gr"),
            separable: 0,
            inseparable: 108,
            words: 109,
        },
        Prefix {
            letters: Cow::Borrowed("il"),
            separable: 0,
            inseparable: 63,
            words: 64,
        },
        Prefix {
            letters: Cow::Borrowed("im"),
            separable: 0,
            inseparable: 302,
            words: 304,
        },
        Prefix {
            letters: Cow::Borrowed("in"),
            separable: 0,
            inseparable: 1590,
            words: 1600,
        },
        Prefix {
            letters: Cow::Borrowed("inter"),
            separable: 0,
            inseparable: 204,
            words: 206,
        },
        Prefix {
            letters: Cow::Borrowed("intrō"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ir"),
            separable: 0,
            inseparable: 72,
            words: 73,
        },
        Prefix {
            letters: Cow::Borrowed("la"),
            separable: 0,
            inseparable: 260,
            words: 263,
        },
        Prefix {
            letters: Cow::Borrowed("labe"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("male"),
            separable: 0,
            inseparable: 17,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("me"),
            separable: 0,
            inseparable: 212,
            words: 214,
        },
        Prefix {
            letters: Cow::Borrowed("mi"),
            separable: 0,
            inseparable: 134,
            words: 135,
        },
        Prefix {
            letters: Cow::Borrowed("multi"),
            separable: 0,
            inseparable: 28,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("ne"),
            separable: 0,
            inseparable: 94,
            words: 94,
        },
        Prefix {
            letters: Cow::Borrowed("ob"),
            separable: 0,
            inseparable: 401,
            words: 402,
        },
        Prefix {
            letters: Cow::Borrowed("oc"),
            separable: 0,
            inseparable: 84,
            words: 84,
        },
        Prefix {
            letters: Cow::Borrowed("of"),
            separable: 0,
            inseparable: 35,
            words: 35,
        },
        Prefix {
            letters: Cow::Borrowed("op"),
            separable: 0,
            inseparable: 125,
            words: 125,
        },
        Prefix {
            letters: Cow::Borrowed("os"),
            separable: 0,
            inseparable: 33,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("pate"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("per"),
            separable: 0,
            inseparable: 724,
            words: 731,
        },
        Prefix {
            letters: Cow::Borrowed("pl"),
            separable: 0,
            inseparable: 123,
            words: 124,
        },
        Prefix {
            letters: Cow::Borrowed("pol"),
            separable: 0,
            inseparable: 51,
            words: 51,
        },
        Prefix {
            letters: Cow::Borrowed("por"),
            separable: 0,
            inseparable: 29,
            words: 29,
        },
        Prefix {
            letters: Cow::Borrowed("post"),
            separable: 0,
            inseparable: 32,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("pr"),
            separable: 0,
            inseparable: 571,
            words: 576,
        },
        Prefix {
            letters: Cow::Borrowed("prae"),
            separable: 0,
            inseparable: 492,
            words: 496,
        },
        Prefix {
            letters: Cow::Borrowed("praeter"),
            separable: 0,
            inseparable: 36,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("prō"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("quadri"),
            separable: 0,
            inseparable: 36,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("re"),
            separable: 0,
            inseparable: 733,
            words: 734,
        },
        Prefix {
            letters: Cow::Borrowed("red"),
            separable: 0,
            inseparable: 54,
            words: 54,
        },
        Prefix {
            letters: Cow::Borrowed("res"),
            separable: 0,
            inseparable: 134,
            words: 136,
        },
        Prefix {
            letters: Cow::Borrowed("retrō"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("st"),
            separable: 0,
            inseparable: 164,
            words: 165,
        },
        Prefix {
            letters: Cow::Borrowed("su"),
            separable: 0,
            inseparable: 78,
            words: 78,
        },
        Prefix {
            letters: Cow::Borrowed("sub"),
            separable: 0,
            inseparable: 378,
            words: 380,
        },
        Prefix {
            letters: Cow::Borrowed("subter"),
            separable: 0,
            inseparable: 15,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("suc"),
            separable: 0,
            inseparable: 51,
            words: 51,
        },
        Prefix {
            letters: Cow::Borrowed("suf"),
            separable: 0,
            inseparable: 53,
            words: 54,
        },
        Prefix {
            letters: Cow::Borrowed("sum"),
            separable: 0,
            inseparable: 40,
            words: 40,
        },
        Prefix {
            letters: Cow::Borrowed("sup"),
            separable: 0,
            inseparable: 62,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("super"),
            separable: 0,
            inseparable: 190,
            words: 192,
        },
        Prefix {
            letters: Cow::Borrowed("sur"),
            separable: 0,
            inseparable: 34,
            words: 34,
        },
        Prefix {
            letters: Cow::Borrowed("sus"),
            separable: 0,
            inseparable: 50,
            words: 50,
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
            letters: Cow::Borrowed("te"),
            separable: 0,
            inseparable: 133,
            words: 134,
        },
        Prefix {
            letters: Cow::Borrowed("ter"),
            separable: 0,
            inseparable: 55,
            words: 55,
        },
        Prefix {
            letters: Cow::Borrowed("tr"),
            separable: 0,
            inseparable: 285,
            words: 286,
        },
        Prefix {
            letters: Cow::Borrowed("tri"),
            separable: 0,
            inseparable: 116,
            words: 118,
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
            letters: Cow::Borrowed("va"),
            separable: 0,
            inseparable: 105,
            words: 105,
        },
        Prefix {
            letters: Cow::Borrowed("vale"),
            separable: 0,
            inseparable: 16,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("vē"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ēm"),
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
        Prefix {
            letters: Cow::Borrowed("īns"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ūni"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("abiciendus"), Separation::Neither),
        (Cow::Borrowed("aborsus"), Separation::Neither),
        (Cow::Borrowed("absorbitus"), Separation::Neither),
        (Cow::Borrowed("adamicus"), Separation::Neither),
        (Cow::Borrowed("adflictus"), Separation::Neither),
        (Cow::Borrowed("adiciendus"), Separation::Neither),
        (Cow::Borrowed("adsolandus"), Separation::Neither),
        (Cow::Borrowed("adulter"), Separation::Neither),
        (Cow::Borrowed("adustus"), Separation::Neither),
        (Cow::Borrowed("affrictus"), Separation::Neither),
        (Cow::Borrowed("aganiensis"), Separation::Neither),
        (Cow::Borrowed("alacer"), Separation::Neither),
        (Cow::Borrowed("alacrior"), Separation::Neither),
        (Cow::Borrowed("amiciendus"), Separation::Neither),
        (Cow::Borrowed("amplus"), Separation::Neither),
        (Cow::Borrowed("arvernus"), Separation::Neither),
        (Cow::Borrowed("assolandus"), Separation::Neither),
        (Cow::Borrowed("astutus"), Separation::Neither),
        (Cow::Borrowed("benevestitus"), Separation::Neither),
        (Cow::Borrowed("bicinctus"), Separation::Neither),
        (Cow::Borrowed("bifidus"), Separation::Neither),
        (Cow::Borrowed("biiugus"), Separation::Neither),
        (Cow::Borrowed("bisextilis"), Separation::Neither),
        (Cow::Borrowed("caesus"), Separation::Neither),
        (Cow::Borrowed("circumiciendus"), Separation::Neither),
        (Cow::Borrowed("coemendus"), Separation::Neither),
        (Cow::Borrowed("coemens"), Separation::Neither),
        (Cow::Borrowed("coiciendus"), Separation::Neither),
        (Cow::Borrowed("coiens"), Separation::Neither),
        (Cow::Borrowed("commictus"), Separation::Neither),
        (Cow::Borrowed("commolitus"), Separation::Neither),
        (Cow::Borrowed("comparticeps"), Separation::Neither),
        (Cow::Borrowed("componendus"), Separation::Neither),
        (Cow::Borrowed("confertus"), Separation::Neither),
        (Cow::Borrowed("coniciendus"), Separation::Neither),
        (Cow::Borrowed("conparticeps"), Separation::Neither),
        (Cow::Borrowed("conponendus"), Separation::Neither),
        (Cow::Borrowed("corniger"), Separation::Neither),
        (Cow::Borrowed("cornubicus"), Separation::Neither),
        (Cow::Borrowed("defessus"), Separation::Neither),
        (Cow::Borrowed("deiciendus"), Separation::Neither),
        (Cow::Borrowed("deiugis"), Separation::Neither),
        (Cow::Borrowed("delibrandus"), Separation::Neither),
        (Cow::Borrowed("deperitus"), Separation::Neither),
        (Cow::Borrowed("deustus"), Separation::Neither),
        (Cow::Borrowed("dicax"), Separation::Neither),
        (Cow::Borrowed("didius"), Separation::Neither),
        (Cow::Borrowed("disiciendus"), Separation::Neither),
        (Cow::Borrowed("disparendus"), Separation::Neither),
        (Cow::Borrowed("diversicolor"), Separation::Neither),
        (Cow::Borrowed("exustus"), Separation::Neither),
        (Cow::Borrowed("gressus"), Separation::Neither),
        (Cow::Borrowed("illectus"), Separation::Neither),
        (Cow::Borrowed("impacatus"), Separation::Neither),
        (Cow::Borrowed("impedicandus"), Separation::Neither),
        (Cow::Borrowed("indeflexus"), Separation::Neither),
        (Cow::Borrowed("infestus"), Separation::Neither),
        (Cow::Borrowed("iniciendus"), Separation::Neither),
        (Cow::Borrowed("iniugis"), Separation::Neither),
        (Cow::Borrowed("inlectus"), Separation::Neither),
        (Cow::Borrowed("inpar"), Separation::Neither),
        (Cow::Borrowed("insalubris"), Separation::Neither),
        (Cow::Borrowed("insulsus"), Separation::Neither),
        (Cow::Borrowed("insumptuosus"), Separation::Neither),
        (Cow::Borrowed("intericiendus"), Separation::Neither),
        (Cow::Borrowed("interior"), Separation::Neither),
        (Cow::Borrowed("inustus"), Separation::Neither),
        (Cow::Borrowed("irrisibilis"), Separation::Neither),
        (Cow::Borrowed("lacerandus"), Separation::Neither),
        (Cow::Borrowed("laesus"), Separation::Neither),
        (Cow::Borrowed("laniger"), Separation::Neither),
        (Cow::Borrowed("meditandus"), Separation::Neither),
        (Cow::Borrowed("medius"), Separation::Neither),
        (Cow::Borrowed("microlepis"), Separation::Neither),
        (Cow::Borrowed("multifidus"), Separation::Neither),
        (Cow::Borrowed("multiplus"), Separation::Neither),
        (Cow::Borrowed("obiciendus"), Separation::Neither),
        (Cow::Borrowed("perceler"), Separation::Neither),
        (Cow::Borrowed("perditior"), Separation::Neither),
        (Cow::Borrowed("perditissimus"), Separation::Neither),
        (Cow::Borrowed("perfidus"), Separation::Neither),
        (Cow::Borrowed("perlucidus"), Separation::Neither),
        (Cow::Borrowed("perustus"), Separation::Neither),
        (Cow::Borrowed("perusus"), Separation::Neither),
        (Cow::Borrowed("plexilis"), Separation::Neither),
        (Cow::Borrowed("praedestinandus"), Separation::Neither),
        (Cow::Borrowed("praefiniendus"), Separation::Neither),
        (Cow::Borrowed("praeiciendus"), Separation::Neither),
        (Cow::Borrowed("praeustus"), Separation::Neither),
        (Cow::Borrowed("pressus"), Separation::Neither),
        (Cow::Borrowed("propinandus"), Separation::Neither),
        (Cow::Borrowed("propinans"), Separation::Neither),
        (Cow::Borrowed("propinaturus"), Separation::Neither),
        (Cow::Borrowed("propinatus"), Separation::Neither),
        (Cow::Borrowed("quadrifidus"), Separation::Neither),
        (Cow::Borrowed("quadriiugis"), Separation::Neither),
        (Cow::Borrowed("quadriiugus"), Separation::Neither),
        (Cow::Borrowed("reiciendus"), Separation::Neither),
        (Cow::Borrowed("resessus"), Separation::Neither),
        (Cow::Borrowed("residuus"), Separation::Neither),
        (Cow::Borrowed("sterilis"), Separation::Neither),
        (Cow::Borrowed("subiciendus"), Separation::Neither),
        (Cow::Borrowed("subustus"), Separation::Neither),
        (Cow::Borrowed("suffiendus"), Separation::Neither),
        (Cow::Borrowed("superarcanus"), Separation::Neither),
        (Cow::Borrowed("superior"), Separation::Neither),
        (Cow::Borrowed("tectus"), Separation::Neither),
        (Cow::Borrowed("tractus"), Separation::Neither),
        (Cow::Borrowed("trifidus"), Separation::Neither),
        (Cow::Borrowed("tripoliticus"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // NOM_M_SG
        Before, // NOM_M_PL
        Before, // NOM_F_SG
        Before, // NOM_F_PL
        Before, // NOM_N_SG
        Before, // NOM_N_PL
        Before, // GEN_M_SG
        Before, // GEN_M_PL
        Before, // GEN_F_SG
        Before, // GEN_F_PL
        Before, // GEN_N_SG
        Before, // GEN_N_PL
        Before, // DAT_M_SG
        Before, // DAT_M_PL
        Before, // DAT_F_SG
        Before, // DAT_F_PL
        Before, // DAT_N_SG
        Before, // DAT_N_PL
        Before, // ACC_M_SG
        Before, // ACC_M_PL
        Before, // ACC_F_SG
        Before, // ACC_F_PL
        Before, // ACC_N_SG
        Before, // ACC_N_PL
        Before, // ABL_M_SG
        Before, // ABL_M_PL
        Before, // ABL_F_SG
        Before, // ABL_F_PL
        Before, // ABL_N_SG
        Before, // ABL_N_PL
        Before, // VOC_M_SG
        Before, // VOC_M_PL
        Before, // VOC_F_SG
        Before, // VOC_F_PL
        Before, // VOC_N_SG
        Before, // VOC_N_PL
        Before, // COMPARATIVE
        Before, // SUPERLATIVE
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // NOM_M_SG
        Cow::Borrowed(""), // NOM_M_PL
        Cow::Borrowed(""), // NOM_F_SG
        Cow::Borrowed(""), // NOM_F_PL
        Cow::Borrowed(""), // NOM_N_SG
        Cow::Borrowed(""), // NOM_N_PL
        Cow::Borrowed(""), // GEN_M_SG
        Cow::Borrowed(""), // GEN_M_PL
        Cow::Borrowed(""), // GEN_F_SG
        Cow::Borrowed(""), // GEN_F_PL
        Cow::Borrowed(""), // GEN_N_SG
        Cow::Borrowed(""), // GEN_N_PL
        Cow::Borrowed(""), // DAT_M_SG
        Cow::Borrowed(""), // DAT_M_PL
        Cow::Borrowed(""), // DAT_F_SG
        Cow::Borrowed(""), // DAT_F_PL
        Cow::Borrowed(""), // DAT_N_SG
        Cow::Borrowed(""), // DAT_N_PL
        Cow::Borrowed(""), // ACC_M_SG
        Cow::Borrowed(""), // ACC_M_PL
        Cow::Borrowed(""), // ACC_F_SG
        Cow::Borrowed(""), // ACC_F_PL
        Cow::Borrowed(""), // ACC_N_SG
        Cow::Borrowed(""), // ACC_N_PL
        Cow::Borrowed(""), // ABL_M_SG
        Cow::Borrowed(""), // ABL_M_PL
        Cow::Borrowed(""), // ABL_F_SG
        Cow::Borrowed(""), // ABL_F_PL
        Cow::Borrowed(""), // ABL_N_SG
        Cow::Borrowed(""), // ABL_N_PL
        Cow::Borrowed(""), // VOC_M_SG
        Cow::Borrowed(""), // VOC_M_PL
        Cow::Borrowed(""), // VOC_F_SG
        Cow::Borrowed(""), // VOC_F_PL
        Cow::Borrowed(""), // VOC_N_SG
        Cow::Borrowed(""), // VOC_N_PL
        Cow::Borrowed(""), // COMPARATIVE
        Cow::Borrowed(""), // SUPERLATIVE
    ]),
};
