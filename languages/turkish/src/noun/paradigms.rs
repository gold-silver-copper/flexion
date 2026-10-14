//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`I`, `A`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `nom` (sg, pl)
/// `acc` (sg, pl)
/// `dat` (sg, pl)
/// `loc` (sg, pl)
/// `abl` (sg, pl)
/// `gen_` (sg, pl)
/// `possessive_nom_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_nom_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_acc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_acc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_dat_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_dat_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_loc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_loc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_abl_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_abl_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_gen_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_gen_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_nom_sg: $s0:tt;)?
     $(stem noun_acc_sg: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(acc: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(loc: $c6:tt, $c7:tt;)?
     $(abl: $c8:tt, $c9:tt;)?
     $(gen_: $c10:tt, $c11:tt;)?
     $(possessive_nom_one: $c12:tt, $c13:tt, $c14:tt, $c15:tt, $c16:tt, $c17:tt;)?
     $(possessive_nom_many: $c18:tt, $c19:tt, $c20:tt, $c21:tt, $c22:tt, $c23:tt;)?
     $(possessive_acc_one: $c24:tt, $c25:tt, $c26:tt, $c27:tt, $c28:tt, $c29:tt;)?
     $(possessive_acc_many: $c30:tt, $c31:tt, $c32:tt, $c33:tt, $c34:tt, $c35:tt;)?
     $(possessive_dat_one: $c36:tt, $c37:tt, $c38:tt, $c39:tt, $c40:tt, $c41:tt;)?
     $(possessive_dat_many: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(possessive_loc_one: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt, $c53:tt;)?
     $(possessive_loc_many: $c54:tt, $c55:tt, $c56:tt, $c57:tt, $c58:tt, $c59:tt;)?
     $(possessive_abl_one: $c60:tt, $c61:tt, $c62:tt, $c63:tt, $c64:tt, $c65:tt;)?
     $(possessive_abl_many: $c66:tt, $c67:tt, $c68:tt, $c69:tt, $c70:tt, $c71:tt;)?
     $(possessive_gen_one: $c72:tt, $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(possessive_gen_many: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt, $c83:tt;)?
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
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
                $(flexion::cell!($c56),)? "\t",
                $(flexion::cell!($c57),)? "\t",
                $(flexion::cell!($c58),)? "\t",
                $(flexion::cell!($c59),)? "\t",
                $(flexion::cell!($c60),)? "\t",
                $(flexion::cell!($c61),)? "\t",
                $(flexion::cell!($c62),)? "\t",
                $(flexion::cell!($c63),)? "\t",
                $(flexion::cell!($c64),)? "\t",
                $(flexion::cell!($c65),)? "\t",
                $(flexion::cell!($c66),)? "\t",
                $(flexion::cell!($c67),)? "\t",
                $(flexion::cell!($c68),)? "\t",
                $(flexion::cell!($c69),)? "\t",
                $(flexion::cell!($c70),)? "\t",
                $(flexion::cell!($c71),)? "\t",
                $(flexion::cell!($c72),)? "\t",
                $(flexion::cell!($c73),)? "\t",
                $(flexion::cell!($c74),)? "\t",
                $(flexion::cell!($c75),)? "\t",
                $(flexion::cell!($c76),)? "\t",
                $(flexion::cell!($c77),)? "\t",
                $(flexion::cell!($c78),)? "\t",
                $(flexion::cell!($c79),)? "\t",
                $(flexion::cell!($c80),)? "\t",
                $(flexion::cell!($c81),)? "\t",
                $(flexion::cell!($c82),)? "\t",
                $(flexion::cell!($c83),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
            )),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `dalga`: 3,955 words.
    Dalga,
    /// Like `sav`: 2,910 words.
    Sav,
    /// Like `ant`: 1,717 words.
    Ant,
    /// Like `makas`: 755 words.
    Makas,
    /// Like `kiriş`: 681 words.
    Kiriş,
    /// Like `yer`: 105 words.
    Yer,
    /// Like `devir`: 47 words.
    Devir,
    /// Like `akıl`: 25 words.
    Akıl,
    /// Like `göz`: 20 words.
    Göz,
    /// Like `ekolog`: 17 words.
    Ekolog,
    /// Like `gönül`: 17 words.
    Gönül,
    /// Like `hesap`: 16 words.
    Hesap,
    /// Like `mazak`: 13 words.
    Mazak,
    /// Like `kıskaç`: 13 words.
    Kıskaç,
    /// Like `keyif`: 12 words.
    Keyif,
    /// Like `kâğıt`: 11 words.
    Kâğıt,
    /// Like `nakış`: 9 words.
    Nakış,
    /// Like `efkâr`: 7 words.
    Efkâr,
    /// Like `burun`: 5 words.
    Burun,
    /// Like `set`: 4 words.
    Set,
    /// Like `ahit`: 3 words.
    Ahit,
    /// Like `ufuk`: 3 words.
    Ufuk,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 22] = [
        Declension::Dalga,
        Declension::Sav,
        Declension::Ant,
        Declension::Makas,
        Declension::Kiriş,
        Declension::Yer,
        Declension::Devir,
        Declension::Akıl,
        Declension::Göz,
        Declension::Ekolog,
        Declension::Gönül,
        Declension::Hesap,
        Declension::Mazak,
        Declension::Kıskaç,
        Declension::Keyif,
        Declension::Kâğıt,
        Declension::Nakış,
        Declension::Efkâr,
        Declension::Burun,
        Declension::Set,
        Declension::Ahit,
        Declension::Ufuk,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        DECLENSIONS.get(self as usize)
    }
}

impl flexion::Label for Declension {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static DECLENSIONS: [Paradigm<'static>; 22] = [
    // Dalga: 3,955 words (abide, bone, nine, sela, banko, yeti).
    declension!("dalga", 3955, "~";
    stem noun_acc_sg: "~- like dalga";
    nom: "~", "~lAr";
    loc: "~dA", _;
    possessive_nom_one: _, _, _, _, _, "~lArI";
    possessive_nom_many: _, "~lArIn", "~lArI", _, _, "~lArI";
    possessive_acc_many: _, "~lArInI", "~lArInI", _, _, "~lArInI";
    possessive_dat_many: _, "~lArInA", "~lArInA", _, _, "~lArInA";
    possessive_loc_many: _, "~lArIndA", "~lArIndA", _, _, "~lArIndA";
    possessive_abl_many: _, "~lArIndAn", "~lArIndAn", _, _, "~lArIndAn";
    possessive_gen_many: _, "~lArInIn", "~lArInIn", _, _, "~lArInIn";
    ),
    // Sav: 2,910 words (pound, ab, abdal, en, in, seven). Like Dalga, except in 0 cells: .
    declension!("sav", 2910, "~", like Dalga;
    stem noun_acc_sg: "~- like sav";
    ),
    // Ant: 1,717 words (ant, art, bap, şibidak, müzik, kayak).
    declension!("ant", 1717, "~ˇ";
    stem noun_acc_sg: "~- like ant";
    nom: "~ˇ", "~lArˇ";
    loc: "~tAˇ", _;
    possessive_nom_one: _, _, _, _, _, "~lArIˇ";
    possessive_nom_many: _, "~lArInˇ", "~lArIˇ", _, _, "~lArIˇ";
    possessive_acc_many: _, "~lArInIˇ", "~lArInIˇ", _, _, "~lArInIˇ";
    possessive_dat_many: _, "~lArInAˇ", "~lArInAˇ", _, _, "~lArInAˇ";
    possessive_loc_many: _, "~lArIndAˇ", "~lArIndAˇ", _, _, "~lArIndAˇ";
    possessive_abl_many: _, "~lArIndAnˇ", "~lArIndAnˇ", _, _, "~lArIndAnˇ";
    possessive_gen_many: _, "~lArInInˇ", "~lArInInˇ", _, _, "~lArInInˇ";
    ),
    // Makas: 755 words (robot, port, bot, bank, but, mars). Like Sav, except in 0 cells: .
    declension!("makas", 755, "~", like Sav;
    stem noun_nom_sg: "~- like makas";
    ),
    // Kiriş: 681 words (abdest, lens, planet, bit, is, market). Like Makas, except in 38 cells: possessive_nom_one _1sg, possessive_nom_one _3sg, possessive_nom_one _1pl, possessive_nom_one _2pl, possessive_nom_one _3pl, possessive_nom_many _2sg, ….
    declension!("kiriş", 681, "~", like Makas;
    possessive_nom_one: "~im", _, "~i", "~imIz", "~inIz", "~lerI";
    possessive_nom_many: _, "~lerIn", "~lerI", _, _, "~lerI";
    possessive_acc_one: _, "~inI", _, _, "~inIzI", _;
    possessive_acc_many: _, "~lerInI", "~lerInI", _, _, "~lerInI";
    possessive_dat_one: "~imA", "~inA", _, _, "~inIzA", _;
    possessive_dat_many: _, "~lerInA", "~lerInA", _, _, "~lerInA";
    possessive_loc_one: "~imdA", "~indA", "~indA", _, "~inIzdA", _;
    possessive_loc_many: _, "~lerIndA", "~lerIndA", _, _, "~lerIndA";
    possessive_abl_one: "~imdAn", "~indAn", "~indAn", _, "~inIzdAn", _;
    possessive_abl_many: _, "~lerIndAn", "~lerIndAn", _, _, "~lerIndAn";
    possessive_gen_one: _, "~inIn", _, _, "~inIzIn", _;
    possessive_gen_many: _, "~lerInIn", "~lerInIn", _, _, "~lerInIn";
    ),
    // Yer: 105 words (general, montreal, ideal, metal, terminal, meal). Like Kiriş, except in 2 cells: nom pl, loc sg.
    declension!("yer", 105, "~", like Kiriş;
    stem noun_acc_sg: "~- like yer";
    nom: _, "~ler";
    loc: "~de", _;
    ),
    // Devir: 47 words (zehir, emir, isim, dahil, devir, aciz).
    declension!("devir", 47, "~i~";
    stem noun_nom_sg: "~i~- like devir";
    stem noun_acc_sg: "~~- like yer";
    ),
    // Akıl: 25 words (hayır, ağız, akıl, asır, bağır, kayın).
    declension!("akıl", 25, "~ı~";
    stem noun_nom_sg: "~ı~- like akıl";
    stem noun_acc_sg: "~~- like sav";
    ),
    // Göz: 20 words (petrol, spoiler, gol, ampul, ful, rol). Like Yer, except in 0 cells: .
    declension!("göz", 20, "~", like Yer;
    stem noun_acc_sg: "~- like göz";
    ),
    // Ekolog: 17 words (monolog, antropolog, ekolog, klimatolog, arkeolog, katalog). Like Ant, except in 1 cell: loc sg.
    declension!("ekolog", 17, "~ˇ", like Ant;
    loc: "~dAˇ", _;
    ),
    // Gönül: 17 words (göğüs, ömür, şükür, zulüm, gönül, özür).
    declension!("gönül", 17, "~ü~";
    stem noun_nom_sg: "~ü~- like devir";
    stem noun_acc_sg: "~~- like göz";
    ),
    // Hesap: 16 words (kalıp, garp, kulüp, darp, celp, teyp).
    declension!("hesap", 16, "~p";
    stem noun_nom_sg: "~p- like makas";
    stem noun_acc_sg: "~b- like sav";
    ),
    // Mazak: 13 words (gök, olgunluk, lakayıtlık, basketbolculuk, soğukkanlılık, plâk).
    declension!("mazak", 13, "~k";
    stem noun_nom_sg: "~k- like makas";
    stem noun_acc_sg: "~ğ- like sav";
    ),
    // Kıskaç: 13 words (öç, güç, felç, zangoç, ilâç, harç).
    declension!("kıskaç", 13, "~ç";
    stem noun_nom_sg: "~ç- like makas";
    stem noun_acc_sg: "~c- like sav";
    ),
    // Keyif: 12 words (vakit, akis, beyit, hapis, bahis, nefis). Like Devir, except in 0 cells: .
    declension!("keyif", 12, "~i~", like Devir;
    stem noun_nom_sg: "~i~- like keyif";
    ),
    // Kâğıt: 11 words (aft, ametist, cilt, külot, sergüzeşt, metot).
    declension!("kâğıt", 11, "~t";
    stem noun_nom_sg: "~t- like makas";
    stem noun_acc_sg: "~d- like sav";
    ),
    // Nakış: 9 words (vakıf, şahıs, satıh, kasıt, vasıf, nakış). Like Akıl, except in 1 cell: loc sg.
    declension!("nakış", 9, "~ı~", like Akıl;
    loc: "~ı~ta", _;
    ),
    // Efkâr: 7 words (abanozgiller, efkâr, evsaf, sübyancılık, ağyar, fokgiller). Like Sav, except in 24 cells: nom pl, acc sg, dat sg, loc sg, gen_ sg, possessive_nom_one _3pl, ….
    declension!("efkâr", 7, "~", like Sav;
    nom: _, "~";
    acc: derived, _;
    dat: derived, _;
    loc: derived, _;
    gen_: derived, _;
    possessive_nom_one: _, _, _, _, _, "~ı";
    possessive_nom_many: _, "~ın", "~ı", _, _, "~ı";
    possessive_acc_many: _, "~ını", "~ını", _, _, "~ını";
    possessive_dat_many: _, "~ına", "~ına", _, _, "~ına";
    possessive_loc_many: _, "~ında", "~ında", _, _, "~ında";
    possessive_abl_many: _, "~ından", "~ından", _, _, "~ından";
    possessive_gen_many: _, "~ının", "~ının", _, _, "~ının";
    ),
    // Burun: 5 words (burun, oğul, uğur, boyun, omuz). Like Akıl, except in 0 cells: .
    declension!("burun", 5, "~u~", like Akıl;
    stem noun_nom_sg: "~u~- like akıl";
    ),
    // Set: 4 words (met, set, ret, cet). Like Kâğıt, except in 0 cells: .
    declension!("set", 4, "~t", like Kâğıt;
    stem noun_acc_sg: "~dd- like set";
    ),
    // Ahit: 3 words (nakit, lahit, ahit).
    declension!("ahit", 3, "~it";
    stem noun_nom_sg: "~it- like makas";
    stem noun_acc_sg: "~d- like yer";
    ),
    // Ufuk: 3 words (ufuk, nutuk, lütuf). Like Burun, except in 1 cell: loc sg.
    declension!("ufuk", 3, "~u~", like Burun;
    stem noun_acc_sg: "~~- like ufuk";
    loc: "~u~ta", _;
    ),
];
