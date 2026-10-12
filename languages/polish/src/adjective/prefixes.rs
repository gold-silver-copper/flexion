//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 155 prefixes and 376 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("ab"),
            separable: 0,
            inseparable: 68,
            words: 71,
        },
        Prefix {
            letters: Cow::Borrowed("ad"),
            separable: 0,
            inseparable: 94,
            words: 95,
        },
        Prefix {
            letters: Cow::Borrowed("aero"),
            separable: 0,
            inseparable: 33,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("agro"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("al"),
            separable: 0,
            inseparable: 90,
            words: 90,
        },
        Prefix {
            letters: Cow::Borrowed("an"),
            separable: 0,
            inseparable: 123,
            words: 123,
        },
        Prefix {
            letters: Cow::Borrowed("antropo"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("anty"),
            separable: 0,
            inseparable: 149,
            words: 149,
        },
        Prefix {
            letters: Cow::Borrowed("archi"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("arcy"),
            separable: 0,
            inseparable: 20,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("areo"),
            separable: 0,
            inseparable: 3,
            words: 3,
        },
        Prefix {
            letters: Cow::Borrowed("astro"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("audio"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("auto"),
            separable: 0,
            inseparable: 61,
            words: 61,
        },
        Prefix {
            letters: Cow::Borrowed("ba"),
            separable: 0,
            inseparable: 210,
            words: 215,
        },
        Prefix {
            letters: Cow::Borrowed("be"),
            separable: 0,
            inseparable: 63,
            words: 64,
        },
        Prefix {
            letters: Cow::Borrowed("bez"),
            separable: 0,
            inseparable: 314,
            words: 321,
        },
        Prefix {
            letters: Cow::Borrowed("bi"),
            separable: 0,
            inseparable: 129,
            words: 133,
        },
        Prefix {
            letters: Cow::Borrowed("biało"),
            separable: 0,
            inseparable: 27,
            words: 27,
        },
        Prefix {
            letters: Cow::Borrowed("bio"),
            separable: 0,
            inseparable: 51,
            words: 51,
        },
        Prefix {
            letters: Cow::Borrowed("blado"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("bu"),
            separable: 0,
            inseparable: 38,
            words: 43,
        },
        Prefix {
            letters: Cow::Borrowed("cało"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("ciemno"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("co"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("czterdziesto"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("cztero"),
            separable: 0,
            inseparable: 16,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("do"),
            separable: 0,
            inseparable: 100,
            words: 121,
        },
        Prefix {
            letters: Cow::Borrowed("dobro"),
            separable: 0,
            inseparable: 4,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("dolno"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("dw"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("dwu"),
            separable: 0,
            inseparable: 56,
            words: 56,
        },
        Prefix {
            letters: Cow::Borrowed("dwudziesto"),
            separable: 0,
            inseparable: 15,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("dziesięcio"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("dziewięcio"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("dziewięć"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("długo"),
            separable: 0,
            inseparable: 16,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("elektro"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("ga"),
            separable: 0,
            inseparable: 46,
            words: 46,
        },
        Prefix {
            letters: Cow::Borrowed("geo"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("górno"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("hydro"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("inter"),
            separable: 0,
            inseparable: 14,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("ja"),
            separable: 0,
            inseparable: 60,
            words: 65,
        },
        Prefix {
            letters: Cow::Borrowed("jasno"),
            separable: 0,
            inseparable: 13,
            words: 13,
        },
        Prefix {
            letters: Cow::Borrowed("je"),
            separable: 0,
            inseparable: 38,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("jedno"),
            separable: 0,
            inseparable: 49,
            words: 49,
        },
        Prefix {
            letters: Cow::Borrowed("ka"),
            separable: 0,
            inseparable: 155,
            words: 158,
        },
        Prefix {
            letters: Cow::Borrowed("ko"),
            separable: 0,
            inseparable: 189,
            words: 194,
        },
        Prefix {
            letters: Cow::Borrowed("kom"),
            separable: 0,
            inseparable: 35,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("krótko"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("lekko"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("lewo"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("li"),
            separable: 0,
            inseparable: 61,
            words: 66,
        },
        Prefix {
            letters: Cow::Borrowed("ma"),
            separable: 0,
            inseparable: 137,
            words: 144,
        },
        Prefix {
            letters: Cow::Borrowed("mał"),
            separable: 0,
            inseparable: 1,
            words: 1,
        },
        Prefix {
            letters: Cow::Borrowed("mało"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("między"),
            separable: 0,
            inseparable: 20,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("mo"),
            separable: 0,
            inseparable: 69,
            words: 76,
        },
        Prefix {
            letters: Cow::Borrowed("mono"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("multi"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("męsko"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("na"),
            separable: 0,
            inseparable: 159,
            words: 160,
        },
        Prefix {
            letters: Cow::Borrowed("nad"),
            separable: 0,
            inseparable: 45,
            words: 46,
        },
        Prefix {
            letters: Cow::Borrowed("naj"),
            separable: 0,
            inseparable: 387,
            words: 387,
        },
        Prefix {
            letters: Cow::Borrowed("neo"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("nie"),
            separable: 0,
            inseparable: 408,
            words: 434,
        },
        Prefix {
            letters: Cow::Borrowed("niedo"),
            separable: 0,
            inseparable: 32,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("nieprze"),
            separable: 0,
            inseparable: 30,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("niewy"),
            separable: 0,
            inseparable: 29,
            words: 29,
        },
        Prefix {
            letters: Cow::Borrowed("niez"),
            separable: 0,
            inseparable: 58,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("nisko"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("nowo"),
            separable: 0,
            inseparable: 21,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("ob"),
            separable: 0,
            inseparable: 86,
            words: 95,
        },
        Prefix {
            letters: Cow::Borrowed("obu"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("od"),
            separable: 0,
            inseparable: 90,
            words: 96,
        },
        Prefix {
            letters: Cow::Borrowed("ogólno"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("około"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("osiem"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("ośmio"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("pa"),
            separable: 0,
            inseparable: 128,
            words: 132,
        },
        Prefix {
            letters: Cow::Borrowed("pan"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("pełno"),
            separable: 0,
            inseparable: 15,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("pierwszo"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("pięcio"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("piętnasto"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("pięć"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("po"),
            separable: 0,
            inseparable: 305,
            words: 350,
        },
        Prefix {
            letters: Cow::Borrowed("pod"),
            separable: 0,
            inseparable: 101,
            words: 106,
        },
        Prefix {
            letters: Cow::Borrowed("ponad"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("post"),
            separable: 0,
            inseparable: 24,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("poza"),
            separable: 0,
            inseparable: 17,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("południowo"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("pr"),
            separable: 0,
            inseparable: 34,
            words: 41,
        },
        Prefix {
            letters: Cow::Borrowed("pra"),
            separable: 0,
            inseparable: 26,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("prawo"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("pre"),
            separable: 0,
            inseparable: 17,
            words: 18,
        },
        Prefix {
            letters: Cow::Borrowed("pro"),
            separable: 0,
            inseparable: 68,
            words: 70,
        },
        Prefix {
            letters: Cow::Borrowed("prze"),
            separable: 0,
            inseparable: 132,
            words: 148,
        },
        Prefix {
            letters: Cow::Borrowed("przeciw"),
            separable: 0,
            inseparable: 36,
            words: 36,
        },
        Prefix {
            letters: Cow::Borrowed("przed"),
            separable: 0,
            inseparable: 31,
            words: 33,
        },
        Prefix {
            letters: Cow::Borrowed("przy"),
            separable: 0,
            inseparable: 103,
            words: 115,
        },
        Prefix {
            letters: Cow::Borrowed("psycho"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("pół"),
            separable: 0,
            inseparable: 24,
            words: 24,
        },
        Prefix {
            letters: Cow::Borrowed("północno"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("ro"),
            separable: 0,
            inseparable: 47,
            words: 51,
        },
        Prefix {
            letters: Cow::Borrowed("ropo"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("roz"),
            separable: 0,
            inseparable: 120,
            words: 135,
        },
        Prefix {
            letters: Cow::Borrowed("równo"),
            separable: 0,
            inseparable: 14,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("różno"),
            separable: 0,
            inseparable: 5,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("samo"),
            separable: 0,
            inseparable: 30,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("siedem"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("siedmio"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("sk"),
            separable: 0,
            inseparable: 60,
            words: 69,
        },
        Prefix {
            letters: Cow::Borrowed("skła"),
            separable: 0,
            inseparable: 13,
            words: 14,
        },
        Prefix {
            letters: Cow::Borrowed("so"),
            separable: 0,
            inseparable: 46,
            words: 49,
        },
        Prefix {
            letters: Cow::Borrowed("sp"),
            separable: 0,
            inseparable: 79,
            words: 88,
        },
        Prefix {
            letters: Cow::Borrowed("spa"),
            separable: 0,
            inseparable: 16,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("staro"),
            separable: 0,
            inseparable: 25,
            words: 26,
        },
        Prefix {
            letters: Cow::Borrowed("stu"),
            separable: 0,
            inseparable: 17,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("sub"),
            separable: 0,
            inseparable: 7,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("sz"),
            separable: 0,
            inseparable: 137,
            words: 158,
        },
        Prefix {
            letters: Cow::Borrowed("sześcio"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("sześć"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("to"),
            separable: 0,
            inseparable: 28,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("tra"),
            separable: 0,
            inseparable: 18,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("trans"),
            separable: 0,
            inseparable: 8,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("tro"),
            separable: 0,
            inseparable: 7,
            words: 8,
        },
        Prefix {
            letters: Cow::Borrowed("trzy"),
            separable: 0,
            inseparable: 18,
            words: 18,
        },
        Prefix {
            letters: Cow::Borrowed("trzydziesto"),
            separable: 0,
            inseparable: 12,
            words: 12,
        },
        Prefix {
            letters: Cow::Borrowed("trój"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("wewnątrz"),
            separable: 0,
            inseparable: 10,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("wi"),
            separable: 0,
            inseparable: 95,
            words: 102,
        },
        Prefix {
            letters: Cow::Borrowed("wiel"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("wielko"),
            separable: 0,
            inseparable: 19,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("wielo"),
            separable: 0,
            inseparable: 56,
            words: 56,
        },
        Prefix {
            letters: Cow::Borrowed("wodo"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("wolno"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("wpół"),
            separable: 0,
            inseparable: 5,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("wschodnio"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("współ"),
            separable: 0,
            inseparable: 14,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("wszech"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("wszystko"),
            separable: 0,
            inseparable: 3,
            words: 3,
        },
        Prefix {
            letters: Cow::Borrowed("wy"),
            separable: 0,
            inseparable: 205,
            words: 226,
        },
        Prefix {
            letters: Cow::Borrowed("wysoko"),
            separable: 0,
            inseparable: 11,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("za"),
            separable: 0,
            inseparable: 278,
            words: 308,
        },
        Prefix {
            letters: Cow::Borrowed("zachodnio"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
        Prefix {
            letters: Cow::Borrowed("ze"),
            separable: 0,
            inseparable: 21,
            words: 21,
        },
        Prefix {
            letters: Cow::Borrowed("zewnątrz"),
            separable: 0,
            inseparable: 4,
            words: 4,
        },
        Prefix {
            letters: Cow::Borrowed("zło"),
            separable: 0,
            inseparable: 19,
            words: 22,
        },
        Prefix {
            letters: Cow::Borrowed("ła"),
            separable: 0,
            inseparable: 29,
            words: 38,
        },
        Prefix {
            letters: Cow::Borrowed("średnio"),
            separable: 0,
            inseparable: 9,
            words: 9,
        },
        Prefix {
            letters: Cow::Borrowed("środkowo"),
            separable: 0,
            inseparable: 3,
            words: 3,
        },
        Prefix {
            letters: Cow::Borrowed("śród"),
            separable: 0,
            inseparable: 7,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("żółto"),
            separable: 0,
            inseparable: 6,
            words: 6,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("absolutny"), Separation::Neither),
        (Cow::Borrowed("abstrakcyjny"), Separation::Neither),
        (Cow::Borrowed("absurdalny"), Separation::Neither),
        (Cow::Borrowed("adekwatny"), Separation::Neither),
        (Cow::Borrowed("baczny"), Separation::Neither),
        (Cow::Borrowed("banalny"), Separation::Neither),
        (Cow::Borrowed("barwisty"), Separation::Neither),
        (Cow::Borrowed("barwny"), Separation::Neither),
        (Cow::Borrowed("bawny"), Separation::Neither),
        (Cow::Borrowed("bezbożny"), Separation::Neither),
        (Cow::Borrowed("bezczelny"), Separation::Neither),
        (Cow::Borrowed("bezecny"), Separation::Neither),
        (Cow::Borrowed("bezpieczny"), Separation::Neither),
        (Cow::Borrowed("bezpośredni"), Separation::Neither),
        (Cow::Borrowed("bezsensowny"), Separation::Neither),
        (Cow::Borrowed("bezwstydny"), Separation::Neither),
        (Cow::Borrowed("bełkotliwy"), Separation::Neither),
        (Cow::Borrowed("biały"), Separation::Neither),
        (Cow::Borrowed("biedny"), Separation::Neither),
        (Cow::Borrowed("bierny"), Separation::Neither),
        (Cow::Borrowed("bitny"), Separation::Neither),
        (Cow::Borrowed("bujny"), Separation::Neither),
        (Cow::Borrowed("bukoliczny"), Separation::Neither),
        (Cow::Borrowed("burzliwy"), Separation::Neither),
        (Cow::Borrowed("butny"), Separation::Neither),
        (Cow::Borrowed("buńczuczny"), Separation::Neither),
        (Cow::Borrowed("dobitny"), Separation::Neither),
        (Cow::Borrowed("dobrotliwy"), Separation::Neither),
        (Cow::Borrowed("dobry"), Separation::Neither),
        (Cow::Borrowed("dociekliwy"), Separation::Neither),
        (Cow::Borrowed("dogodny"), Separation::Neither),
        (Cow::Borrowed("dojny"), Separation::Neither),
        (Cow::Borrowed("dojrzały"), Separation::Neither),
        (Cow::Borrowed("dokuczliwy"), Separation::Neither),
        (Cow::Borrowed("dokładny"), Separation::Neither),
        (Cow::Borrowed("dolegliwy"), Separation::Neither),
        (Cow::Borrowed("domyślny"), Separation::Neither),
        (Cow::Borrowed("doniosły"), Separation::Neither),
        (Cow::Borrowed("donośny"), Separation::Neither),
        (Cow::Borrowed("dorodny"), Separation::Neither),
        (Cow::Borrowed("dorosły"), Separation::Neither),
        (Cow::Borrowed("dosadny"), Separation::Neither),
        (Cow::Borrowed("doskonały"), Separation::Neither),
        (Cow::Borrowed("dostatni"), Separation::Neither),
        (Cow::Borrowed("dostojny"), Separation::Neither),
        (Cow::Borrowed("dostępny"), Separation::Neither),
        (Cow::Borrowed("dotkliwy"), Separation::Neither),
        (Cow::Borrowed("dowcipny"), Separation::Neither),
        (Cow::Borrowed("jadliwy"), Separation::Neither),
        (Cow::Borrowed("jadowity"), Separation::Neither),
        (Cow::Borrowed("jaskrawy"), Separation::Neither),
        (Cow::Borrowed("jasny"), Separation::Neither),
        (Cow::Borrowed("jawny"), Separation::Neither),
        (Cow::Borrowed("jebliwy"), Separation::Neither),
        (Cow::Borrowed("kameralny"), Separation::Neither),
        (Cow::Borrowed("kapryśny"), Separation::Neither),
        (Cow::Borrowed("karkołomny"), Separation::Neither),
        (Cow::Borrowed("komiczny"), Separation::Neither),
        (Cow::Borrowed("konkretny"), Separation::Neither),
        (Cow::Borrowed("korny"), Separation::Neither),
        (Cow::Borrowed("korzystny"), Separation::Neither),
        (Cow::Borrowed("koszmarny"), Separation::Neither),
        (Cow::Borrowed("kosztowny"), Separation::Neither),
        (Cow::Borrowed("lichy"), Separation::Neither),
        (Cow::Borrowed("liczny"), Separation::Neither),
        (Cow::Borrowed("liliowy"), Separation::Neither),
        (Cow::Borrowed("lipny"), Separation::Neither),
        (Cow::Borrowed("litościwy"), Separation::Neither),
        (Cow::Borrowed("majestatyczny"), Separation::Neither),
        (Cow::Borrowed("majętny"), Separation::Neither),
        (Cow::Borrowed("malusi"), Separation::Neither),
        (Cow::Borrowed("manewrowy"), Separation::Neither),
        (Cow::Borrowed("marny"), Separation::Neither),
        (Cow::Borrowed("masny"), Separation::Neither),
        (Cow::Borrowed("masywny"), Separation::Neither),
        (Cow::Borrowed("mocny"), Separation::Neither),
        (Cow::Borrowed("modny"), Separation::Neither),
        (Cow::Borrowed("moralny"), Separation::Neither),
        (Cow::Borrowed("mowny"), Separation::Neither),
        (Cow::Borrowed("mozolny"), Separation::Neither),
        (Cow::Borrowed("możliwy"), Separation::Neither),
        (Cow::Borrowed("możny"), Separation::Neither),
        (Cow::Borrowed("nadobny"), Separation::Neither),
        (Cow::Borrowed("namaszczony"), Separation::Neither),
        (Cow::Borrowed("niebezpieczny"), Separation::Neither),
        (Cow::Borrowed("niebłahy"), Separation::Neither),
        (Cow::Borrowed("niecierpliwy"), Separation::Neither),
        (Cow::Borrowed("niecny"), Separation::Neither),
        (Cow::Borrowed("niedokładny"), Separation::Neither),
        (Cow::Borrowed("niedorzeczny"), Separation::Neither),
        (Cow::Borrowed("niedoskonały"), Separation::Neither),
        (Cow::Borrowed("niedołężny"), Separation::Neither),
        (Cow::Borrowed("nieduży"), Separation::Neither),
        (Cow::Borrowed("nieliczny"), Separation::Neither),
        (Cow::Borrowed("niemiłosierny"), Separation::Neither),
        (Cow::Borrowed("niemoralny"), Separation::Neither),
        (Cow::Borrowed("nieporządny"), Separation::Neither),
        (Cow::Borrowed("nieprzyjazny"), Separation::Neither),
        (Cow::Borrowed("nieprzyjemny"), Separation::Neither),
        (Cow::Borrowed("nieprzyzwoity"), Separation::Neither),
        (Cow::Borrowed("nierozsądny"), Separation::Neither),
        (Cow::Borrowed("nieroztropny"), Separation::Neither),
        (Cow::Borrowed("nierozważny"), Separation::Neither),
        (Cow::Borrowed("niesamowity"), Separation::Neither),
        (Cow::Borrowed("niesforny"), Separation::Neither),
        (Cow::Borrowed("niesmaczny"), Separation::Neither),
        (Cow::Borrowed("niesprawiedliwy"), Separation::Neither),
        (Cow::Borrowed("niestosowny"), Separation::Neither),
        (Cow::Borrowed("nieszczery"), Separation::Neither),
        (Cow::Borrowed("nieszczęśliwy"), Separation::Neither),
        (Cow::Borrowed("nietolerancyjny"), Separation::Neither),
        (Cow::Borrowed("nieudolny"), Separation::Neither),
        (Cow::Borrowed("niewdzięczny"), Separation::Neither),
        (Cow::Borrowed("niezbędny"), Separation::Neither),
        (Cow::Borrowed("niezdarny"), Separation::Neither),
        (Cow::Borrowed("niezgrabny"), Separation::Neither),
        (Cow::Borrowed("niezwykły"), Separation::Neither),
        (Cow::Borrowed("nieżyczliwy"), Separation::Neither),
        (Cow::Borrowed("nowoczesny"), Separation::Neither),
        (Cow::Borrowed("obcisły"), Separation::Neither),
        (Cow::Borrowed("obelżywy"), Separation::Neither),
        (Cow::Borrowed("obfity"), Separation::Neither),
        (Cow::Borrowed("obleśny"), Separation::Neither),
        (Cow::Borrowed("obojętny"), Separation::Neither),
        (Cow::Borrowed("obronny"), Separation::Neither),
        (Cow::Borrowed("obrotny"), Separation::Neither),
        (Cow::Borrowed("obrzydliwy"), Separation::Neither),
        (Cow::Borrowed("obszerny"), Separation::Neither),
        (Cow::Borrowed("odległy"), Separation::Neither),
        (Cow::Borrowed("odmienny"), Separation::Neither),
        (Cow::Borrowed("odporny"), Separation::Neither),
        (Cow::Borrowed("odpowiedni"), Separation::Neither),
        (Cow::Borrowed("odpowiedzialny"), Separation::Neither),
        (Cow::Borrowed("odważny"), Separation::Neither),
        (Cow::Borrowed("papuśny"), Separation::Neither),
        (Cow::Borrowed("parszywy"), Separation::Neither),
        (Cow::Borrowed("paskudny"), Separation::Neither),
        (Cow::Borrowed("pazerny"), Separation::Neither),
        (Cow::Borrowed("pobieżny"), Separation::Neither),
        (Cow::Borrowed("pobożny"), Separation::Neither),
        (Cow::Borrowed("pobłażliwy"), Separation::Neither),
        (Cow::Borrowed("pochlebny"), Separation::Neither),
        (Cow::Borrowed("pochmurny"), Separation::Neither),
        (Cow::Borrowed("pochopny"), Separation::Neither),
        (Cow::Borrowed("poczciwy"), Separation::Neither),
        (Cow::Borrowed("poczesny"), Separation::Neither),
        (Cow::Borrowed("podatny"), Separation::Neither),
        (Cow::Borrowed("podejrzliwy"), Separation::Neither),
        (Cow::Borrowed("podniosły"), Separation::Neither),
        (Cow::Borrowed("podobny"), Separation::Neither),
        (Cow::Borrowed("podstępny"), Separation::Neither),
        (Cow::Borrowed("podły"), Separation::Neither),
        (Cow::Borrowed("poetyczny"), Separation::Neither),
        (Cow::Borrowed("pogardliwy"), Separation::Neither),
        (Cow::Borrowed("pogodny"), Separation::Neither),
        (Cow::Borrowed("pojemny"), Separation::Neither),
        (Cow::Borrowed("pokaźny"), Separation::Neither),
        (Cow::Borrowed("pokorny"), Separation::Neither),
        (Cow::Borrowed("pokraczny"), Separation::Neither),
        (Cow::Borrowed("pomierny"), Separation::Neither),
        (Cow::Borrowed("pomyślny"), Separation::Neither),
        (Cow::Borrowed("ponętny"), Separation::Neither),
        (Cow::Borrowed("poprawny"), Separation::Neither),
        (Cow::Borrowed("popularny"), Separation::Neither),
        (Cow::Borrowed("popędliwy"), Separation::Neither),
        (Cow::Borrowed("porywisty"), Separation::Neither),
        (Cow::Borrowed("porządny"), Separation::Neither),
        (Cow::Borrowed("poręczny"), Separation::Neither),
        (Cow::Borrowed("posilny"), Separation::Neither),
        (Cow::Borrowed("pospolity"), Separation::Neither),
        (Cow::Borrowed("postawny"), Separation::Neither),
        (Cow::Borrowed("postronny"), Separation::Neither),
        (Cow::Borrowed("posępny"), Separation::Neither),
        (Cow::Borrowed("posłuszny"), Separation::Neither),
        (Cow::Borrowed("potulny"), Separation::Neither),
        (Cow::Borrowed("potworny"), Separation::Neither),
        (Cow::Borrowed("potężny"), Separation::Neither),
        (Cow::Borrowed("poufny"), Separation::Neither),
        (Cow::Borrowed("powabny"), Separation::Neither),
        (Cow::Borrowed("poważny"), Separation::Neither),
        (Cow::Borrowed("powierzchowny"), Separation::Neither),
        (Cow::Borrowed("powolny"), Separation::Neither),
        (Cow::Borrowed("powszechny"), Separation::Neither),
        (Cow::Borrowed("powszedni"), Separation::Neither),
        (Cow::Borrowed("pozytywny"), Separation::Neither),
        (Cow::Borrowed("pośledni"), Separation::Neither),
        (Cow::Borrowed("pożyteczny"), Separation::Neither),
        (Cow::Borrowed("pożywny"), Separation::Neither),
        (Cow::Borrowed("pożądany"), Separation::Neither),
        (Cow::Borrowed("pożądliwy"), Separation::Neither),
        (Cow::Borrowed("praktyczny"), Separation::Neither),
        (Cow::Borrowed("prawdopodobny"), Separation::Neither),
        (Cow::Borrowed("prawdziwy"), Separation::Neither),
        (Cow::Borrowed("prawilny"), Separation::Neither),
        (Cow::Borrowed("prawy"), Separation::Neither),
        (Cow::Borrowed("precyzyjny"), Separation::Neither),
        (Cow::Borrowed("promienny"), Separation::Neither),
        (Cow::Borrowed("prosty"), Separation::Neither),
        (Cow::Borrowed("prymitywny"), Separation::Neither),
        (Cow::Borrowed("przaśny"), Separation::Neither),
        (Cow::Borrowed("przebiegły"), Separation::Neither),
        (Cow::Borrowed("przeciętny"), Separation::Neither),
        (Cow::Borrowed("przedni"), Separation::Neither),
        (Cow::Borrowed("przedobry"), Separation::Neither),
        (Cow::Borrowed("przedziwny"), Separation::Neither),
        (Cow::Borrowed("przejrzysty"), Separation::Neither),
        (Cow::Borrowed("przekorny"), Separation::Neither),
        (Cow::Borrowed("przemożny"), Separation::Neither),
        (Cow::Borrowed("przemyślny"), Separation::Neither),
        (Cow::Borrowed("przepyszny"), Separation::Neither),
        (Cow::Borrowed("przeraźliwy"), Separation::Neither),
        (Cow::Borrowed("przeróżny"), Separation::Neither),
        (Cow::Borrowed("przesądny"), Separation::Neither),
        (Cow::Borrowed("przezorny"), Separation::Neither),
        (Cow::Borrowed("przezroczysty"), Separation::Neither),
        (Cow::Borrowed("prześcipny"), Separation::Neither),
        (Cow::Borrowed("prześliczny"), Separation::Neither),
        (Cow::Borrowed("przeźroczysty"), Separation::Neither),
        (Cow::Borrowed("przychylny"), Separation::Neither),
        (Cow::Borrowed("przyduży"), Separation::Neither),
        (Cow::Borrowed("przyjazny"), Separation::Neither),
        (Cow::Borrowed("przyjemny"), Separation::Neither),
        (Cow::Borrowed("przykry"), Separation::Neither),
        (Cow::Borrowed("przykładny"), Separation::Neither),
        (Cow::Borrowed("przymilny"), Separation::Neither),
        (Cow::Borrowed("przystojny"), Separation::Neither),
        (Cow::Borrowed("przystępny"), Separation::Neither),
        (Cow::Borrowed("przytomny"), Separation::Neither),
        (Cow::Borrowed("przytulny"), Separation::Neither),
        (Cow::Borrowed("przyzwoity"), Separation::Neither),
        (Cow::Borrowed("przędny"), Separation::Neither),
        (Cow::Borrowed("próżny"), Separation::Neither),
        (Cow::Borrowed("prędki"), Separation::Neither),
        (Cow::Borrowed("prężny"), Separation::Neither),
        (Cow::Borrowed("robotny"), Separation::Neither),
        (Cow::Borrowed("rojny"), Separation::Neither),
        (Cow::Borrowed("romantyczny"), Separation::Neither),
        (Cow::Borrowed("rosły"), Separation::Neither),
        (Cow::Borrowed("rozciągliwy"), Separation::Neither),
        (Cow::Borrowed("rozkoszny"), Separation::Neither),
        (Cow::Borrowed("rozległy"), Separation::Neither),
        (Cow::Borrowed("rozlewny"), Separation::Neither),
        (Cow::Borrowed("rozliczny"), Separation::Neither),
        (Cow::Borrowed("rozmaity"), Separation::Neither),
        (Cow::Borrowed("rozmowny"), Separation::Neither),
        (Cow::Borrowed("rozmówny"), Separation::Neither),
        (Cow::Borrowed("rozpaczliwy"), Separation::Neither),
        (Cow::Borrowed("rozrzutny"), Separation::Neither),
        (Cow::Borrowed("rozsądny"), Separation::Neither),
        (Cow::Borrowed("roztropny"), Separation::Neither),
        (Cow::Borrowed("rozumny"), Separation::Neither),
        (Cow::Borrowed("rozważny"), Separation::Neither),
        (Cow::Borrowed("rozwlekły"), Separation::Neither),
        (Cow::Borrowed("różnorodny"), Separation::Neither),
        (Cow::Borrowed("samodzielny"), Separation::Neither),
        (Cow::Borrowed("samotny"), Separation::Neither),
        (Cow::Borrowed("skory"), Separation::Neither),
        (Cow::Borrowed("skrajny"), Separation::Neither),
        (Cow::Borrowed("skromny"), Separation::Neither),
        (Cow::Borrowed("skryty"), Separation::Neither),
        (Cow::Borrowed("skrzętny"), Separation::Neither),
        (Cow::Borrowed("skuteczny"), Separation::Neither),
        (Cow::Borrowed("skwapliwy"), Separation::Neither),
        (Cow::Borrowed("skwarny"), Separation::Neither),
        (Cow::Borrowed("składny"), Separation::Neither),
        (Cow::Borrowed("skłonny"), Separation::Neither),
        (Cow::Borrowed("soczysty"), Separation::Neither),
        (Cow::Borrowed("solidny"), Separation::Neither),
        (Cow::Borrowed("sowity"), Separation::Neither),
        (Cow::Borrowed("specjalny"), Separation::Neither),
        (Cow::Borrowed("spieszny"), Separation::Neither),
        (Cow::Borrowed("spokojny"), Separation::Neither),
        (Cow::Borrowed("spolegliwy"), Separation::Neither),
        (Cow::Borrowed("sposobny"), Separation::Neither),
        (Cow::Borrowed("sprawiedliwy"), Separation::Neither),
        (Cow::Borrowed("sprawny"), Separation::Neither),
        (Cow::Borrowed("sprośny"), Separation::Neither),
        (Cow::Borrowed("sprytny"), Separation::Neither),
        (Cow::Borrowed("starodawny"), Separation::Neither),
        (Cow::Borrowed("subtelny"), Separation::Neither),
        (Cow::Borrowed("szacowny"), Separation::Neither),
        (Cow::Borrowed("szalony"), Separation::Neither),
        (Cow::Borrowed("szanowny"), Separation::Neither),
        (Cow::Borrowed("szczególny"), Separation::Neither),
        (Cow::Borrowed("szczelny"), Separation::Neither),
        (Cow::Borrowed("szczery"), Separation::Neither),
        (Cow::Borrowed("szczodrobliwy"), Separation::Neither),
        (Cow::Borrowed("szczodry"), Separation::Neither),
        (Cow::Borrowed("szczupły"), Separation::Neither),
        (Cow::Borrowed("szczwany"), Separation::Neither),
        (Cow::Borrowed("szczytny"), Separation::Neither),
        (Cow::Borrowed("szczęsny"), Separation::Neither),
        (Cow::Borrowed("szczęśliwy"), Separation::Neither),
        (Cow::Borrowed("szeroki"), Separation::Neither),
        (Cow::Borrowed("szkaradny"), Separation::Neither),
        (Cow::Borrowed("szlachetny"), Separation::Neither),
        (Cow::Borrowed("szpetny"), Separation::Neither),
        (Cow::Borrowed("sztuczny"), Separation::Neither),
        (Cow::Borrowed("sztywny"), Separation::Neither),
        (Cow::Borrowed("szybki"), Separation::Neither),
        (Cow::Borrowed("szykowny"), Separation::Neither),
        (Cow::Borrowed("tolerancyjny"), Separation::Neither),
        (Cow::Borrowed("toporny"), Separation::Neither),
        (Cow::Borrowed("tragiczny"), Separation::Neither),
        (Cow::Borrowed("troskliwy"), Separation::Neither),
        (Cow::Borrowed("wiarogodny"), Separation::Neither),
        (Cow::Borrowed("wiarygodny"), Separation::Neither),
        (Cow::Borrowed("widny"), Separation::Neither),
        (Cow::Borrowed("wielki"), Separation::Neither),
        (Cow::Borrowed("wierny"), Separation::Neither),
        (Cow::Borrowed("wierutny"), Separation::Neither),
        (Cow::Borrowed("wilgotny"), Separation::Neither),
        (Cow::Borrowed("współczesny"), Separation::Neither),
        (Cow::Borrowed("wybitny"), Separation::Neither),
        (Cow::Borrowed("wyborny"), Separation::Neither),
        (Cow::Borrowed("wydajny"), Separation::Neither),
        (Cow::Borrowed("wydatny"), Separation::Neither),
        (Cow::Borrowed("wygodny"), Separation::Neither),
        (Cow::Borrowed("wylewny"), Separation::Neither),
        (Cow::Borrowed("wymowny"), Separation::Neither),
        (Cow::Borrowed("wymyślny"), Separation::Neither),
        (Cow::Borrowed("wymówny"), Separation::Neither),
        (Cow::Borrowed("wyniosły"), Separation::Neither),
        (Cow::Borrowed("wyrafinowany"), Separation::Neither),
        (Cow::Borrowed("wyrazisty"), Separation::Neither),
        (Cow::Borrowed("wyraźny"), Separation::Neither),
        (Cow::Borrowed("wyrozumiały"), Separation::Neither),
        (Cow::Borrowed("wysmukły"), Separation::Neither),
        (Cow::Borrowed("wysoki"), Separation::Neither),
        (Cow::Borrowed("wystawny"), Separation::Neither),
        (Cow::Borrowed("wyszukany"), Separation::Neither),
        (Cow::Borrowed("wytrawny"), Separation::Neither),
        (Cow::Borrowed("wytworny"), Separation::Neither),
        (Cow::Borrowed("wyśmienity"), Separation::Neither),
        (Cow::Borrowed("zaangażowany"), Separation::Neither),
        (Cow::Borrowed("zabawny"), Separation::Neither),
        (Cow::Borrowed("zachłanny"), Separation::Neither),
        (Cow::Borrowed("zaciekły"), Separation::Neither),
        (Cow::Borrowed("zaciszny"), Separation::Neither),
        (Cow::Borrowed("zacny"), Separation::Neither),
        (Cow::Borrowed("zadziorny"), Separation::Neither),
        (Cow::Borrowed("zagorzały"), Separation::Neither),
        (Cow::Borrowed("zajadły"), Separation::Neither),
        (Cow::Borrowed("zajebisty"), Separation::Neither),
        (Cow::Borrowed("zajedwabisty"), Separation::Neither),
        (Cow::Borrowed("zajefajny"), Separation::Neither),
        (Cow::Borrowed("zamożny"), Separation::Neither),
        (Cow::Borrowed("zapalczywy"), Separation::Neither),
        (Cow::Borrowed("zapalny"), Separation::Neither),
        (Cow::Borrowed("zapobiegliwy"), Separation::Neither),
        (Cow::Borrowed("zaradny"), Separation::Neither),
        (Cow::Borrowed("zaraźliwy"), Separation::Neither),
        (Cow::Borrowed("zarozumiały"), Separation::Neither),
        (Cow::Borrowed("zarąbisty"), Separation::Neither),
        (Cow::Borrowed("zasadny"), Separation::Neither),
        (Cow::Borrowed("zasobny"), Separation::Neither),
        (Cow::Borrowed("zaszczytny"), Separation::Neither),
        (Cow::Borrowed("zasłużony"), Separation::Neither),
        (Cow::Borrowed("zawiły"), Separation::Neither),
        (Cow::Borrowed("zawrotny"), Separation::Neither),
        (Cow::Borrowed("zawzięty"), Separation::Neither),
        (Cow::Borrowed("zazdrosny"), Separation::Neither),
        (Cow::Borrowed("zażywny"), Separation::Neither),
        (Cow::Borrowed("zażyły"), Separation::Neither),
        (Cow::Borrowed("złocisty"), Separation::Neither),
        (Cow::Borrowed("złomny"), Separation::Neither),
        (Cow::Borrowed("złośliwy"), Separation::Neither),
        (Cow::Borrowed("łachmanowaty"), Separation::Neither),
        (Cow::Borrowed("łacny"), Separation::Neither),
        (Cow::Borrowed("ładny"), Separation::Neither),
        (Cow::Borrowed("łagodny"), Separation::Neither),
        (Cow::Borrowed("łamliwy"), Separation::Neither),
        (Cow::Borrowed("łapczywy"), Separation::Neither),
        (Cow::Borrowed("łaskawy"), Separation::Neither),
        (Cow::Borrowed("łaskotliwy"), Separation::Neither),
        (Cow::Borrowed("łatwy"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // NOM_M_ANIM
        Before, // NOM_M_INAN
        Before, // NOM_F_ANIM
        Before, // NOM_F_INAN
        Before, // NOM_N_ANIM
        Before, // NOM_N_INAN
        Before, // NOM_VIR_ANIM
        Before, // NOM_VIR_INAN
        Before, // NOM_NVIR_ANIM
        Before, // NOM_NVIR_INAN
        Before, // GEN_M_ANIM
        Before, // GEN_M_INAN
        Before, // GEN_F_ANIM
        Before, // GEN_F_INAN
        Before, // GEN_N_ANIM
        Before, // GEN_N_INAN
        Before, // GEN_VIR_ANIM
        Before, // GEN_VIR_INAN
        Before, // GEN_NVIR_ANIM
        Before, // GEN_NVIR_INAN
        Before, // DAT_M_ANIM
        Before, // DAT_M_INAN
        Before, // DAT_F_ANIM
        Before, // DAT_F_INAN
        Before, // DAT_N_ANIM
        Before, // DAT_N_INAN
        Before, // DAT_VIR_ANIM
        Before, // DAT_VIR_INAN
        Before, // DAT_NVIR_ANIM
        Before, // DAT_NVIR_INAN
        Before, // ACC_M_ANIM
        Before, // ACC_M_INAN
        Before, // ACC_F_ANIM
        Before, // ACC_F_INAN
        Before, // ACC_N_ANIM
        Before, // ACC_N_INAN
        Before, // ACC_VIR_ANIM
        Before, // ACC_VIR_INAN
        Before, // ACC_NVIR_ANIM
        Before, // ACC_NVIR_INAN
        Before, // INS_M_ANIM
        Before, // INS_M_INAN
        Before, // INS_F_ANIM
        Before, // INS_F_INAN
        Before, // INS_N_ANIM
        Before, // INS_N_INAN
        Before, // INS_VIR_ANIM
        Before, // INS_VIR_INAN
        Before, // INS_NVIR_ANIM
        Before, // INS_NVIR_INAN
        Before, // LOC_M_ANIM
        Before, // LOC_M_INAN
        Before, // LOC_F_ANIM
        Before, // LOC_F_INAN
        Before, // LOC_N_ANIM
        Before, // LOC_N_INAN
        Before, // LOC_VIR_ANIM
        Before, // LOC_VIR_INAN
        Before, // LOC_NVIR_ANIM
        Before, // LOC_NVIR_INAN
        Before, // COMPARATIVE
        Before, // SUPERLATIVE
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // NOM_M_ANIM
        Cow::Borrowed(""), // NOM_M_INAN
        Cow::Borrowed(""), // NOM_F_ANIM
        Cow::Borrowed(""), // NOM_F_INAN
        Cow::Borrowed(""), // NOM_N_ANIM
        Cow::Borrowed(""), // NOM_N_INAN
        Cow::Borrowed(""), // NOM_VIR_ANIM
        Cow::Borrowed(""), // NOM_VIR_INAN
        Cow::Borrowed(""), // NOM_NVIR_ANIM
        Cow::Borrowed(""), // NOM_NVIR_INAN
        Cow::Borrowed(""), // GEN_M_ANIM
        Cow::Borrowed(""), // GEN_M_INAN
        Cow::Borrowed(""), // GEN_F_ANIM
        Cow::Borrowed(""), // GEN_F_INAN
        Cow::Borrowed(""), // GEN_N_ANIM
        Cow::Borrowed(""), // GEN_N_INAN
        Cow::Borrowed(""), // GEN_VIR_ANIM
        Cow::Borrowed(""), // GEN_VIR_INAN
        Cow::Borrowed(""), // GEN_NVIR_ANIM
        Cow::Borrowed(""), // GEN_NVIR_INAN
        Cow::Borrowed(""), // DAT_M_ANIM
        Cow::Borrowed(""), // DAT_M_INAN
        Cow::Borrowed(""), // DAT_F_ANIM
        Cow::Borrowed(""), // DAT_F_INAN
        Cow::Borrowed(""), // DAT_N_ANIM
        Cow::Borrowed(""), // DAT_N_INAN
        Cow::Borrowed(""), // DAT_VIR_ANIM
        Cow::Borrowed(""), // DAT_VIR_INAN
        Cow::Borrowed(""), // DAT_NVIR_ANIM
        Cow::Borrowed(""), // DAT_NVIR_INAN
        Cow::Borrowed(""), // ACC_M_ANIM
        Cow::Borrowed(""), // ACC_M_INAN
        Cow::Borrowed(""), // ACC_F_ANIM
        Cow::Borrowed(""), // ACC_F_INAN
        Cow::Borrowed(""), // ACC_N_ANIM
        Cow::Borrowed(""), // ACC_N_INAN
        Cow::Borrowed(""), // ACC_VIR_ANIM
        Cow::Borrowed(""), // ACC_VIR_INAN
        Cow::Borrowed(""), // ACC_NVIR_ANIM
        Cow::Borrowed(""), // ACC_NVIR_INAN
        Cow::Borrowed(""), // INS_M_ANIM
        Cow::Borrowed(""), // INS_M_INAN
        Cow::Borrowed(""), // INS_F_ANIM
        Cow::Borrowed(""), // INS_F_INAN
        Cow::Borrowed(""), // INS_N_ANIM
        Cow::Borrowed(""), // INS_N_INAN
        Cow::Borrowed(""), // INS_VIR_ANIM
        Cow::Borrowed(""), // INS_VIR_INAN
        Cow::Borrowed(""), // INS_NVIR_ANIM
        Cow::Borrowed(""), // INS_NVIR_INAN
        Cow::Borrowed(""), // LOC_M_ANIM
        Cow::Borrowed(""), // LOC_M_INAN
        Cow::Borrowed(""), // LOC_F_ANIM
        Cow::Borrowed(""), // LOC_F_INAN
        Cow::Borrowed(""), // LOC_N_ANIM
        Cow::Borrowed(""), // LOC_N_INAN
        Cow::Borrowed(""), // LOC_VIR_ANIM
        Cow::Borrowed(""), // LOC_VIR_INAN
        Cow::Borrowed(""), // LOC_NVIR_ANIM
        Cow::Borrowed(""), // LOC_NVIR_INAN
        Cow::Borrowed(""), // COMPARATIVE
        Cow::Borrowed(""), // SUPERLATIVE
    ]),
};
