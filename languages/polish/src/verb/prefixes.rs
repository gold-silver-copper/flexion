//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 69 prefixes and 303 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("ab"),
            separable: 0,
            inseparable: 20,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("ad"),
            separable: 0,
            inseparable: 20,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("ak"),
            separable: 0,
            inseparable: 17,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("ba"),
            separable: 0,
            inseparable: 53,
            words: 58,
        },
        Prefix {
            letters: Cow::Borrowed("bi"),
            separable: 0,
            inseparable: 36,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("ch"),
            separable: 0,
            inseparable: 105,
            words: 107,
        },
        Prefix {
            letters: Cow::Borrowed("ci"),
            separable: 0,
            inseparable: 35,
            words: 37,
        },
        Prefix {
            letters: Cow::Borrowed("cy"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("de"),
            separable: 0,
            inseparable: 74,
            words: 74,
        },
        Prefix {
            letters: Cow::Borrowed("do"),
            separable: 0,
            inseparable: 296,
            words: 308,
        },
        Prefix {
            letters: Cow::Borrowed("in"),
            separable: 0,
            inseparable: 43,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("ka"),
            separable: 0,
            inseparable: 63,
            words: 65,
        },
        Prefix {
            letters: Cow::Borrowed("ko"),
            separable: 0,
            inseparable: 78,
            words: 80,
        },
        Prefix {
            letters: Cow::Borrowed("kon"),
            separable: 0,
            inseparable: 39,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("la"),
            separable: 0,
            inseparable: 18,
            words: 18,
        },
        Prefix {
            letters: Cow::Borrowed("na"),
            separable: 0,
            inseparable: 357,
            words: 365,
        },
        Prefix {
            letters: Cow::Borrowed("nad"),
            separable: 0,
            inseparable: 68,
            words: 71,
        },
        Prefix {
            letters: Cow::Borrowed("ob"),
            separable: 0,
            inseparable: 216,
            words: 223,
        },
        Prefix {
            letters: Cow::Borrowed("obe"),
            separable: 0,
            inseparable: 9,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("od"),
            separable: 0,
            inseparable: 392,
            words: 413,
        },
        Prefix {
            letters: Cow::Borrowed("ode"),
            separable: 0,
            inseparable: 12,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("ok"),
            separable: 0,
            inseparable: 55,
            words: 56,
        },
        Prefix {
            letters: Cow::Borrowed("op"),
            separable: 0,
            inseparable: 105,
            words: 105,
        },
        Prefix {
            letters: Cow::Borrowed("oś"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("po"),
            separable: 0,
            inseparable: 584,
            words: 606,
        },
        Prefix {
            letters: Cow::Borrowed("pod"),
            separable: 0,
            inseparable: 229,
            words: 239,
        },
        Prefix {
            letters: Cow::Borrowed("pode"),
            separable: 0,
            inseparable: 11,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("pow"),
            separable: 0,
            inseparable: 61,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("pr"),
            separable: 0,
            inseparable: 25,
            words: 27,
        },
        Prefix {
            letters: Cow::Borrowed("pre"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("pro"),
            separable: 0,
            inseparable: 32,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("prze"),
            separable: 0,
            inseparable: 455,
            words: 471,
        },
        Prefix {
            letters: Cow::Borrowed("przeci"),
            separable: 0,
            inseparable: 14,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("przed"),
            separable: 0,
            inseparable: 30,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("przy"),
            separable: 0,
            inseparable: 339,
            words: 358,
        },
        Prefix {
            letters: Cow::Borrowed("re"),
            separable: 0,
            inseparable: 72,
            words: 73,
        },
        Prefix {
            letters: Cow::Borrowed("ro"),
            separable: 0,
            inseparable: 11,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("roz"),
            separable: 0,
            inseparable: 465,
            words: 481,
        },
        Prefix {
            letters: Cow::Borrowed("roze"),
            separable: 0,
            inseparable: 13,
            words: 21,
        },
        Prefix {
            letters: Cow::Borrowed("samo"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("sk"),
            separable: 0,
            inseparable: 131,
            words: 133,
        },
        Prefix {
            letters: Cow::Borrowed("sko"),
            separable: 0,
            inseparable: 39,
            words: 41,
        },
        Prefix {
            letters: Cow::Borrowed("skon"),
            separable: 0,
            inseparable: 18,
            words: 18,
        },
        Prefix {
            letters: Cow::Borrowed("sp"),
            separable: 0,
            inseparable: 173,
            words: 174,
        },
        Prefix {
            letters: Cow::Borrowed("spo"),
            separable: 0,
            inseparable: 52,
            words: 54,
        },
        Prefix {
            letters: Cow::Borrowed("st"),
            separable: 0,
            inseparable: 125,
            words: 128,
        },
        Prefix {
            letters: Cow::Borrowed("sz"),
            separable: 0,
            inseparable: 111,
            words: 113,
        },
        Prefix {
            letters: Cow::Borrowed("trans"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("ub"),
            separable: 0,
            inseparable: 30,
            words: 31,
        },
        Prefix {
            letters: Cow::Borrowed("uch"),
            separable: 0,
            inseparable: 12,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("ud"),
            separable: 0,
            inseparable: 38,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("up"),
            separable: 0,
            inseparable: 109,
            words: 111,
        },
        Prefix {
            letters: Cow::Borrowed("uś"),
            separable: 0,
            inseparable: 25,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("we"),
            separable: 0,
            inseparable: 20,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("ws"),
            separable: 0,
            inseparable: 50,
            words: 52,
        },
        Prefix {
            letters: Cow::Borrowed("współ"),
            separable: 0,
            inseparable: 15,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("wy"),
            separable: 0,
            inseparable: 794,
            words: 826,
        },
        Prefix {
            letters: Cow::Borrowed("wyp"),
            separable: 0,
            inseparable: 118,
            words: 120,
        },
        Prefix {
            letters: Cow::Borrowed("wz"),
            separable: 0,
            inseparable: 45,
            words: 48,
        },
        Prefix {
            letters: Cow::Borrowed("za"),
            separable: 0,
            inseparable: 1036,
            words: 1074,
        },
        Prefix {
            letters: Cow::Borrowed("zanie"),
            separable: 0,
            inseparable: 10,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("zb"),
            separable: 0,
            inseparable: 74,
            words: 74,
        },
        Prefix {
            letters: Cow::Borrowed("zd"),
            separable: 0,
            inseparable: 58,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("zde"),
            separable: 0,
            inseparable: 48,
            words: 48,
        },
        Prefix {
            letters: Cow::Borrowed("ze"),
            separable: 0,
            inseparable: 70,
            words: 81,
        },
        Prefix {
            letters: Cow::Borrowed("zi"),
            separable: 0,
            inseparable: 26,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("znie"),
            separable: 0,
            inseparable: 22,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("zo"),
            separable: 0,
            inseparable: 24,
            words: 25,
        },
        Prefix {
            letters: Cow::Borrowed("zre"),
            separable: 0,
            inseparable: 33,
            words: 33,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("babrać"), Separation::Neither),
        (Cow::Borrowed("bachować"), Separation::Neither),
        (Cow::Borrowed("badać"), Separation::Neither),
        (Cow::Borrowed("bawić"), Separation::Neither),
        (Cow::Borrowed("bazgrać"), Separation::Neither),
        (Cow::Borrowed("chlać"), Separation::Neither),
        (Cow::Borrowed("chrypnąć"), Separation::Neither),
        (Cow::Borrowed("ciorać"), Separation::Neither),
        (Cow::Borrowed("ciszyć"), Separation::Neither),
        (Cow::Borrowed("dobiec"), Separation::Neither),
        (Cow::Borrowed("dobyć"), Separation::Neither),
        (Cow::Borrowed("dociec"), Separation::Neither),
        (Cow::Borrowed("dociągnąć"), Separation::Neither),
        (Cow::Borrowed("dogasnąć"), Separation::Neither),
        (Cow::Borrowed("dolec"), Separation::Neither),
        (Cow::Borrowed("dopełznąć"), Separation::Neither),
        (Cow::Borrowed("dorozumieć"), Separation::Neither),
        (Cow::Borrowed("dostroić"), Separation::Neither),
        (Cow::Borrowed("dosłodzić"), Separation::Neither),
        (Cow::Borrowed("dosłyszeć"), Separation::Neither),
        (Cow::Borrowed("doznawać"), Separation::Neither),
        (Cow::Borrowed("kalać"), Separation::Neither),
        (Cow::Borrowed("kasłać"), Separation::Neither),
        (Cow::Borrowed("kopić"), Separation::Neither),
        (Cow::Borrowed("kołatać"), Separation::Neither),
        (Cow::Borrowed("nabrzmieć"), Separation::Neither),
        (Cow::Borrowed("nabyć"), Separation::Neither),
        (Cow::Borrowed("naciągnąć"), Separation::Neither),
        (Cow::Borrowed("nadciągnąć"), Separation::Neither),
        (Cow::Borrowed("nadciąć"), Separation::Neither),
        (Cow::Borrowed("nadsypać"), Separation::Neither),
        (Cow::Borrowed("nagryzmolić"), Separation::Neither),
        (Cow::Borrowed("nakierować"), Separation::Neither),
        (Cow::Borrowed("narosnąć"), Separation::Neither),
        (Cow::Borrowed("nawinąć"), Separation::Neither),
        (Cow::Borrowed("nawyknąć"), Separation::Neither),
        (Cow::Borrowed("obciec"), Separation::Neither),
        (Cow::Borrowed("obciągnąć"), Separation::Neither),
        (Cow::Borrowed("obciąć"), Separation::Neither),
        (Cow::Borrowed("obedrzeć"), Separation::Neither),
        (Cow::Borrowed("obejść"), Separation::Neither),
        (Cow::Borrowed("obelżyć"), Separation::Neither),
        (Cow::Borrowed("obeschnąć"), Separation::Neither),
        (Cow::Borrowed("obetrzeć"), Separation::Neither),
        (Cow::Borrowed("obeznawać"), Separation::Neither),
        (Cow::Borrowed("obeżreć"), Separation::Neither),
        (Cow::Borrowed("objąć"), Separation::Neither),
        (Cow::Borrowed("oblec"), Separation::Neither),
        (Cow::Borrowed("obstać"), Separation::Neither),
        (Cow::Borrowed("obstąpić"), Separation::Neither),
        (Cow::Borrowed("odbiec"), Separation::Neither),
        (Cow::Borrowed("odbyć"), Separation::Neither),
        (Cow::Borrowed("odciągnąć"), Separation::Neither),
        (Cow::Borrowed("odciąć"), Separation::Neither),
        (Cow::Borrowed("odczołgać"), Separation::Neither),
        (Cow::Borrowed("odebrać"), Separation::Neither),
        (Cow::Borrowed("odechcieć"), Separation::Neither),
        (Cow::Borrowed("odedrzeć"), Separation::Neither),
        (Cow::Borrowed("odejść"), Separation::Neither),
        (Cow::Borrowed("odeprać"), Separation::Neither),
        (Cow::Borrowed("odeprzeć"), Separation::Neither),
        (Cow::Borrowed("odetrzeć"), Separation::Neither),
        (Cow::Borrowed("odewrzeć"), Separation::Neither),
        (Cow::Borrowed("odgiąć"), Separation::Neither),
        (Cow::Borrowed("odgradzać"), Separation::Neither),
        (Cow::Borrowed("odgrodzić"), Separation::Neither),
        (Cow::Borrowed("odjąć"), Separation::Neither),
        (Cow::Borrowed("odkaszleć"), Separation::Neither),
        (Cow::Borrowed("odkluczyć"), Separation::Neither),
        (Cow::Borrowed("odnaleźć"), Separation::Neither),
        (Cow::Borrowed("odnawiać"), Separation::Neither),
        (Cow::Borrowed("odpełznąć"), Separation::Neither),
        (Cow::Borrowed("odpiąć"), Separation::Neither),
        (Cow::Borrowed("odrodzić"), Separation::Neither),
        (Cow::Borrowed("odrosnąć"), Separation::Neither),
        (Cow::Borrowed("odrzec"), Separation::Neither),
        (Cow::Borrowed("odsłonić"), Separation::Neither),
        (Cow::Borrowed("odwilgnąć"), Separation::Neither),
        (Cow::Borrowed("odwodzić"), Separation::Neither),
        (Cow::Borrowed("okpić"), Separation::Neither),
        (Cow::Borrowed("pobiec"), Separation::Neither),
        (Cow::Borrowed("pobijać"), Separation::Neither),
        (Cow::Borrowed("pobyć"), Separation::Neither),
        (Cow::Borrowed("pociec"), Separation::Neither),
        (Cow::Borrowed("pociągnąć"), Separation::Neither),
        (Cow::Borrowed("podbiec"), Separation::Neither),
        (Cow::Borrowed("podciągnąć"), Separation::Neither),
        (Cow::Borrowed("podciąć"), Separation::Neither),
        (Cow::Borrowed("podebrać"), Separation::Neither),
        (Cow::Borrowed("podedrzeć"), Separation::Neither),
        (Cow::Borrowed("podejść"), Separation::Neither),
        (Cow::Borrowed("podeprzeć"), Separation::Neither),
        (Cow::Borrowed("podeschnąć"), Separation::Neither),
        (Cow::Borrowed("podetrzeć"), Separation::Neither),
        (Cow::Borrowed("podgiąć"), Separation::Neither),
        (Cow::Borrowed("podjąć"), Separation::Neither),
        (Cow::Borrowed("podlec"), Separation::Neither),
        (Cow::Borrowed("podpiąć"), Separation::Neither),
        (Cow::Borrowed("podszczypać"), Separation::Neither),
        (Cow::Borrowed("podwinąć"), Separation::Neither),
        (Cow::Borrowed("podżec"), Separation::Neither),
        (Cow::Borrowed("pofrunąć"), Separation::Neither),
        (Cow::Borrowed("pogrzebać"), Separation::Neither),
        (Cow::Borrowed("pogrześć"), Separation::Neither),
        (Cow::Borrowed("pogłębić"), Separation::Neither),
        (Cow::Borrowed("pojrzeć"), Separation::Neither),
        (Cow::Borrowed("polec"), Separation::Neither),
        (Cow::Borrowed("pomorzyć"), Separation::Neither),
        (Cow::Borrowed("ponawiać"), Separation::Neither),
        (Cow::Borrowed("popalić"), Separation::Neither),
        (Cow::Borrowed("popatrzyć"), Separation::Neither),
        (Cow::Borrowed("poprzysiąc"), Separation::Neither),
        (Cow::Borrowed("popytać"), Separation::Neither),
        (Cow::Borrowed("porodzić"), Separation::Neither),
        (Cow::Borrowed("postać"), Separation::Neither),
        (Cow::Borrowed("potknąć"), Separation::Neither),
        (Cow::Borrowed("powlec"), Separation::Neither),
        (Cow::Borrowed("poznawać"), Separation::Neither),
        (Cow::Borrowed("połazić"), Separation::Neither),
        (Cow::Borrowed("prużyć"), Separation::Neither),
        (Cow::Borrowed("przearfować"), Separation::Neither),
        (Cow::Borrowed("przebiec"), Separation::Neither),
        (Cow::Borrowed("przeboleć"), Separation::Neither),
        (Cow::Borrowed("przebyć"), Separation::Neither),
        (Cow::Borrowed("przebóść"), Separation::Neither),
        (Cow::Borrowed("przemeblować"), Separation::Neither),
        (Cow::Borrowed("przemoknąć"), Separation::Neither),
        (Cow::Borrowed("przenieść"), Separation::Neither),
        (Cow::Borrowed("przepełznąć"), Separation::Neither),
        (Cow::Borrowed("przestygnąć"), Separation::Neither),
        (Cow::Borrowed("przesycać"), Separation::Neither),
        (Cow::Borrowed("przesłonić"), Separation::Neither),
        (Cow::Borrowed("przewidzieć"), Separation::Neither),
        (Cow::Borrowed("przewinąć"), Separation::Neither),
        (Cow::Borrowed("przewodzić"), Separation::Neither),
        (Cow::Borrowed("przeznawać"), Separation::Neither),
        (Cow::Borrowed("przeć"), Separation::Neither),
        (Cow::Borrowed("przybiec"), Separation::Neither),
        (Cow::Borrowed("przybyć"), Separation::Neither),
        (Cow::Borrowed("przyciągnąć"), Separation::Neither),
        (Cow::Borrowed("przygasnąć"), Separation::Neither),
        (Cow::Borrowed("przygodzić"), Separation::Neither),
        (Cow::Borrowed("przyklęknąć"), Separation::Neither),
        (Cow::Borrowed("przylać"), Separation::Neither),
        (Cow::Borrowed("przyoblec"), Separation::Neither),
        (Cow::Borrowed("przyozdobić"), Separation::Neither),
        (Cow::Borrowed("przypełznąć"), Separation::Neither),
        (Cow::Borrowed("przyrodzić"), Separation::Neither),
        (Cow::Borrowed("przyrzec"), Separation::Neither),
        (Cow::Borrowed("przystanąć"), Separation::Neither),
        (Cow::Borrowed("przysłonić"), Separation::Neither),
        (Cow::Borrowed("przywodzić"), Separation::Neither),
        (Cow::Borrowed("przywrzeć"), Separation::Neither),
        (Cow::Borrowed("przywyknąć"), Separation::Neither),
        (Cow::Borrowed("przyznawać"), Separation::Neither),
        (Cow::Borrowed("przyśnić"), Separation::Neither),
        (Cow::Borrowed("respić"), Separation::Neither),
        (Cow::Borrowed("robić"), Separation::Neither),
        (Cow::Borrowed("rodzić"), Separation::Neither),
        (Cow::Borrowed("rosnąć"), Separation::Neither),
        (Cow::Borrowed("rozbiec"), Separation::Neither),
        (Cow::Borrowed("rozbroić"), Separation::Neither),
        (Cow::Borrowed("rozciągnąć"), Separation::Neither),
        (Cow::Borrowed("rozciąć"), Separation::Neither),
        (Cow::Borrowed("rozebrać"), Separation::Neither),
        (Cow::Borrowed("rozedrzeć"), Separation::Neither),
        (Cow::Borrowed("rozejść"), Separation::Neither),
        (Cow::Borrowed("rozeprzeć"), Separation::Neither),
        (Cow::Borrowed("rozeschnąć"), Separation::Neither),
        (Cow::Borrowed("rozetrzeć"), Separation::Neither),
        (Cow::Borrowed("rozewrzeć"), Separation::Neither),
        (Cow::Borrowed("rozeznawać"), Separation::Neither),
        (Cow::Borrowed("roziskrzyć"), Separation::Neither),
        (Cow::Borrowed("rozjąć"), Separation::Neither),
        (Cow::Borrowed("rozkwitnąć"), Separation::Neither),
        (Cow::Borrowed("rozlatywać"), Separation::Neither),
        (Cow::Borrowed("rozlec"), Separation::Neither),
        (Cow::Borrowed("rozmnożyć"), Separation::Neither),
        (Cow::Borrowed("rozpiąć"), Separation::Neither),
        (Cow::Borrowed("rozprysnąć"), Separation::Neither),
        (Cow::Borrowed("roztworzyć"), Separation::Neither),
        (Cow::Borrowed("rozweselić"), Separation::Neither),
        (Cow::Borrowed("rozwinąć"), Separation::Neither),
        (Cow::Borrowed("rozwodzić"), Separation::Neither),
        (Cow::Borrowed("skomleć"), Separation::Neither),
        (Cow::Borrowed("skopić"), Separation::Neither),
        (Cow::Borrowed("skulać"), Separation::Neither),
        (Cow::Borrowed("skupić"), Separation::Neither),
        (Cow::Borrowed("spiąć"), Separation::Neither),
        (Cow::Borrowed("spojrzeć"), Separation::Neither),
        (Cow::Borrowed("sporzyć"), Separation::Neither),
        (Cow::Borrowed("stopić"), Separation::Neither),
        (Cow::Borrowed("strzec"), Separation::Neither),
        (Cow::Borrowed("stulać"), Separation::Neither),
        (Cow::Borrowed("szumieć"), Separation::Neither),
        (Cow::Borrowed("szustać"), Separation::Neither),
        (Cow::Borrowed("ubożyć"), Separation::Neither),
        (Cow::Borrowed("uchlać"), Separation::Neither),
        (Cow::Borrowed("uchwalać"), Separation::Neither),
        (Cow::Borrowed("udupić"), Separation::Neither),
        (Cow::Borrowed("uporać"), Separation::Neither),
        (Cow::Borrowed("upupić"), Separation::Neither),
        (Cow::Borrowed("uśpić"), Separation::Neither),
        (Cow::Borrowed("wedrzeć"), Separation::Neither),
        (Cow::Borrowed("wejść"), Separation::Neither),
        (Cow::Borrowed("weprzeć"), Separation::Neither),
        (Cow::Borrowed("wesprzeć"), Separation::Neither),
        (Cow::Borrowed("wetrzeć"), Separation::Neither),
        (Cow::Borrowed("wezbrać"), Separation::Neither),
        (Cow::Borrowed("wspiąć"), Separation::Neither),
        (Cow::Borrowed("wstąpić"), Separation::Neither),
        (Cow::Borrowed("wybawić"), Separation::Neither),
        (Cow::Borrowed("wybiec"), Separation::Neither),
        (Cow::Borrowed("wybyć"), Separation::Neither),
        (Cow::Borrowed("wychłodzić"), Separation::Neither),
        (Cow::Borrowed("wyciec"), Separation::Neither),
        (Cow::Borrowed("wycieknąć"), Separation::Neither),
        (Cow::Borrowed("wyciągnąć"), Separation::Neither),
        (Cow::Borrowed("wygasnąć"), Separation::Neither),
        (Cow::Borrowed("wyiskrzyć"), Separation::Neither),
        (Cow::Borrowed("wyjałowić"), Separation::Neither),
        (Cow::Borrowed("wykupić"), Separation::Neither),
        (Cow::Borrowed("wylęgnąć"), Separation::Neither),
        (Cow::Borrowed("wymięknąć"), Separation::Neither),
        (Cow::Borrowed("wymiętolić"), Separation::Neither),
        (Cow::Borrowed("wymądrzyć"), Separation::Neither),
        (Cow::Borrowed("wynagrodzić"), Separation::Neither),
        (Cow::Borrowed("wynaleźć"), Separation::Neither),
        (Cow::Borrowed("wyniknąć"), Separation::Neither),
        (Cow::Borrowed("wyoblać"), Separation::Neither),
        (Cow::Borrowed("wypomnieć"), Separation::Neither),
        (Cow::Borrowed("wypsnąć"), Separation::Neither),
        (Cow::Borrowed("wyrodzić"), Separation::Neither),
        (Cow::Borrowed("wysamplować"), Separation::Neither),
        (Cow::Borrowed("wyschnąć"), Separation::Neither),
        (Cow::Borrowed("wystać"), Separation::Neither),
        (Cow::Borrowed("wystygnąć"), Separation::Neither),
        (Cow::Borrowed("wyswobodzić"), Separation::Neither),
        (Cow::Borrowed("wyszarpać"), Separation::Neither),
        (Cow::Borrowed("wytworzyć"), Separation::Neither),
        (Cow::Borrowed("wytłuścić"), Separation::Neither),
        (Cow::Borrowed("wywinąć"), Separation::Neither),
        (Cow::Borrowed("wywrzeć"), Separation::Neither),
        (Cow::Borrowed("wyznawać"), Separation::Neither),
        (Cow::Borrowed("wyżłobić"), Separation::Neither),
        (Cow::Borrowed("wzdąć"), Separation::Neither),
        (Cow::Borrowed("wziąć"), Separation::Neither),
        (Cow::Borrowed("wznawiać"), Separation::Neither),
        (Cow::Borrowed("zaadaptować"), Separation::Neither),
        (Cow::Borrowed("zabajerować"), Separation::Neither),
        (Cow::Borrowed("zabiec"), Separation::Neither),
        (Cow::Borrowed("zabyć"), Separation::Neither),
        (Cow::Borrowed("zachcieć"), Separation::Neither),
        (Cow::Borrowed("zaciągnąć"), Separation::Neither),
        (Cow::Borrowed("zafajdać"), Separation::Neither),
        (Cow::Borrowed("zagasnąć"), Separation::Neither),
        (Cow::Borrowed("zagrabić"), Separation::Neither),
        (Cow::Borrowed("zagrodzić"), Separation::Neither),
        (Cow::Borrowed("zagłębić"), Separation::Neither),
        (Cow::Borrowed("zakwitnąć"), Separation::Neither),
        (Cow::Borrowed("zakłopotać"), Separation::Neither),
        (Cow::Borrowed("zalec"), Separation::Neither),
        (Cow::Borrowed("zaląc"), Separation::Neither),
        (Cow::Borrowed("zaniemieć"), Separation::Neither),
        (Cow::Borrowed("zapewnić"), Separation::Neither),
        (Cow::Borrowed("zaplątać"), Separation::Neither),
        (Cow::Borrowed("zaprogramować"), Separation::Neither),
        (Cow::Borrowed("zaprzedać"), Separation::Neither),
        (Cow::Borrowed("zaprzysiąc"), Separation::Neither),
        (Cow::Borrowed("zapędzić"), Separation::Neither),
        (Cow::Borrowed("zapętlić"), Separation::Neither),
        (Cow::Borrowed("zarchaizować"), Separation::Neither),
        (Cow::Borrowed("zarzec"), Separation::Neither),
        (Cow::Borrowed("zaróżowić"), Separation::Neither),
        (Cow::Borrowed("zaschnąć"), Separation::Neither),
        (Cow::Borrowed("zastygnąć"), Separation::Neither),
        (Cow::Borrowed("zaszklić"), Separation::Neither),
        (Cow::Borrowed("zasłonić"), Separation::Neither),
        (Cow::Borrowed("zasłynąć"), Separation::Neither),
        (Cow::Borrowed("zatajać"), Separation::Neither),
        (Cow::Borrowed("zatrudnić"), Separation::Neither),
        (Cow::Borrowed("zawieść"), Separation::Neither),
        (Cow::Borrowed("zawinąć"), Separation::Neither),
        (Cow::Borrowed("zawrzeć"), Separation::Neither),
        (Cow::Borrowed("zawyżyć"), Separation::Neither),
        (Cow::Borrowed("zawziąć"), Separation::Neither),
        (Cow::Borrowed("zaznawać"), Separation::Neither),
        (Cow::Borrowed("zdjąć"), Separation::Neither),
        (Cow::Borrowed("zdobić"), Separation::Neither),
        (Cow::Borrowed("zdogmatyzować"), Separation::Neither),
        (Cow::Borrowed("zdumieć"), Separation::Neither),
        (Cow::Borrowed("zebrać"), Separation::Neither),
        (Cow::Borrowed("zedrzeć"), Separation::Neither),
        (Cow::Borrowed("zejść"), Separation::Neither),
        (Cow::Borrowed("zemleć"), Separation::Neither),
        (Cow::Borrowed("zemrzeć"), Separation::Neither),
        (Cow::Borrowed("zeprzeć"), Separation::Neither),
        (Cow::Borrowed("zeschnąć"), Separation::Neither),
        (Cow::Borrowed("zeszyć"), Separation::Neither),
        (Cow::Borrowed("zetrzeć"), Separation::Neither),
        (Cow::Borrowed("zewrzeć"), Separation::Neither),
        (Cow::Borrowed("zeznawać"), Separation::Neither),
        (Cow::Borrowed("zolić"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // INFINITIVE
        Before, // NON_PAST_1_SG
        Before, // NON_PAST_1_PL
        Before, // NON_PAST_2_SG
        Before, // NON_PAST_2_PL
        Before, // NON_PAST_3_SG
        Before, // NON_PAST_3_PL
        Before, // PAST_1_M
        Before, // PAST_1_F
        Before, // PAST_1_N
        Before, // PAST_1_VIR
        Before, // PAST_1_NVIR
        Before, // PAST_2_M
        Before, // PAST_2_F
        Before, // PAST_2_N
        Before, // PAST_2_VIR
        Before, // PAST_2_NVIR
        Before, // PAST_3_M
        Before, // PAST_3_F
        Before, // PAST_3_N
        Before, // PAST_3_VIR
        Before, // PAST_3_NVIR
        Before, // CONDITIONAL_1_M
        Before, // CONDITIONAL_1_F
        Before, // CONDITIONAL_1_N
        Before, // CONDITIONAL_1_VIR
        Before, // CONDITIONAL_1_NVIR
        Before, // CONDITIONAL_2_M
        Before, // CONDITIONAL_2_F
        Before, // CONDITIONAL_2_N
        Before, // CONDITIONAL_2_VIR
        Before, // CONDITIONAL_2_NVIR
        Before, // CONDITIONAL_3_M
        Before, // CONDITIONAL_3_F
        Before, // CONDITIONAL_3_N
        Before, // CONDITIONAL_3_VIR
        Before, // CONDITIONAL_3_NVIR
        Before, // IMPERATIVE_1_SG
        Before, // IMPERATIVE_1_PL
        Before, // IMPERATIVE_2_SG
        Before, // IMPERATIVE_2_PL
        Before, // IMPERATIVE_3_SG
        Before, // IMPERATIVE_3_PL
        Before, // IMPERSONAL
        Before, // PARTICIPLE_ACT
        Before, // PARTICIPLE_PASS
        Before, // ADVERBIAL_PARTICIPLE_CONTEMP
        Before, // ADVERBIAL_PARTICIPLE_ANT
        Before, // VERBAL_NOUN
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // INFINITIVE
        Cow::Borrowed(""), // NON_PAST_1_SG
        Cow::Borrowed(""), // NON_PAST_1_PL
        Cow::Borrowed(""), // NON_PAST_2_SG
        Cow::Borrowed(""), // NON_PAST_2_PL
        Cow::Borrowed(""), // NON_PAST_3_SG
        Cow::Borrowed(""), // NON_PAST_3_PL
        Cow::Borrowed(""), // PAST_1_M
        Cow::Borrowed(""), // PAST_1_F
        Cow::Borrowed(""), // PAST_1_N
        Cow::Borrowed(""), // PAST_1_VIR
        Cow::Borrowed(""), // PAST_1_NVIR
        Cow::Borrowed(""), // PAST_2_M
        Cow::Borrowed(""), // PAST_2_F
        Cow::Borrowed(""), // PAST_2_N
        Cow::Borrowed(""), // PAST_2_VIR
        Cow::Borrowed(""), // PAST_2_NVIR
        Cow::Borrowed(""), // PAST_3_M
        Cow::Borrowed(""), // PAST_3_F
        Cow::Borrowed(""), // PAST_3_N
        Cow::Borrowed(""), // PAST_3_VIR
        Cow::Borrowed(""), // PAST_3_NVIR
        Cow::Borrowed(""), // CONDITIONAL_1_M
        Cow::Borrowed(""), // CONDITIONAL_1_F
        Cow::Borrowed(""), // CONDITIONAL_1_N
        Cow::Borrowed(""), // CONDITIONAL_1_VIR
        Cow::Borrowed(""), // CONDITIONAL_1_NVIR
        Cow::Borrowed(""), // CONDITIONAL_2_M
        Cow::Borrowed(""), // CONDITIONAL_2_F
        Cow::Borrowed(""), // CONDITIONAL_2_N
        Cow::Borrowed(""), // CONDITIONAL_2_VIR
        Cow::Borrowed(""), // CONDITIONAL_2_NVIR
        Cow::Borrowed(""), // CONDITIONAL_3_M
        Cow::Borrowed(""), // CONDITIONAL_3_F
        Cow::Borrowed(""), // CONDITIONAL_3_N
        Cow::Borrowed(""), // CONDITIONAL_3_VIR
        Cow::Borrowed(""), // CONDITIONAL_3_NVIR
        Cow::Borrowed(""), // IMPERATIVE_1_SG
        Cow::Borrowed(""), // IMPERATIVE_1_PL
        Cow::Borrowed(""), // IMPERATIVE_2_SG
        Cow::Borrowed(""), // IMPERATIVE_2_PL
        Cow::Borrowed(""), // IMPERATIVE_3_SG
        Cow::Borrowed(""), // IMPERATIVE_3_PL
        Cow::Borrowed(""), // IMPERSONAL
        Cow::Borrowed(""), // PARTICIPLE_ACT
        Cow::Borrowed(""), // PARTICIPLE_PASS
        Cow::Borrowed(""), // ADVERBIAL_PARTICIPLE_CONTEMP
        Cow::Borrowed(""), // ADVERBIAL_PARTICIPLE_ANT
        Cow::Borrowed(""), // VERBAL_NOUN
    ]),
};
