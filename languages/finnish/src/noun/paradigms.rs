//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`A`, `V`, `Ä`, `T`)
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
/// `gen_` (sg, pl)
/// `par` (sg, pl)
/// `ine` (sg, pl)
/// `ela` (sg, pl)
/// `ill` (sg, pl)
/// `ade` (sg, pl)
/// `abl` (sg, pl)
/// `all` (sg, pl)
/// `ess` (sg, pl)
/// `tra` (sg, pl)
/// `abe` (sg, pl)
/// `ins` (sg, pl)
/// `com` (sg, pl)
/// `possessive_nom_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_nom_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_gen_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_gen_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_par_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_par_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ine_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ine_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ela_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ela_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ill_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ill_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ade_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ade_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abl_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abl_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_all_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_all_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ess_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ess_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_tra_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_tra_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abe_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abe_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ins_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ins_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_com_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_com_pl` (_1sg, _2sg, _1pl, _2pl, _3)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem sg: $s0:tt;)?
     $(stem noun_nom_pl: $s1:tt;)?
     $(stem pl: $s2:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(par: $c4:tt, $c5:tt;)?
     $(ine: $c6:tt, $c7:tt;)?
     $(ela: $c8:tt, $c9:tt;)?
     $(ill: $c10:tt, $c11:tt;)?
     $(ade: $c12:tt, $c13:tt;)?
     $(abl: $c14:tt, $c15:tt;)?
     $(all: $c16:tt, $c17:tt;)?
     $(ess: $c18:tt, $c19:tt;)?
     $(tra: $c20:tt, $c21:tt;)?
     $(abe: $c22:tt, $c23:tt;)?
     $(ins: $c24:tt, $c25:tt;)?
     $(com: $c26:tt, $c27:tt;)?
     $(possessive_nom_sg: $c28:tt, $c29:tt, $c30:tt, $c31:tt, $c32:tt;)?
     $(possessive_nom_pl: $c33:tt, $c34:tt, $c35:tt, $c36:tt, $c37:tt;)?
     $(possessive_gen_sg: $c38:tt, $c39:tt, $c40:tt, $c41:tt, $c42:tt;)?
     $(possessive_gen_pl: $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(possessive_par_sg: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt;)?
     $(possessive_par_pl: $c53:tt, $c54:tt, $c55:tt, $c56:tt, $c57:tt;)?
     $(possessive_ine_sg: $c58:tt, $c59:tt, $c60:tt, $c61:tt, $c62:tt;)?
     $(possessive_ine_pl: $c63:tt, $c64:tt, $c65:tt, $c66:tt, $c67:tt;)?
     $(possessive_ela_sg: $c68:tt, $c69:tt, $c70:tt, $c71:tt, $c72:tt;)?
     $(possessive_ela_pl: $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(possessive_ill_sg: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt;)?
     $(possessive_ill_pl: $c83:tt, $c84:tt, $c85:tt, $c86:tt, $c87:tt;)?
     $(possessive_ade_sg: $c88:tt, $c89:tt, $c90:tt, $c91:tt, $c92:tt;)?
     $(possessive_ade_pl: $c93:tt, $c94:tt, $c95:tt, $c96:tt, $c97:tt;)?
     $(possessive_abl_sg: $c98:tt, $c99:tt, $c100:tt, $c101:tt, $c102:tt;)?
     $(possessive_abl_pl: $c103:tt, $c104:tt, $c105:tt, $c106:tt, $c107:tt;)?
     $(possessive_all_sg: $c108:tt, $c109:tt, $c110:tt, $c111:tt, $c112:tt;)?
     $(possessive_all_pl: $c113:tt, $c114:tt, $c115:tt, $c116:tt, $c117:tt;)?
     $(possessive_ess_sg: $c118:tt, $c119:tt, $c120:tt, $c121:tt, $c122:tt;)?
     $(possessive_ess_pl: $c123:tt, $c124:tt, $c125:tt, $c126:tt, $c127:tt;)?
     $(possessive_tra_sg: $c128:tt, $c129:tt, $c130:tt, $c131:tt, $c132:tt;)?
     $(possessive_tra_pl: $c133:tt, $c134:tt, $c135:tt, $c136:tt, $c137:tt;)?
     $(possessive_abe_sg: $c138:tt, $c139:tt, $c140:tt, $c141:tt, $c142:tt;)?
     $(possessive_abe_pl: $c143:tt, $c144:tt, $c145:tt, $c146:tt, $c147:tt;)?
     $(possessive_ins_sg: $c148:tt, $c149:tt, $c150:tt, $c151:tt, $c152:tt;)?
     $(possessive_ins_pl: $c153:tt, $c154:tt, $c155:tt, $c156:tt, $c157:tt;)?
     $(possessive_com_sg: $c158:tt, $c159:tt, $c160:tt, $c161:tt, $c162:tt;)?
     $(possessive_com_pl: $c163:tt, $c164:tt, $c165:tt, $c166:tt, $c167:tt;)?
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
                $(flexion::cell!($c84),)? "\t",
                $(flexion::cell!($c85),)? "\t",
                $(flexion::cell!($c86),)? "\t",
                $(flexion::cell!($c87),)? "\t",
                $(flexion::cell!($c88),)? "\t",
                $(flexion::cell!($c89),)? "\t",
                $(flexion::cell!($c90),)? "\t",
                $(flexion::cell!($c91),)? "\t",
                $(flexion::cell!($c92),)? "\t",
                $(flexion::cell!($c93),)? "\t",
                $(flexion::cell!($c94),)? "\t",
                $(flexion::cell!($c95),)? "\t",
                $(flexion::cell!($c96),)? "\t",
                $(flexion::cell!($c97),)? "\t",
                $(flexion::cell!($c98),)? "\t",
                $(flexion::cell!($c99),)? "\t",
                $(flexion::cell!($c100),)? "\t",
                $(flexion::cell!($c101),)? "\t",
                $(flexion::cell!($c102),)? "\t",
                $(flexion::cell!($c103),)? "\t",
                $(flexion::cell!($c104),)? "\t",
                $(flexion::cell!($c105),)? "\t",
                $(flexion::cell!($c106),)? "\t",
                $(flexion::cell!($c107),)? "\t",
                $(flexion::cell!($c108),)? "\t",
                $(flexion::cell!($c109),)? "\t",
                $(flexion::cell!($c110),)? "\t",
                $(flexion::cell!($c111),)? "\t",
                $(flexion::cell!($c112),)? "\t",
                $(flexion::cell!($c113),)? "\t",
                $(flexion::cell!($c114),)? "\t",
                $(flexion::cell!($c115),)? "\t",
                $(flexion::cell!($c116),)? "\t",
                $(flexion::cell!($c117),)? "\t",
                $(flexion::cell!($c118),)? "\t",
                $(flexion::cell!($c119),)? "\t",
                $(flexion::cell!($c120),)? "\t",
                $(flexion::cell!($c121),)? "\t",
                $(flexion::cell!($c122),)? "\t",
                $(flexion::cell!($c123),)? "\t",
                $(flexion::cell!($c124),)? "\t",
                $(flexion::cell!($c125),)? "\t",
                $(flexion::cell!($c126),)? "\t",
                $(flexion::cell!($c127),)? "\t",
                $(flexion::cell!($c128),)? "\t",
                $(flexion::cell!($c129),)? "\t",
                $(flexion::cell!($c130),)? "\t",
                $(flexion::cell!($c131),)? "\t",
                $(flexion::cell!($c132),)? "\t",
                $(flexion::cell!($c133),)? "\t",
                $(flexion::cell!($c134),)? "\t",
                $(flexion::cell!($c135),)? "\t",
                $(flexion::cell!($c136),)? "\t",
                $(flexion::cell!($c137),)? "\t",
                $(flexion::cell!($c138),)? "\t",
                $(flexion::cell!($c139),)? "\t",
                $(flexion::cell!($c140),)? "\t",
                $(flexion::cell!($c141),)? "\t",
                $(flexion::cell!($c142),)? "\t",
                $(flexion::cell!($c143),)? "\t",
                $(flexion::cell!($c144),)? "\t",
                $(flexion::cell!($c145),)? "\t",
                $(flexion::cell!($c146),)? "\t",
                $(flexion::cell!($c147),)? "\t",
                $(flexion::cell!($c148),)? "\t",
                $(flexion::cell!($c149),)? "\t",
                $(flexion::cell!($c150),)? "\t",
                $(flexion::cell!($c151),)? "\t",
                $(flexion::cell!($c152),)? "\t",
                $(flexion::cell!($c153),)? "\t",
                $(flexion::cell!($c154),)? "\t",
                $(flexion::cell!($c155),)? "\t",
                $(flexion::cell!($c156),)? "\t",
                $(flexion::cell!($c157),)? "\t",
                $(flexion::cell!($c158),)? "\t",
                $(flexion::cell!($c159),)? "\t",
                $(flexion::cell!($c160),)? "\t",
                $(flexion::cell!($c161),)? "\t",
                $(flexion::cell!($c162),)? "\t",
                $(flexion::cell!($c163),)? "\t",
                $(flexion::cell!($c164),)? "\t",
                $(flexion::cell!($c165),)? "\t",
                $(flexion::cell!($c166),)? "\t",
                $(flexion::cell!($c167),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `purkaminen`: 525 words.
    Purkaminen,
    /// Like `taso`: 498 words.
    Taso,
    /// Like `pato`: 411 words.
    Pato,
    /// Like `tunnus`: 385 words.
    Tunnus,
    /// Like `pari`: 333 words.
    Pari,
    /// Like `kaappi`: 324 words.
    Kaappi,
    /// Like `osa`: 322 words.
    Osa,
    /// Like `sarja`: 311 words.
    Sarja,
    /// Like `ankkuri`: 254 words.
    Ankkuri,
    /// Like `ykkönen`: 208 words.
    Ykkönen,
    /// Like `pata`: 196 words.
    Pata,
    /// Like `erä`: 170 words.
    Erä,
    /// Like `nappula`: 161 words.
    Nappula,
    /// Like `oikeus`: 137 words.
    Oikeus,
    /// Like `keri`: 121 words.
    Keri,
    /// Like `tykitys`: 115 words.
    Tykitys,
    /// Like `aste`: 111 words.
    Aste,
    /// Like `konversio`: 107 words.
    Konversio,
    /// Like `levy`: 106 words.
    Levy,
    /// Like `suka`: 105 words.
    Suka,
    /// Like `piste`: 56 words.
    Piste,
    /// Like `yhteys`: 56 words.
    Yhteys,
    /// Like `silmukka`: 52 words.
    Silmukka,
    /// Like `avain`: 46 words.
    Avain,
    /// Like `selkä`: 44 words.
    Selkä,
    /// Like `kuori`: 43 words.
    Kuori,
    /// Like `suhde`: 41 words.
    Suhde,
    /// Like `parvi`: 34 words.
    Parvi,
    /// Like `tekijä`: 31 words.
    Tekijä,
    /// Like `kivi`: 30 words.
    Kivi,
    /// Like `tarvas`: 25 words.
    Tarvas,
    /// Like `kausi`: 25 words.
    Kausi,
    /// Like `osoitin`: 25 words.
    Osoitin,
    /// Like `kate`: 23 words.
    Kate,
    /// Like `liike`: 21 words.
    Liike,
    /// Like `hmong`: 20 words.
    Hmong,
    /// Like `äänenvaimennin`: 20 words.
    Äänenvaimennin,
    /// Like `kytkin`: 18 words.
    Kytkin,
    /// Like `puu`: 17 words.
    Puu,
    /// Like `etumies`: 17 words.
    Etumies,
    /// Like `käsi`: 17 words.
    Käsi,
    /// Like `luku`: 17 words.
    Luku,
    /// Like `peite`: 16 words.
    Peite,
    /// Like `rengas`: 15 words.
    Rengas,
    /// Like `ruis`: 15 words.
    Ruis,
    /// Like `lehti`: 14 words.
    Lehti,
    /// Like `sakset`: 14 words.
    Sakset,
    /// Like `aika`: 13 words.
    Aika,
    /// Like `kaanon`: 13 words.
    Kaanon,
    /// Like `lähde`: 13 words.
    Lähde,
    /// Like `olot`: 13 words.
    Olot,
    /// Like `siemen`: 12 words.
    Siemen,
    /// Like `pää`: 11 words.
    Pää,
    /// Like `koi`: 11 words.
    Koi,
    /// Like `hattara`: 10 words.
    Hattara,
    /// Like `perintä`: 10 words.
    Perintä,
    /// Like `väki`: 10 words.
    Väki,
    /// Like `maa`: 9 words.
    Maa,
    /// Like `työ`: 9 words.
    Työ,
    /// Like `kärki`: 9 words.
    Kärki,
    /// Like `verkko`: 8 words.
    Verkko,
    /// Like `kieli`: 8 words.
    Kieli,
    /// Like `jousi`: 8 words.
    Jousi,
    /// Like `hiili`: 8 words.
    Hiili,
    /// Like `siipi`: 8 words.
    Siipi,
    /// Like `toimi`: 7 words.
    Toimi,
    /// Like `keihäs`: 6 words.
    Keihäs,
    /// Like `kuningatar`: 6 words.
    Kuningatar,
    /// Like `kanapee`: 6 words.
    Kanapee,
    /// Like `maamerkki`: 6 words.
    Maamerkki,
    /// Like `värit`: 6 words.
    Värit,
    /// Like `koe`: 5 words.
    Koe,
    /// Like `jänne`: 5 words.
    Jänne,
    /// Like `khmer`: 5 words.
    Khmer,
    /// Like `taival`: 5 words.
    Taival,
    /// Like `syy`: 5 words.
    Syy,
    /// Like `askel`: 5 words.
    Askel,
    /// Like `kissanristiäiset`: 5 words.
    Kissanristiäiset,
    /// Like `lupa`: 5 words.
    Lupa,
    /// Like `paikka`: 4 words.
    Paikka,
    /// Like `kupu`: 4 words.
    Kupu,
    /// Like `tee`: 4 words.
    Tee,
    /// Like `poika`: 4 words.
    Poika,
    /// Like `hopea`: 4 words.
    Hopea,
    /// Like `ydin`: 4 words.
    Ydin,
    /// Like `kauris`: 4 words.
    Kauris,
    /// Like `rättänä`: 4 words.
    Rättänä,
    /// Like `takuu`: 4 words.
    Takuu,
    /// Like `käräjät`: 4 words.
    Käräjät,
    /// Like `vehje`: 4 words.
    Vehje,
    /// Like `onki`: 4 words.
    Onki,
    /// Like `piennar`: 4 words.
    Piennar,
    /// Like `leipä`: 3 words.
    Leipä,
    /// Like `body`: 3 words.
    Body,
    /// Like `hammas`: 3 words.
    Hammas,
    /// Like `komme`: 3 words.
    Komme,
    /// Like `leve`: 3 words.
    Leve,
    /// Like `bokserit`: 3 words.
    Bokserit,
    /// Like `opas`: 3 words.
    Opas,
    /// Like `lapsi`: 3 words.
    Lapsi,
    /// Like `markkinat`: 3 words.
    Markkinat,
    /// Like `ahdin`: 3 words.
    Ahdin,
    /// Like `valvojaiset`: 3 words.
    Valvojaiset,
    /// Like `hässäkkä`: 3 words.
    Hässäkkä,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 104] = [
        Declension::Purkaminen,
        Declension::Taso,
        Declension::Pato,
        Declension::Tunnus,
        Declension::Pari,
        Declension::Kaappi,
        Declension::Osa,
        Declension::Sarja,
        Declension::Ankkuri,
        Declension::Ykkönen,
        Declension::Pata,
        Declension::Erä,
        Declension::Nappula,
        Declension::Oikeus,
        Declension::Keri,
        Declension::Tykitys,
        Declension::Aste,
        Declension::Konversio,
        Declension::Levy,
        Declension::Suka,
        Declension::Piste,
        Declension::Yhteys,
        Declension::Silmukka,
        Declension::Avain,
        Declension::Selkä,
        Declension::Kuori,
        Declension::Suhde,
        Declension::Parvi,
        Declension::Tekijä,
        Declension::Kivi,
        Declension::Tarvas,
        Declension::Kausi,
        Declension::Osoitin,
        Declension::Kate,
        Declension::Liike,
        Declension::Hmong,
        Declension::Äänenvaimennin,
        Declension::Kytkin,
        Declension::Puu,
        Declension::Etumies,
        Declension::Käsi,
        Declension::Luku,
        Declension::Peite,
        Declension::Rengas,
        Declension::Ruis,
        Declension::Lehti,
        Declension::Sakset,
        Declension::Aika,
        Declension::Kaanon,
        Declension::Lähde,
        Declension::Olot,
        Declension::Siemen,
        Declension::Pää,
        Declension::Koi,
        Declension::Hattara,
        Declension::Perintä,
        Declension::Väki,
        Declension::Maa,
        Declension::Työ,
        Declension::Kärki,
        Declension::Verkko,
        Declension::Kieli,
        Declension::Jousi,
        Declension::Hiili,
        Declension::Siipi,
        Declension::Toimi,
        Declension::Keihäs,
        Declension::Kuningatar,
        Declension::Kanapee,
        Declension::Maamerkki,
        Declension::Värit,
        Declension::Koe,
        Declension::Jänne,
        Declension::Khmer,
        Declension::Taival,
        Declension::Syy,
        Declension::Askel,
        Declension::Kissanristiäiset,
        Declension::Lupa,
        Declension::Paikka,
        Declension::Kupu,
        Declension::Tee,
        Declension::Poika,
        Declension::Hopea,
        Declension::Ydin,
        Declension::Kauris,
        Declension::Rättänä,
        Declension::Takuu,
        Declension::Käräjät,
        Declension::Vehje,
        Declension::Onki,
        Declension::Piennar,
        Declension::Leipä,
        Declension::Body,
        Declension::Hammas,
        Declension::Komme,
        Declension::Leve,
        Declension::Bokserit,
        Declension::Opas,
        Declension::Lapsi,
        Declension::Markkinat,
        Declension::Ahdin,
        Declension::Valvojaiset,
        Declension::Hässäkkä,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 104] = [
    // Purkaminen: 525 words (nainen, suomalainen, valkoinen, punainen, hevonen, lautanen).
    declension!("purkaminen", 525, "~nen";
    stem pl: "~s- like purkaminen";
    nom: "~nen", "~set";
    par: "~stA", _;
    ine: "~sessA", "~sissA";
    ela: "~sestA", "~sistA";
    ill: "~seen", _;
    ade: "~sellA", "~sillA";
    abl: "~seltA", "~siltA";
    all: _, "~sille";
    ess: "~senA", _;
    abe: "~settA", "~sittA";
    possessive_nom_sg: "~seni", _, _, _, "~sensA";
    possessive_nom_pl: _, "~sesi", "~semme", "~senne", _;
    possessive_gen_sg: _, "~sesi", "~semme", "~senne", _;
    possessive_par_sg: _, _, _, _, "~stAVn|~stansa";
    possessive_ine_sg: _, _, _, _, "~sessAVn|~sessansa";
    possessive_ine_pl: "~sissAni", _, _, _, "~sissAVn|~sissansa";
    possessive_ela_sg: _, _, _, _, "~sestAVn|~sestansa";
    possessive_ela_pl: "~sistAni", _, _, _, "~sistAVn|~sistansa";
    possessive_ill_sg: _, _, _, _, "~seensA";
    possessive_ade_sg: _, _, _, _, "~sellAVn|~sellansa";
    possessive_ade_pl: "~sillAni", _, _, _, "~sillAVn|~sillansa";
    possessive_abl_sg: _, _, _, _, "~seltAVn|~seltansa";
    possessive_abl_pl: "~siltAni", _, _, _, "~siltAVn|~siltansa";
    possessive_all_sg: _, _, _, _, "~selleen|~sellensa";
    possessive_all_pl: "~silleni", _, _, _, "~silleen|~sillensa";
    possessive_ess_sg: _, _, _, _, "~senAVn|~senansa";
    possessive_tra_sg: _, _, _, _, "~sekseen|~seksensa";
    possessive_tra_pl: _, _, _, _, "~sikseen|~siksensa";
    possessive_abe_sg: _, _, _, _, "~settAVn|~settansa";
    possessive_abe_pl: "~sittAni", _, _, _, "~sittAVn|~sittansa";
    ),
    // Taso: 498 words (tempo, indigo, sombrero, ajo, dodo, menu). Like Purkaminen, except in 47 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("taso", 498, "~", like Purkaminen;
    stem sg: "~- like taso";
    stem pl: "~- like taso";
    nom: _, "~t";
    ine: "~ssA", "~issA";
    ela: "~stA", "~istA";
    ill: "~Vn", _;
    ade: "~llA", "~illA";
    abl: "~ltA", "~iltA";
    all: _, "~ille";
    ess: "~nA", _;
    abe: "~ttA", "~ittA";
    possessive_nom_sg: "~ni", _, _, _, "~nsA";
    possessive_nom_pl: _, "~si", "~mme", "~nne", _;
    possessive_gen_sg: _, "~si", "~mme", "~nne", _;
    possessive_gen_pl: _, derived, derived, derived, _;
    possessive_ine_sg: _, _, _, _, "~ssAVn|~ssansa";
    possessive_ine_pl: "~issAni", _, _, _, "~issAVn|~issansa";
    possessive_ela_sg: _, _, _, _, "~stAVn|~stansa";
    possessive_ela_pl: "~istAni", _, _, _, "~istAVn|~istansa";
    possessive_ill_sg: _, _, _, _, "~VnsA";
    possessive_ade_sg: _, _, _, _, "~llAVn|~llansa";
    possessive_ade_pl: "~illAni", _, _, _, "~illAVn|~illansa";
    possessive_abl_sg: _, _, _, _, "~ltAVn|~ltansa";
    possessive_abl_pl: "~iltAni", _, _, _, "~iltAVn|~iltansa";
    possessive_all_sg: _, _, _, _, "~lleen|~llensa";
    possessive_all_pl: "~illeni", _, _, _, "~illeen|~illensa";
    possessive_ess_sg: _, _, _, _, "~nAVn|~nansa";
    possessive_tra_sg: _, _, _, _, "~kseen|~ksensa";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensa";
    possessive_abe_sg: _, _, _, _, "~ttAVn|~ttansa";
    possessive_abe_pl: "~ittAni", _, _, _, "~ittAVn|~ittansa";
    ),
    // Pato: 411 words (pato, itu, satu, postiennakko, otto, veto). Like Taso with `ˇ` after each pattern.
    declension!("pato", 411, "~ˇ", like Taso with "ˇ";
    ),
    // Tunnus: 385 words (genus, ape, atlas, opus, tytär, bonus).
    declension!("tunnus", 385, "~~";
    stem sg: "~~- like tunnus";
    nom: _, "~k~et";
    gen_: _, "~~ten|~k~ien";
    par: _, "~k~iA";
    ine: "~k~essA", "~k~issA";
    ela: "~k~estA", "~k~istA";
    ill: "~k~een", "~k~iin";
    ade: "~k~ellA", "~k~illA";
    abl: "~k~eltA", "~k~iltA";
    all: _, "~k~ille";
    ess: "~k~enA", "~k~inA";
    abe: "~k~ettA", "~k~ittA";
    possessive_nom_sg: "~k~eni", _, _, _, "~k~ensA";
    possessive_nom_pl: _, "~k~esi", "~k~emme", "~k~enne", _;
    possessive_gen_sg: _, "~k~esi", "~k~emme", "~k~enne", _;
    possessive_gen_pl: "~~teni|~k~ieni", "~~tesi|~k~iesi", "~~temme|~k~iemme", "~~tenne|~k~ienne", "~~tensA|~k~iensa";
    possessive_par_pl: "~k~iAni", _, _, _, "~k~iAVn|~k~iansa";
    possessive_ine_sg: _, _, _, _, "~k~essAVn|~k~essansa";
    possessive_ine_pl: "~k~issAni", _, _, _, "~k~issAVn|~k~issansa";
    possessive_ela_sg: _, _, _, _, "~k~estAVn|~k~estansa";
    possessive_ela_pl: "~k~istAni", _, _, _, "~k~istAVn|~k~istansa";
    possessive_ill_sg: _, _, _, _, "~k~eensA";
    possessive_ill_pl: "~k~iini", _, _, _, "~k~iinsA";
    possessive_ade_sg: _, _, _, _, "~k~ellAVn|~k~ellansa";
    possessive_ade_pl: "~k~illAni", _, _, _, "~k~illAVn|~k~illansa";
    possessive_abl_sg: _, _, _, _, "~k~eltAVn|~k~eltansa";
    possessive_abl_pl: "~k~iltAni", _, _, _, "~k~iltAVn|~k~iltansa";
    possessive_all_sg: _, _, _, _, "~k~elleen|~k~ellensa";
    possessive_all_pl: "~k~illeni", _, _, _, "~k~illeen|~k~illensa";
    possessive_ess_sg: _, _, _, _, "~k~enAVn|~k~enansa";
    possessive_ess_pl: "~k~inAni", _, _, _, "~k~inAVn|~k~inansa";
    possessive_tra_sg: _, _, _, _, "~k~ekseen|~k~eksensa";
    possessive_tra_pl: _, _, _, _, "~k~ikseen|~k~iksensa";
    possessive_abe_sg: _, _, _, _, "~k~ettAVn|~k~ettansa";
    possessive_abe_pl: "~k~ittAni", _, _, _, "~k~ittAVn|~k~ittansa";
    possessive_com_pl: "~k~ineni", _, _, _, "~k~ineen|~k~inensa";
    ),
    // Pari: 333 words (khaki, kali, wasabi, kaali, tali, uuni). Like Taso, except in 58 cells: nom pl, gen_ pl, par pl, ine sg, ine pl, ela sg, ….
    declension!("pari", 333, "~i", like Taso;
    stem sg: "~i- like taso";
    nom: _, "~it";
    gen_: _, "~ien";
    par: _, "~ejA";
    ine: "~issA", "~eissA";
    ela: "~istA", "~eistA";
    ill: "~iVn", "~eihin";
    ade: "~illA", "~eillA";
    abl: "~iltA", "~eiltA";
    all: _, "~eille";
    ess: "~inA", "~einA";
    abe: "~ittA", "~eittA";
    possessive_nom_sg: "~ini", _, _, _, "~insA";
    possessive_nom_pl: _, "~isi", "~imme", "~inne", _;
    possessive_gen_sg: _, "~isi", "~imme", "~inne", _;
    possessive_gen_pl: "~ieni", _, _, _, "~iensA";
    possessive_par_pl: "~ejAni", _, _, _, "~ejAVn|~ejansa";
    possessive_ine_sg: _, _, _, _, "~issAVn|~issansa";
    possessive_ine_pl: "~eissAni", _, _, _, "~eissAVn|~eissansa";
    possessive_ela_sg: _, _, _, _, "~istAVn|~istansa";
    possessive_ela_pl: "~eistAni", _, _, _, "~eistAVn|~eistansa";
    possessive_ill_sg: _, _, _, _, "~iVnsA";
    possessive_ill_pl: "~eihVni", _, _, _, "~eihinsA";
    possessive_ade_sg: _, _, _, _, "~illAVn|~illansa";
    possessive_ade_pl: "~eillAni", _, _, _, "~eillAVn|~eillansa";
    possessive_abl_sg: _, _, _, _, "~iltAVn|~iltansa";
    possessive_abl_pl: "~eiltAni", _, _, _, "~eiltAVn|~eiltansa";
    possessive_all_sg: _, _, _, _, "~illeen|~illensa";
    possessive_all_pl: "~eilleni", _, _, _, "~eilleen|~eillensa";
    possessive_ess_sg: _, _, _, _, "~inAVn|~inansa";
    possessive_ess_pl: "~einAni", _, _, _, "~einAVn|~einansa";
    possessive_tra_sg: _, _, _, _, "~ikseen|~iksensa";
    possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensa";
    possessive_abe_sg: _, _, _, _, "~ittAVn|~ittansa";
    possessive_abe_pl: "~eittAni", _, _, _, "~eittAVn|~eittansa";
    possessive_com_pl: "~eineni", _, _, _, "~eineen|~einensa";
    ),
    // Kaappi: 324 words (kaappi, äiti, sekunti, voltti, täti, turkki). Like Pari with `ˇ` after each pattern.
    declension!("kaappi", 324, "~iˇ", like Pari with "ˇ";
    ),
    // Osa: 322 words (koira, pula, musta, korva, kuula, kura). Like Taso with `` after each pattern (`a` after ~), except in 36 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("osa", 322, "~a", like Taso with "", "a" after "~";
    nom: _, "~at";
    gen_: _, "~ien";
    par: "~aV", "~iA";
    ine: "~assV", _;
    ela: "~astV", _;
    ill: "~aVn", "~iin";
    ade: "~allV", _;
    abl: "~altV", _;
    ess: "~anV", _;
    abe: "~attV", _;
    possessive_nom_sg: "~ani", _, _, _, "~ansV";
    possessive_nom_pl: _, "~asi", "~amme", "~anne", _;
    possessive_gen_sg: _, "~asi", "~amme", "~anne", _;
    possessive_gen_pl: "~ieni", _, _, _, "~iensA";
    possessive_par_sg: _, _, _, _, "~aVnsV";
    possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iansa";
    possessive_ine_sg: _, _, _, _, "~assVVn|~assansa";
    possessive_ela_sg: _, _, _, _, "~astVVn|~astansa";
    possessive_ill_sg: _, _, _, _, "~aVnsV";
    possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    possessive_ade_sg: _, _, _, _, "~allVVn|~allansa";
    possessive_abl_sg: _, _, _, _, "~altVVn|~altansa";
    possessive_all_sg: _, _, _, _, "~alleen|~allensa";
    possessive_ess_sg: _, _, _, _, "~anVVn|~anansa";
    possessive_tra_sg: _, _, _, _, "~akseen|~aksensa";
    possessive_abe_sg: _, _, _, _, "~attVVn|~attansa";
    ),
    // Sarja: 311 words (lama, salsa, matka, kara, sana, gamma). Like Osa, except in 19 cells: ine pl, ela pl, ade pl, abl pl, all pl, abe pl, ….
    declension!("sarja", 311, "~a", like Osa;
    stem pl: "~o- like sarja";
    ine: _, "~oissA";
    ela: _, "~oistA";
    ade: _, "~oillA";
    abl: _, "~oiltA";
    all: _, "~oille";
    abe: _, "~oittA";
    possessive_ine_pl: "~oissAni", _, _, _, "~oissAVn|~oissansa";
    possessive_ela_pl: "~oistAni", _, _, _, "~oistAVn|~oistansa";
    possessive_ade_pl: "~oillAni", _, _, _, "~oillAVn|~oillansa";
    possessive_abl_pl: "~oiltAni", _, _, _, "~oiltAVn|~oiltansa";
    possessive_all_pl: "~oilleni", _, _, _, "~oilleen|~oillensa";
    possessive_tra_pl: _, _, _, _, "~oikseen|~oiksensa";
    possessive_abe_pl: "~oittAni", _, _, _, "~oittAVn|~oittansa";
    ),
    // Ankkuri: 254 words (ämpäri, banaani, mittari, versaali, paperi, moottori). Like Hmong with `` after each pattern (`i` after ~), except in 18 cells: gen_ pl, par pl, com pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, ….
    declension!("ankkuri", 254, "~i", like Hmong with "", "i" after "~";
    gen_: _, "~ien|~eiden|~eitten";
    par: _, "~eitA|~eja";
    com: _, derived;
    possessive_gen_pl: "~ieni|~eideni|~eitteni", "~iesi|~eidesi|~eittesi", "~iemme|~eidemme|~eittemme", "~ienne|~eidenne|~eittenne", "~iensA|~eidensa|~eittensa";
    possessive_par_pl: "~eitAni|~ejani", "~eitAsi|~ejasi", "~eitAmme|~ejamme", "~eitAnne|~ejanne", "~eitAVn|~ejaan|~eitansa|~ejansa";
    possessive_ill_sg: derived, derived, derived, derived, _;
    possessive_ill_pl: "~eihVni", _, _, _, _;
    ),
    // Ykkönen: 208 words (kääntäminen, etiäinen, mehiläinen, ihminen, herhiläinen, pistiäinen). Like Purkaminen, except in 19 cells: possessive_par_sg _3, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, ….
    declension!("ykkönen", 208, "~nen", like Purkaminen;
    possessive_par_sg: _, _, _, _, "~stAVn|~stänsä";
    possessive_par_pl: _, _, _, _, "~siAVn|~siänsä";
    possessive_ine_sg: _, _, _, _, "~sessAVn|~sessänsä";
    possessive_ine_pl: _, _, _, _, "~sissAVn|~sissänsä";
    possessive_ela_sg: _, _, _, _, "~sestAVn|~sestänsä";
    possessive_ela_pl: _, _, _, _, "~sistAVn|~sistänsä";
    possessive_ade_sg: _, _, _, _, "~sellAVn|~sellänsä";
    possessive_ade_pl: _, _, _, _, "~sillAVn|~sillänsä";
    possessive_abl_sg: _, _, _, _, "~seltAVn|~seltänsä";
    possessive_abl_pl: _, _, _, _, "~siltAVn|~siltänsä";
    possessive_all_sg: _, _, _, _, "~selleen|~sellensä";
    possessive_all_pl: _, _, _, _, "~silleen|~sillensä";
    possessive_ess_sg: _, _, _, _, "~senAVn|~senänsä";
    possessive_ess_pl: _, _, _, _, "~sinAVn|~sinänsä";
    possessive_tra_sg: _, _, _, _, "~sekseen|~seksensä";
    possessive_tra_pl: _, _, _, _, "~sikseen|~siksensä";
    possessive_abe_sg: _, _, _, _, "~settAVn|~settänsä";
    possessive_abe_pl: _, _, _, _, "~sittAVn|~sittänsä";
    possessive_com_pl: _, _, _, _, "~sineen|~sinensä";
    ),
    // Pata: 196 words (pata, rata, kita, kappa, markka, virta). Like Sarja with `ˇ` after each pattern.
    declension!("pata", 196, "~aˇ", like Sarja with "ˇ";
    ),
    // Erä: 170 words (päivä, pitkä, isä, kynä, vihreä, kesä). Like Osa, except in 42 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("erä", 170, "~ä", like Osa;
    stem sg: "~ä- like osa";
    stem pl: "~i- like erä";
    nom: _, "~ät";
    ine: "~ässV", "~issä";
    ela: "~ästV", "~istä";
    ill: "~äVn", _;
    ade: "~ällV", "~illä";
    abl: "~ältV", "~iltä";
    ess: "~änV", _;
    abe: "~ättV", "~ittä";
    possessive_nom_sg: "~äni", _, _, _, "~änsV";
    possessive_nom_pl: _, "~äsi", "~ämme", "~änne", _;
    possessive_gen_sg: _, "~äsi", "~ämme", "~änne", _;
    possessive_ine_sg: _, _, _, _, "~ässVVn|~ässänsä";
    possessive_ine_pl: "~issäni", _, _, _, "~issäVn|~issänsä";
    possessive_ela_sg: _, _, _, _, "~ästVVn|~ästänsä";
    possessive_ela_pl: "~istäni", _, _, _, "~istäVn|~istänsä";
    possessive_ill_sg: _, _, _, _, "~äVnsV";
    possessive_ade_sg: _, _, _, _, "~ällVVn|~ällänsä";
    possessive_ade_pl: "~illäni", _, _, _, "~illäVn|~illänsä";
    possessive_abl_sg: _, _, _, _, "~ältVVn|~ältänsä";
    possessive_abl_pl: "~iltäni", _, _, _, "~iltäVn|~iltänsä";
    possessive_all_sg: _, _, _, _, "~älleen|~ällensä";
    possessive_all_pl: _, _, _, _, "~illeen|~illensä";
    possessive_ess_sg: _, _, _, _, "~änVVn|~änänsä";
    possessive_tra_sg: _, _, _, _, "~äkseen|~äksensä";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensä";
    possessive_abe_sg: _, _, _, _, "~ättVVn|~ättänsä";
    possessive_abe_pl: "~ittäni", _, _, _, "~ittäVn|~ittänsä";
    ),
    // Nappula: 161 words (tortilla, harmonia, porkkana, ikkuna, gemena, vasara). Like Sarja.
    declension!("nappula", 161, "~a", like Sarja;
    stem pl: "~oi- like nappula";
    ),
    // Oikeus: 137 words (nopeus, tilavuus, pituus, hiljaisuus, vinous, avaruus). Like Taso, except in 47 cells: nom sg, nom pl, par sg, ine sg, ine pl, ela sg, ….
    declension!("oikeus", 137, "~s", like Taso;
    stem pl: "~ksi- like oikeus";
    nom: "~s", "~det";
    par: "~tTA", _;
    ine: "~dessA", "~ksissA";
    ela: "~destA", "~ksistA";
    ill: "~teVn", _;
    ade: "~dellA", "~ksillA";
    abl: "~deltA", "~ksiltA";
    all: _, "~ksille";
    ess: "~tenA", _;
    abe: "~dettA", "~ksittA";
    possessive_nom_sg: "~teni", _, _, _, "~tensA";
    possessive_nom_pl: _, "~tesi", "~temme", "~tenne", _;
    possessive_gen_sg: _, "~tesi", "~temme", "~tenne", _;
    possessive_par_sg: _, _, _, _, "~tTAVn|~ttansa";
    possessive_ine_sg: _, _, _, _, "~dessAVn|~dessansa";
    possessive_ine_pl: "~ksissAni", _, _, _, "~ksissAVn|~ksissansa";
    possessive_ela_sg: _, _, _, _, "~destAVn|~destansa";
    possessive_ela_pl: "~ksistAni", _, _, _, "~ksistAVn|~ksistansa";
    possessive_ill_sg: _, _, _, _, "~teVnsA";
    possessive_ade_sg: _, _, _, _, "~dellAVn|~dellansa";
    possessive_ade_pl: "~ksillAni", _, _, _, "~ksillAVn|~ksillansa";
    possessive_abl_sg: _, _, _, _, "~deltAVn|~deltansa";
    possessive_abl_pl: "~ksiltAni", _, _, _, "~ksiltAVn|~ksiltansa";
    possessive_all_sg: _, _, _, _, "~delleen|~dellensa";
    possessive_all_pl: "~ksilleni", _, _, _, "~ksilleen|~ksillensa";
    possessive_ess_sg: _, _, _, _, "~tenAVn|~tenansa";
    possessive_tra_sg: _, _, _, _, "~dekseen|~deksensa";
    possessive_tra_pl: _, _, _, _, "~ksikseen|~ksiksensa";
    possessive_abe_sg: _, _, _, _, "~dettAVn|~dettansa";
    possessive_abe_pl: "~ksittAni", _, _, _, "~ksittAVn|~ksittansa";
    ),
    // Keri: 121 words (keksi, peili, väri, seksi, peli, viini). Like Pari, except in 35 cells: par sg, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("keri", 121, "~i", like Pari;
    stem pl: "~- like keri";
    par: "~iä", _;
    ine: "~issä", "~eissä";
    ela: "~istä", "~eistä";
    ade: "~illä", "~eillä";
    abl: "~iltä", "~eiltä";
    ess: "~inä", _;
    abe: "~ittä", "~eittä";
    possessive_nom_sg: _, _, _, _, "~insä";
    possessive_par_sg: _, _, _, _, "~iäVn|~iänsä";
    possessive_ine_sg: _, _, _, _, "~issäVn|~issänsä";
    possessive_ine_pl: "~eissäni", _, _, _, "~eissäVn|~eissänsä";
    possessive_ela_sg: _, _, _, _, "~istäVn|~istänsä";
    possessive_ela_pl: "~eistäni", _, _, _, "~eistäVn|~eistänsä";
    possessive_ill_sg: _, _, _, _, "~iVnsä";
    possessive_ade_sg: _, _, _, _, "~illäVn|~illänsä";
    possessive_ade_pl: "~eilläni", _, _, _, "~eilläVn|~eillänsä";
    possessive_abl_sg: _, _, _, _, "~iltäVn|~iltänsä";
    possessive_abl_pl: "~eiltäni", _, _, _, "~eiltäVn|~eiltänsä";
    possessive_all_sg: _, _, _, _, "~illeen|~illensä";
    possessive_all_pl: _, _, _, _, "~eilleen|~eillensä";
    possessive_ess_sg: _, _, _, _, "~inäVn|~inänsä";
    possessive_tra_sg: _, _, _, _, "~ikseen|~iksensä";
    possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensä";
    possessive_abe_sg: _, _, _, _, "~ittäVn|~ittänsä";
    possessive_abe_pl: "~eittäni", _, _, _, "~eittäVn|~eittänsä";
    ),
    // Tykitys: 115 words (kehys, kysymys, jänis, kives, käännös, pyhimys). Like Tunnus, except in 19 cells: possessive_gen_pl _3, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, ….
    declension!("tykitys", 115, "~~", like Tunnus;
    stem sg: "~~- like tykitys";
    possessive_gen_pl: _, _, _, _, "~~tensA|~k~iensä";
    possessive_par_pl: _, _, _, _, "~k~iAVn|~k~iänsä";
    possessive_ine_sg: _, _, _, _, "~k~essAVn|~k~essänsä";
    possessive_ine_pl: _, _, _, _, "~k~issAVn|~k~issänsä";
    possessive_ela_sg: _, _, _, _, "~k~estAVn|~k~estänsä";
    possessive_ela_pl: _, _, _, _, "~k~istAVn|~k~istänsä";
    possessive_ade_sg: _, _, _, _, "~k~ellAVn|~k~ellänsä";
    possessive_ade_pl: _, _, _, _, "~k~illAVn|~k~illänsä";
    possessive_abl_sg: _, _, _, _, "~k~eltAVn|~k~eltänsä";
    possessive_abl_pl: _, _, _, _, "~k~iltAVn|~k~iltänsä";
    possessive_all_sg: _, _, _, _, "~k~elleen|~k~ellensä";
    possessive_all_pl: _, _, _, _, "~k~illeen|~k~illensä";
    possessive_ess_sg: _, _, _, _, "~k~enAVn|~k~enänsä";
    possessive_ess_pl: _, _, _, _, "~k~inAVn|~k~inänsä";
    possessive_tra_sg: _, _, _, _, "~k~ekseen|~k~eksensä";
    possessive_tra_pl: _, _, _, _, "~k~ikseen|~k~iksensä";
    possessive_abe_sg: _, _, _, _, "~k~ettAVn|~k~ettänsä";
    possessive_abe_pl: _, _, _, _, "~k~ittAVn|~k~ittänsä";
    possessive_com_pl: _, _, _, _, "~k~ineen|~k~inensä";
    ),
    // Aste: 111 words (kale, tietokone, purse, sane, huone, aave). Like Taso, except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("aste", 111, "~", like Taso;
    stem sg: "~- like aste";
    stem pl: "~i- like aste";
    nom: _, "~Vt";
    ine: "~VssA", _;
    ela: "~VstA", _;
    ill: "~eseen", _;
    ade: "~VllA", _;
    abl: "~VltA", _;
    ess: "~VnA", _;
    abe: "~VttA", _;
    possessive_nom_sg: "~Vni", _, _, _, "~VnsA";
    possessive_nom_pl: _, "~Vsi", "~Vmme", "~Vnne", _;
    possessive_gen_sg: _, "~Vsi", "~Vmme", "~Vnne", _;
    possessive_ine_sg: _, _, _, _, "~VssAVn|~essansa";
    possessive_ela_sg: _, _, _, _, "~VstAVn|~estansa";
    possessive_ill_sg: _, _, _, _, "~eseensA";
    possessive_ade_sg: _, _, _, _, "~VllAVn|~ellansa";
    possessive_abl_sg: _, _, _, _, "~VltAVn|~eltansa";
    possessive_all_sg: _, _, _, _, "~Vlleen|~ellensa";
    possessive_ess_sg: _, _, _, _, "~VnAVn|~enansa";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~eksensa";
    possessive_abe_sg: _, _, _, _, "~VttAVn|~ettansa";
    ),
    // Konversio: 107 words (radio, valtio, tau, video, olio, sikiö). Like Taso.
    declension!("konversio", 107, "~", like Taso;
    stem sg: "~- like tunnus";
    stem pl: "~- like konversio";
    ),
    // Levy: 106 words (baby, ewe, öljy, whisky, käly, levy). Like Taso, except in 16 cells: possessive_par_sg _3, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, possessive_ade_sg _3, ….
    declension!("levy", 106, "~", like Taso;
    stem pl: "~- like levy";
    possessive_par_sg: _, _, _, _, "~AVn|~änsä";
    possessive_ine_sg: _, _, _, _, "~ssAVn|~ssänsä";
    possessive_ine_pl: _, _, _, _, "~issAVn|~issänsä";
    possessive_ela_sg: _, _, _, _, "~stAVn|~stänsä";
    possessive_ela_pl: _, _, _, _, "~istAVn|~istänsä";
    possessive_ade_sg: _, _, _, _, "~llAVn|~llänsä";
    possessive_ade_pl: _, _, _, _, "~illAVn|~illänsä";
    possessive_abl_sg: _, _, _, _, "~ltAVn|~ltänsä";
    possessive_abl_pl: _, _, _, _, "~iltAVn|~iltänsä";
    possessive_all_sg: _, _, _, _, "~lleen|~llensä";
    possessive_all_pl: _, _, _, _, "~illeen|~illensä";
    possessive_ess_sg: _, _, _, _, "~nAVn|~nänsä";
    possessive_tra_sg: _, _, _, _, "~kseen|~ksensä";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensä";
    possessive_abe_sg: _, _, _, _, "~ttAVn|~ttänsä";
    possessive_abe_pl: _, _, _, _, "~ittAVn|~ittänsä";
    ),
    // Suka: 105 words (suka, lupa, toukka, kulta, kukka, sukka). Like Maa with `ˇ` after each pattern, except in 16 cells: gen_ pl, par sg, par pl, ill sg, ill pl, possessive_gen_pl _1sg, ….
    declension!("suka", 105, "~aˇ", like Maa with "ˇ";
    gen_: _, "~ienˇ";
    par: "~aVˇ", "~iAˇ";
    ill: "~aVnˇ", "~iinˇ";
    possessive_gen_pl: "~ieniˇ", derived, derived, derived, "~iensAˇ";
    possessive_par_sg: _, _, _, _, "~aVnsVˇ";
    possessive_par_pl: "~iAniˇ", _, _, _, "~iAVnˇ|~iansaˇ";
    possessive_ill_sg: _, _, _, _, "~aVnsVˇ";
    possessive_ill_pl: "~iiniˇ", _, _, _, "~iinsAˇ";
    ),
    // Piste: 56 words (sine, piste, herne, hie, este, neste). Like Erä, except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("piste", 56, "~", like Erä;
    stem sg: "~- like piste";
    stem pl: "~i- like piste";
    nom: _, "~Vt";
    ine: "~Vssä", _;
    ela: "~Vstä", _;
    ill: "~eseen", _;
    ade: "~Vllä", _;
    abl: "~Vltä", _;
    ess: "~Vnä", _;
    abe: "~Vttä", _;
    possessive_nom_sg: "~Vni", _, _, _, "~Vnsä";
    possessive_nom_pl: _, "~Vsi", "~Vmme", "~Vnne", _;
    possessive_gen_sg: _, "~Vsi", "~Vmme", "~Vnne", _;
    possessive_ine_sg: _, _, _, _, "~VssäVn|~essänsä";
    possessive_ela_sg: _, _, _, _, "~VstäVn|~estänsä";
    possessive_ill_sg: _, _, _, _, "~eseensä";
    possessive_ade_sg: _, _, _, _, "~VlläVn|~ellänsä";
    possessive_abl_sg: _, _, _, _, "~VltäVn|~eltänsä";
    possessive_all_sg: _, _, _, _, "~Vlleen|~ellensä";
    possessive_ess_sg: _, _, _, _, "~VnäVn|~enänsä";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~eksensä";
    possessive_abe_sg: _, _, _, _, "~VttäVn|~ettänsä";
    ),
    // Yhteys: 56 words (tiheys, leveys, syyllisyys, typeryys, näkyvyys, kansainvälisyys). Like Oikeus, except in 19 cells: possessive_par_sg _3, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, ….
    declension!("yhteys", 56, "~s", like Oikeus;
    possessive_par_sg: _, _, _, _, "~tTAVn|~ttänsä";
    possessive_par_pl: _, _, _, _, "~ksiAVn|~ksiänsä";
    possessive_ine_sg: _, _, _, _, "~dessAVn|~dessänsä";
    possessive_ine_pl: _, _, _, _, "~ksissAVn|~ksissänsä";
    possessive_ela_sg: _, _, _, _, "~destAVn|~destänsä";
    possessive_ela_pl: _, _, _, _, "~ksistAVn|~ksistänsä";
    possessive_ade_sg: _, _, _, _, "~dellAVn|~dellänsä";
    possessive_ade_pl: _, _, _, _, "~ksillAVn|~ksillänsä";
    possessive_abl_sg: _, _, _, _, "~deltAVn|~deltänsä";
    possessive_abl_pl: _, _, _, _, "~ksiltAVn|~ksiltänsä";
    possessive_all_sg: _, _, _, _, "~delleen|~dellensä";
    possessive_all_pl: _, _, _, _, "~ksilleen|~ksillensä";
    possessive_ess_sg: _, _, _, _, "~tenAVn|~tenänsä";
    possessive_ess_pl: _, _, _, _, "~ksinAVn|~ksinänsä";
    possessive_tra_sg: _, _, _, _, "~dekseen|~deksensä";
    possessive_tra_pl: _, _, _, _, "~ksikseen|~ksiksensä";
    possessive_abe_sg: _, _, _, _, "~dettAVn|~dettänsä";
    possessive_abe_pl: _, _, _, _, "~ksittAVn|~ksittänsä";
    possessive_com_pl: _, _, _, _, "~ksineen|~ksinensä";
    ),
    // Silmukka: 52 words (lusikka, haarukka, harakka, mustikka, puolukka, tupakka). Like Sarja, except in 12 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _3, possessive_nom_pl _2sg, possessive_nom_pl _1pl, ….
    declension!("silmukka", 52, "~ka", like Sarja;
    stem sg: "~ka- like osa";
    stem pl: "~- like silmukka";
    ill: "~kaVn", _;
    ess: "~kanV", _;
    possessive_nom_sg: "~kani", _, _, _, "~kansV";
    possessive_nom_pl: _, "~kasi", "~kamme", "~kanne", _;
    possessive_gen_sg: _, "~kasi", "~kamme", "~kanne", _;
    possessive_ill_sg: _, _, _, _, "~kaVnsV";
    possessive_ess_sg: _, _, _, _, "~kanVVn|~kanansa";
    ),
    // Avain: 46 words (puhelin, kasvain, valaisin, morsian, avain, vahvistin). Like Purkaminen, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("avain", 46, "~n", like Purkaminen;
    stem sg: "~n- like tunnus";
    stem pl: "~- like avain";
    nom: _, "~met";
    ine: "~messA", "~missA";
    ela: "~mestA", "~mistA";
    ill: "~meVn", _;
    ade: "~mellA", "~millA";
    abl: "~meltA", "~miltA";
    all: _, "~mille";
    ess: "~menA", _;
    abe: "~mettA", "~mittA";
    possessive_nom_sg: "~meni", _, _, _, "~mensA";
    possessive_nom_pl: _, "~mesi", "~memme", "~menne", _;
    possessive_gen_sg: _, "~mesi", "~memme", "~menne", _;
    possessive_ine_sg: _, _, _, _, "~messAVn|~messansa";
    possessive_ine_pl: "~missAni", _, _, _, "~missAVn|~missansa";
    possessive_ela_sg: _, _, _, _, "~mestAVn|~mestansa";
    possessive_ela_pl: "~mistAni", _, _, _, "~mistAVn|~mistansa";
    possessive_ill_sg: _, _, _, _, "~meVnsA";
    possessive_ade_sg: _, _, _, _, "~mellAVn|~mellansa";
    possessive_ade_pl: "~millAni", _, _, _, "~millAVn|~millansa";
    possessive_abl_sg: _, _, _, _, "~meltAVn|~meltansa";
    possessive_abl_pl: "~miltAni", _, _, _, "~miltAVn|~miltansa";
    possessive_all_sg: _, _, _, _, "~melleen|~mellensa";
    possessive_all_pl: "~milleni", _, _, _, "~milleen|~millensa";
    possessive_ess_sg: _, _, _, _, "~menAVn|~menansa";
    possessive_tra_sg: _, _, _, _, "~mekseen|~meksensa";
    possessive_tra_pl: _, _, _, _, "~mikseen|~miksensa";
    possessive_abe_sg: _, _, _, _, "~mettAVn|~mettansa";
    possessive_abe_pl: "~mittAni", _, _, _, "~mittAVn|~mittansa";
    ),
    // Selkä: 44 words (leipä, kenkä, setä, ikä, itä, kenttä). Like Erä with `ˇ` after each pattern.
    declension!("selkä", 44, "~äˇ", like Erä with "ˇ";
    ),
    // Kuori: 43 words (tuuli, nuori, vuori, kieli, ääni, juuri). Like Konversio with `` after each pattern (`i` after ~), except in 42 cells: nom pl, gen_ pl, par pl, ine sg, ela sg, ill sg, ….
    declension!("kuori", 43, "~i", like Konversio with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ten|~ien";
    par: _, "~iA";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~eVn", "~iin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    ess: "~enA", _;
    abe: "~ettA", _;
    ins: "~en", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensA";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensA|~iensA";
    possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iansa";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essansa";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estansa";
    possessive_ill_sg: _, _, _, _, "~eVnsA";
    possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellansa";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltansa";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensa";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enansa";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensa";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettansa";
    possessive_ins_sg: _, "~esi", "~emme", "~enne", "~ensa";
    ),
    // Suhde: 41 words (sade, aarre, anne, luode, vanne, helle). Like Aste with `ˇ` after each pattern.
    declension!("suhde", 41, "~ˇ", like Aste with "ˇ";
    ),
    // Parvi: 34 words (kuusi, sampi, takatalvi, suvi, sormi, povi). Like Taso with `` after each pattern (`i` after ~), except in 36 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("parvi", 34, "~i", like Taso with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ien";
    par: "~eA", "~iA";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~eVn", "~iin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    ess: "~enA", _;
    abe: "~ettA", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensA";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_gen_pl: "~ieni", _, _, _, "~iensA";
    possessive_par_sg: _, _, _, _, "~eAVn|~eansa";
    possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iansa";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essansa";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estansa";
    possessive_ill_sg: _, _, _, _, "~eVnsA";
    possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellansa";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltansa";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensa";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enansa";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensa";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettansa";
    ),
    // Tekijä: 31 words (päärynä, ympyrä, pähkinä, tekijä, kylpylä, näsiä). Like Erä, except in 19 cells: ine pl, ela pl, ade pl, abl pl, all pl, abe pl, ….
    declension!("tekijä", 31, "~ä", like Erä;
    stem pl: "~öi- like tekijä";
    ine: _, "~öissA";
    ela: _, "~öistA";
    ade: _, "~öillA";
    abl: _, "~öiltA";
    all: _, "~öille";
    abe: _, "~öittA";
    possessive_ine_pl: "~öissAni", _, _, _, "~öissAVn|~öissänsä";
    possessive_ela_pl: "~öistAni", _, _, _, "~öistAVn|~öistänsä";
    possessive_ade_pl: "~öillAni", _, _, _, "~öillAVn|~öillänsä";
    possessive_abl_pl: "~öiltAni", _, _, _, "~öiltAVn|~öiltänsä";
    possessive_all_pl: "~öilleni", _, _, _, "~öilleen|~öillensä";
    possessive_tra_pl: _, _, _, _, "~öikseen|~öiksensä";
    possessive_abe_pl: "~öittAni", _, _, _, "~öittAVn|~öittänsä";
    ),
    // Kivi: 30 words (pilvi, hirvi, kivi, liemi, tyvi, helmi). Like Piste with `` after each pattern (`i` after ~), except in 42 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("kivi", 30, "~i", like Piste with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ien";
    par: "~eä", "~iä";
    ine: "~essä", _;
    ela: "~estä", _;
    ill: "~eVn", "~iin";
    ade: "~ellä", _;
    abl: "~eltä", _;
    ess: "~enä", _;
    abe: "~että", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensä";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_gen_pl: "~ieni", derived, derived, derived, "~iensä";
    possessive_par_sg: _, _, _, _, "~eäVn|~eänsä";
    possessive_par_pl: "~iäni", _, _, _, "~iäVn|~iänsä";
    possessive_ine_sg: _, _, _, _, "~essäVn|~essänsä";
    possessive_ela_sg: _, _, _, _, "~estäVn|~estänsä";
    possessive_ill_sg: _, _, _, _, "~eVnsä";
    possessive_ill_pl: "~iini", derived, derived, derived, "~iinsä";
    possessive_ade_sg: _, _, _, _, "~elläVn|~ellänsä";
    possessive_abl_sg: _, _, _, _, "~eltäVn|~eltänsä";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensä";
    possessive_ess_sg: _, _, _, _, "~enäVn|~enänsä";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensä";
    possessive_abe_sg: _, _, _, _, "~ettäVn|~ettänsä";
    ),
    // Tarvas: 25 words (taivas, porsas, oinas, ankerias, lounas, patsas). Like Aste with `` after each pattern (`s` after ~), except in 25 cells: par sg, ine sg, ela sg, ill sg, ill pl, ade sg, ….
    declension!("tarvas", 25, "~s", like Aste with "", "s" after "~";
    par: "~stV", _;
    ine: "~VssV", _;
    ela: "~VstV", _;
    ill: "~VseVn", "~isVin";
    ade: "~VllV", _;
    abl: "~VltV", _;
    ess: "~VnV", _;
    abe: "~VttV", _;
    possessive_nom_sg: _, _, _, _, "~VnsV";
    possessive_par_sg: _, _, _, _, "~stVVn|~stansa";
    possessive_ine_sg: _, _, _, _, "~VssVVn|~assansa";
    possessive_ela_sg: _, _, _, _, "~VstVVn|~astansa";
    possessive_ill_sg: _, _, _, _, "~VseVnsA";
    possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    possessive_ade_sg: _, _, _, _, "~VllVVn|~allansa";
    possessive_abl_sg: _, _, _, _, "~VltVVn|~altansa";
    possessive_all_sg: _, _, _, _, "~Vlleen|~allensa";
    possessive_ess_sg: _, _, _, _, "~VnVVn|~anansa";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~aksensa";
    possessive_abe_sg: _, _, _, _, "~VttVVn|~attansa";
    ),
    // Kausi: 25 words (kynsi, susi, länsi, kausi, vuorokausi, varsi). Like Oikeus with `` after each pattern (`i` after s), except in 33 cells: gen_ pl, par pl, ine pl, ela pl, ill pl, ade pl, ….
    declension!("kausi", 25, "~si", like Oikeus with "", "i" after "s";
    gen_: _, "~sien";
    par: _, "~siA";
    ine: _, "~sissA";
    ela: _, "~sistA";
    ill: _, "~siin";
    ade: _, "~sillA";
    abl: _, "~siltA";
    all: _, "~sille";
    ess: _, "~sinA";
    abe: _, "~sittA";
    possessive_gen_pl: "~sieni", _, _, _, "~siensA";
    possessive_par_pl: "~siAni", _, _, _, "~siAVn|~siansa";
    possessive_ine_pl: "~sissAni", _, _, _, "~sissAVn|~sissansa";
    possessive_ela_pl: "~sistAni", _, _, _, "~sistAVn|~sistansa";
    possessive_ill_pl: "~siini", _, _, _, "~siinsA";
    possessive_ade_pl: "~sillAni", _, _, _, "~sillAVn|~sillansa";
    possessive_abl_pl: "~siltAni", _, _, _, "~siltAVn|~siltansa";
    possessive_all_pl: "~silleni", _, _, _, "~silleen|~sillensa";
    possessive_ess_pl: "~sinAni", _, _, _, "~sinAVn|~sinansa";
    possessive_tra_pl: _, _, _, _, "~sikseen|~siksensa";
    possessive_abe_pl: "~sittAni", _, _, _, "~sittAVn|~sittansa";
    possessive_com_pl: "~sineni", _, _, _, "~sineen|~sinensa";
    ),
    // Osoitin: 25 words (ääretön, nimetön, tuntematon, viivoitin, osoitin, soitin). Like Tunnus, except in 61 cells: nom pl, gen_ pl, par pl, ine sg, ine pl, ela sg, ….
    declension!("osoitin", 25, "~~n", like Tunnus;
    stem sg: "~~n- like tunnus";
    nom: _, "~t~met";
    gen_: _, "~t~mVen|~~nten";
    par: _, "~t~mVA";
    ine: "~t~messA", "~t~missA";
    ela: "~t~mestA", "~t~mistA";
    ill: "~t~meVn", "~t~miin";
    ade: "~t~mellA", "~t~millA";
    abl: "~t~meltA", "~t~miltA";
    all: _, "~t~mille";
    ess: "~t~menA", "~t~minA";
    abe: "~t~mettA", "~t~mittA";
    possessive_nom_sg: "~t~meni", _, _, _, "~t~mensA";
    possessive_nom_pl: _, "~t~mesi", "~t~memme", "~t~menne", _;
    possessive_gen_sg: _, "~t~mesi", "~t~memme", "~t~menne", _;
    possessive_gen_pl: "~t~mVeni|~~nteni", "~t~mVesi|~~ntesi", "~t~mVemme|~~ntemme", "~t~mVenne|~~ntenne", "~t~mVensA|~~ntensa";
    possessive_par_pl: "~t~mVAni", _, _, _, "~t~mVAVn|~t~miansa";
    possessive_ine_sg: _, _, _, _, "~t~messAVn|~t~messansa";
    possessive_ine_pl: "~t~missAni", _, _, _, "~t~missAVn|~t~missansa";
    possessive_ela_sg: _, _, _, _, "~t~mestAVn|~t~mestansa";
    possessive_ela_pl: "~t~mistAni", _, _, _, "~t~mistAVn|~t~mistansa";
    possessive_ill_sg: _, _, _, _, "~t~meVnsA";
    possessive_ill_pl: "~t~miini", _, _, _, "~t~miinsA";
    possessive_ade_sg: _, _, _, _, "~t~mellAVn|~t~mellansa";
    possessive_ade_pl: "~t~millAni", _, _, _, "~t~millAVn|~t~millansa";
    possessive_abl_sg: _, _, _, _, "~t~meltAVn|~t~meltansa";
    possessive_abl_pl: "~t~miltAni", _, _, _, "~t~miltAVn|~t~miltansa";
    possessive_all_sg: _, _, _, _, "~t~melleen|~t~mellensa";
    possessive_all_pl: "~t~milleni", _, _, _, "~t~milleen|~t~millensa";
    possessive_ess_sg: _, _, _, _, "~t~menAVn|~t~menansa";
    possessive_ess_pl: "~t~minAni", _, _, _, "~t~minAVn|~t~minansa";
    possessive_tra_sg: _, _, _, _, "~t~mekseen|~t~meksensa";
    possessive_tra_pl: _, _, _, _, "~t~mikseen|~t~miksensa";
    possessive_abe_sg: _, _, _, _, "~t~mettAVn|~t~mettansa";
    possessive_abe_pl: "~t~mittAni", _, _, _, "~t~mittAVn|~t~mittansa";
    possessive_com_pl: "~t~mineni", _, _, _, "~t~mineen|~t~minensa";
    ),
    // Kate: 23 words (ote, kate, aate, vaate, uute, toimintaperiaate). Like Tunnus, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("kate", 23, "~~", like Tunnus;
    stem sg: "~~- like aste";
    stem pl: "~t~i- like kate";
    nom: _, "~t~et";
    ine: "~t~essA", "~t~issA";
    ela: "~t~estA", "~t~istA";
    ill: "~t~eseen", _;
    ade: "~t~ellA", "~t~illA";
    abl: "~t~eltA", "~t~iltA";
    all: _, "~t~ille";
    ess: "~t~enA", _;
    abe: "~t~ettA", "~t~ittA";
    possessive_nom_sg: "~t~eni", _, _, _, "~t~ensA";
    possessive_nom_pl: _, "~t~esi", "~t~emme", "~t~enne", _;
    possessive_gen_sg: _, "~t~esi", "~t~emme", "~t~enne", _;
    possessive_ine_sg: _, _, _, _, "~t~essAVn|~t~essansa";
    possessive_ine_pl: "~t~issAni", _, _, _, "~t~issAVn|~t~issansa";
    possessive_ela_sg: _, _, _, _, "~t~estAVn|~t~estansa";
    possessive_ela_pl: "~t~istAni", _, _, _, "~t~istAVn|~t~istansa";
    possessive_ill_sg: _, _, _, _, "~t~eseensA";
    possessive_ade_sg: _, _, _, _, "~t~ellAVn|~t~ellansa";
    possessive_ade_pl: "~t~illAni", _, _, _, "~t~illAVn|~t~illansa";
    possessive_abl_sg: _, _, _, _, "~t~eltAVn|~t~eltansa";
    possessive_abl_pl: "~t~iltAni", _, _, _, "~t~iltAVn|~t~iltansa";
    possessive_all_sg: _, _, _, _, "~t~elleen|~t~ellensa";
    possessive_all_pl: "~t~illeni", _, _, _, "~t~illeen|~t~illensa";
    possessive_ess_sg: _, _, _, _, "~t~enAVn|~t~enansa";
    possessive_tra_sg: _, _, _, _, "~t~ekseen|~t~eksensa";
    possessive_tra_pl: _, _, _, _, "~t~ikseen|~t~iksensa";
    possessive_abe_sg: _, _, _, _, "~t~ettAVn|~t~ettansa";
    possessive_abe_pl: "~t~ittAni", _, _, _, "~t~ittAVn|~t~ittansa";
    ),
    // Liike: 21 words (rike, säie, pyyhe, liike, ärsyke, levyke). Like Tykitys, except in 30 cells: ine sg, ine pl, ela sg, ela pl, ill sg, ade sg, ….
    declension!("liike", 21, "~~", like Tykitys;
    stem sg: "~~- like piste";
    stem pl: "~k~i- like liike";
    ine: "~k~essä", "~k~issä";
    ela: "~k~estä", "~k~istä";
    ill: "~k~eseen", _;
    ade: "~k~ellä", "~k~illä";
    abl: "~k~eltä", "~k~iltä";
    ess: "~k~enä", _;
    abe: "~k~että", "~k~ittä";
    possessive_nom_sg: _, _, _, _, "~k~ensä";
    possessive_ine_sg: _, _, _, _, "~k~essäVn|~k~essänsä";
    possessive_ine_pl: "~k~issäni", _, _, _, "~k~issäVn|~k~issänsä";
    possessive_ela_sg: _, _, _, _, "~k~estäVn|~k~estänsä";
    possessive_ela_pl: "~k~istäni", _, _, _, "~k~istäVn|~k~istänsä";
    possessive_ill_sg: _, _, _, _, "~k~eseensä";
    possessive_ade_sg: _, _, _, _, "~k~elläVn|~k~ellänsä";
    possessive_ade_pl: "~k~illäni", _, _, _, "~k~illäVn|~k~illänsä";
    possessive_abl_sg: _, _, _, _, "~k~eltäVn|~k~eltänsä";
    possessive_abl_pl: "~k~iltäni", _, _, _, "~k~iltäVn|~k~iltänsä";
    possessive_ess_sg: _, _, _, _, "~k~enäVn|~k~enänsä";
    possessive_abe_sg: _, _, _, _, "~k~ettäVn|~k~ettänsä";
    possessive_abe_pl: "~k~ittäni", _, _, _, "~k~ittäVn|~k~ittänsä";
    ),
    // Hmong: 20 words (swap, avatar, output, wok, trap, atrium). Like Pari, except in 7 cells: nom sg, com pl, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, ….
    declension!("hmong", 20, "~", like Pari;
    nom: "~", _;
    com: _, "~peineen";
    possessive_ill_sg: "~iVni", "~iVsi", "~iVmme", "~iVnne", _;
    possessive_ill_pl: "~eihini", _, _, _, _;
    ),
    // Äänenvaimennin: 20 words (äänenvaimennin, kiihdytin, jäähdytin, puhallin, linnunpelätin, heitin). Like Avain with `ˇ` after each pattern.
    declension!("äänenvaimennin", 20, "~nˇ", like Avain with "ˇ";
    ),
    // Kytkin: 18 words (sydän, elin, eläin, kavioeläin, pistin, koiraeläin). Like Avain, except in 15 cells: possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, possessive_ade_sg _3, possessive_ade_pl _3, ….
    declension!("kytkin", 18, "~n", like Avain;
    stem sg: "~n- like tykitys";
    stem pl: "~- like kytkin";
    possessive_ine_sg: _, _, _, _, "~messAVn|~messänsä";
    possessive_ine_pl: _, _, _, _, "~missAVn|~missänsä";
    possessive_ela_sg: _, _, _, _, "~mestAVn|~mestänsä";
    possessive_ela_pl: _, _, _, _, "~mistAVn|~mistänsä";
    possessive_ade_sg: _, _, _, _, "~mellAVn|~mellänsä";
    possessive_ade_pl: _, _, _, _, "~millAVn|~millänsä";
    possessive_abl_sg: _, _, _, _, "~meltAVn|~meltänsä";
    possessive_abl_pl: _, _, _, _, "~miltAVn|~miltänsä";
    possessive_all_sg: _, _, _, _, "~melleen|~mellensä";
    possessive_all_pl: _, _, _, _, "~milleen|~millensä";
    possessive_ess_sg: _, _, _, _, "~menAVn|~menänsä";
    possessive_tra_sg: _, _, _, _, "~mekseen|~meksensä";
    possessive_tra_pl: _, _, _, _, "~mikseen|~miksensä";
    possessive_abe_sg: _, _, _, _, "~mettAVn|~mettänsä";
    possessive_abe_pl: _, _, _, _, "~mittAVn|~mittänsä";
    ),
    // Puu: 17 words (luu, kuu, suu, puu, puolikuu, havupuu). Like Konversio with `` after each pattern (`u` after ~), except in 30 cells: nom pl, par sg, par pl, ine sg, ela sg, ill sg, ….
    declension!("puu", 17, "~u", like Konversio with "", "u" after "~";
    nom: _, "~ut";
    par: "~utA", "~itA";
    ine: "~ussA", _;
    ela: "~ustA", _;
    ill: "~uhVn", _;
    ade: "~ullA", _;
    abl: "~ultA", _;
    ess: "~unA", _;
    abe: "~uttA", _;
    possessive_nom_sg: "~uni", _, _, _, "~unsA";
    possessive_nom_pl: _, "~usi", "~umme", "~unne", _;
    possessive_gen_sg: _, "~usi", "~umme", "~unne", _;
    possessive_par_sg: _, _, _, _, "~utAVn|~utansa";
    possessive_par_pl: "~itAni", _, _, _, "~itAVn|~itansa";
    possessive_ine_sg: _, _, _, _, "~ussAVn|~ussansa";
    possessive_ela_sg: _, _, _, _, "~ustAVn|~ustansa";
    possessive_ill_sg: _, _, _, _, "~uhVnsA";
    possessive_ade_sg: _, _, _, _, "~ullAVn|~ullansa";
    possessive_abl_sg: _, _, _, _, "~ultAVn|~ultansa";
    possessive_all_sg: _, _, _, _, "~ulleen|~ullensa";
    possessive_ess_sg: _, _, _, _, "~unAVn|~unansa";
    possessive_tra_sg: _, _, _, _, "~ukseen|~uksensa";
    possessive_abe_sg: _, _, _, _, "~uttAVn|~uttansa";
    ),
    // Etumies: 17 words (mies, luottamusmies, jousimies, sotamies, pursimies, perämies). Like Oikeus, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("etumies", 17, "~s", like Oikeus;
    stem sg: "~s- like etumies";
    stem pl: "~- like etumies";
    nom: _, "~het";
    ine: "~hessä", "~hissä";
    ela: "~hestä", "~histä";
    ill: "~hVVn", _;
    ade: "~hellä", "~hillä";
    abl: "~heltä", "~hiltä";
    all: _, "~hille";
    ess: "~henä", _;
    abe: "~hettä", "~hittä";
    possessive_nom_sg: "~heni", _, _, _, "~hensä";
    possessive_nom_pl: _, "~hesi", "~hemme", "~henne", _;
    possessive_gen_sg: _, "~hesi", "~hemme", "~henne", _;
    possessive_ine_sg: _, _, _, _, "~hessäVn|~hessänsä";
    possessive_ine_pl: "~hissäni", _, _, _, "~hissäVn|~hissänsä";
    possessive_ela_sg: _, _, _, _, "~hestäVn|~hestänsä";
    possessive_ela_pl: "~histäni", _, _, _, "~histäVn|~histänsä";
    possessive_ill_sg: _, _, _, _, "~hVVnsä";
    possessive_ade_sg: _, _, _, _, "~helläVn|~hellänsä";
    possessive_ade_pl: "~hilläni", _, _, _, "~hilläVn|~hillänsä";
    possessive_abl_sg: _, _, _, _, "~heltäVn|~heltänsä";
    possessive_abl_pl: "~hiltäni", _, _, _, "~hiltäVn|~hiltänsä";
    possessive_all_sg: _, _, _, _, "~helleen|~hellensä";
    possessive_all_pl: "~hilleni", _, _, _, "~hilleen|~hillensä";
    possessive_ess_sg: _, _, _, _, "~henäVn|~henänsä";
    possessive_tra_sg: _, _, _, _, "~hekseen|~heksensä";
    possessive_tra_pl: _, _, _, _, "~hikseen|~hiksensä";
    possessive_abe_sg: _, _, _, _, "~hettäVn|~hettänsä";
    possessive_abe_pl: "~hittäni", _, _, _, "~hittäVn|~hittänsä";
    ),
    // Käsi: 17 words (vesi, käsi, mesi, hiisi, hirsi, reisi). Like Kausi, except in 35 cells: par sg, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("käsi", 17, "~si", like Kausi;
    stem pl: "~si- like erä";
    par: "~tTä", _;
    ine: "~dessä", "~sissä";
    ela: "~destä", "~sistä";
    ade: "~dellä", "~sillä";
    abl: "~deltä", "~siltä";
    ess: "~tenä", _;
    abe: "~dettä", "~sittä";
    possessive_nom_sg: _, _, _, _, "~tensä";
    possessive_par_sg: _, _, _, _, "~tTäVn|~ttänsä";
    possessive_ine_sg: _, _, _, _, "~dessäVn|~dessänsä";
    possessive_ine_pl: "~sissäni", _, _, _, "~sissäVn|~sissänsä";
    possessive_ela_sg: _, _, _, _, "~destäVn|~destänsä";
    possessive_ela_pl: "~sistäni", _, _, _, "~sistäVn|~sistänsä";
    possessive_ill_sg: _, _, _, _, "~teVnsä";
    possessive_ade_sg: _, _, _, _, "~delläVn|~dellänsä";
    possessive_ade_pl: "~silläni", _, _, _, "~silläVn|~sillänsä";
    possessive_abl_sg: _, _, _, _, "~deltäVn|~deltänsä";
    possessive_abl_pl: "~siltäni", _, _, _, "~siltäVn|~siltänsä";
    possessive_all_sg: _, _, _, _, "~delleen|~dellensä";
    possessive_all_pl: _, _, _, _, "~silleen|~sillensä";
    possessive_ess_sg: _, _, _, _, "~tenäVn|~tenänsä";
    possessive_tra_sg: _, _, _, _, "~dekseen|~deksensä";
    possessive_tra_pl: _, _, _, _, "~sikseen|~siksensä";
    possessive_abe_sg: _, _, _, _, "~dettäVn|~dettänsä";
    possessive_abe_pl: "~sittäni", _, _, _, "~sittäVn|~sittänsä";
    ),
    // Luku: 17 words (suku, puku, luku, alkuluku, kyky, oikoluku). Like Tunnus, except in 47 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("luku", 17, "~k~", like Tunnus;
    stem sg: "~k~- like taso";
    stem pl: "~k~- like taso";
    nom: _, "~v~t";
    ine: "~v~ssA", "~v~issA";
    ela: "~v~stA", "~v~istA";
    ill: "~k~Vn", _;
    ade: "~v~llA", "~v~illA";
    abl: "~v~ltA", "~v~iltA";
    all: _, "~v~ille";
    ess: "~k~nA", _;
    abe: "~v~ttA", "~v~ittA";
    possessive_nom_sg: "~k~ni", _, _, _, "~k~nsA";
    possessive_nom_pl: _, "~k~si", "~k~mme", "~k~nne", _;
    possessive_gen_sg: _, "~k~si", "~k~mme", "~k~nne", _;
    possessive_gen_pl: _, derived, derived, derived, _;
    possessive_ine_sg: _, _, _, _, "~v~ssAVn|~v~ssansa";
    possessive_ine_pl: "~v~issAni", _, _, _, "~v~issAVn|~v~issansa";
    possessive_ela_sg: _, _, _, _, "~v~stAVn|~v~stansa";
    possessive_ela_pl: "~v~istAni", _, _, _, "~v~istAVn|~v~istansa";
    possessive_ill_sg: _, _, _, _, "~k~VnsA";
    possessive_ade_sg: _, _, _, _, "~v~llAVn|~v~llansa";
    possessive_ade_pl: "~v~illAni", _, _, _, "~v~illAVn|~v~illansa";
    possessive_abl_sg: _, _, _, _, "~v~ltAVn|~v~ltansa";
    possessive_abl_pl: "~v~iltAni", _, _, _, "~v~iltAVn|~v~iltansa";
    possessive_all_sg: _, _, _, _, "~v~lleen|~v~llensa";
    possessive_all_pl: "~v~illeni", _, _, _, "~v~illeen|~v~illensa";
    possessive_ess_sg: _, _, _, _, "~k~nAVn|~k~nansa";
    possessive_tra_sg: _, _, _, _, "~v~kseen|~v~ksensa";
    possessive_tra_pl: _, _, _, _, "~v~ikseen|~v~iksensa";
    possessive_abe_sg: _, _, _, _, "~v~ttAVn|~v~ttansa";
    possessive_abe_pl: "~v~ittAni", _, _, _, "~v~ittAVn|~v~ittansa";
    ),
    // Peite: 16 words (jännite, hete, heikkojännite, viite, liite, väite). Like Kate, except in 33 cells: ine sg, ine pl, ela sg, ela pl, ade sg, ade pl, ….
    declension!("peite", 16, "~~", like Kate;
    stem sg: "~~- like piste";
    stem pl: "~t~i- like liike";
    ine: "~t~essä", "~t~issä";
    ela: "~t~estä", "~t~istä";
    ade: "~t~ellä", "~t~illä";
    abl: "~t~eltä", "~t~iltä";
    ess: "~t~enä", _;
    abe: "~t~että", "~t~ittä";
    possessive_nom_sg: _, _, _, _, "~t~ensä";
    possessive_ine_sg: _, _, _, _, "~t~essäVn|~t~essänsä";
    possessive_ine_pl: "~t~issäni", _, _, _, "~t~issäVn|~t~issänsä";
    possessive_ela_sg: _, _, _, _, "~t~estäVn|~t~estänsä";
    possessive_ela_pl: "~t~istäni", _, _, _, "~t~istäVn|~t~istänsä";
    possessive_ill_sg: _, _, _, _, "~t~eseensä";
    possessive_ade_sg: _, _, _, _, "~t~elläVn|~t~ellänsä";
    possessive_ade_pl: "~t~illäni", _, _, _, "~t~illäVn|~t~illänsä";
    possessive_abl_sg: _, _, _, _, "~t~eltäVn|~t~eltänsä";
    possessive_abl_pl: "~t~iltäni", _, _, _, "~t~iltäVn|~t~iltänsä";
    possessive_all_sg: _, _, _, _, "~t~elleen|~t~ellensä";
    possessive_all_pl: _, _, _, _, "~t~illeen|~t~illensä";
    possessive_ess_sg: _, _, _, _, "~t~enäVn|~t~enänsä";
    possessive_tra_sg: _, _, _, _, "~t~ekseen|~t~eksensä";
    possessive_tra_pl: _, _, _, _, "~t~ikseen|~t~iksensä";
    possessive_abe_sg: _, _, _, _, "~t~ettäVn|~t~ettänsä";
    possessive_abe_pl: "~t~ittäni", _, _, _, "~t~ittäVn|~t~ittänsä";
    ),
    // Rengas: 15 words (kuningas, allas, parras, kangas, rengas, marras). Like Keihäs with `ˇ` after each pattern, except in 21 cells: par sg, possessive_gen_pl _3, possessive_par_sg _3, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, ….
    declension!("rengas", 15, "~sˇ", like Keihäs with "ˇ";
    par: "~stVˇ", _;
    possessive_gen_pl: _, _, _, _, "~idensAˇ|~ittensaˇ";
    possessive_par_sg: _, _, _, _, "~stVVnˇ|~stansaˇ";
    possessive_par_pl: _, _, _, _, "~itAVnˇ|~itansaˇ";
    possessive_ine_sg: _, _, _, _, "~VssVVnˇ|~assansaˇ";
    possessive_ine_pl: _, _, _, _, "~issAVnˇ|~issansaˇ";
    possessive_ela_sg: _, _, _, _, "~VstVVnˇ|~astansaˇ";
    possessive_ela_pl: _, _, _, _, "~istAVnˇ|~istansaˇ";
    possessive_ade_sg: _, _, _, _, "~VllVVnˇ|~allansaˇ";
    possessive_ade_pl: _, _, _, _, "~illAVnˇ|~illansaˇ";
    possessive_abl_sg: _, _, _, _, "~VltVVnˇ|~altansaˇ";
    possessive_abl_pl: _, _, _, _, "~iltAVnˇ|~iltansaˇ";
    possessive_all_sg: _, _, _, _, "~Vlleenˇ|~allensaˇ";
    possessive_all_pl: _, _, _, _, "~illeenˇ|~illensaˇ";
    possessive_ess_sg: _, _, _, _, "~VnVVnˇ|~anansaˇ";
    possessive_ess_pl: _, _, _, _, "~inAVnˇ|~inansaˇ";
    possessive_tra_sg: _, _, _, _, "~Vkseenˇ|~aksensaˇ";
    possessive_tra_pl: _, _, _, _, "~ikseenˇ|~iksensaˇ";
    possessive_abe_sg: _, _, _, _, "~VttVVnˇ|~attansaˇ";
    possessive_abe_pl: _, _, _, _, "~ittAVnˇ|~ittansaˇ";
    possessive_com_pl: _, _, _, _, "~ineenˇ|~inensaˇ";
    ),
    // Ruis: 15 words (ies, ruis, asiakas, osterivinokas, ehdokas, sokerijuurikas). Like Koe with `` after each pattern (`s` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("ruis", 15, "~~s", like Koe with "", "s" after "~";
    nom: _, "~k~Vt";
    par: "~~stA", _;
    ine: "~k~VssV", _;
    ela: "~k~VstV", _;
    ill: "~k~Vseen", "~k~isiin";
    ade: "~k~VllV", _;
    abl: "~k~VltV", _;
    ess: "~k~VnV", _;
    abe: "~k~VttV", _;
    possessive_nom_sg: "~k~Vni", _, _, _, "~k~VnsV";
    possessive_nom_pl: _, "~k~Vsi", "~k~Vmme", "~k~Vnne", _;
    possessive_gen_sg: _, "~k~Vsi", "~k~Vmme", "~k~Vnne", _;
    possessive_par_sg: _, _, _, _, "~~stAVn|~~stansa";
    possessive_ine_sg: _, _, _, _, "~k~VssVVn|~k~assansa";
    possessive_ela_sg: _, _, _, _, "~k~VstVVn|~k~astansa";
    possessive_ill_sg: _, _, _, _, "~k~VseensA";
    possessive_ill_pl: "~k~isiini", derived, derived, derived, "~k~isiinsA";
    possessive_ade_sg: _, _, _, _, "~k~VllVVn|~k~allansa";
    possessive_abl_sg: _, _, _, _, "~k~VltVVn|~k~altansa";
    possessive_all_sg: _, _, _, _, "~k~Vlleen|~k~allensa";
    possessive_ess_sg: _, _, _, _, "~k~VnVVn|~k~anansa";
    possessive_tra_sg: _, _, _, _, "~k~Vkseen|~k~aksensa";
    possessive_abe_sg: _, _, _, _, "~k~VttVVn|~k~attansa";
    ),
    // Lehti: 14 words (tähti, aamutähti, käärmetähti, lehti, vyyhti, iltatähti). Like Käsi, except in 19 cells: ine pl, ela pl, ade pl, abl pl, all pl, abe pl, ….
    declension!("lehti", 14, "~ti", like Käsi;
    stem sg: "~t- like kivi";
    stem pl: "~ti- like erä";
    ine: _, "~dissä";
    ela: _, "~distä";
    ade: _, "~dillä";
    abl: _, "~diltä";
    all: _, "~dille";
    abe: _, "~dittä";
    possessive_ine_pl: "~dissäni", _, _, _, "~dissäVn|~dissänsä";
    possessive_ela_pl: "~distäni", _, _, _, "~distäVn|~distänsä";
    possessive_ade_pl: "~dilläni", _, _, _, "~dilläVn|~dillänsä";
    possessive_abl_pl: "~diltäni", _, _, _, "~diltäVn|~diltänsä";
    possessive_all_pl: "~dilleni", _, _, _, "~dilleen|~dillensä";
    possessive_tra_pl: _, _, _, _, "~dikseen|~diksensä";
    possessive_abe_pl: "~dittäni", _, _, _, "~dittäVn|~dittänsä";
    ),
    // Sakset: 14 words (helmet, sakset, alkeet, lauteet, työvaatteet, revontulet). Like Taso with `` after each pattern (`et` after ~), except in 38 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("sakset", 14, "~et", like Taso with "", "et" after "~";
    nom: _, "~et";
    gen_: _, "~ien";
    par: derived, "~iA";
    ine: derived, _;
    ela: derived, _;
    ill: derived, "~iin";
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~eni", "~esi", "~emme", "~enne", "~ensA";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_gen_pl: "~ieni", _, _, _, "~iensA";
    possessive_par_sg: _, _, _, _, derived;
    possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iansa";
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Aika: 13 words (aika, vuodenaika, kesäaika, talviaika, keskiaika, kiertoaika). Like Taso, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("aika", 13, "~ika", like Taso;
    stem sg: "~ika- like aika";
    stem pl: "~iko- like aika";
    nom: _, "~jVt";
    ine: "~jVssV", "~joissa";
    ela: "~jVstV", "~joista";
    ill: "~ikaan", _;
    ade: "~jVllV", "~joilla";
    abl: "~jVltV", "~joilta";
    all: _, "~joille";
    ess: "~ikana", _;
    abe: "~jVttV", "~joitta";
    possessive_nom_sg: "~ikani", _, _, _, "~ikansa";
    possessive_nom_pl: _, "~ikasi", "~ikamme", "~ikanne", _;
    possessive_gen_sg: _, "~ikasi", "~ikamme", "~ikanne", _;
    possessive_ine_sg: _, _, _, _, "~jVssVVn|~jassansa";
    possessive_ine_pl: "~joissani", _, _, _, "~joissaan|~joissansa";
    possessive_ela_sg: _, _, _, _, "~jVstVVn|~jastansa";
    possessive_ela_pl: "~joistani", _, _, _, "~joistaan|~joistansa";
    possessive_ill_sg: _, _, _, _, "~ikaansa";
    possessive_ade_sg: _, _, _, _, "~jVllVVn|~jallansa";
    possessive_ade_pl: "~joillani", _, _, _, "~joillaan|~joillansa";
    possessive_abl_sg: _, _, _, _, "~jVltVVn|~jaltansa";
    possessive_abl_pl: "~joiltani", _, _, _, "~joiltaan|~joiltansa";
    possessive_all_sg: _, _, _, _, "~jVlleen|~jallensa";
    possessive_all_pl: "~joilleni", _, _, _, "~joilleen|~joillensa";
    possessive_ess_sg: _, _, _, _, "~ikanaan|~ikanansa";
    possessive_tra_sg: _, _, _, _, "~jVkseen|~jaksensa";
    possessive_tra_pl: _, _, _, _, "~joikseen|~joiksensa";
    possessive_abe_sg: _, _, _, _, "~jVttVVn|~jattansa";
    possessive_abe_pl: "~joittani", _, _, _, "~joittaan|~joittansa";
    ),
    // Kaanon: 13 words (diesel, magnum, tandem, denier, afar, hangul). Like Ankkuri, except in 2 cells: nom sg, possessive_ill_pl _1sg.
    declension!("kaanon", 13, "~", like Ankkuri;
    nom: "~", _;
    possessive_ill_pl: "~eihini", _, _, _, _;
    ),
    // Lähde: 13 words (side, lähde, toimenpide, räjähde, säde, päivänsäde). Like Piste with `ˇ` after each pattern.
    declension!("lähde", 13, "~ˇ", like Piste with "ˇ";
    ),
    // Olot: 13 words (aivot, olut, airut, pikkuaivot, palvelut, farkut). Like Taso with `` after each pattern (`t` after ~), except in 25 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("olot", 13, "~t", like Taso with "", "t" after "~";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~ni", _, _, _, "~nsa";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_par_sg: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Siemen: 12 words (siemen, ahven, nivel, paimen, jäsen, höyhen). Like Aste, except in 20 cells: ill sg, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, possessive_ill_sg _3, ….
    declension!("siemen", 12, "~", like Aste;
    stem sg: "~- like tykitys";
    stem pl: "~- like siemen";
    ill: "~VVn", _;
    possessive_ine_sg: _, _, _, _, "~VssAVn|~essänsä";
    possessive_ine_pl: _, _, _, _, "~issAVn|~issänsä";
    possessive_ela_sg: _, _, _, _, "~VstAVn|~estänsä";
    possessive_ela_pl: _, _, _, _, "~istAVn|~istänsä";
    possessive_ill_sg: _, _, _, _, "~VVnsA";
    possessive_ill_pl: _, derived, derived, derived, _;
    possessive_ade_sg: _, _, _, _, "~VllAVn|~ellänsä";
    possessive_ade_pl: _, _, _, _, "~illAVn|~illänsä";
    possessive_abl_sg: _, _, _, _, "~VltAVn|~eltänsä";
    possessive_abl_pl: _, _, _, _, "~iltAVn|~iltänsä";
    possessive_all_sg: _, _, _, _, "~Vlleen|~ellensä";
    possessive_all_pl: _, _, _, _, "~illeen|~illensä";
    possessive_ess_sg: _, _, _, _, "~VnAVn|~enänsä";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~eksensä";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensä";
    possessive_abe_sg: _, _, _, _, "~VttAVn|~ettänsä";
    possessive_abe_pl: _, _, _, _, "~ittAVn|~ittänsä";
    ),
    // Pää: 11 words (jää, pää, kyynärpää, luupää, akilleenkantapää, peräpää). Like Levy with `` after each pattern (`ä` after ~), except in 38 cells: nom pl, gen_ sg, gen_ pl, par sg, par pl, ine sg, ….
    declension!("pää", 11, "~ä", like Levy with "", "ä" after "~";
    nom: _, "~ät";
    gen_: "~än", "~iden|~itten";
    par: "~ätV", "~itA";
    ine: "~ässV", _;
    ela: "~ästV", _;
    ill: "~ähVn", _;
    ade: "~ällV", _;
    abl: "~ältV", _;
    all: "~älle", _;
    ess: "~änV", _;
    abe: "~ättV", _;
    possessive_nom_sg: "~äni", _, _, _, "~änsV";
    possessive_nom_pl: _, "~äsi", "~ämme", "~änne", _;
    possessive_gen_sg: _, "~äsi", "~ämme", "~änne", _;
    possessive_gen_pl: "~ideni|~itteni", "~idesi|~ittesi", "~idemme|~ittemme", "~idenne|~ittenne", "~idensA|~ittensä";
    possessive_par_sg: _, _, _, _, "~ätVVn|~ätänsä";
    possessive_par_pl: "~itAni", _, _, _, "~itAVn|~itänsä";
    possessive_ine_sg: _, _, _, _, "~ässVVn|~ässänsä";
    possessive_ela_sg: _, _, _, _, "~ästVVn|~ästänsä";
    possessive_ill_sg: _, _, _, _, "~ähVnsV";
    possessive_ade_sg: _, _, _, _, "~ällVVn|~ällänsä";
    possessive_abl_sg: _, _, _, _, "~ältVVn|~ältänsä";
    possessive_all_sg: _, _, _, _, "~älleen|~ällensä";
    possessive_ess_sg: _, _, _, _, "~änVVn|~änänsä";
    possessive_tra_sg: _, _, _, _, "~äkseen|~äksensä";
    possessive_abe_sg: _, _, _, _, "~ättVVn|~ättänsä";
    ),
    // Koi: 11 words (tie, thai, pii, vuo, koi, täi). Like Konversio, except in 21 cells: ine pl, ela pl, ill sg, ade pl, abl pl, all pl, ….
    declension!("koi", 11, "~", like Konversio;
    stem pl: "~- like puu";
    ine: _, "~ssA";
    ela: _, "~stA";
    ill: "~hVn", _;
    ade: _, "~llA";
    abl: _, "~ltA";
    all: _, "~lle";
    abe: _, "~ttA";
    possessive_ine_pl: "~ssAni", _, _, _, "~ssAVn|~ssansa";
    possessive_ela_pl: "~stAni", _, _, _, "~stAVn|~stansa";
    possessive_ill_sg: _, _, _, _, "~hVnsA";
    possessive_ade_pl: "~llAni", _, _, _, "~llAVn|~llansa";
    possessive_abl_pl: "~ltAni", _, _, _, "~ltAVn|~ltansa";
    possessive_all_pl: "~lleni", _, _, _, "~lleen|~llensa";
    possessive_tra_pl: _, _, _, _, "~kseen|~ksensa";
    possessive_abe_pl: "~ttAni", _, _, _, "~ttAVn|~ttansa";
    ),
    // Hattara: 10 words (granaattiomena, maruna, ohrana, harava, korona, hattara). Like Osa, except in 43 cells: ine pl, ela pl, ade pl, abl pl, all pl, tra pl, ….
    declension!("hattara", 10, "~a", like Osa;
    stem pl: "~- like hattara";
    ine: _, "~oissA|~issa";
    ela: _, "~oistA|~ista";
    ade: _, "~oillA|~illa";
    abl: _, "~oiltA|~ilta";
    all: _, "~oille|~ille";
    tra: _, "~oiksi|~iksi";
    abe: _, "~oittA|~itta";
    ins: _, "~oin|~in";
    possessive_ine_pl: "~oissAni|~issani", "~oissAsi|~issasi", "~oissAmme|~issamme", "~oissAnne|~issanne", "~oissAVn|~issaan|~oissansa|~issansa";
    possessive_ela_pl: "~oistAni|~istani", "~oistAsi|~istasi", "~oistAmme|~istamme", "~oistAnne|~istanne", "~oistAVn|~istaan|~oistansa|~istansa";
    possessive_ade_pl: "~oillAni|~illani", "~oillAsi|~illasi", "~oillAmme|~illamme", "~oillAnne|~illanne", "~oillAVn|~illaan|~oillansa|~illansa";
    possessive_abl_pl: "~oiltAni|~iltani", "~oiltAsi|~iltasi", "~oiltAmme|~iltamme", "~oiltAnne|~iltanne", "~oiltAVn|~iltaan|~oiltansa|~iltansa";
    possessive_all_pl: "~oilleni|~illeni", "~oillesi|~illesi", "~oillemme|~illemme", "~oillenne|~illenne", "~oilleen|~illeen|~oillensa|~illensa";
    possessive_tra_pl: "~oikseni|~ikseni", "~oiksesi|~iksesi", "~oiksemme|~iksemme", "~oiksenne|~iksenne", "~oikseen|~ikseen|~oiksensa|~iksensa";
    possessive_abe_pl: "~oittAni|~ittani", "~oittAsi|~ittasi", "~oittAmme|~ittamme", "~oittAnne|~ittanne", "~oittAVn|~ittaan|~oittansa|~ittansa";
    ),
    // Perintä: 10 words (yhdyntä, kysyntä, itsehillintä, hyväksyntä, häirintä, liitäntä). Like Taso, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("perintä", 10, "~tä", like Taso;
    stem sg: "~tä- like osa";
    stem pl: "~tö- like levy";
    nom: _, "~nAt";
    ine: "~nAssV", "~nöissA";
    ela: "~nAstV", "~nöistA";
    ill: "~täVn", _;
    ade: "~nAllV", "~nöillA";
    abl: "~nAltV", "~nöiltA";
    all: _, "~nöille";
    ess: "~tänV", _;
    abe: "~nAttV", "~nöittA";
    possessive_nom_sg: "~täni", _, _, _, "~tänsV";
    possessive_nom_pl: _, "~täsi", "~tämme", "~tänne", _;
    possessive_gen_sg: _, "~täsi", "~tämme", "~tänne", _;
    possessive_ine_sg: _, _, _, _, "~nAssVVn|~nässänsä";
    possessive_ine_pl: "~nöissAni", _, _, _, "~nöissAVn|~nöissänsä";
    possessive_ela_sg: _, _, _, _, "~nAstVVn|~nästänsä";
    possessive_ela_pl: "~nöistAni", _, _, _, "~nöistAVn|~nöistänsä";
    possessive_ill_sg: _, _, _, _, "~täVnsV";
    possessive_ade_sg: _, _, _, _, "~nAllVVn|~nällänsä";
    possessive_ade_pl: "~nöillAni", _, _, _, "~nöillAVn|~nöillänsä";
    possessive_abl_sg: _, _, _, _, "~nAltVVn|~nältänsä";
    possessive_abl_pl: "~nöiltAni", _, _, _, "~nöiltAVn|~nöiltänsä";
    possessive_all_sg: _, _, _, _, "~nAlleen|~nällensä";
    possessive_all_pl: "~nöilleni", _, _, _, "~nöilleen|~nöillensä";
    possessive_ess_sg: _, _, _, _, "~tänVVn|~tänänsä";
    possessive_tra_sg: _, _, _, _, "~nAkseen|~näksensä";
    possessive_tra_pl: _, _, _, _, "~nöikseen|~nöiksensä";
    possessive_abe_sg: _, _, _, _, "~nAttVVn|~nättänsä";
    possessive_abe_pl: "~nöittAni", _, _, _, "~nöittAVn|~nöittänsä";
    ),
    // Väki: 10 words (hauki, noki, mäki, käki, väki, jalkaväki). Like Levy with `` after each pattern (`ki` after ~), except in 41 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("väki", 10, "~ki", like Levy with "", "ki" after "~";
    nom: _, "~et";
    gen_: _, "~kien";
    par: "~keA", "~kiA";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~keVn", "~kiin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    ess: "~kenA", "~kinA";
    abe: "~ettA", _;
    possessive_nom_sg: "~keni", _, _, _, "~kensA";
    possessive_nom_pl: _, "~kesi", "~kemme", "~kenne", _;
    possessive_gen_sg: _, "~kesi", "~kemme", "~kenne", _;
    possessive_gen_pl: "~kieni", _, _, _, "~kiensA";
    possessive_par_sg: _, _, _, _, "~keAVn|~keänsä";
    possessive_par_pl: "~kiAni", _, _, _, "~kiAVn|~kiänsä";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essänsä";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estänsä";
    possessive_ill_sg: _, _, _, _, "~keVnsA";
    possessive_ill_pl: "~kiini", _, _, _, "~kiinsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellänsä";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltänsä";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensä";
    possessive_ess_sg: _, _, _, _, "~kenAVn|~kenänsä";
    possessive_ess_pl: "~kinAni", _, _, _, "~kinAVn|~kinänsä";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensä";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettänsä";
    possessive_com_pl: "~kineni", _, _, _, "~kineen|~kinensä";
    ),
    // Maa: 9 words (harmaa, maa, autiomaa, erämaa, isänmaa, takamaa). Like Konversio with `` after each pattern (`a` after ~), except in 30 cells: nom pl, par sg, par pl, ine sg, ela sg, ill sg, ….
    declension!("maa", 9, "~a", like Konversio with "", "a" after "~";
    nom: _, "~at";
    par: "~atV", "~itA";
    ine: "~assV", _;
    ela: "~astV", _;
    ill: "~ahVn", _;
    ade: "~allV", _;
    abl: "~altV", _;
    ess: "~anV", _;
    abe: "~attV", _;
    possessive_nom_sg: "~ani", _, _, _, "~ansV";
    possessive_nom_pl: _, "~asi", "~amme", "~anne", _;
    possessive_gen_sg: _, "~asi", "~amme", "~anne", _;
    possessive_par_sg: _, _, _, _, "~atVVn|~atansa";
    possessive_par_pl: "~itAni", _, _, _, "~itAVn|~itansa";
    possessive_ine_sg: _, _, _, _, "~assVVn|~assansa";
    possessive_ela_sg: _, _, _, _, "~astVVn|~astansa";
    possessive_ill_sg: _, _, _, _, "~ahVnsV";
    possessive_ade_sg: _, _, _, _, "~allVVn|~allansa";
    possessive_abl_sg: _, _, _, _, "~altVVn|~altansa";
    possessive_all_sg: _, _, _, _, "~alleen|~allensa";
    possessive_ess_sg: _, _, _, _, "~anVVn|~anansa";
    possessive_tra_sg: _, _, _, _, "~akseen|~aksensa";
    possessive_abe_sg: _, _, _, _, "~attVVn|~attansa";
    ),
    // Työ: 9 words (vyö, työ, keskiyö, lapsityö, käsityö, päivätyö). Like Tunnus, except in 47 cells: nom sg, nom pl, par sg, ine sg, ine pl, ela sg, ….
    declension!("työ", 9, "~y~", like Tunnus;
    stem pl: "~~i- like työ";
    nom: "~y~", "~y~t";
    par: "~y~tä", _;
    ine: "~y~ssä", "~~issA";
    ela: "~y~stä", "~~istA";
    ill: "~y~hön", _;
    ade: "~y~llä", "~~illA";
    abl: "~y~ltä", "~~iltA";
    all: _, "~~ille";
    ess: "~y~nä", _;
    abe: "~y~ttä", "~~ittA";
    possessive_nom_sg: "~y~ni", _, _, _, "~y~nsä";
    possessive_nom_pl: _, "~y~si", "~y~mme", "~y~nne", _;
    possessive_gen_sg: _, "~y~si", "~y~mme", "~y~nne", _;
    possessive_par_sg: _, _, _, _, "~y~tään|~y~tänsä";
    possessive_ine_sg: _, _, _, _, "~y~ssään|~y~ssänsä";
    possessive_ine_pl: "~~issAni", _, _, _, "~~issAVn|~~issänsä";
    possessive_ela_sg: _, _, _, _, "~y~stään|~y~stänsä";
    possessive_ela_pl: "~~istAni", _, _, _, "~~istAVn|~~istänsä";
    possessive_ill_sg: _, _, _, _, "~y~hönsä";
    possessive_ade_sg: _, _, _, _, "~y~llään|~y~llänsä";
    possessive_ade_pl: "~~illAni", _, _, _, "~~illAVn|~~illänsä";
    possessive_abl_sg: _, _, _, _, "~y~ltään|~y~ltänsä";
    possessive_abl_pl: "~~iltAni", _, _, _, "~~iltAVn|~~iltänsä";
    possessive_all_sg: _, _, _, _, "~y~lleen|~y~llensä";
    possessive_all_pl: "~~illeni", _, _, _, "~~illeen|~~illensä";
    possessive_ess_sg: _, _, _, _, "~y~nään|~y~nänsä";
    possessive_tra_sg: _, _, _, _, "~y~kseen|~y~ksensä";
    possessive_tra_pl: _, _, _, _, "~~ikseen|~~iksensä";
    possessive_abe_sg: _, _, _, _, "~y~ttään|~y~ttänsä";
    possessive_abe_pl: "~~ittAni", _, _, _, "~~ittAVn|~~ittänsä";
    ),
    // Kärki: 9 words (kurki, jälki, olki, järki, arki, kylki). Like Väki, except in 32 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("kärki", 9, "~ki", like Väki;
    nom: _, "~jet";
    ine: "~jessA", "~jissA";
    ela: "~jestA", "~jistA";
    ade: "~jellA", "~jillA";
    abl: "~jeltA", "~jiltA";
    all: _, "~jille";
    abe: "~jettA", "~jittA";
    possessive_ine_sg: _, _, _, _, "~jessAVn|~jessänsä";
    possessive_ine_pl: "~jissAni", _, _, _, "~jissAVn|~jissänsä";
    possessive_ela_sg: _, _, _, _, "~jestAVn|~jestänsä";
    possessive_ela_pl: "~jistAni", _, _, _, "~jistAVn|~jistänsä";
    possessive_ade_sg: _, _, _, _, "~jellAVn|~jellänsä";
    possessive_ade_pl: "~jillAni", _, _, _, "~jillAVn|~jillänsä";
    possessive_abl_sg: _, _, _, _, "~jeltAVn|~jeltänsä";
    possessive_abl_pl: "~jiltAni", _, _, _, "~jiltAVn|~jiltänsä";
    possessive_all_sg: _, _, _, _, "~jelleen|~jellensä";
    possessive_all_pl: "~jilleni", _, _, _, "~jilleen|~jillensä";
    possessive_tra_sg: _, _, _, _, "~jekseen|~jeksensä";
    possessive_tra_pl: _, _, _, _, "~jikseen|~jiksensä";
    possessive_abe_sg: _, _, _, _, "~jettAVn|~jettänsä";
    possessive_abe_pl: "~jittAni", _, _, _, "~jittAVn|~jittänsä";
    ),
    // Verkko: 8 words (nukke, teko, ruoko, lieko, keko, unikeko). Like Luku, except in 32 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("verkko", 8, "~k~", like Luku;
    nom: _, "~~t";
    ine: "~~ssA", "~~issA";
    ela: "~~stA", "~~istA";
    ade: "~~llA", "~~illA";
    abl: "~~ltA", "~~iltA";
    all: _, "~~ille";
    abe: "~~ttA", "~~ittA";
    possessive_ine_sg: _, _, _, _, "~~ssAVn|~~ssansa";
    possessive_ine_pl: "~~issAni", _, _, _, "~~issAVn|~~issansa";
    possessive_ela_sg: _, _, _, _, "~~stAVn|~~stansa";
    possessive_ela_pl: "~~istAni", _, _, _, "~~istAVn|~~istansa";
    possessive_ade_sg: _, _, _, _, "~~llAVn|~~llansa";
    possessive_ade_pl: "~~illAni", _, _, _, "~~illAVn|~~illansa";
    possessive_abl_sg: _, _, _, _, "~~ltAVn|~~ltansa";
    possessive_abl_pl: "~~iltAni", _, _, _, "~~iltAVn|~~iltansa";
    possessive_all_sg: _, _, _, _, "~~lleen|~~llensa";
    possessive_all_pl: "~~illeni", _, _, _, "~~illeen|~~illensa";
    possessive_tra_sg: _, _, _, _, "~~kseen|~~ksensa";
    possessive_tra_pl: _, _, _, _, "~~ikseen|~~iksensa";
    possessive_abe_sg: _, _, _, _, "~~ttAVn|~~ttansa";
    possessive_abe_pl: "~~ittAni", _, _, _, "~~ittAVn|~~ittansa";
    ),
    // Kieli: 8 words (veri, herkkusieni, korvasieni, puhekieli, huhtasieni, satakieli). Like Piste with `` after each pattern (`i` after ~), except in 42 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("kieli", 8, "~i", like Piste with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ten|~ien";
    par: "~tä", "~iä";
    ine: "~essä", _;
    ela: "~estä", _;
    ill: "~eVn", "~iin";
    ade: "~ellä", _;
    abl: "~eltä", _;
    ess: "~enä", _;
    abe: "~että", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensä";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensä|~iensä";
    possessive_par_sg: _, _, _, _, "~täVn|~tänsä";
    possessive_par_pl: "~iäni", _, _, _, "~iäVn|~iänsä";
    possessive_ine_sg: _, _, _, _, "~essäVn|~essänsä";
    possessive_ela_sg: _, _, _, _, "~estäVn|~estänsä";
    possessive_ill_sg: _, _, _, _, "~eVnsä";
    possessive_ill_pl: "~iini", derived, derived, derived, "~iinsä";
    possessive_ade_sg: _, _, _, _, "~elläVn|~ellänsä";
    possessive_abl_sg: _, _, _, _, "~eltäVn|~eltänsä";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensä";
    possessive_ess_sg: _, _, _, _, "~enäVn|~enänsä";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensä";
    possessive_abe_sg: _, _, _, _, "~ettäVn|~ettänsä";
    ),
    // Jousi: 8 words (uni, lohi, tuli, tuohi, vuohi, huuli). Like Siemen with `` after each pattern (`i` after ~), except in 36 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("jousi", 8, "~i", like Siemen with "", "i" after "~";
    nom: _, "~et";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~eVn", _;
    ade: "~ellA", _;
    abl: "~eltA", _;
    ess: "~enA", _;
    abe: "~ettA", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensA";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_par_sg: _, _, _, _, "~tAVn|~tansa";
    possessive_par_pl: _, _, _, _, "~iAVn|~iansa";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essansa";
    possessive_ine_pl: _, _, _, _, "~issAVn|~issansa";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estansa";
    possessive_ela_pl: _, _, _, _, "~istAVn|~istansa";
    possessive_ill_sg: _, _, _, _, "~eVnsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellansa";
    possessive_ade_pl: _, _, _, _, "~illAVn|~illansa";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltansa";
    possessive_abl_pl: _, _, _, _, "~iltAVn|~iltansa";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensa";
    possessive_all_pl: _, _, _, _, "~illeen|~illensa";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enansa";
    possessive_ess_pl: _, _, _, _, "~inAVn|~inansa";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensa";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensa";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettansa";
    possessive_abe_pl: _, _, _, _, "~ittAVn|~ittansa";
    possessive_com_pl: _, _, _, _, "~ineen|~inensa";
    ),
    // Hiili: 8 words (hiiri, hiili, kotihiiri, metsähiiri, vaivaishiiri, koivuhiiri). Like Piste with `` after each pattern (`i` after ~), except in 42 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("hiili", 8, "~i", like Piste with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ien|~ten";
    par: "~tä", "~iä";
    ine: "~essä", _;
    ela: "~estä", _;
    ill: "~eVn", "~iin";
    ade: "~ellä", _;
    abl: "~eltä", _;
    ess: "~enä", _;
    abe: "~että", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensä";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_gen_pl: "~ieni|~teni", "~iesi|~tesi", "~iemme|~temme", "~ienne|~tenne", "~iensä|~tensä";
    possessive_par_sg: _, _, _, _, "~täVn|~tänsä";
    possessive_par_pl: "~iäni", _, _, _, "~iäVn|~iänsä";
    possessive_ine_sg: _, _, _, _, "~essäVn|~essänsä";
    possessive_ela_sg: _, _, _, _, "~estäVn|~estänsä";
    possessive_ill_sg: _, _, _, _, "~eVnsä";
    possessive_ill_pl: "~iini", derived, derived, derived, "~iinsä";
    possessive_ade_sg: _, _, _, _, "~elläVn|~ellänsä";
    possessive_abl_sg: _, _, _, _, "~eltäVn|~eltänsä";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensä";
    possessive_ess_sg: _, _, _, _, "~enäVn|~enänsä";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensä";
    possessive_abe_sg: _, _, _, _, "~ettäVn|~ettänsä";
    ),
    // Siipi: 8 words (siipi, korpi, kilpi, rupi, läpi, koipi). Like Taso, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("siipi", 8, "~pi", like Taso;
    stem sg: "~p- like väki";
    stem pl: "~pi- like väki";
    nom: _, "~vet";
    ine: "~vessA", "~vissA";
    ela: "~vestA", "~vistA";
    ill: "~peVn", _;
    ade: "~vellA", "~villA";
    abl: "~veltA", "~viltA";
    all: _, "~ville";
    ess: "~penA", _;
    abe: "~vettA", "~vittA";
    possessive_nom_sg: "~peni", _, _, _, "~pensA";
    possessive_nom_pl: _, "~pesi", "~pemme", "~penne", _;
    possessive_gen_sg: _, "~pesi", "~pemme", "~penne", _;
    possessive_ine_sg: _, _, _, _, "~vessAVn|~vessänsä";
    possessive_ine_pl: "~vissAni", _, _, _, "~vissAVn|~vissänsä";
    possessive_ela_sg: _, _, _, _, "~vestAVn|~vestänsä";
    possessive_ela_pl: "~vistAni", _, _, _, "~vistAVn|~vistänsä";
    possessive_ill_sg: _, _, _, _, "~peVnsA";
    possessive_ade_sg: _, _, _, _, "~vellAVn|~vellänsä";
    possessive_ade_pl: "~villAni", _, _, _, "~villAVn|~villänsä";
    possessive_abl_sg: _, _, _, _, "~veltAVn|~veltänsä";
    possessive_abl_pl: "~viltAni", _, _, _, "~viltAVn|~viltänsä";
    possessive_all_sg: _, _, _, _, "~velleen|~vellensä";
    possessive_all_pl: "~villeni", _, _, _, "~villeen|~villensä";
    possessive_ess_sg: _, _, _, _, "~penAVn|~penänsä";
    possessive_tra_sg: _, _, _, _, "~vekseen|~veksensä";
    possessive_tra_pl: _, _, _, _, "~vikseen|~viksensä";
    possessive_abe_sg: _, _, _, _, "~vettAVn|~vettänsä";
    possessive_abe_pl: "~vittAni", _, _, _, "~vittAVn|~vittänsä";
    ),
    // Toimi: 7 words (lumi, luomi, tuomi, toimi, taimi, vastatoimi). Like Avain, except in 18 cells: nom sg, gen_ pl, par sg, par pl, ill pl, possessive_gen_pl _1sg, ….
    declension!("toimi", 7, "~mi", like Avain;
    nom: "~mi", _;
    gen_: _, "~mien|~nten";
    par: "~ntA|~meA", "~miA";
    ill: _, "~miin";
    possessive_gen_pl: "~mieni|~nteni", "~miesi|~ntesi", "~miemme|~ntemme", "~mienne|~ntenne", "~miensA|~ntensA";
    possessive_par_sg: _, _, _, _, "~ntAVn|~meAVn|~ntansa|~meansa";
    possessive_par_pl: "~meAni|~miAni", "~meAsi|~miAsi", "~meAmme|~miAmme", "~meAnne|~miAnne", "~miAVn|~miansa";
    possessive_ill_pl: "~miini", _, _, _, "~miinsA";
    ),
    // Keihäs: 6 words (kirves, pylväs, keihäs, räystäs, eväs, lähettiläs). Like Siemen with `` after each pattern (`s` after ~), except in 31 cells: gen_ pl, par sg, par pl, ine sg, ela sg, ill sg, ….
    declension!("keihäs", 6, "~s", like Siemen with "", "s" after "~";
    gen_: _, "~iden|~itten";
    par: "~stA", "~itA";
    ine: "~VssV", _;
    ela: "~VstV", _;
    ill: "~VseVn", "~isVin";
    ade: "~VllV", _;
    abl: "~VltV", _;
    ess: "~VnV", _;
    abe: "~VttV", _;
    possessive_nom_sg: _, _, _, _, "~VnsV";
    possessive_gen_pl: "~ideni|~itteni", "~idesi|~ittesi", "~idemme|~ittemme", "~idenne|~ittenne", "~idensA|~ittensä";
    possessive_par_sg: _, _, _, _, "~stAVn|~stänsä";
    possessive_par_pl: "~itAni", _, _, _, "~itAVn|~itänsä";
    possessive_ine_sg: _, _, _, _, "~VssVVn|~ässänsä";
    possessive_ela_sg: _, _, _, _, "~VstVVn|~ästänsä";
    possessive_ill_sg: _, _, _, _, "~VseVnsA";
    possessive_ill_pl: "~isVVni", _, _, _, "~isVinsA";
    possessive_ade_sg: _, _, _, _, "~VllVVn|~ällänsä";
    possessive_abl_sg: _, _, _, _, "~VltVVn|~ältänsä";
    possessive_all_sg: _, _, _, _, "~Vlleen|~ällensä";
    possessive_ess_sg: _, _, _, _, "~VnVVn|~änänsä";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~äksensä";
    possessive_abe_sg: _, _, _, _, "~VttVVn|~ättänsä";
    ),
    // Kuningatar: 6 words (kuningatar, ruhtinatar, haltijatar, onnetar, taiteilijatar, kiertotatar). Like Kate, except in 48 cells: gen_ pl, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("kuningatar", 6, "~~", like Kate;
    stem sg: "~~- like tarvas";
    gen_: _, "~t~ien|~~ten";
    par: _, "~t~ia";
    ine: "~t~essa", "~t~issa";
    ela: "~t~esta", "~t~ista";
    ill: "~t~een", "~t~iin";
    ade: "~t~ella", "~t~illa";
    abl: "~t~elta", "~t~ilta";
    ess: "~t~ena", "~t~ina";
    abe: "~t~etta", "~t~itta";
    possessive_nom_sg: _, _, _, _, "~t~ensa";
    possessive_gen_pl: "~t~ieni|~~teni", "~t~iesi|~~tesi", "~t~iemme|~~temme", "~t~ienne|~~tenne", "~t~iensa|~~tensa";
    possessive_par_pl: "~t~iani", _, _, _, "~t~iaan|~t~iansa";
    possessive_ine_sg: _, _, _, _, "~t~essaan|~t~essansa";
    possessive_ine_pl: "~t~issani", _, _, _, "~t~issaan|~t~issansa";
    possessive_ela_sg: _, _, _, _, "~t~estaan|~t~estansa";
    possessive_ela_pl: "~t~istani", _, _, _, "~t~istaan|~t~istansa";
    possessive_ill_sg: _, _, _, _, "~t~eensa";
    possessive_ill_pl: "~t~iini", derived, derived, derived, "~t~iinsa";
    possessive_ade_sg: _, _, _, _, "~t~ellaan|~t~ellansa";
    possessive_ade_pl: "~t~illani", _, _, _, "~t~illaan|~t~illansa";
    possessive_abl_sg: _, _, _, _, "~t~eltaan|~t~eltansa";
    possessive_abl_pl: "~t~iltani", _, _, _, "~t~iltaan|~t~iltansa";
    possessive_ess_sg: _, _, _, _, "~t~enaan|~t~enansa";
    possessive_ess_pl: "~t~inani", _, _, _, "~t~inaan|~t~inansa";
    possessive_abe_sg: _, _, _, _, "~t~ettaan|~t~ettansa";
    possessive_abe_pl: "~t~ittani", _, _, _, "~t~ittaan|~t~ittansa";
    ),
    // Kanapee: 6 words (glasee, filee, sisäfilee, essee, kanapee, shawnee). Like Aste with `` after each pattern (`e` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("kanapee", 6, "~e", like Aste with "", "e" after "~";
    nom: _, "~et";
    par: "~etA", _;
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~ehVn|~eseen", "~ihin|~isiin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    ess: "~enA", _;
    abe: "~ettA", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensA";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_par_sg: _, _, _, _, "~etAVn|~etansa";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essansa";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estansa";
    possessive_ill_sg: _, _, _, _, "~ehVnsA|~eseensa";
    possessive_ill_pl: "~esVVni|~ihini|~isVVni", "~esVVsi|~ihisi|~isVVsi", "~esVVmme|~ihimme|~isVVmme", "~esVVnne|~ihinne|~isVVnne", "~ihinsA|~isiinsa";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellansa";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltansa";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensa";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enansa";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensa";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettansa";
    ),
    // Maamerkki: 6 words (maamerkki, takapenkki, polttomerkki, jumbojetti, puumerkki, loppumerkki). Like Keri with `ˇ` after each pattern.
    declension!("maamerkki", 6, "~iˇ", like Keri with "ˇ";
    ),
    // Värit: 6 words (lasit, rillit, tanssit, värit, näpit, pihdit). Like Hmong with `` after each pattern (`it` after ~), except in 40 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("värit", 6, "~it", like Hmong with "", "it" after "~";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    com: _, derived;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~ini", _, _, _, "~insA";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_par_sg: _, _, _, _, derived;
    possessive_par_pl: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ine_pl: _, _, _, _, "~eissAVn|~eissänsä";
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ela_pl: _, _, _, _, "~eistAVn|~eistänsä";
    possessive_ill_sg: derived, derived, derived, derived, derived;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_ade_pl: _, _, _, _, "~eillAVn|~eillänsä";
    possessive_abl_sg: _, _, _, _, derived;
    possessive_abl_pl: _, _, _, _, "~eiltAVn|~eiltänsä";
    possessive_all_sg: _, _, _, _, derived;
    possessive_all_pl: _, _, _, _, "~eilleen|~eillensä";
    possessive_ess_sg: _, _, _, _, derived;
    possessive_ess_pl: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensä";
    possessive_abe_sg: _, _, _, _, derived;
    possessive_abe_pl: _, _, _, _, "~eittAVn|~eittänsä";
    possessive_com_pl: _, _, _, _, derived;
    ),
    // Koe: 5 words (koe, rae, eläinkoe, jae, hajukoe). Like Tunnus, except in 2 cells: ill sg, possessive_ill_sg _3.
    declension!("koe", 5, "~~", like Tunnus;
    stem sg: "~~- like aste";
    stem pl: "~k~i- like kate";
    ill: "~k~eseen", _;
    possessive_ill_sg: _, _, _, _, "~k~eseensA";
    ),
    // Jänne: 5 words (pinne, jänne, rinne, liikenne, käänne). Like Kate, except in 20 cells: possessive_gen_pl _3, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, ….
    declension!("jänne", 5, "~n~", like Kate;
    stem sg: "~n~- like jänne";
    possessive_gen_pl: _, _, _, _, "~t~idensA|~t~ittensä";
    possessive_par_pl: _, _, _, _, "~t~itAVn|~t~itänsä";
    possessive_ine_sg: _, _, _, _, "~t~essAVn|~t~essänsä";
    possessive_ine_pl: _, _, _, _, "~t~issAVn|~t~issänsä";
    possessive_ela_sg: _, _, _, _, "~t~estAVn|~t~estänsä";
    possessive_ela_pl: _, _, _, _, "~t~istAVn|~t~istänsä";
    possessive_ill_pl: _, _, _, _, "~t~isiinsA|~t~ihinsä";
    possessive_ade_sg: _, _, _, _, "~t~ellAVn|~t~ellänsä";
    possessive_ade_pl: _, _, _, _, "~t~illAVn|~t~illänsä";
    possessive_abl_sg: _, _, _, _, "~t~eltAVn|~t~eltänsä";
    possessive_abl_pl: _, _, _, _, "~t~iltAVn|~t~iltänsä";
    possessive_all_sg: _, _, _, _, "~t~elleen|~t~ellensä";
    possessive_all_pl: _, _, _, _, "~t~illeen|~t~illensä";
    possessive_ess_sg: _, _, _, _, "~t~enAVn|~t~enänsä";
    possessive_ess_pl: _, _, _, _, "~t~inAVn|~t~inänsä";
    possessive_tra_sg: _, _, _, _, "~t~ekseen|~t~eksensä";
    possessive_tra_pl: _, _, _, _, "~t~ikseen|~t~iksensä";
    possessive_abe_sg: _, _, _, _, "~t~ettAVn|~t~ettänsä";
    possessive_abe_pl: _, _, _, _, "~t~ittAVn|~t~ittänsä";
    possessive_com_pl: _, _, _, _, "~t~ineen|~t~inensä";
    ),
    // Khmer: 5 words (sandwich, blackjack, khmer, hi-tech, preesens). Like Keri, except in 2 cells: nom sg, possessive_ill_pl _1sg.
    declension!("khmer", 5, "~", like Keri;
    nom: "~", _;
    possessive_ill_pl: "~eihini", _, _, _, _;
    ),
    // Taival: 5 words (rove, tarve, taive, kuve, taival). Like Tunnus, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("taival", 5, "~v~", like Tunnus;
    stem sg: "~v~- like aste";
    stem pl: "~p~i- like kate";
    nom: _, "~p~et";
    ine: "~p~essA", "~p~issA";
    ela: "~p~estA", "~p~istA";
    ill: "~p~eseen", _;
    ade: "~p~ellA", "~p~illA";
    abl: "~p~eltA", "~p~iltA";
    all: _, "~p~ille";
    ess: "~p~enA", _;
    abe: "~p~ettA", "~p~ittA";
    possessive_nom_sg: "~p~eni", _, _, _, "~p~ensA";
    possessive_nom_pl: _, "~p~esi", "~p~emme", "~p~enne", _;
    possessive_gen_sg: _, "~p~esi", "~p~emme", "~p~enne", _;
    possessive_ine_sg: _, _, _, _, "~p~essAVn|~p~essansa";
    possessive_ine_pl: "~p~issAni", _, _, _, "~p~issAVn|~p~issansa";
    possessive_ela_sg: _, _, _, _, "~p~estAVn|~p~estansa";
    possessive_ela_pl: "~p~istAni", _, _, _, "~p~istAVn|~p~istansa";
    possessive_ill_sg: _, _, _, _, "~p~eseensA";
    possessive_ade_sg: _, _, _, _, "~p~ellAVn|~p~ellansa";
    possessive_ade_pl: "~p~illAni", _, _, _, "~p~illAVn|~p~illansa";
    possessive_abl_sg: _, _, _, _, "~p~eltAVn|~p~eltansa";
    possessive_abl_pl: "~p~iltAni", _, _, _, "~p~iltAVn|~p~iltansa";
    possessive_all_sg: _, _, _, _, "~p~elleen|~p~ellensa";
    possessive_all_pl: "~p~illeni", _, _, _, "~p~illeen|~p~illensa";
    possessive_ess_sg: _, _, _, _, "~p~enAVn|~p~enansa";
    possessive_tra_sg: _, _, _, _, "~p~ekseen|~p~eksensa";
    possessive_tra_pl: _, _, _, _, "~p~ikseen|~p~iksensa";
    possessive_abe_sg: _, _, _, _, "~p~ettAVn|~p~ettansa";
    possessive_abe_pl: "~p~ittAni", _, _, _, "~p~ittAVn|~p~ittansa";
    ),
    // Syy: 5 words (syy, kyy, tekosyy, peltopyy, menyy). Like Levy with `` after each pattern (`y` after ~), except in 36 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("syy", 5, "~y", like Levy with "", "y" after "~";
    nom: _, "~yt";
    gen_: _, "~iden|~itten";
    par: "~ytA", "~itA";
    ine: "~yssA", _;
    ela: "~ystA", _;
    ill: "~yhVn", _;
    ade: "~yllA", _;
    abl: "~yltA", _;
    ess: "~ynA", _;
    abe: "~yttA", _;
    possessive_nom_sg: "~yni", _, _, _, "~ynsA";
    possessive_nom_pl: _, "~ysi", "~ymme", "~ynne", _;
    possessive_gen_sg: _, "~ysi", "~ymme", "~ynne", _;
    possessive_gen_pl: "~ideni|~itteni", "~idesi|~ittesi", "~idemme|~ittemme", "~idenne|~ittenne", "~idensA|~ittensä";
    possessive_par_sg: _, _, _, _, "~ytAVn|~ytänsä";
    possessive_par_pl: "~itAni", _, _, _, "~itAVn|~itänsä";
    possessive_ine_sg: _, _, _, _, "~yssAVn|~yssänsä";
    possessive_ela_sg: _, _, _, _, "~ystAVn|~ystänsä";
    possessive_ill_sg: _, _, _, _, "~yhVnsA";
    possessive_ade_sg: _, _, _, _, "~yllAVn|~yllänsä";
    possessive_abl_sg: _, _, _, _, "~yltAVn|~yltänsä";
    possessive_all_sg: _, _, _, _, "~ylleen|~yllensä";
    possessive_ess_sg: _, _, _, _, "~ynAVn|~ynänsä";
    possessive_tra_sg: _, _, _, _, "~ykseen|~yksensä";
    possessive_abe_sg: _, _, _, _, "~yttAVn|~yttänsä";
    ),
    // Askel: 5 words (sammal, askel, lehtisammal, kivisammal, harmosammal).
    declension!("askel", 5, "~";
    stem pl: "~- like askel";
    nom: "~|~e", "~et|~eet";
    gen_: "~en|~een", _;
    par: "~tA|~etta", _;
    ine: "~essA|~eessa", "~issA|~eissa";
    ela: "~estA|~eesta", "~istA|~eista";
    ill: "~eeseen", _;
    ade: "~ellA|~eella", "~illA|~eilla";
    abl: "~eltA|~eelta", "~iltA|~eilta";
    all: "~elle|~eelle", "~ille|~eille";
    ess: "~enA|~eena", _;
    tra: "~eksi|~eeksi", "~iksi|~eiksi";
    abe: "~ettA|~eetta", "~ittA|~eitta";
    ins: _, "~in|~ein";
    possessive_nom_sg: "~eni", _, _, _, "~ensA|~eensa";
    possessive_nom_pl: "~eeni", "~eesi", "~eemme", "~eenne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_par_sg: _, _, _, _, "~tAVn|~ettaan|~tansa|~ettansa";
    possessive_ine_sg: _, _, _, _, "~essAVn|~eessaan|~essansa|~eessansa";
    possessive_ine_pl: "~eVssAni|~issani|~eVssAni", "~eVssAsi|~issasi|~eVssAsi", "~eVssAmme|~issamme|~eVssAmme", "~eVssAnne|~issanne|~eVssAnne", "~issAVn|~eissaan|~issansa|~eissansa";
    possessive_ela_sg: _, _, _, _, "~estAVn|~eestaan|~estansa|~eestansa";
    possessive_ela_pl: "~eVstAni|~istani|~eVstAni", "~eVstAsi|~istasi|~eVstAsi", "~eVstAmme|~istamme|~eVstAmme", "~eVstAnne|~istanne|~eVstAnne", "~istAVn|~eistaan|~istansa|~eistansa";
    possessive_ill_sg: _, _, _, _, "~eeseensa";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~eellaan|~ellansa|~eellansa";
    possessive_ade_pl: "~eVllAni|~illani|~eVllAni", "~eVllAsi|~illasi|~eVllAsi", "~eVllAmme|~illamme|~eVllAmme", "~eVllAnne|~illanne|~eVllAnne", "~illAVn|~eillaan|~illansa|~eillansa";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eeltaan|~eltansa|~eeltansa";
    possessive_abl_pl: "~eVltAni|~iltani|~eVltAni", "~eVltAsi|~iltasi|~eVltAsi", "~eVltAmme|~iltamme|~eVltAmme", "~eVltAnne|~iltanne|~eVltAnne", "~iltAVn|~eiltaan|~iltansa|~eiltansa";
    possessive_all_sg: _, _, _, _, "~elleen|~eelleen|~ellensa|~eellensa";
    possessive_all_pl: "~eVlleni|~illeni|~eVlleni", "~eVllesi|~illesi|~eVllesi", "~eVllemme|~illemme|~eVllemme", "~eVllenne|~illenne|~eVllenne", "~illeen|~eilleen|~illensa|~eillensa";
    possessive_ess_sg: _, _, _, _, "~enAVn|~eenaan|~enansa|~eenansa";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eekseen|~eksensa|~eeksensa";
    possessive_tra_pl: "~eVkseni|~ikseni|~eVkseni", "~eVksesi|~iksesi|~eVksesi", "~eVksemme|~iksemme|~eVksemme", "~eVksenne|~iksenne|~eVksenne", "~ikseen|~eikseen|~iksensa|~eiksensa";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~eettaan|~ettansa|~eettansa";
    possessive_abe_pl: "~eVttAni|~ittani|~eVttAni", "~eVttAsi|~ittasi|~eVttAsi", "~eVttAmme|~ittamme|~eVttAmme", "~eVttAnne|~ittanne|~eVttAnne", "~ittAVn|~eittaan|~ittansa|~eittansa";
    ),
    // Kissanristiäiset: 5 words (päättäjäiset, metsäpäästäiset, myötäjäiset, kädelliset, kissanristiäiset). Like Siemen with `` after each pattern (`et` after ~), except in 35 cells: nom pl, gen_ pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("kissanristiäiset", 5, "~et", like Siemen with "", "et" after "~";
    nom: _, "~et";
    gen_: _, "~ten|~ien";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~eni", "~esi", "~emme", "~enne", "~ensA";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensA|~iensA";
    possessive_par_sg: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Lupa: 5 words (sopa, salpa, kalpa, soopa, huopa). Like Siipi, except in 32 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("lupa", 5, "~pa", like Siipi;
    stem sg: "~pa- like osa";
    stem pl: "~pi- like osa";
    nom: _, "~vAt";
    ine: "~vAssV", _;
    ela: "~vAstV", _;
    ill: "~paVn", _;
    ade: "~vAllV", _;
    abl: "~vAltV", _;
    ess: "~panV", _;
    abe: "~vAttV", _;
    possessive_nom_sg: "~pani", _, _, _, "~pansV";
    possessive_nom_pl: _, "~pasi", "~pamme", "~panne", _;
    possessive_gen_sg: _, "~pasi", "~pamme", "~panne", _;
    possessive_ine_sg: _, _, _, _, "~vAssVVn|~vassansa";
    possessive_ine_pl: _, _, _, _, "~vissAVn|~vissansa";
    possessive_ela_sg: _, _, _, _, "~vAstVVn|~vastansa";
    possessive_ela_pl: _, _, _, _, "~vistAVn|~vistansa";
    possessive_ill_sg: _, _, _, _, "~paVnsV";
    possessive_ade_sg: _, _, _, _, "~vAllVVn|~vallansa";
    possessive_ade_pl: _, _, _, _, "~villAVn|~villansa";
    possessive_abl_sg: _, _, _, _, "~vAltVVn|~valtansa";
    possessive_abl_pl: _, _, _, _, "~viltAVn|~viltansa";
    possessive_all_sg: _, _, _, _, "~vAlleen|~vallensa";
    possessive_all_pl: _, _, _, _, "~villeen|~villensa";
    possessive_ess_sg: _, _, _, _, "~panVVn|~panansa";
    possessive_tra_sg: _, _, _, _, "~vAkseen|~vaksensa";
    possessive_tra_pl: _, _, _, _, "~vikseen|~viksensa";
    possessive_abe_sg: _, _, _, _, "~vAttVVn|~vattansa";
    possessive_abe_pl: _, _, _, _, "~vittAVn|~vittansa";
    ),
    // Paikka: 4 words (jalka, taka, nahka, oravannahka). Like Silmukka, except in 14 cells: ins sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_par_pl _2sg, possessive_par_pl _1pl, ….
    declension!("paikka", 4, "~ka", like Silmukka;
    stem pl: "~ko- like taso";
    ins: "~an", _;
    possessive_gen_pl: _, derived, derived, derived, _;
    possessive_par_pl: _, derived, derived, derived, _;
    possessive_ill_pl: _, derived, derived, derived, _;
    possessive_ins_sg: _, "~kasi", "~kamme", "~kanne", "~kansa";
    ),
    // Kupu: 4 words (riepu, hepo, virtahepo, ropo). Like Luku, except in 12 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _3, possessive_nom_pl _2sg, possessive_nom_pl _1pl, ….
    declension!("kupu", 4, "~p~", like Luku;
    stem sg: "~p~- like taso";
    stem pl: "~p~- like taso";
    ill: "~p~Vn", _;
    ess: "~p~nA", _;
    possessive_nom_sg: "~p~ni", _, _, _, "~p~nsA";
    possessive_nom_pl: _, "~p~si", "~p~mme", "~p~nne", _;
    possessive_gen_sg: _, "~p~si", "~p~mme", "~p~nne", _;
    possessive_ill_sg: _, _, _, _, "~p~VnsA";
    possessive_ess_sg: _, _, _, _, "~p~nAVn|~p~nansa";
    ),
    // Tee: 4 words (tee, gee, lee, kaksoisvee). Like Piste with `` after each pattern (`e` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("tee", 4, "~e", like Piste with "", "e" after "~";
    nom: _, "~et";
    par: "~etä", _;
    ine: "~essä", _;
    ela: "~estä", _;
    ill: "~ehVn", "~ihin";
    ade: "~ellä", _;
    abl: "~eltä", _;
    ess: "~enä", _;
    abe: "~että", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensä";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_par_sg: _, _, _, _, "~etäVn|~etänsä";
    possessive_ine_sg: _, _, _, _, "~essäVn|~essänsä";
    possessive_ela_sg: _, _, _, _, "~estäVn|~estänsä";
    possessive_ill_sg: _, _, _, _, "~ehVnsä";
    possessive_ill_pl: "~ihini", derived, derived, derived, "~ihinsä";
    possessive_ade_sg: _, _, _, _, "~elläVn|~ellänsä";
    possessive_abl_sg: _, _, _, _, "~eltäVn|~eltänsä";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensä";
    possessive_ess_sg: _, _, _, _, "~enäVn|~enänsä";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensä";
    possessive_abe_sg: _, _, _, _, "~ettäVn|~ettänsä";
    ),
    // Poika: 4 words (poika, talonpoika, rattopoika, sulhaspoika). Like Aika, except in 32 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("poika", 4, "~ika", like Aika;
    stem pl: "~ik- like poika";
    nom: _, "~jAt";
    ine: "~jAssV", "~jissA";
    ela: "~jAstV", "~jistA";
    ade: "~jAllV", "~jillA";
    abl: "~jAltV", "~jiltA";
    all: _, "~jille";
    abe: "~jAttV", "~jittA";
    possessive_ine_sg: _, _, _, _, "~jAssVVn|~jassansa";
    possessive_ine_pl: "~jissAni", _, _, _, "~jissAVn|~jissansa";
    possessive_ela_sg: _, _, _, _, "~jAstVVn|~jastansa";
    possessive_ela_pl: "~jistAni", _, _, _, "~jistAVn|~jistansa";
    possessive_ade_sg: _, _, _, _, "~jAllVVn|~jallansa";
    possessive_ade_pl: "~jillAni", _, _, _, "~jillAVn|~jillansa";
    possessive_abl_sg: _, _, _, _, "~jAltVVn|~jaltansa";
    possessive_abl_pl: "~jiltAni", _, _, _, "~jiltAVn|~jiltansa";
    possessive_all_sg: _, _, _, _, "~jAlleen|~jallensa";
    possessive_all_pl: "~jilleni", _, _, _, "~jilleen|~jillensa";
    possessive_tra_sg: _, _, _, _, "~jAkseen|~jaksensa";
    possessive_tra_pl: _, _, _, _, "~jikseen|~jiksensa";
    possessive_abe_sg: _, _, _, _, "~jAttVVn|~jattansa";
    possessive_abe_pl: "~jittAni", _, _, _, "~jittAVn|~jittansa";
    ),
    // Hopea: 4 words (valkea, elohopea, hopea, kulovalkea). Like Aste with `` after each pattern (`a` after ~), except in 35 cells: nom pl, par sg, ine sg, ela sg, ill sg, ade sg, ….
    declension!("hopea", 4, "~a", like Aste with "", "a" after "~";
    nom: _, "~at";
    par: "~aV|~ata", _;
    ine: "~assV", _;
    ela: "~astV", _;
    ill: "~aVn", _;
    ade: "~allV", _;
    abl: "~altV", _;
    ess: "~anV", _;
    abe: "~attV", _;
    possessive_nom_sg: "~ani", _, _, _, "~ansV";
    possessive_nom_pl: _, "~asi", "~amme", "~anne", _;
    possessive_gen_sg: _, "~asi", "~amme", "~anne", _;
    possessive_par_sg: _, _, _, _, "~atVVn|~aansa|~atansa";
    possessive_par_pl: "~atVni|~itVni", "~atVsi|~itVsi", "~atVmme|~itVmme", "~atVnne|~itVnne", _;
    possessive_ine_sg: _, _, _, _, "~assVVn|~assansa";
    possessive_ela_sg: _, _, _, _, "~astVVn|~astansa";
    possessive_ill_sg: _, _, _, _, "~aVnsV";
    possessive_ill_pl: "~isVVni|~ihini", "~isVVsi|~ihisi", "~isVVmme|~ihimme", "~isVVnne|~ihinne", _;
    possessive_ade_sg: _, _, _, _, "~allVVn|~allansa";
    possessive_abl_sg: _, _, _, _, "~altVVn|~altansa";
    possessive_all_sg: _, _, _, _, "~alleen|~allensa";
    possessive_ess_sg: _, _, _, _, "~anVVn|~anansa";
    possessive_tra_sg: _, _, _, _, "~akseen|~aksensa";
    possessive_abe_sg: _, _, _, _, "~attVVn|~attansa";
    ),
    // Ydin: 4 words (ydin, säädin, vedin, ulosvedin). Like Osoitin, except in 48 cells: gen_ pl, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("ydin", 4, "~d~n", like Osoitin;
    stem sg: "~d~n- like etumies";
    gen_: _, "~t~mVen|~d~nten";
    par: _, "~t~mVä";
    ine: "~t~messä", "~t~missä";
    ela: "~t~mestä", "~t~mistä";
    ill: "~t~meen", _;
    ade: "~t~mellä", "~t~millä";
    abl: "~t~meltä", "~t~miltä";
    ess: "~t~menä", "~t~minä";
    abe: "~t~mettä", "~t~mittä";
    possessive_nom_sg: _, _, _, _, "~t~mensä";
    possessive_gen_pl: "~t~mVeni|~d~nteni", "~t~mVesi|~d~ntesi", "~t~mVemme|~d~ntemme", "~t~mVenne|~d~ntenne", "~t~mVensä|~d~ntensä";
    possessive_par_pl: "~t~mVäni", _, _, _, "~t~mVäVn|~t~miänsä";
    possessive_ine_sg: _, _, _, _, "~t~messäVn|~t~messänsä";
    possessive_ine_pl: "~t~missäni", _, _, _, "~t~missäVn|~t~missänsä";
    possessive_ela_sg: _, _, _, _, "~t~mestäVn|~t~mestänsä";
    possessive_ela_pl: "~t~mistäni", _, _, _, "~t~mistäVn|~t~mistänsä";
    possessive_ill_sg: _, _, _, _, "~t~meensä";
    possessive_ill_pl: _, _, _, _, "~t~miinsä";
    possessive_ade_sg: _, _, _, _, "~t~melläVn|~t~mellänsä";
    possessive_ade_pl: "~t~milläni", _, _, _, "~t~milläVn|~t~millänsä";
    possessive_abl_sg: _, _, _, _, "~t~meltäVn|~t~meltänsä";
    possessive_abl_pl: "~t~miltäni", _, _, _, "~t~miltäVn|~t~miltänsä";
    possessive_all_sg: _, _, _, _, "~t~melleen|~t~mellensä";
    possessive_all_pl: _, _, _, _, "~t~milleen|~t~millensä";
    possessive_ess_sg: _, _, _, _, "~t~menäVn|~t~menänsä";
    possessive_ess_pl: "~t~minäni", _, _, _, "~t~minäVn|~t~minänsä";
    possessive_tra_sg: _, _, _, _, "~t~mekseen|~t~meksensä";
    possessive_tra_pl: _, _, _, _, "~t~mikseen|~t~miksensä";
    possessive_abe_sg: _, _, _, _, "~t~mettäVn|~t~mettänsä";
    possessive_abe_pl: "~t~mittäni", _, _, _, "~t~mittäVn|~t~mittänsä";
    possessive_com_pl: _, _, _, _, "~t~mineen|~t~minensä";
    ),
    // Kauris: 4 words (kauris, saalis, ruumis, metsäkauris). Like Aste with `` after each pattern (`s` after ~), except in 27 cells: gen_ pl, par sg, par pl, ill sg, ill pl, possessive_gen_pl _1sg, ….
    declension!("kauris", 4, "~s", like Aste with "", "s" after "~";
    gen_: _, "~Vden|~itten";
    par: "~stA", "~VtA";
    ill: "~VseVn", "~VsVin";
    possessive_gen_pl: "~Vdeni|~itteni", "~Vdesi|~ittesi", "~Vdemme|~ittemme", "~Vdenne|~ittenne", "~VdensA|~ittensa";
    possessive_par_sg: _, _, _, _, "~stAVn|~stansa";
    possessive_par_pl: "~VtAni", _, _, _, "~VtAVn|~itansa";
    possessive_ine_sg: _, _, _, _, "~VssAVn|~issansa";
    possessive_ela_sg: _, _, _, _, "~VstAVn|~istansa";
    possessive_ill_sg: _, _, _, _, "~VseVnsA";
    possessive_ill_pl: "~VsVVni", derived, derived, derived, "~VsVinsA";
    possessive_ade_sg: _, _, _, _, "~VllAVn|~illansa";
    possessive_abl_sg: _, _, _, _, "~VltAVn|~iltansa";
    possessive_all_sg: _, _, _, _, "~Vlleen|~illensa";
    possessive_ess_sg: _, _, _, _, "~VnAVn|~inansa";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~iksensa";
    possessive_abe_sg: _, _, _, _, "~VttAVn|~ittansa";
    ),
    // Rättänä: 4 words (äpärä, käpälä, kissankäpälä, rättänä). Like Erä, except in 43 cells: ine pl, ela pl, ade pl, abl pl, all pl, tra pl, ….
    declension!("rättänä", 4, "~ä", like Erä;
    stem pl: "~- like rättänä";
    ine: _, "~öissA|~issä";
    ela: _, "~öistA|~istä";
    ade: _, "~öillA|~illä";
    abl: _, "~öiltA|~iltä";
    all: _, "~öille|~ille";
    tra: _, "~öiksi|~iksi";
    abe: _, "~öittA|~ittä";
    ins: _, "~öin|~in";
    possessive_ine_pl: "~öissAni|~issäni", "~öissAsi|~issäsi", "~öissAmme|~issämme", "~öissAnne|~issänne", "~öissAVn|~issään|~öissänsä|~issänsä";
    possessive_ela_pl: "~öistAni|~istäni", "~öistAsi|~istäsi", "~öistAmme|~istämme", "~öistAnne|~istänne", "~öistAVn|~istään|~öistänsä|~istänsä";
    possessive_ade_pl: "~öillAni|~illäni", "~öillAsi|~illäsi", "~öillAmme|~illämme", "~öillAnne|~illänne", "~öillAVn|~illään|~öillänsä|~illänsä";
    possessive_abl_pl: "~öiltAni|~iltäni", "~öiltAsi|~iltäsi", "~öiltAmme|~iltämme", "~öiltAnne|~iltänne", "~öiltAVn|~iltään|~öiltänsä|~iltänsä";
    possessive_all_pl: "~öilleni|~illeni", "~öillesi|~illesi", "~öillemme|~illemme", "~öillenne|~illenne", "~öilleen|~illeen|~öillensä|~illensä";
    possessive_tra_pl: "~öikseni|~ikseni", "~öiksesi|~iksesi", "~öiksemme|~iksemme", "~öiksenne|~iksenne", "~öikseen|~ikseen|~öiksensä|~iksensä";
    possessive_abe_pl: "~öittAni|~ittäni", "~öittAsi|~ittäsi", "~öittAmme|~ittämme", "~öittAnne|~ittänne", "~öittAVn|~ittään|~öittänsä|~ittänsä";
    ),
    // Takuu: 4 words (takuu, paluu, vastuu, makuu). Like Aste with `` after each pattern (`u` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("takuu", 4, "~u", like Aste with "", "u" after "~";
    nom: _, "~ut";
    par: "~utA", _;
    ine: "~ussA", _;
    ela: "~ustA", _;
    ill: "~useVn", "~isVin";
    ade: "~ullA", _;
    abl: "~ultA", _;
    ess: "~unA", _;
    abe: "~uttA", _;
    possessive_nom_sg: "~uni", _, _, _, "~unsA";
    possessive_nom_pl: _, "~usi", "~umme", "~unne", _;
    possessive_gen_sg: _, "~usi", "~umme", "~unne", _;
    possessive_par_sg: _, _, _, _, "~utAVn|~utansa";
    possessive_ine_sg: _, _, _, _, "~ussAVn|~ussansa";
    possessive_ela_sg: _, _, _, _, "~ustAVn|~ustansa";
    possessive_ill_sg: _, _, _, _, "~useVnsA";
    possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    possessive_ade_sg: _, _, _, _, "~ullAVn|~ullansa";
    possessive_abl_sg: _, _, _, _, "~ultAVn|~ultansa";
    possessive_all_sg: _, _, _, _, "~ulleen|~ullensa";
    possessive_ess_sg: _, _, _, _, "~unAVn|~unansa";
    possessive_tra_sg: _, _, _, _, "~ukseen|~uksensa";
    possessive_abe_sg: _, _, _, _, "~uttAVn|~uttansa";
    ),
    // Käräjät: 4 words (myyrät, nisäkkäät, valtiopäivät, käräjät). Like Siemen with `` after each pattern (`ät` after ~), except in 35 cells: nom pl, gen_ pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("käräjät", 4, "~ät", like Siemen with "", "ät" after "~";
    nom: _, "~ät";
    gen_: _, "~ien";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~Ani", "~Asi", "~Amme", "~Anne", "~AnsV";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_gen_pl: "~ieni", derived, derived, derived, "~iensA";
    possessive_par_sg: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Vehje: 4 words (hylje, vehje, sulje, rahje). Like Koe, except in 24 cells: possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, ….
    declension!("vehje", 4, "~j~", like Koe;
    stem sg: "~j~- like jänne";
    stem pl: "~k~i- like vehje";
    possessive_gen_pl: _, derived, derived, derived, _;
    possessive_par_pl: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ine_pl: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ela_pl: _, _, _, _, derived;
    possessive_ill_pl: _, derived, derived, derived, _;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_ade_pl: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_abl_pl: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_all_pl: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_ess_pl: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_tra_pl: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    possessive_abe_pl: _, _, _, _, derived;
    possessive_com_pl: _, _, _, _, derived;
    ),
    // Onki: 4 words (henki, onki, kanki, kiertokanki). Like Väki, except in 34 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("onki", 4, "~ki", like Väki;
    stem pl: "~ki- like osa";
    nom: _, "~get";
    ine: "~gessA", "~gissA";
    ela: "~gestA", "~gistA";
    ade: "~gellA", "~gillA";
    abl: "~geltA", "~giltA";
    all: _, "~gille";
    abe: "~gettA", "~gittA";
    possessive_par_sg: _, _, _, _, "~keAVn|~keansa";
    possessive_ine_sg: _, _, _, _, "~gessAVn|~gessansa";
    possessive_ine_pl: "~gissAni", _, _, _, "~gissAVn|~gissansa";
    possessive_ela_sg: _, _, _, _, "~gestAVn|~gestansa";
    possessive_ela_pl: "~gistAni", _, _, _, "~gistAVn|~gistansa";
    possessive_ade_sg: _, _, _, _, "~gellAVn|~gellansa";
    possessive_ade_pl: "~gillAni", _, _, _, "~gillAVn|~gillansa";
    possessive_abl_sg: _, _, _, _, "~geltAVn|~geltansa";
    possessive_abl_pl: "~giltAni", _, _, _, "~giltAVn|~giltansa";
    possessive_all_sg: _, _, _, _, "~gelleen|~gellensa";
    possessive_all_pl: "~gilleni", _, _, _, "~gilleen|~gillensa";
    possessive_ess_sg: _, _, _, _, "~kenAVn|~kenansa";
    possessive_tra_sg: _, _, _, _, "~gekseen|~geksensa";
    possessive_tra_pl: _, _, _, _, "~gikseen|~giksensa";
    possessive_abe_sg: _, _, _, _, "~gettAVn|~gettansa";
    possessive_abe_pl: "~gittAni", _, _, _, "~gittAVn|~gittansa";
    ),
    // Piennar: 4 words (manner, tanner, kinner, piennar). Like Askel with `ˇ` after each pattern.
    declension!("piennar", 4, "~ˇ", like Askel with "ˇ";
    ),
    // Leipä: 3 words (syöpä, verisyöpä, sepä). Like Siipi, except in 40 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("leipä", 3, "~pä", like Siipi;
    stem sg: "~pä- like osa";
    stem pl: "~pi- like erä";
    nom: _, "~vät";
    ine: "~vässV", "~vissä";
    ela: "~västV", "~vistä";
    ill: "~päVn", _;
    ade: "~vällV", "~villä";
    abl: "~vältV", "~viltä";
    ess: "~pänV", _;
    abe: "~vättV", "~vittä";
    possessive_nom_sg: "~päni", _, _, _, "~pänsV";
    possessive_nom_pl: _, "~päsi", "~pämme", "~pänne", _;
    possessive_gen_sg: _, "~päsi", "~pämme", "~pänne", _;
    possessive_ine_sg: _, _, _, _, "~vässVVn|~vässänsä";
    possessive_ine_pl: "~vissäni", _, _, _, "~vissäVn|~vissänsä";
    possessive_ela_sg: _, _, _, _, "~västVVn|~västänsä";
    possessive_ela_pl: "~vistäni", _, _, _, "~vistäVn|~vistänsä";
    possessive_ill_sg: _, _, _, _, "~päVnsV";
    possessive_ade_sg: _, _, _, _, "~vällVVn|~vällänsä";
    possessive_ade_pl: "~villäni", _, _, _, "~villäVn|~villänsä";
    possessive_abl_sg: _, _, _, _, "~vältVVn|~vältänsä";
    possessive_abl_pl: "~viltäni", _, _, _, "~viltäVn|~viltänsä";
    possessive_all_sg: _, _, _, _, "~välleen|~vällensä";
    possessive_ess_sg: _, _, _, _, "~pänVVn|~pänänsä";
    possessive_tra_sg: _, _, _, _, "~väkseen|~väksensä";
    possessive_abe_sg: _, _, _, _, "~vättVVn|~vättänsä";
    possessive_abe_pl: "~vittäni", _, _, _, "~vittäVn|~vittänsä";
    ),
    // Body: 3 words (body, curry, copy). Like Taso, except in 39 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("body", 3, "~", like Taso;
    par: "~Ä", "~jÄ";
    ine: "~ssÄ", "~issT";
    ela: "~stÄ", "~ista";
    ade: "~llÄ", "~illT";
    abl: "~ltÄ", "~ilta";
    ess: "~nÄ", "~inT";
    abe: "~ttÄ", "~itta";
    possessive_nom_sg: _, _, _, _, "~nsÄ";
    possessive_gen_pl: _, _, _, _, "~jensÄ";
    possessive_par_sg: _, _, _, _, "~ÄVn|~ansa";
    possessive_par_pl: "~jÄni", _, _, _, "~jÄVn|~jansa";
    possessive_ine_sg: _, _, _, _, "~ssÄVn|~ssansa";
    possessive_ine_pl: "~issTni", _, _, _, "~issTVn|~issansa";
    possessive_ela_sg: _, _, _, _, "~stÄVn|~stansa";
    possessive_ela_pl: "~istani", _, _, _, "~istaVn|~istansa";
    possessive_ill_sg: _, _, _, _, "~VnsÄ";
    possessive_ill_pl: _, _, _, _, "~ihinsT";
    possessive_ade_sg: _, _, _, _, "~llÄVn|~llansa";
    possessive_ade_pl: "~illTni", _, _, _, "~illTVn|~illansa";
    possessive_abl_sg: _, _, _, _, "~ltÄVn|~ltansa";
    possessive_abl_pl: "~iltani", _, _, _, "~iltaVn|~iltansa";
    possessive_ess_sg: _, _, _, _, "~nÄVn|~nansa";
    possessive_ess_pl: "~inTni", _, _, _, "~inTVn|~inansa";
    possessive_abe_sg: _, _, _, _, "~ttÄVn|~ttansa";
    possessive_abe_pl: "~ittani", _, _, _, "~ittaVn|~ittansa";
    ),
    // Hammas: 3 words (lammas, hammas, kulmahammas). Like Taival, except in 31 cells: nom sg, nom pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("hammas", 3, "~m~s", like Taival;
    stem pl: "~p~i- like ruis";
    nom: "~m~s", "~p~Vt";
    par: "~m~sta", _;
    ine: "~p~VssV", _;
    ela: "~p~VstV", _;
    ill: "~p~Vseen", _;
    ade: "~p~VllV", _;
    abl: "~p~VltV", _;
    ess: "~p~VnV", _;
    abe: "~p~VttV", _;
    possessive_nom_sg: "~p~Vni", _, _, _, "~p~VnsV";
    possessive_nom_pl: _, "~p~Vsi", "~p~Vmme", "~p~Vnne", _;
    possessive_gen_sg: _, "~p~Vsi", "~p~Vmme", "~p~Vnne", _;
    possessive_par_sg: _, _, _, _, "~m~staan|~m~stansa";
    possessive_ine_sg: _, _, _, _, "~p~VssVVn|~p~assansa";
    possessive_ela_sg: _, _, _, _, "~p~VstVVn|~p~astansa";
    possessive_ill_sg: _, _, _, _, "~p~VseensA";
    possessive_ill_pl: _, derived, derived, derived, _;
    possessive_ade_sg: _, _, _, _, "~p~VllVVn|~p~allansa";
    possessive_abl_sg: _, _, _, _, "~p~VltVVn|~p~altansa";
    possessive_all_sg: _, _, _, _, "~p~Vlleen|~p~allensa";
    possessive_ess_sg: _, _, _, _, "~p~VnVVn|~p~anansa";
    possessive_tra_sg: _, _, _, _, "~p~Vkseen|~p~aksensa";
    possessive_abe_sg: _, _, _, _, "~p~VttVVn|~p~attansa";
    ),
    // Komme: 3 words (komme, lumme, ommel). Like Taival, except in 27 cells: nom sg, par sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_par_sg _3, ….
    declension!("komme", 3, "~m~", like Taival;
    stem pl: "~p~i- like vehje";
    nom: "~m~", _;
    par: "~m~tta", _;
    possessive_gen_pl: _, derived, derived, derived, _;
    possessive_par_sg: _, _, _, _, "~m~ttaan|~m~ttansa";
    possessive_par_pl: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ine_pl: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ela_pl: _, _, _, _, derived;
    possessive_ill_pl: _, derived, derived, derived, _;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_ade_pl: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_abl_pl: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_all_pl: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_ess_pl: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_tra_pl: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    possessive_abe_pl: _, _, _, _, derived;
    possessive_com_pl: _, _, _, _, derived;
    ),
    // Leve: 3 words (leve, helve, lieve). Like Taival, except in 20 cells: possessive_gen_pl _3, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, ….
    declension!("leve", 3, "~v~", like Taival;
    stem sg: "~v~- like jänne";
    possessive_gen_pl: _, _, _, _, "~p~idensA|~p~ittensä";
    possessive_par_pl: _, _, _, _, "~p~itAVn|~p~itänsä";
    possessive_ine_sg: _, _, _, _, "~p~essAVn|~p~essänsä";
    possessive_ine_pl: _, _, _, _, "~p~issAVn|~p~issänsä";
    possessive_ela_sg: _, _, _, _, "~p~estAVn|~p~estänsä";
    possessive_ela_pl: _, _, _, _, "~p~istAVn|~p~istänsä";
    possessive_ill_pl: _, _, _, _, "~p~isiinsA|~p~ihinsä";
    possessive_ade_sg: _, _, _, _, "~p~ellAVn|~p~ellänsä";
    possessive_ade_pl: _, _, _, _, "~p~illAVn|~p~illänsä";
    possessive_abl_sg: _, _, _, _, "~p~eltAVn|~p~eltänsä";
    possessive_abl_pl: _, _, _, _, "~p~iltAVn|~p~iltänsä";
    possessive_all_sg: _, _, _, _, "~p~elleen|~p~ellensä";
    possessive_all_pl: _, _, _, _, "~p~illeen|~p~illensä";
    possessive_ess_sg: _, _, _, _, "~p~enAVn|~p~enänsä";
    possessive_ess_pl: _, _, _, _, "~p~inAVn|~p~inänsä";
    possessive_tra_sg: _, _, _, _, "~p~ekseen|~p~eksensä";
    possessive_tra_pl: _, _, _, _, "~p~ikseen|~p~iksensä";
    possessive_abe_sg: _, _, _, _, "~p~ettAVn|~p~ettänsä";
    possessive_abe_pl: _, _, _, _, "~p~ittAVn|~p~ittänsä";
    possessive_com_pl: _, _, _, _, "~p~ineen|~p~inensä";
    ),
    // Bokserit: 3 words (polttarit, kroaatit, bokserit). Like Kaanon with `` after each pattern (`it` after ~), except in 25 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("bokserit", 3, "~it", like Kaanon with "", "it" after "~";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~ini", _, _, _, "~insA";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_par_sg: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Opas: 3 words (opas, lipas, matkaopas). Like Hammas, except in 48 cells: nom pl, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("opas", 3, "~~s", like Hammas;
    stem sg: "~~s- like tarvas";
    nom: _, "~p~at";
    par: _, "~p~ita";
    ine: "~p~assa", "~p~issa";
    ela: "~p~asta", "~p~ista";
    ill: "~p~aseen", _;
    ade: "~p~alla", "~p~illa";
    abl: "~p~alta", "~p~ilta";
    ess: "~p~ana", "~p~ina";
    abe: "~p~atta", "~p~itta";
    possessive_nom_sg: "~p~ani", _, _, _, "~p~ansa";
    possessive_nom_pl: _, "~p~asi", "~p~amme", "~p~anne", _;
    possessive_gen_sg: _, "~p~asi", "~p~amme", "~p~anne", _;
    possessive_gen_pl: _, _, _, _, "~p~idensa|~p~ittensa";
    possessive_par_pl: "~p~itani", _, _, _, "~p~itaan|~p~itansa";
    possessive_ine_sg: _, _, _, _, "~p~assaan|~p~assansa";
    possessive_ine_pl: "~p~issani", _, _, _, "~p~issaan|~p~issansa";
    possessive_ela_sg: _, _, _, _, "~p~astaan|~p~astansa";
    possessive_ela_pl: "~p~istani", _, _, _, "~p~istaan|~p~istansa";
    possessive_ill_sg: _, _, _, _, "~p~aseensa";
    possessive_ill_pl: _, _, _, _, "~p~isiinsa";
    possessive_ade_sg: _, _, _, _, "~p~allaan|~p~allansa";
    possessive_ade_pl: "~p~illani", _, _, _, "~p~illaan|~p~illansa";
    possessive_abl_sg: _, _, _, _, "~p~altaan|~p~altansa";
    possessive_abl_pl: "~p~iltani", _, _, _, "~p~iltaan|~p~iltansa";
    possessive_all_sg: _, _, _, _, "~p~alleen|~p~allensa";
    possessive_ess_sg: _, _, _, _, "~p~anaan|~p~anansa";
    possessive_ess_pl: "~p~inani", _, _, _, "~p~inaan|~p~inansa";
    possessive_tra_sg: _, _, _, _, "~p~akseen|~p~aksensa";
    possessive_abe_sg: _, _, _, _, "~p~attaan|~p~attansa";
    possessive_abe_pl: "~p~ittani", _, _, _, "~p~ittaan|~p~ittansa";
    ),
    // Lapsi: 3 words (lapsi, opetuslapsi, sotalapsi). Like Taival, except in 20 cells: nom sg, gen_ pl, par sg, par pl, ill sg, ill pl, ….
    declension!("lapsi", 3, "~p~i", like Taival;
    nom: "~p~i", _;
    gen_: _, "~p~ien|~~ten";
    par: "~~ta", "~p~ia";
    ill: "~p~een", "~p~iin";
    possessive_gen_pl: "~p~ieni|~~teni", "~p~iesi|~~tesi", "~p~iemme|~~temme", "~p~ienne|~~tenne", "~p~iensa|~~tensa";
    possessive_par_sg: _, _, _, _, "~~taan|~~tansa";
    possessive_par_pl: "~p~iani", _, _, _, "~p~iaan|~p~iansa";
    possessive_ill_sg: _, _, _, _, "~p~eensa";
    possessive_ill_pl: "~p~iini", derived, derived, derived, "~p~iinsa";
    ),
    // Markkinat: 3 words (kakkulat, talvikisat, markkinat). Like Nappula with `` after each pattern (`t` after a), except in 40 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("markkinat", 3, "~at", like Nappula with "", "t" after "a";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~Ani", "~Asi", "~Amme", "~Anne", "~AnsV";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_gen_pl: _, _, _, _, "~oidensA|~oittensa";
    possessive_par_sg: _, _, _, _, derived;
    possessive_par_pl: _, _, _, _, "~oitAVn|~oitansa";
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ine_pl: _, _, _, _, "~oissAVn|~oissansa";
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ela_pl: _, _, _, _, "~oistAVn|~oistansa";
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ill_pl: "~oihini", _, _, _, _;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_ade_pl: _, _, _, _, "~oillAVn|~oillansa";
    possessive_abl_sg: _, _, _, _, derived;
    possessive_abl_pl: _, _, _, _, "~oiltAVn|~oiltansa";
    possessive_all_sg: _, _, _, _, derived;
    possessive_all_pl: _, _, _, _, "~oilleen|~oillensa";
    possessive_ess_sg: _, _, _, _, derived;
    possessive_ess_pl: _, _, _, _, "~oinAVn|~oinansa";
    possessive_tra_sg: _, _, _, _, derived;
    possessive_tra_pl: _, _, _, _, "~oikseen|~oiksensa";
    possessive_abe_sg: _, _, _, _, derived;
    possessive_abe_pl: _, _, _, _, "~oittAVn|~oittansa";
    possessive_com_pl: _, _, _, _, "~oineen|~oinensa";
    ),
    // Ahdin: 3 words (kudin, johdin, ahdin). Like Osoitin, except in 8 cells: gen_ pl, ill sg, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, ….
    declension!("ahdin", 3, "~d~n", like Osoitin;
    stem sg: "~d~n- like tunnus";
    gen_: _, "~t~mVen|~d~nten";
    ill: "~t~meen", _;
    possessive_gen_pl: "~t~mVeni|~d~nteni", "~t~mVesi|~d~ntesi", "~t~mVemme|~d~ntemme", "~t~mVenne|~d~ntenne", "~t~mVensA|~d~ntensa";
    possessive_ill_sg: _, _, _, _, "~t~meensA";
    ),
    // Valvojaiset: 3 words (karhunpeijaiset, arpajaiset, valvojaiset). Like Taso with `` after each pattern (`et` after ~), except in 41 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("valvojaiset", 3, "~et", like Taso with "", "et" after "~";
    nom: _, "~et";
    gen_: _, "~ten|~ien";
    par: derived, "~iA";
    ine: derived, _;
    ela: derived, _;
    ill: derived, "~iin";
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~eni", "~esi", "~emme", "~enne", "~ensA";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensA|~iensA";
    possessive_par_sg: _, _, _, _, derived;
    possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iansa";
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Hässäkkä: 3 words (lehdykkä, hässäkkä, räpäkkä). Like Tekijä, except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("hässäkkä", 3, "~kä", like Tekijä;
    stem sg: "~kä- like osa";
    stem pl: "~- like hässäkkä";
    nom: _, "~At";
    ine: "~AssV", _;
    ela: "~AstV", _;
    ill: "~käVn", _;
    ade: "~AllV", _;
    abl: "~AltV", _;
    ess: "~känV", _;
    abe: "~AttV", _;
    possessive_nom_sg: "~käni", _, _, _, "~känsV";
    possessive_nom_pl: _, "~käsi", "~kämme", "~känne", _;
    possessive_gen_sg: _, "~käsi", "~kämme", "~känne", _;
    possessive_ine_sg: _, _, _, _, "~AssVVn|~ässänsä";
    possessive_ela_sg: _, _, _, _, "~AstVVn|~ästänsä";
    possessive_ill_sg: _, _, _, _, "~käVnsV";
    possessive_ade_sg: _, _, _, _, "~AllVVn|~ällänsä";
    possessive_abl_sg: _, _, _, _, "~AltVVn|~ältänsä";
    possessive_all_sg: _, _, _, _, "~Alleen|~ällensä";
    possessive_ess_sg: _, _, _, _, "~känVVn|~känänsä";
    possessive_tra_sg: _, _, _, _, "~Akseen|~äksensä";
    possessive_abe_sg: _, _, _, _, "~AttVVn|~ättänsä";
    ),
];
