//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~nen`, `purkaminen`: the stem and `nen`). A stem in two pieces (`~~`, `tunnus`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A capital (`A`, `Ä`, `V`, `I`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `pato`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem noun_gen_sg: \"~s- like purkaminen\"`: the slot's cells are `purkaminen`'s table's patterns
/// with `~s` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
     $(stem lemma: $s0:tt;)?
     $(stem noun_gen_sg: $s1:tt;)?
     $(stem noun_par_pl: $s2:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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
    /// Like `purkaminen`: 733 words.
    Purkaminen,
    /// Like `taso`: 603 words.
    Taso,
    /// Like `tunnus`: 500 words.
    Tunnus,
    /// Like `pato`: 395 words.
    Pato,
    /// Like `pari`: 331 words.
    Pari,
    /// Like `kaappi`: 326 words.
    Kaappi,
    /// Like `osa`: 322 words.
    Osa,
    /// Like `sarja`: 311 words.
    Sarja,
    /// Like `ankkuri`: 254 words.
    Ankkuri,
    /// Like `oikeus`: 193 words.
    Oikeus,
    /// Like `pata`: 190 words.
    Pata,
    /// Like `erä`: 170 words.
    Erä,
    /// Like `nappula`: 161 words.
    Nappula,
    /// Like `keri`: 121 words.
    Keri,
    /// Like `aste`: 111 words.
    Aste,
    /// Like `konversio`: 107 words.
    Konversio,
    /// Like `suka`: 104 words.
    Suka,
    /// Like `piste`: 56 words.
    Piste,
    /// Like `silmukka`: 52 words.
    Silmukka,
    /// Like `avain`: 46 words.
    Avain,
    /// Like `suhde`: 46 words.
    Suhde,
    /// Like `selkä`: 44 words.
    Selkä,
    /// Like `kuori`: 43 words.
    Kuori,
    /// Like `tekijä`: 31 words.
    Tekijä,
    /// Like `parvi`: 31 words.
    Parvi,
    /// Like `kivi`: 29 words.
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
    /// Like `hmong`: 21 words.
    Hmong,
    /// Like `äänenvaimennin`: 20 words.
    Äänenvaimennin,
    /// Like `kytkin`: 18 words.
    Kytkin,
    /// Like `puu`: 17 words.
    Puu,
    /// Like `etumies`: 17 words.
    Etumies,
    /// Like `luku`: 17 words.
    Luku,
    /// Like `käsi`: 17 words.
    Käsi,
    /// Like `peite`: 16 words.
    Peite,
    /// Like `allas`: 15 words.
    Allas,
    /// Like `istukas`: 15 words.
    Istukas,
    /// Like `lehti`: 14 words.
    Lehti,
    /// Like `aika`: 13 words.
    Aika,
    /// Like `lähde`: 13 words.
    Lähde,
    /// Like `olot`: 13 words.
    Olot,
    /// Like `verkko`: 12 words.
    Verkko,
    /// Like `siemen`: 12 words.
    Siemen,
    /// Like `kaanon`: 12 words.
    Kaanon,
    /// Like `siipi`: 12 words.
    Siipi,
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
    /// Like `sakset`: 9 words.
    Sakset,
    /// Like `paikka`: 9 words.
    Paikka,
    /// Like `kieli`: 8 words.
    Kieli,
    /// Like `jousi`: 8 words.
    Jousi,
    /// Like `hiili`: 8 words.
    Hiili,
    /// Like `kuve`: 8 words.
    Kuve,
    /// Like `kissanristiäiset`: 8 words.
    Kissanristiäiset,
    /// Like `toimi`: 7 words.
    Toimi,
    /// Like `veto`: 6 words.
    Veto,
    /// Like `kupu`: 6 words.
    Kupu,
    /// Like `keihäs`: 6 words.
    Keihäs,
    /// Like `kiertotatar`: 6 words.
    Kiertotatar,
    /// Like `filee`: 6 words.
    Filee,
    /// Like `maamerkki`: 6 words.
    Maamerkki,
    /// Like `värit`: 6 words.
    Värit,
    /// Like `lippu`: 5 words.
    Lippu,
    /// Like `koe`: 5 words.
    Koe,
    /// Like `khmer`: 5 words.
    Khmer,
    /// Like `syy`: 5 words.
    Syy,
    /// Like `askel`: 5 words.
    Askel,
    /// Like `alkeet`: 5 words.
    Alkeet,
    /// Like `lupa`: 4 words.
    Lupa,
    /// Like `tee`: 4 words.
    Tee,
    /// Like `poika`: 4 words.
    Poika,
    /// Like `hopea`: 4 words.
    Hopea,
    /// Like `ydin`: 4 words.
    Ydin,
    /// Like `metsäkauris`: 4 words.
    Metsäkauris,
    /// Like `rättänä`: 4 words.
    Rättänä,
    /// Like `takuu`: 4 words.
    Takuu,
    /// Like `käräjät`: 4 words.
    Käräjät,
    /// Like `onki`: 4 words.
    Onki,
    /// Like `vehje`: 4 words.
    Vehje,
    /// Like `piennar`: 4 words.
    Piennar,
    /// Like `lapa`: 3 words.
    Lapa,
    /// Like `leipä`: 3 words.
    Leipä,
    /// Like `maahockey`: 3 words.
    Maahockey,
    /// Like `hammas`: 3 words.
    Hammas,
    /// Like `komme`: 3 words.
    Komme,
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
    /// Like `hässäkkä`: 3 words.
    Hässäkkä,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 101] = [
        Declension::Purkaminen,
        Declension::Taso,
        Declension::Tunnus,
        Declension::Pato,
        Declension::Pari,
        Declension::Kaappi,
        Declension::Osa,
        Declension::Sarja,
        Declension::Ankkuri,
        Declension::Oikeus,
        Declension::Pata,
        Declension::Erä,
        Declension::Nappula,
        Declension::Keri,
        Declension::Aste,
        Declension::Konversio,
        Declension::Suka,
        Declension::Piste,
        Declension::Silmukka,
        Declension::Avain,
        Declension::Suhde,
        Declension::Selkä,
        Declension::Kuori,
        Declension::Tekijä,
        Declension::Parvi,
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
        Declension::Luku,
        Declension::Käsi,
        Declension::Peite,
        Declension::Allas,
        Declension::Istukas,
        Declension::Lehti,
        Declension::Aika,
        Declension::Lähde,
        Declension::Olot,
        Declension::Verkko,
        Declension::Siemen,
        Declension::Kaanon,
        Declension::Siipi,
        Declension::Pää,
        Declension::Koi,
        Declension::Hattara,
        Declension::Perintä,
        Declension::Väki,
        Declension::Maa,
        Declension::Työ,
        Declension::Kärki,
        Declension::Sakset,
        Declension::Paikka,
        Declension::Kieli,
        Declension::Jousi,
        Declension::Hiili,
        Declension::Kuve,
        Declension::Kissanristiäiset,
        Declension::Toimi,
        Declension::Veto,
        Declension::Kupu,
        Declension::Keihäs,
        Declension::Kiertotatar,
        Declension::Filee,
        Declension::Maamerkki,
        Declension::Värit,
        Declension::Lippu,
        Declension::Koe,
        Declension::Khmer,
        Declension::Syy,
        Declension::Askel,
        Declension::Alkeet,
        Declension::Lupa,
        Declension::Tee,
        Declension::Poika,
        Declension::Hopea,
        Declension::Ydin,
        Declension::Metsäkauris,
        Declension::Rättänä,
        Declension::Takuu,
        Declension::Käräjät,
        Declension::Onki,
        Declension::Vehje,
        Declension::Piennar,
        Declension::Lapa,
        Declension::Leipä,
        Declension::Maahockey,
        Declension::Hammas,
        Declension::Komme,
        Declension::Bokserit,
        Declension::Opas,
        Declension::Lapsi,
        Declension::Markkinat,
        Declension::Ahdin,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 101] = [
    // Purkaminen: 733 words (nainen, suomalainen, valkoinen, punainen, hevonen, lautanen).
    declension!("purkaminen", 733, "~nen";
    stem noun_gen_sg: "~s- like purkaminen";
    stem noun_par_pl: "~s- like purkaminen";
    nom: "~nen", _;
    par: "~stA", _;
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    possessive_par_sg: _, _, _, _, "~stAVn|~stAnsV";
    ),
    // Taso: 603 words (baby, tempo, indigo, sombrero, ajo, dodo). Like Purkaminen, except in 47 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("taso", 603, "~", like Purkaminen;
    stem lemma: "~- like taso";
    stem noun_par_pl: "~- like taso";
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
    possessive_ine_sg: _, _, _, _, "~ssAVn|~ssAnsV";
    possessive_ine_pl: "~issAni", _, _, _, "~issAVn|~issAnsV";
    possessive_ela_sg: _, _, _, _, "~stAVn|~stAnsV";
    possessive_ela_pl: "~istAni", _, _, _, "~istAVn|~istAnsV";
    possessive_ill_sg: _, _, _, _, "~VnsA";
    possessive_ade_sg: _, _, _, _, "~llAVn|~llAnsV";
    possessive_ade_pl: "~illAni", _, _, _, "~illAVn|~illAnsV";
    possessive_abl_sg: _, _, _, _, "~ltAVn|~ltAnsV";
    possessive_abl_pl: "~iltAni", _, _, _, "~iltAVn|~iltAnsV";
    possessive_all_sg: _, _, _, _, "~lleen|~llensA";
    possessive_all_pl: "~illeni", _, _, _, "~illeen|~illensA";
    possessive_ess_sg: _, _, _, _, "~nAVn|~nAnsV";
    possessive_tra_sg: _, _, _, _, "~kseen|~ksensA";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensA";
    possessive_abe_sg: _, _, _, _, "~ttAVn|~ttAnsV";
    possessive_abe_pl: "~ittAni", _, _, _, "~ittAVn|~ittAnsV";
    ),
    // Tunnus: 500 words (genus, ape, atlas, opus, kehys, tytär).
    declension!("tunnus", 500, "~~";
    stem lemma: "~~- like tunnus";
    stem noun_gen_sg: "~k~- like purkaminen";
    gen_: _, "~~ten|~k~ien";
    par: _, "~k~iA";
    ill: _, "~k~iin";
    ess: _, "~k~inA";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    possessive_gen_pl: "~~teni|~k~ieni", "~~tesi|~k~iesi", "~~temme|~k~iemme", "~~tenne|~k~ienne", "~~tensA|~k~iensA";
    possessive_par_pl: "~k~iAni", _, _, _, "~k~iAVn|~k~iAnsV";
    possessive_ill_pl: "~k~iini", _, _, _, "~k~iinsA";
    possessive_ess_pl: "~k~inAni", _, _, _, "~k~inAVn|~k~inAnsV";
    possessive_com_pl: "~k~ineni", _, _, _, "~k~ineen|~k~inensA";
    ),
    // Pato: 395 words (pato, itu, satu, postiennakko, otto, veto). Like Taso with `ˇ` after each pattern.
    declension!("pato", 395, "~ˇ", like Taso with "ˇ";
    ),
    // Pari: 331 words (khaki, kali, wasabi, kaali, tali, uuni). Like Taso, except in 58 cells: nom pl, gen_ pl, par pl, ine sg, ine pl, ela sg, ….
    declension!("pari", 331, "~i", like Taso;
    stem lemma: "~i- like taso";
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
    possessive_par_pl: "~ejAni", _, _, _, "~ejAVn|~ejAnsV";
    possessive_ine_sg: _, _, _, _, "~issAVn|~issAnsV";
    possessive_ine_pl: "~eissAni", _, _, _, "~eissAVn|~eissAnsV";
    possessive_ela_sg: _, _, _, _, "~istAVn|~istAnsV";
    possessive_ela_pl: "~eistAni", _, _, _, "~eistAVn|~eistAnsV";
    possessive_ill_sg: _, _, _, _, "~iVnsA";
    possessive_ill_pl: "~eihVni", _, _, _, "~eihinsA";
    possessive_ade_sg: _, _, _, _, "~illAVn|~illAnsV";
    possessive_ade_pl: "~eillAni", _, _, _, "~eillAVn|~eillAnsV";
    possessive_abl_sg: _, _, _, _, "~iltAVn|~iltAnsV";
    possessive_abl_pl: "~eiltAni", _, _, _, "~eiltAVn|~eiltAnsV";
    possessive_all_sg: _, _, _, _, "~illeen|~illensA";
    possessive_all_pl: "~eilleni", _, _, _, "~eilleen|~eillensA";
    possessive_ess_sg: _, _, _, _, "~inAVn|~inAnsV";
    possessive_ess_pl: "~einAni", _, _, _, "~einAVn|~einAnsV";
    possessive_tra_sg: _, _, _, _, "~ikseen|~iksensA";
    possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensA";
    possessive_abe_sg: _, _, _, _, "~ittAVn|~ittAnsV";
    possessive_abe_pl: "~eittAni", _, _, _, "~eittAVn|~eittAnsV";
    possessive_com_pl: "~eineni", _, _, _, "~eineen|~einensA";
    ),
    // Kaappi: 326 words (kaappi, äiti, sekunti, voltti, täti, turkki). Like Pari with `ˇ` after each pattern.
    declension!("kaappi", 326, "~iˇ", like Pari with "ˇ";
    ),
    // Osa: 322 words (koira, pula, musta, korva, kuula, kura). Like Siemen with `` after each pattern (`a` after ~), except in 33 cells: nom pl, gen_ pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("osa", 322, "~a", like Siemen with "", "a" after "~";
    nom: _, "~at";
    gen_: _, "~ien";
    par: "~aV", _;
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
    possessive_gen_pl: "~ieni", derived, derived, derived, "~iensA";
    possessive_par_sg: _, _, _, _, "~aVnsV";
    possessive_ine_sg: _, _, _, _, "~assVVn|~assVnsV";
    possessive_ela_sg: _, _, _, _, "~astVVn|~astVnsV";
    possessive_ill_sg: _, _, _, _, "~aVnsV";
    possessive_ade_sg: _, _, _, _, "~allVVn|~allVnsV";
    possessive_abl_sg: _, _, _, _, "~altVVn|~altVnsV";
    possessive_all_sg: _, _, _, _, "~alleen|~allensA";
    possessive_ess_sg: _, _, _, _, "~anVVn|~anVnsV";
    possessive_tra_sg: _, _, _, _, "~akseen|~aksensA";
    possessive_abe_sg: _, _, _, _, "~attVVn|~attVnsV";
    ),
    // Sarja: 311 words (lama, salsa, matka, kara, sana, gamma). Like Osa, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("sarja", 311, "~a", like Osa;
    stem noun_gen_sg: "~- like sarja";
    stem noun_par_pl: "~o- like sarja";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Ankkuri: 254 words (ämpäri, banaani, mittari, versaali, paperi, moottori). Like Hmong with `` after each pattern (`i` after ~), except in 18 cells: gen_ pl, par pl, com pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, ….
    declension!("ankkuri", 254, "~i", like Hmong with "", "i" after "~";
    gen_: _, "~ien|~eiden|~eitten";
    par: _, "~eitA|~ejA";
    com: _, derived;
    possessive_gen_pl: "~ieni|~eideni|~eitteni", "~iesi|~eidesi|~eittesi", "~iemme|~eidemme|~eittemme", "~ienne|~eidenne|~eittenne", "~iensA|~eidensA|~eittensA";
    possessive_par_pl: "~eitAni|~ejAni", "~eitAsi|~ejAsi", "~eitAmme|~ejAmme", "~eitAnne|~ejAnne", "~eitAVn|~ejAVn|~eitAnsV|~ejansa";
    possessive_ill_sg: derived, derived, derived, derived, _;
    possessive_ill_pl: "~eihVni", _, _, _, _;
    ),
    // Oikeus: 193 words (nopeus, tilavuus, pituus, hiljaisuus, vinous, avaruus). Like Taso, except in 47 cells: nom sg, nom pl, par sg, ine sg, ine pl, ela sg, ….
    declension!("oikeus", 193, "~s", like Taso;
    stem noun_par_pl: "~ksi- like oikeus";
    nom: "~s", "~det";
    par: "~ttA", _;
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
    possessive_par_sg: _, _, _, _, "~ttAVn|~ttAnsV";
    possessive_ine_sg: _, _, _, _, "~dessAVn|~dessAnsV";
    possessive_ine_pl: "~ksissAni", _, _, _, "~ksissAVn|~ksissAnsV";
    possessive_ela_sg: _, _, _, _, "~destAVn|~destAnsV";
    possessive_ela_pl: "~ksistAni", _, _, _, "~ksistAVn|~ksistAnsV";
    possessive_ill_sg: _, _, _, _, "~teVnsA";
    possessive_ade_sg: _, _, _, _, "~dellAVn|~dellAnsV";
    possessive_ade_pl: "~ksillAni", _, _, _, "~ksillAVn|~ksillAnsV";
    possessive_abl_sg: _, _, _, _, "~deltAVn|~deltAnsV";
    possessive_abl_pl: "~ksiltAni", _, _, _, "~ksiltAVn|~ksiltAnsV";
    possessive_all_sg: _, _, _, _, "~delleen|~dellensA";
    possessive_all_pl: "~ksilleni", _, _, _, "~ksilleen|~ksillensA";
    possessive_ess_sg: _, _, _, _, "~tenAVn|~tenAnsV";
    possessive_tra_sg: _, _, _, _, "~dekseen|~deksensA";
    possessive_tra_pl: _, _, _, _, "~ksikseen|~ksiksensA";
    possessive_abe_sg: _, _, _, _, "~dettAVn|~dettAnsV";
    possessive_abe_pl: "~ksittAni", _, _, _, "~ksittAVn|~ksittAnsV";
    ),
    // Pata: 190 words (pata, rata, kita, kappa, markka, virta). Like Sarja with `ˇ` after each pattern.
    declension!("pata", 190, "~aˇ", like Sarja with "ˇ";
    ),
    // Erä: 170 words (päivä, pitkä, isä, kynä, vihreä, kesä). Like Osa, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("erä", 170, "~ä", like Osa;
    stem lemma: "~ä- like osa";
    stem noun_gen_sg: "~- like erä";
    stem noun_par_pl: "~i- like erä";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Nappula: 161 words (tortilla, harmonia, porkkana, ikkuna, gemena, vasara). Like Sarja.
    declension!("nappula", 161, "~a", like Sarja;
    stem noun_par_pl: "~oi- like nappula";
    ),
    // Keri: 121 words (keksi, peili, väri, seksi, peli, viini). Like Pari, except in 42 cells: par pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("keri", 121, "~i", like Pari;
    stem lemma: "~i- like keri";
    par: _, "~ejÄ";
    ine: "~issä", "~eissÄ";
    ela: "~istä", "~eistÄ";
    ade: "~illä", "~eillÄ";
    abl: "~iltä", "~eiltÄ";
    ess: "~inä", "~einÄ";
    abe: "~ittä", "~eittÄ";
    possessive_nom_sg: _, _, _, _, "~insä";
    possessive_gen_pl: _, _, _, _, "~iensÄ";
    possessive_par_pl: "~ejÄni", _, _, _, "~ejÄVn|~ejÄnsV";
    possessive_ine_sg: _, _, _, _, "~issäVn|~issänsV";
    possessive_ine_pl: "~eissÄni", _, _, _, "~eissÄVn|~eissÄnsV";
    possessive_ela_sg: _, _, _, _, "~istäVn|~istänsV";
    possessive_ela_pl: "~eistÄni", _, _, _, "~eistÄVn|~eistÄnsV";
    possessive_ill_sg: _, _, _, _, "~iVnsä";
    possessive_ill_pl: _, _, _, _, "~eihinsÄ";
    possessive_ade_sg: _, _, _, _, "~illäVn|~illänsV";
    possessive_ade_pl: "~eillÄni", _, _, _, "~eillÄVn|~eillÄnsV";
    possessive_abl_sg: _, _, _, _, "~iltäVn|~iltänsV";
    possessive_abl_pl: "~eiltÄni", _, _, _, "~eiltÄVn|~eiltÄnsV";
    possessive_all_sg: _, _, _, _, "~illeen|~illensÄ";
    possessive_all_pl: _, _, _, _, "~eilleen|~eillensÄ";
    possessive_ess_sg: _, _, _, _, "~inäVn|~inänsV";
    possessive_ess_pl: "~einÄni", _, _, _, "~einÄVn|~einÄnsV";
    possessive_tra_sg: _, _, _, _, "~ikseen|~iksensÄ";
    possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensÄ";
    possessive_abe_sg: _, _, _, _, "~ittäVn|~ittänsV";
    possessive_abe_pl: "~eittÄni", _, _, _, "~eittÄVn|~eittÄnsV";
    possessive_com_pl: _, _, _, _, "~eineen|~einensÄ";
    ),
    // Aste: 111 words (kale, tietokone, purse, sane, huone, aave). Like Taso, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("aste", 111, "~", like Taso;
    stem lemma: "~- like aste";
    stem noun_gen_sg: "~- like aste";
    stem noun_par_pl: "~i- like aste";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Konversio: 107 words (radio, valtio, tau, video, olio, sikiö). Like Taso, except in 7 cells: possessive_ine_pl _3, possessive_ela_pl _3, possessive_ade_pl _3, possessive_abl_pl _3, possessive_all_pl _3, possessive_tra_pl _3, ….
    declension!("konversio", 107, "~", like Taso;
    stem lemma: "~- like tunnus";
    stem noun_par_pl: "~- like konversio";
    possessive_ine_pl: _, _, _, _, "~issAVn|~IssAnsV";
    possessive_ela_pl: _, _, _, _, "~istAVn|~IstAnsV";
    possessive_ade_pl: _, _, _, _, "~illAVn|~IllAnsV";
    possessive_abl_pl: _, _, _, _, "~iltAVn|~IltAnsV";
    possessive_all_pl: _, _, _, _, "~illeen|~IllensA";
    possessive_tra_pl: _, _, _, _, "~ikseen|~IksensA";
    possessive_abe_pl: _, _, _, _, "~ittAVn|~IttAnsV";
    ),
    // Suka: 104 words (suka, lupa, toukka, kulta, kukka, sukka). Like Maa with `ˇ` after each pattern, except in 16 cells: gen_ pl, par sg, par pl, ill sg, ill pl, possessive_gen_pl _1sg, ….
    declension!("suka", 104, "~aˇ", like Maa with "ˇ";
    gen_: _, "~ienˇ";
    par: "~aVˇ", "~iAˇ";
    ill: "~aVnˇ", "~iinˇ";
    possessive_gen_pl: "~ieniˇ", derived, derived, derived, "~iensAˇ";
    possessive_par_sg: _, _, _, _, "~aVnsVˇ";
    possessive_par_pl: "~iAniˇ", _, _, _, "~iAVnˇ|~iAnsVˇ";
    possessive_ill_sg: _, _, _, _, "~aVnsVˇ";
    possessive_ill_pl: "~iiniˇ", _, _, _, "~iinsAˇ";
    ),
    // Piste: 56 words (sine, piste, herne, hie, este, neste). Like Aste, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("piste", 56, "~", like Aste;
    stem lemma: "~- like piste";
    stem noun_gen_sg: "~- like piste";
    stem noun_par_pl: "~i- like piste";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Silmukka: 52 words (lusikka, haarukka, harakka, mustikka, puolukka, tupakka). Like Sarja, except in 12 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _3, possessive_nom_pl _2sg, possessive_nom_pl _1pl, ….
    declension!("silmukka", 52, "~ka", like Sarja;
    stem lemma: "~ka- like osa";
    stem noun_par_pl: "~- like silmukka";
    ill: "~kaVn", _;
    ess: "~kanV", _;
    possessive_nom_sg: "~kani", _, _, _, "~kansV";
    possessive_nom_pl: _, "~kasi", "~kamme", "~kanne", _;
    possessive_gen_sg: _, "~kasi", "~kamme", "~kanne", _;
    possessive_ill_sg: _, _, _, _, "~kaVnsV";
    possessive_ess_sg: _, _, _, _, "~kanVVn|~kanVnsV";
    ),
    // Avain: 46 words (puhelin, kasvain, valaisin, morsian, avain, vahvistin). Like Purkaminen, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("avain", 46, "~n", like Purkaminen;
    stem lemma: "~n- like tunnus";
    stem noun_gen_sg: "~m- like avain";
    stem noun_par_pl: "~- like avain";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Suhde: 46 words (sade, aarre, anne, luode, vanne, pinne). Like Aste with `ˇ` after each pattern.
    declension!("suhde", 46, "~ˇ", like Aste with "ˇ";
    ),
    // Selkä: 44 words (leipä, kenkä, setä, ikä, itä, kenttä). Like Erä with `ˇ` after each pattern.
    declension!("selkä", 44, "~äˇ", like Erä with "ˇ";
    ),
    // Kuori: 43 words (tuuli, nuori, vuori, kieli, ääni, juuri). Like Siemen with `` after each pattern (`i` after ~), except in 36 cells: nom pl, gen_ pl, ine sg, ela sg, ill sg, ade sg, ….
    declension!("kuori", 43, "~i", like Siemen with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ten|~ien";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~eVn", _;
    ade: "~ellA", _;
    abl: "~eltA", _;
    ess: "~enA", _;
    abe: "~ettA", _;
    ins: "~en", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensA";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensA|~iensA";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    possessive_ill_sg: _, _, _, _, "~eVnsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enAnsV";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    possessive_ins_sg: _, "~esi", "~emme", "~enne", "~ensa";
    ),
    // Tekijä: 31 words (päärynä, ympyrä, pähkinä, tekijä, kylpylä, näsiä). Like Erä, except in 19 cells: ine pl, ela pl, ade pl, abl pl, all pl, abe pl, ….
    declension!("tekijä", 31, "~ä", like Erä;
    stem noun_par_pl: "~öi- like nappula";
    ine: _, "~öissA";
    ela: _, "~öistA";
    ade: _, "~öillA";
    abl: _, "~öiltA";
    all: _, "~öille";
    abe: _, "~öittA";
    possessive_ine_pl: "~öissAni", _, _, _, "~öissAVn|~öissAnsV";
    possessive_ela_pl: "~öistAni", _, _, _, "~öistAVn|~öistAnsV";
    possessive_ade_pl: "~öillAni", _, _, _, "~öillAVn|~öillAnsV";
    possessive_abl_pl: "~öiltAni", _, _, _, "~öiltAVn|~öiltAnsV";
    possessive_all_pl: "~öilleni", _, _, _, "~öilleen|~öillensA";
    possessive_tra_pl: _, _, _, _, "~öikseen|~öiksensA";
    possessive_abe_pl: "~öittAni", _, _, _, "~öittAVn|~öittAnsV";
    ),
    // Parvi: 31 words (kuusi, takatalvi, suvi, sormi, povi, tammi). Like Siemen with `` after each pattern (`i` after ~), except in 33 cells: nom pl, gen_ pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("parvi", 31, "~i", like Siemen with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ien";
    par: "~eA", _;
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
    possessive_gen_pl: "~ieni", derived, derived, derived, "~iensA";
    possessive_par_sg: _, _, _, _, "~eAVn|~eAnsV";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    possessive_ill_sg: _, _, _, _, "~eVnsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enAnsV";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    ),
    // Kivi: 29 words (pilvi, hirvi, kivi, liemi, tyvi, helmi). Like Erä, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("kivi", 29, "~i", like Erä;
    stem lemma: "~- like kivi";
    stem noun_gen_sg: "~- like kivi";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
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
    possessive_par_sg: _, _, _, _, "~stVVn|~stVnsV";
    possessive_ine_sg: _, _, _, _, "~VssVVn|~VssVnsV";
    possessive_ela_sg: _, _, _, _, "~VstVVn|~VstVnsV";
    possessive_ill_sg: _, _, _, _, "~VseVnsA";
    possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    possessive_ade_sg: _, _, _, _, "~VllVVn|~VllVnsV";
    possessive_abl_sg: _, _, _, _, "~VltVVn|~VltVnsV";
    possessive_all_sg: _, _, _, _, "~Vlleen|~VllensA";
    possessive_ess_sg: _, _, _, _, "~VnVVn|~VnVnsV";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~VksensA";
    possessive_abe_sg: _, _, _, _, "~VttVVn|~VttVnsV";
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
    possessive_par_pl: "~siAni", _, _, _, "~siAVn|~siAnsV";
    possessive_ine_pl: "~sissAni", _, _, _, "~sissAVn|~sissAnsV";
    possessive_ela_pl: "~sistAni", _, _, _, "~sistAVn|~sistAnsV";
    possessive_ill_pl: "~siini", _, _, _, "~siinsA";
    possessive_ade_pl: "~sillAni", _, _, _, "~sillAVn|~sillAnsV";
    possessive_abl_pl: "~siltAni", _, _, _, "~siltAVn|~siltAnsV";
    possessive_all_pl: "~silleni", _, _, _, "~silleen|~sillensA";
    possessive_ess_pl: "~sinAni", _, _, _, "~sinAVn|~sinAnsV";
    possessive_tra_pl: _, _, _, _, "~sikseen|~siksensA";
    possessive_abe_pl: "~sittAni", _, _, _, "~sittAVn|~sittAnsV";
    possessive_com_pl: "~sineni", _, _, _, "~sineen|~sinensA";
    ),
    // Osoitin: 25 words (ääretön, nimetön, tuntematon, viivoitin, osoitin, soitin). Like Tunnus, except in 21 cells: gen_ pl, par pl, ill pl, ess pl, possessive_nom_pl _1sg, possessive_nom_pl _3, ….
    declension!("osoitin", 25, "~~n", like Tunnus;
    stem lemma: "~~n- like tunnus";
    stem noun_gen_sg: "~t~m- like avain";
    gen_: _, "~t~mVen|~~nten";
    par: _, "~t~mVA";
    ill: _, "~t~miin";
    ess: _, "~t~minA";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    possessive_gen_pl: "~t~mVeni|~~nteni", "~t~mVesi|~~ntesi", "~t~mVemme|~~ntemme", "~t~mVenne|~~ntenne", "~t~mVensA|~~ntensA";
    possessive_par_pl: "~t~mVAni", _, _, _, "~t~mVAVn|~t~mVAnsV";
    possessive_ill_pl: "~t~miini", _, _, _, "~t~miinsA";
    possessive_ess_pl: "~t~minAni", _, _, _, "~t~minAVn|~t~minAnsV";
    possessive_com_pl: "~t~mineni", _, _, _, "~t~mineen|~t~minensA";
    ),
    // Kate: 23 words (ote, kate, aate, vaate, uute, toimintaperiaate). Like Tunnus, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("kate", 23, "~~", like Tunnus;
    stem lemma: "~~- like aste";
    stem noun_gen_sg: "~t~- like kate";
    stem noun_par_pl: "~t~i- like kate";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Liike: 21 words (rike, säie, pyyhe, liike, ärsyke, levyke). Like Tunnus, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("liike", 21, "~~", like Tunnus;
    stem lemma: "~~- like piste";
    stem noun_gen_sg: "~k~- like liike";
    stem noun_par_pl: "~k~i- like liike";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Hmong: 21 words (swap, avatar, output, wok, trap, atrium). Like Pari, except in 7 cells: nom sg, com pl, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, ….
    declension!("hmong", 21, "~", like Pari;
    nom: "~", _;
    com: _, "~peineen";
    possessive_ill_sg: "~iVni", "~iVsi", "~iVmme", "~iVnne", _;
    possessive_ill_pl: "~eihini", _, _, _, _;
    ),
    // Äänenvaimennin: 20 words (äänenvaimennin, kiihdytin, jäähdytin, puhallin, linnunpelätin, heitin). Like Avain with `ˇ` after each pattern.
    declension!("äänenvaimennin", 20, "~nˇ", like Avain with "ˇ";
    ),
    // Kytkin: 18 words (sydän, elin, eläin, kavioeläin, pistin, koiraeläin). Like Avain.
    declension!("kytkin", 18, "~n", like Avain;
    stem noun_par_pl: "~- like kytkin";
    ),
    // Puu: 17 words (luu, kuu, suu, puu, puolikuu, havupuu). Like Aste with `` after each pattern (`u` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("puu", 17, "~u", like Aste with "", "u" after "~";
    nom: _, "~ut";
    par: "~utA", _;
    ine: "~ussA", _;
    ela: "~ustA", _;
    ill: "~uhVn", "~ihin";
    ade: "~ullA", _;
    abl: "~ultA", _;
    ess: "~unA", _;
    abe: "~uttA", _;
    possessive_nom_sg: "~uni", _, _, _, "~unsA";
    possessive_nom_pl: _, "~usi", "~umme", "~unne", _;
    possessive_gen_sg: _, "~usi", "~umme", "~unne", _;
    possessive_par_sg: _, _, _, _, "~utAVn|~utAnsV";
    possessive_ine_sg: _, _, _, _, "~ussAVn|~ussAnsV";
    possessive_ela_sg: _, _, _, _, "~ustAVn|~ustAnsV";
    possessive_ill_sg: _, _, _, _, "~uhVnsA";
    possessive_ill_pl: "~ihini", derived, derived, derived, "~ihinsA";
    possessive_ade_sg: _, _, _, _, "~ullAVn|~ullAnsV";
    possessive_abl_sg: _, _, _, _, "~ultAVn|~ultAnsV";
    possessive_all_sg: _, _, _, _, "~ulleen|~ullensA";
    possessive_ess_sg: _, _, _, _, "~unAVn|~unAnsV";
    possessive_tra_sg: _, _, _, _, "~ukseen|~uksensA";
    possessive_abe_sg: _, _, _, _, "~uttAVn|~uttAnsV";
    ),
    // Etumies: 17 words (mies, luottamusmies, jousimies, sotamies, pursimies, perämies). Like Oikeus, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("etumies", 17, "~s", like Oikeus;
    stem lemma: "~s- like etumies";
    stem noun_gen_sg: "~h- like etumies";
    stem noun_par_pl: "~- like etumies";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Luku: 17 words (suku, puku, luku, alkuluku, kyky, oikoluku). Like Tunnus, except in 47 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("luku", 17, "~k~", like Tunnus;
    stem lemma: "~k~- like taso";
    stem noun_par_pl: "~k~- like taso";
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
    possessive_ine_sg: _, _, _, _, "~v~ssAVn|~v~ssAnsV";
    possessive_ine_pl: "~v~issAni", _, _, _, "~v~issAVn|~v~issAnsV";
    possessive_ela_sg: _, _, _, _, "~v~stAVn|~v~stAnsV";
    possessive_ela_pl: "~v~istAni", _, _, _, "~v~istAVn|~v~istAnsV";
    possessive_ill_sg: _, _, _, _, "~k~VnsA";
    possessive_ade_sg: _, _, _, _, "~v~llAVn|~v~llAnsV";
    possessive_ade_pl: "~v~illAni", _, _, _, "~v~illAVn|~v~illAnsV";
    possessive_abl_sg: _, _, _, _, "~v~ltAVn|~v~ltAnsV";
    possessive_abl_pl: "~v~iltAni", _, _, _, "~v~iltAVn|~v~iltAnsV";
    possessive_all_sg: _, _, _, _, "~v~lleen|~v~llensA";
    possessive_all_pl: "~v~illeni", _, _, _, "~v~illeen|~v~illensA";
    possessive_ess_sg: _, _, _, _, "~k~nAVn|~k~nAnsV";
    possessive_tra_sg: _, _, _, _, "~v~kseen|~v~ksensA";
    possessive_tra_pl: _, _, _, _, "~v~ikseen|~v~iksensA";
    possessive_abe_sg: _, _, _, _, "~v~ttAVn|~v~ttAnsV";
    possessive_abe_pl: "~v~ittAni", _, _, _, "~v~ittAVn|~v~ittAnsV";
    ),
    // Käsi: 17 words (vesi, käsi, mesi, hiisi, hirsi, reisi). Like Kausi, except in 35 cells: par sg, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("käsi", 17, "~si", like Kausi;
    stem noun_par_pl: "~si- like erä";
    par: "~ttä", _;
    ine: "~dessÄ", "~sissä";
    ela: "~destÄ", "~sistä";
    ade: "~dellÄ", "~sillä";
    abl: "~deltÄ", "~siltä";
    ess: "~tenÄ", _;
    abe: "~dettÄ", "~sittä";
    possessive_nom_sg: _, _, _, _, "~tensÄ";
    possessive_par_sg: _, _, _, _, "~ttäVn|~ttänsV";
    possessive_ine_sg: _, _, _, _, "~dessÄVn|~dessÄnsV";
    possessive_ine_pl: "~sissäni", _, _, _, "~sissäVn|~sissänsV";
    possessive_ela_sg: _, _, _, _, "~destÄVn|~destÄnsV";
    possessive_ela_pl: "~sistäni", _, _, _, "~sistäVn|~sistänsV";
    possessive_ill_sg: _, _, _, _, "~teVnsÄ";
    possessive_ade_sg: _, _, _, _, "~dellÄVn|~dellÄnsV";
    possessive_ade_pl: "~silläni", _, _, _, "~silläVn|~sillänsV";
    possessive_abl_sg: _, _, _, _, "~deltÄVn|~deltÄnsV";
    possessive_abl_pl: "~siltäni", _, _, _, "~siltäVn|~siltänsV";
    possessive_all_sg: _, _, _, _, "~delleen|~dellensÄ";
    possessive_all_pl: _, _, _, _, "~silleen|~sillensÄ";
    possessive_ess_sg: _, _, _, _, "~tenÄVn|~tenÄnsV";
    possessive_tra_sg: _, _, _, _, "~dekseen|~deksensÄ";
    possessive_tra_pl: _, _, _, _, "~sikseen|~siksensÄ";
    possessive_abe_sg: _, _, _, _, "~dettÄVn|~dettÄnsV";
    possessive_abe_pl: "~sittäni", _, _, _, "~sittäVn|~sittänsV";
    ),
    // Peite: 16 words (jännite, hete, heikkojännite, viite, liite, väite). Like Kate, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("peite", 16, "~~", like Kate;
    stem lemma: "~~- like piste";
    stem noun_gen_sg: "~t~- like liike";
    stem noun_par_pl: "~t~i- like liike";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Allas: 15 words (kuningas, allas, parras, kangas, rengas, marras). Like Keihäs with `ˇ` after each pattern, except in 10 cells: par sg, possessive_par_sg _3, possessive_ine_sg _3, possessive_ela_sg _3, possessive_ade_sg _3, possessive_abl_sg _3, ….
    declension!("allas", 15, "~sˇ", like Keihäs with "ˇ";
    par: "~stVˇ", _;
    possessive_par_sg: _, _, _, _, "~stVVnˇ|~stVnsVˇ";
    possessive_ine_sg: _, _, _, _, "~VssVVnˇ|~VssVnsVˇ";
    possessive_ela_sg: _, _, _, _, "~VstVVnˇ|~VstVnsVˇ";
    possessive_ade_sg: _, _, _, _, "~VllVVnˇ|~VllVnsVˇ";
    possessive_abl_sg: _, _, _, _, "~VltVVnˇ|~VltVnsVˇ";
    possessive_all_sg: _, _, _, _, "~Vlleenˇ|~VllensAˇ";
    possessive_ess_sg: _, _, _, _, "~VnVVnˇ|~VnVnsVˇ";
    possessive_tra_sg: _, _, _, _, "~Vkseenˇ|~VksensAˇ";
    possessive_abe_sg: _, _, _, _, "~VttVVnˇ|~VttVnsVˇ";
    ),
    // Istukas: 15 words (ies, ruis, asiakas, osterivinokas, ehdokas, sokerijuurikas). Like Tunnus, except in 32 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("istukas", 15, "~~s", like Tunnus;
    stem lemma: "~~s- like tunnus";
    stem noun_par_pl: "~k~i- like ruis";
    nom: _, "~k~Vt";
    ine: "~k~VssV", _;
    ela: "~k~VstV", _;
    ill: "~k~Vseen", _;
    ade: "~k~VllV", _;
    abl: "~k~VltV", _;
    ess: "~k~VnV", _;
    abe: "~k~VttV", _;
    possessive_nom_sg: "~k~Vni", _, _, _, "~k~VnsV";
    possessive_nom_pl: _, "~k~Vsi", "~k~Vmme", "~k~Vnne", _;
    possessive_gen_sg: _, "~k~Vsi", "~k~Vmme", "~k~Vnne", _;
    possessive_ine_sg: _, _, _, _, "~k~VssVVn|~k~assansa";
    possessive_ine_pl: _, _, _, _, "~k~issAVn|~k~issansa";
    possessive_ela_sg: _, _, _, _, "~k~VstVVn|~k~astansa";
    possessive_ela_pl: _, _, _, _, "~k~istAVn|~k~istansa";
    possessive_ill_sg: _, _, _, _, "~k~VseensA";
    possessive_ade_sg: _, _, _, _, "~k~VllVVn|~k~allansa";
    possessive_ade_pl: _, _, _, _, "~k~illAVn|~k~illansa";
    possessive_abl_sg: _, _, _, _, "~k~VltVVn|~k~altansa";
    possessive_abl_pl: _, _, _, _, "~k~iltAVn|~k~iltansa";
    possessive_all_sg: _, _, _, _, "~k~Vlleen|~k~allensa";
    possessive_all_pl: _, _, _, _, "~k~illeen|~k~illensa";
    possessive_ess_sg: _, _, _, _, "~k~VnVVn|~k~anansa";
    possessive_tra_sg: _, _, _, _, "~k~Vkseen|~k~aksensa";
    possessive_tra_pl: _, _, _, _, "~k~ikseen|~k~iksensa";
    possessive_abe_sg: _, _, _, _, "~k~VttVVn|~k~attansa";
    possessive_abe_pl: _, _, _, _, "~k~ittAVn|~k~ittansa";
    ),
    // Lehti: 14 words (tähti, aamutähti, käärmetähti, lehti, vyyhti, iltatähti). Like Käsi, except in 19 cells: ine pl, ela pl, ade pl, abl pl, all pl, abe pl, ….
    declension!("lehti", 14, "~ti", like Käsi;
    stem lemma: "~t- like kivi";
    stem noun_par_pl: "~ti- like lehti";
    ine: _, "~dissÄ";
    ela: _, "~distÄ";
    ade: _, "~dillÄ";
    abl: _, "~diltÄ";
    all: _, "~dille";
    abe: _, "~dittÄ";
    possessive_ine_pl: "~dissÄni", _, _, _, "~dissÄVn|~dissÄnsV";
    possessive_ela_pl: "~distÄni", _, _, _, "~distÄVn|~distÄnsV";
    possessive_ade_pl: "~dillÄni", _, _, _, "~dillÄVn|~dillÄnsV";
    possessive_abl_pl: "~diltÄni", _, _, _, "~diltÄVn|~diltÄnsV";
    possessive_all_pl: "~dilleni", _, _, _, "~dilleen|~dillensÄ";
    possessive_tra_pl: _, _, _, _, "~dikseen|~diksensÄ";
    possessive_abe_pl: "~dittÄni", _, _, _, "~dittÄVn|~dittÄnsV";
    ),
    // Aika: 13 words (aika, vuodenaika, kesäaika, talviaika, keskiaika, kiertoaika). Like Taso, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("aika", 13, "~ika", like Taso;
    stem lemma: "~ika- like aika";
    stem noun_par_pl: "~iko- like aika";
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
    possessive_ine_sg: _, _, _, _, "~jVssVVn|~jVssVnsV";
    possessive_ine_pl: "~joissani", _, _, _, "~joissaan|~joissansa";
    possessive_ela_sg: _, _, _, _, "~jVstVVn|~jVstVnsV";
    possessive_ela_pl: "~joistani", _, _, _, "~joistaan|~joistansa";
    possessive_ill_sg: _, _, _, _, "~ikaansa";
    possessive_ade_sg: _, _, _, _, "~jVllVVn|~jVllVnsV";
    possessive_ade_pl: "~joillani", _, _, _, "~joillaan|~joillansa";
    possessive_abl_sg: _, _, _, _, "~jVltVVn|~jVltVnsV";
    possessive_abl_pl: "~joiltani", _, _, _, "~joiltaan|~joiltansa";
    possessive_all_sg: _, _, _, _, "~jVlleen|~jVllensA";
    possessive_all_pl: "~joilleni", _, _, _, "~joilleen|~joillensa";
    possessive_ess_sg: _, _, _, _, "~ikanaan|~ikanansa";
    possessive_tra_sg: _, _, _, _, "~jVkseen|~jVksensA";
    possessive_tra_pl: _, _, _, _, "~joikseen|~joiksensa";
    possessive_abe_sg: _, _, _, _, "~jVttVVn|~jVttVnsV";
    possessive_abe_pl: "~joittani", _, _, _, "~joittaan|~joittansa";
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
    // Verkko: 12 words (nukke, myrkky, teko, näky, hylky, lieko). Like Luku, except in 32 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("verkko", 12, "~k~", like Luku;
    nom: _, "~~t";
    ine: "~~ssA", "~~issA";
    ela: "~~stA", "~~istA";
    ade: "~~llA", "~~illA";
    abl: "~~ltA", "~~iltA";
    all: _, "~~ille";
    abe: "~~ttA", "~~ittA";
    possessive_ine_sg: _, _, _, _, "~~ssAVn|~~ssAnsV";
    possessive_ine_pl: "~~issAni", _, _, _, "~~issAVn|~~issAnsV";
    possessive_ela_sg: _, _, _, _, "~~stAVn|~~stAnsV";
    possessive_ela_pl: "~~istAni", _, _, _, "~~istAVn|~~istAnsV";
    possessive_ade_sg: _, _, _, _, "~~llAVn|~~llAnsV";
    possessive_ade_pl: "~~illAni", _, _, _, "~~illAVn|~~illAnsV";
    possessive_abl_sg: _, _, _, _, "~~ltAVn|~~ltAnsV";
    possessive_abl_pl: "~~iltAni", _, _, _, "~~iltAVn|~~iltAnsV";
    possessive_all_sg: _, _, _, _, "~~lleen|~~llensA";
    possessive_all_pl: "~~illeni", _, _, _, "~~illeen|~~illensA";
    possessive_tra_sg: _, _, _, _, "~~kseen|~~ksensA";
    possessive_tra_pl: _, _, _, _, "~~ikseen|~~iksensA";
    possessive_abe_sg: _, _, _, _, "~~ttAVn|~~ttAnsV";
    possessive_abe_pl: "~~ittAni", _, _, _, "~~ittAVn|~~ittAnsV";
    ),
    // Siemen: 12 words (siemen, ahven, nivel, paimen, jäsen, höyhen). Like Aste, except in 5 cells: ill sg, possessive_ill_sg _3, possessive_ill_pl _2sg, possessive_ill_pl _1pl, possessive_ill_pl _2pl.
    declension!("siemen", 12, "~", like Aste;
    stem lemma: "~- like tunnus";
    stem noun_par_pl: "~- like siemen";
    ill: "~VVn", _;
    possessive_ill_sg: _, _, _, _, "~VVnsA";
    possessive_ill_pl: _, derived, derived, derived, _;
    ),
    // Kaanon: 12 words (diesel, magnum, tandem, denier, afar, röntgen). Like Ankkuri, except in 2 cells: nom sg, possessive_ill_pl _1sg.
    declension!("kaanon", 12, "~", like Ankkuri;
    nom: "~", _;
    possessive_ill_pl: "~eihini", _, _, _, _;
    ),
    // Siipi: 12 words (sampi, siipi, korpi, kilpi, rupi, läpi). Like Taso, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("siipi", 12, "~pi", like Taso;
    stem lemma: "~p- like parvi";
    stem noun_par_pl: "~pi- like osa";
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
    possessive_ine_sg: _, _, _, _, "~vessAVn|~vessAnsV";
    possessive_ine_pl: "~vissAni", _, _, _, "~vissAVn|~vissAnsV";
    possessive_ela_sg: _, _, _, _, "~vestAVn|~vestAnsV";
    possessive_ela_pl: "~vistAni", _, _, _, "~vistAVn|~vistAnsV";
    possessive_ill_sg: _, _, _, _, "~peVnsA";
    possessive_ade_sg: _, _, _, _, "~vellAVn|~vellAnsV";
    possessive_ade_pl: "~villAni", _, _, _, "~villAVn|~villAnsV";
    possessive_abl_sg: _, _, _, _, "~veltAVn|~veltAnsV";
    possessive_abl_pl: "~viltAni", _, _, _, "~viltAVn|~viltAnsV";
    possessive_all_sg: _, _, _, _, "~velleen|~vellensA";
    possessive_all_pl: "~villeni", _, _, _, "~villeen|~villensA";
    possessive_ess_sg: _, _, _, _, "~penAVn|~penAnsV";
    possessive_tra_sg: _, _, _, _, "~vekseen|~veksensA";
    possessive_tra_pl: _, _, _, _, "~vikseen|~viksensA";
    possessive_abe_sg: _, _, _, _, "~vettAVn|~vettAnsV";
    possessive_abe_pl: "~vittAni", _, _, _, "~vittAVn|~vittAnsV";
    ),
    // Pää: 11 words (jää, pää, kyynärpää, luupää, akilleenkantapää, peräpää). Like Aste with `` after each pattern (`ä` after ~), except in 35 cells: nom pl, gen_ sg, par sg, ine sg, ela sg, ill sg, ….
    declension!("pää", 11, "~ä", like Aste with "", "ä" after "~";
    nom: _, "~ät";
    gen_: "~än", _;
    par: "~ätV", _;
    ine: "~ässV", _;
    ela: "~ästV", _;
    ill: "~ähVn", "~ihin";
    ade: "~ällV", _;
    abl: "~ältV", _;
    all: "~älle", _;
    ess: "~änV", _;
    abe: "~ättV", _;
    possessive_nom_sg: "~äni", _, _, _, "~änsV";
    possessive_nom_pl: _, "~äsi", "~ämme", "~änne", _;
    possessive_gen_sg: _, "~äsi", "~ämme", "~änne", _;
    possessive_par_sg: _, _, _, _, "~ätVVn|~ätVnsV";
    possessive_ine_sg: _, _, _, _, "~ässVVn|~ässVnsV";
    possessive_ela_sg: _, _, _, _, "~ästVVn|~ästVnsV";
    possessive_ill_sg: _, _, _, _, "~ähVnsV";
    possessive_ill_pl: "~ihini", derived, derived, derived, "~ihinsA";
    possessive_ade_sg: _, _, _, _, "~ällVVn|~ällVnsV";
    possessive_abl_sg: _, _, _, _, "~ältVVn|~ältVnsV";
    possessive_all_sg: _, _, _, _, "~älleen|~ällensA";
    possessive_ess_sg: _, _, _, _, "~änVVn|~änVnsV";
    possessive_tra_sg: _, _, _, _, "~äkseen|~äksensA";
    possessive_abe_sg: _, _, _, _, "~ättVVn|~ättVnsV";
    ),
    // Koi: 11 words (tie, thai, pii, vuo, koi, täi). Like Konversio, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("koi", 11, "~", like Konversio;
    stem noun_gen_sg: "~- like koi";
    stem noun_par_pl: "~- like puu";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Hattara: 10 words (granaattiomena, maruna, ohrana, harava, korona, hattara). Like Osa, except in 43 cells: ine pl, ela pl, ade pl, abl pl, all pl, tra pl, ….
    declension!("hattara", 10, "~a", like Osa;
    stem noun_par_pl: "~- like hattara";
    ine: _, "~oissA|~issA";
    ela: _, "~oistA|~istA";
    ade: _, "~oillA|~illA";
    abl: _, "~oiltA|~iltA";
    all: _, "~oille|~ille";
    tra: _, "~oiksi|~iksi";
    abe: _, "~oittA|~ittA";
    ins: _, "~oin|~in";
    possessive_ine_pl: "~oissAni|~issAni", "~oissAsi|~issAsi", "~oissAmme|~issAmme", "~oissAnne|~issAnne", "~oissAVn|~issAVn|~oissAnsV|~issansa";
    possessive_ela_pl: "~oistAni|~istAni", "~oistAsi|~istAsi", "~oistAmme|~istAmme", "~oistAnne|~istAnne", "~oistAVn|~istAVn|~oistAnsV|~istansa";
    possessive_ade_pl: "~oillAni|~illAni", "~oillAsi|~illAsi", "~oillAmme|~illAmme", "~oillAnne|~illAnne", "~oillAVn|~illAVn|~oillAnsV|~illansa";
    possessive_abl_pl: "~oiltAni|~iltAni", "~oiltAsi|~iltAsi", "~oiltAmme|~iltAmme", "~oiltAnne|~iltAnne", "~oiltAVn|~iltAVn|~oiltAnsV|~iltansa";
    possessive_all_pl: "~oilleni|~illeni", "~oillesi|~illesi", "~oillemme|~illemme", "~oillenne|~illenne", "~oilleen|~illeen|~oillensA|~illensa";
    possessive_tra_pl: "~oikseni|~ikseni", "~oiksesi|~iksesi", "~oiksemme|~iksemme", "~oiksenne|~iksenne", "~oikseen|~ikseen|~oiksensA|~iksensa";
    possessive_abe_pl: "~oittAni|~ittAni", "~oittAsi|~ittAsi", "~oittAmme|~ittAmme", "~oittAnne|~ittAnne", "~oittAVn|~ittAVn|~oittAnsV|~ittansa";
    ),
    // Perintä: 10 words (yhdyntä, kysyntä, itsehillintä, hyväksyntä, häirintä, liitäntä). Like Taso, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("perintä", 10, "~tä", like Taso;
    stem lemma: "~tä- like osa";
    stem noun_par_pl: "~tö- like taso";
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
    possessive_ine_sg: _, _, _, _, "~nAssVVn|~nAssVnsV";
    possessive_ine_pl: "~nöissAni", _, _, _, "~nöissAVn|~nöissAnsV";
    possessive_ela_sg: _, _, _, _, "~nAstVVn|~nAstVnsV";
    possessive_ela_pl: "~nöistAni", _, _, _, "~nöistAVn|~nöistAnsV";
    possessive_ill_sg: _, _, _, _, "~täVnsV";
    possessive_ade_sg: _, _, _, _, "~nAllVVn|~nAllVnsV";
    possessive_ade_pl: "~nöillAni", _, _, _, "~nöillAVn|~nöillAnsV";
    possessive_abl_sg: _, _, _, _, "~nAltVVn|~nAltVnsV";
    possessive_abl_pl: "~nöiltAni", _, _, _, "~nöiltAVn|~nöiltAnsV";
    possessive_all_sg: _, _, _, _, "~nAlleen|~nAllensA";
    possessive_all_pl: "~nöilleni", _, _, _, "~nöilleen|~nöillensA";
    possessive_ess_sg: _, _, _, _, "~tänVVn|~tänVnsV";
    possessive_tra_sg: _, _, _, _, "~nAkseen|~nAksensA";
    possessive_tra_pl: _, _, _, _, "~nöikseen|~nöiksensA";
    possessive_abe_sg: _, _, _, _, "~nAttVVn|~nAttVnsV";
    possessive_abe_pl: "~nöittAni", _, _, _, "~nöittAVn|~nöittAnsV";
    ),
    // Väki: 10 words (hauki, noki, mäki, käki, väki, jalkaväki). Like Taso with `` after each pattern (`ki` after ~), except in 41 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("väki", 10, "~ki", like Taso with "", "ki" after "~";
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
    possessive_par_sg: _, _, _, _, "~keAVn|~keAnsV";
    possessive_par_pl: "~kiAni", _, _, _, "~kiAVn|~kiAnsV";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    possessive_ill_sg: _, _, _, _, "~keVnsA";
    possessive_ill_pl: "~kiini", _, _, _, "~kiinsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    possessive_ess_sg: _, _, _, _, "~kenAVn|~kenAnsV";
    possessive_ess_pl: "~kinAni", _, _, _, "~kinAVn|~kinAnsV";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    possessive_com_pl: "~kineni", _, _, _, "~kineen|~kinensA";
    ),
    // Maa: 9 words (harmaa, maa, autiomaa, erämaa, isänmaa, takamaa). Like Aste with `` after each pattern (`a` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("maa", 9, "~a", like Aste with "", "a" after "~";
    nom: _, "~at";
    par: "~atV", _;
    ine: "~assV", _;
    ela: "~astV", _;
    ill: "~ahVn", "~ihin";
    ade: "~allV", _;
    abl: "~altV", _;
    ess: "~anV", _;
    abe: "~attV", _;
    possessive_nom_sg: "~ani", _, _, _, "~ansV";
    possessive_nom_pl: _, "~asi", "~amme", "~anne", _;
    possessive_gen_sg: _, "~asi", "~amme", "~anne", _;
    possessive_par_sg: _, _, _, _, "~atVVn|~atVnsV";
    possessive_ine_sg: _, _, _, _, "~assVVn|~assVnsV";
    possessive_ela_sg: _, _, _, _, "~astVVn|~astVnsV";
    possessive_ill_sg: _, _, _, _, "~ahVnsV";
    possessive_ill_pl: "~ihini", derived, derived, derived, "~ihinsA";
    possessive_ade_sg: _, _, _, _, "~allVVn|~allVnsV";
    possessive_abl_sg: _, _, _, _, "~altVVn|~altVnsV";
    possessive_all_sg: _, _, _, _, "~alleen|~allensA";
    possessive_ess_sg: _, _, _, _, "~anVVn|~anVnsV";
    possessive_tra_sg: _, _, _, _, "~akseen|~aksensA";
    possessive_abe_sg: _, _, _, _, "~attVVn|~attVnsV";
    ),
    // Työ: 9 words (vyö, työ, keskiyö, lapsityö, käsityö, päivätyö). Like Verkko, except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("työ", 9, "~y~", like Verkko;
    stem lemma: "~y~- like työ";
    stem noun_par_pl: "~~i- like työ";
    nom: _, "~y~t";
    ine: "~y~ssä", _;
    ela: "~y~stä", _;
    ill: "~y~hön", _;
    ade: "~y~llä", _;
    abl: "~y~ltä", _;
    ess: "~y~nä", _;
    abe: "~y~ttä", _;
    possessive_nom_sg: "~y~ni", _, _, _, "~y~nsä";
    possessive_nom_pl: _, "~y~si", "~y~mme", "~y~nne", _;
    possessive_gen_sg: _, "~y~si", "~y~mme", "~y~nne", _;
    possessive_ine_sg: _, _, _, _, "~y~ssään|~y~ssänsä";
    possessive_ela_sg: _, _, _, _, "~y~stään|~y~stänsä";
    possessive_ill_sg: _, _, _, _, "~y~hönsä";
    possessive_ade_sg: _, _, _, _, "~y~llään|~y~llänsä";
    possessive_abl_sg: _, _, _, _, "~y~ltään|~y~ltänsä";
    possessive_all_sg: _, _, _, _, "~y~lleen|~y~llensä";
    possessive_ess_sg: _, _, _, _, "~y~nään|~y~nänsä";
    possessive_tra_sg: _, _, _, _, "~y~kseen|~y~ksensä";
    possessive_abe_sg: _, _, _, _, "~y~ttään|~y~ttänsä";
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
    possessive_ine_sg: _, _, _, _, "~jessAVn|~jessAnsV";
    possessive_ine_pl: "~jissAni", _, _, _, "~jissAVn|~jissAnsV";
    possessive_ela_sg: _, _, _, _, "~jestAVn|~jestAnsV";
    possessive_ela_pl: "~jistAni", _, _, _, "~jistAVn|~jistAnsV";
    possessive_ade_sg: _, _, _, _, "~jellAVn|~jellAnsV";
    possessive_ade_pl: "~jillAni", _, _, _, "~jillAVn|~jillAnsV";
    possessive_abl_sg: _, _, _, _, "~jeltAVn|~jeltAnsV";
    possessive_abl_pl: "~jiltAni", _, _, _, "~jiltAVn|~jiltAnsV";
    possessive_all_sg: _, _, _, _, "~jelleen|~jellensA";
    possessive_all_pl: "~jilleni", _, _, _, "~jilleen|~jillensA";
    possessive_tra_sg: _, _, _, _, "~jekseen|~jeksensA";
    possessive_tra_pl: _, _, _, _, "~jikseen|~jiksensA";
    possessive_abe_sg: _, _, _, _, "~jettAVn|~jettAnsV";
    possessive_abe_pl: "~jittAni", _, _, _, "~jittAVn|~jittAnsV";
    ),
    // Sakset: 9 words (helmet, sakset, revontulet, nimikirjaimet, näätäeläimet, viikset). Like Siemen with `` after each pattern (`et` after ~), except in 35 cells: nom pl, gen_ pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("sakset", 9, "~et", like Siemen with "", "et" after "~";
    nom: _, "~et";
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
    possessive_nom_pl: "~eni", "~esi", "~emme", "~enne", "~ensA";
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
    // Paikka: 9 words (jalka, taka, nahka, vaaka, velka, puujalka). Like Silmukka, except in 14 cells: ins sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_par_pl _2sg, possessive_par_pl _1pl, ….
    declension!("paikka", 9, "~ka", like Silmukka;
    stem noun_par_pl: "~ko- like taso";
    ins: "~an", _;
    possessive_gen_pl: _, derived, derived, derived, _;
    possessive_par_pl: _, derived, derived, derived, _;
    possessive_ill_pl: _, derived, derived, derived, _;
    possessive_ins_sg: _, "~kasi", "~kamme", "~kanne", "~kansa";
    ),
    // Kieli: 8 words (veri, herkkusieni, korvasieni, puhekieli, huhtasieni, satakieli). Like Kivi.
    declension!("kieli", 8, "~i", like Kivi;
    stem lemma: "~- like kieli";
    stem noun_par_pl: "~- like kieli";
    ),
    // Jousi: 8 words (uni, lohi, tuli, tuohi, vuohi, huuli). Like Siemen with `` after each pattern (`i` after ~), except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
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
    possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    possessive_ill_sg: _, _, _, _, "~eVnsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enAnsV";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    ),
    // Hiili: 8 words (hiiri, hiili, kotihiiri, metsähiiri, vaivaishiiri, koivuhiiri). Like Kieli.
    declension!("hiili", 8, "~i", like Kieli;
    stem noun_par_pl: "~- like hiili";
    ),
    // Kuve: 8 words (rove, leve, helve, tarve, taive, lieve). Like Tunnus, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("kuve", 8, "~v~", like Tunnus;
    stem lemma: "~v~- like aste";
    stem noun_gen_sg: "~p~- like kate";
    stem noun_par_pl: "~p~i- like kate";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Kissanristiäiset: 8 words (päättäjäiset, metsäpäästäiset, karhunpeijaiset, arpajaiset, myötäjäiset, kädelliset). Like Siemen with `` after each pattern (`et` after ~), except in 36 cells: nom pl, gen_ pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("kissanristiäiset", 8, "~et", like Siemen with "", "et" after "~";
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
    possessive_par_pl: _, _, _, _, "~iAVn|~VAnsV";
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
    // Toimi: 7 words (lumi, luomi, tuomi, toimi, taimi, vastatoimi). Like Kytkin, except in 7 cells: nom sg, par sg, possessive_par_sg _3, possessive_par_pl _1sg, possessive_par_pl _2sg, possessive_par_pl _1pl, ….
    declension!("toimi", 7, "~mi", like Kytkin;
    nom: "~mi", _;
    par: "~ntA|~meA", _;
    possessive_par_sg: _, _, _, _, "~ntAVn|~meAVn|~ntansa|~meansa";
    possessive_par_pl: "~meAni|~miAni", "~meAsi|~miAsi", "~meAmme|~miAmme", "~meAnne|~miAnne", _;
    ),
    // Veto: 6 words (vety, lyhty, hyöty, yty, sääty, pääty). Like Luku, except in 44 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("veto", 6, "~t~", like Luku;
    stem lemma: "~t~- like taso";
    stem noun_par_pl: "~t~- like taso";
    nom: _, "~d~t";
    ine: "~d~ssA", "~d~issA";
    ela: "~d~stA", "~d~istA";
    ill: "~t~Vn", _;
    ade: "~d~llA", "~d~illA";
    abl: "~d~ltA", "~d~iltA";
    all: _, "~d~ille";
    ess: "~t~nA", _;
    abe: "~d~ttA", "~d~ittA";
    possessive_nom_sg: "~t~ni", _, _, _, "~t~nsA";
    possessive_nom_pl: _, "~t~si", "~t~mme", "~t~nne", _;
    possessive_gen_sg: _, "~t~si", "~t~mme", "~t~nne", _;
    possessive_ine_sg: _, _, _, _, "~d~ssAVn|~d~ssAnsV";
    possessive_ine_pl: "~d~issAni", _, _, _, "~d~issAVn|~d~issAnsV";
    possessive_ela_sg: _, _, _, _, "~d~stAVn|~d~stAnsV";
    possessive_ela_pl: "~d~istAni", _, _, _, "~d~istAVn|~d~istAnsV";
    possessive_ill_sg: _, _, _, _, "~t~VnsA";
    possessive_ade_sg: _, _, _, _, "~d~llAVn|~d~llAnsV";
    possessive_ade_pl: "~d~illAni", _, _, _, "~d~illAVn|~d~illAnsV";
    possessive_abl_sg: _, _, _, _, "~d~ltAVn|~d~ltAnsV";
    possessive_abl_pl: "~d~iltAni", _, _, _, "~d~iltAVn|~d~iltAnsV";
    possessive_all_sg: _, _, _, _, "~d~lleen|~d~llensA";
    possessive_all_pl: "~d~illeni", _, _, _, "~d~illeen|~d~illensA";
    possessive_ess_sg: _, _, _, _, "~t~nAVn|~t~nAnsV";
    possessive_tra_sg: _, _, _, _, "~d~kseen|~d~ksensA";
    possessive_tra_pl: _, _, _, _, "~d~ikseen|~d~iksensA";
    possessive_abe_sg: _, _, _, _, "~d~ttAVn|~d~ttAnsV";
    possessive_abe_pl: "~d~ittAni", _, _, _, "~d~ittAVn|~d~ittAnsV";
    ),
    // Kupu: 6 words (riepu, hepo, virtahepo, käpy, häpy, ropo). Like Luku, except in 12 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _3, possessive_nom_pl _2sg, possessive_nom_pl _1pl, ….
    declension!("kupu", 6, "~p~", like Luku;
    stem lemma: "~p~- like taso";
    stem noun_par_pl: "~p~- like taso";
    ill: "~p~Vn", _;
    ess: "~p~nA", _;
    possessive_nom_sg: "~p~ni", _, _, _, "~p~nsA";
    possessive_nom_pl: _, "~p~si", "~p~mme", "~p~nne", _;
    possessive_gen_sg: _, "~p~si", "~p~mme", "~p~nne", _;
    possessive_ill_sg: _, _, _, _, "~p~VnsA";
    possessive_ess_sg: _, _, _, _, "~p~nAVn|~p~nAnsV";
    ),
    // Keihäs: 6 words (kirves, pylväs, keihäs, räystäs, eväs, lähettiläs). Like Aste with `` after each pattern (`s` after ~), except in 25 cells: par sg, ine sg, ela sg, ill sg, ill pl, ade sg, ….
    declension!("keihäs", 6, "~s", like Aste with "", "s" after "~";
    par: "~stA", _;
    ine: "~VssV", _;
    ela: "~VstV", _;
    ill: "~VseVn", "~isVin";
    ade: "~VllV", _;
    abl: "~VltV", _;
    ess: "~VnV", _;
    abe: "~VttV", _;
    possessive_nom_sg: _, _, _, _, "~VnsV";
    possessive_par_sg: _, _, _, _, "~stAVn|~stAnsV";
    possessive_ine_sg: _, _, _, _, "~VssVVn|~AssVnsV";
    possessive_ela_sg: _, _, _, _, "~VstVVn|~AstVnsV";
    possessive_ill_sg: _, _, _, _, "~VseVnsA";
    possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    possessive_ade_sg: _, _, _, _, "~VllVVn|~AllVnsV";
    possessive_abl_sg: _, _, _, _, "~VltVVn|~AltVnsV";
    possessive_all_sg: _, _, _, _, "~Vlleen|~AllensA";
    possessive_ess_sg: _, _, _, _, "~VnVVn|~AnVnsV";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~AksensA";
    possessive_abe_sg: _, _, _, _, "~VttVVn|~AttVnsV";
    ),
    // Kiertotatar: 6 words (kuningatar, ruhtinatar, haltijatar, onnetar, taiteilijatar, kiertotatar). Like Kate, except in 48 cells: gen_ pl, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("kiertotatar", 6, "~~", like Kate;
    stem lemma: "~~- like tarvas";
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
    possessive_gen_pl: "~t~ieni|~~teni", "~t~iesi|~~tesi", "~t~iemme|~~temme", "~t~ienne|~~tenne", "~t~iensa|~~tensA";
    possessive_par_pl: "~t~iani", _, _, _, "~t~iaan|~t~iAnsV";
    possessive_ine_sg: _, _, _, _, "~t~essaan|~t~essAnsV";
    possessive_ine_pl: "~t~issani", _, _, _, "~t~issaan|~t~issAnsV";
    possessive_ela_sg: _, _, _, _, "~t~estaan|~t~estAnsV";
    possessive_ela_pl: "~t~istani", _, _, _, "~t~istaan|~t~istAnsV";
    possessive_ill_sg: _, _, _, _, "~t~eensa";
    possessive_ill_pl: "~t~iini", derived, derived, derived, "~t~iinsa";
    possessive_ade_sg: _, _, _, _, "~t~ellaan|~t~ellAnsV";
    possessive_ade_pl: "~t~illani", _, _, _, "~t~illaan|~t~illAnsV";
    possessive_abl_sg: _, _, _, _, "~t~eltaan|~t~eltAnsV";
    possessive_abl_pl: "~t~iltani", _, _, _, "~t~iltaan|~t~iltAnsV";
    possessive_ess_sg: _, _, _, _, "~t~enaan|~t~enAnsV";
    possessive_ess_pl: "~t~inani", _, _, _, "~t~inaan|~t~inAnsV";
    possessive_abe_sg: _, _, _, _, "~t~ettaan|~t~ettAnsV";
    possessive_abe_pl: "~t~ittani", _, _, _, "~t~ittaan|~t~ittAnsV";
    ),
    // Filee: 6 words (glasee, filee, sisäfilee, essee, kanapee, shawnee). Like Aste with `` after each pattern (`e` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("filee", 6, "~e", like Aste with "", "e" after "~";
    nom: _, "~et";
    par: "~etA", _;
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~ehVn|~eseen", "~ihin|~isVin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    ess: "~enA", _;
    abe: "~ettA", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensA";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_par_sg: _, _, _, _, "~etAVn|~etAnsV";
    possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    possessive_ill_sg: _, _, _, _, "~ehVnsA|~eseensA";
    possessive_ill_pl: "~esVVni|~ihini|~isVVni", "~esVVsi|~ihisi|~isVVsi", "~esVVmme|~ihimme|~isVVmme", "~esVVnne|~ihinne|~isVVnne", "~ihinsA|~isVinsA";
    possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    possessive_ess_sg: _, _, _, _, "~enAVn|~enAnsV";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    ),
    // Maamerkki: 6 words (maamerkki, takapenkki, polttomerkki, jumbojetti, puumerkki, loppumerkki). Like Keri with `ˇ` after each pattern.
    declension!("maamerkki", 6, "~iˇ", like Keri with "ˇ";
    ),
    // Värit: 6 words (lasit, rillit, tanssit, värit, näpit, pihdit). Like Hmong with `` after each pattern (`it` after ~), except in 33 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
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
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: derived, derived, derived, derived, derived;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_ess_pl: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    possessive_com_pl: _, _, _, _, derived;
    ),
    // Lippu: 5 words (ryyppy, hyppy, ryöppy, syrjähyppy, nyppy). Like Verkko, except in 12 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _3, possessive_nom_pl _2sg, possessive_nom_pl _1pl, ….
    declension!("lippu", 5, "~p~", like Verkko;
    stem lemma: "~p~- like taso";
    stem noun_par_pl: "~p~- like taso";
    ill: "~p~Vn", _;
    ess: "~p~nA", _;
    possessive_nom_sg: "~p~ni", _, _, _, "~p~nsA";
    possessive_nom_pl: _, "~p~si", "~p~mme", "~p~nne", _;
    possessive_gen_sg: _, "~p~si", "~p~mme", "~p~nne", _;
    possessive_ill_sg: _, _, _, _, "~p~VnsA";
    possessive_ess_sg: _, _, _, _, "~p~nAVn|~p~nAnsV";
    ),
    // Koe: 5 words (koe, rae, eläinkoe, jae, hajukoe). Like Tunnus, except in 4 cells: possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3.
    declension!("koe", 5, "~~", like Tunnus;
    stem lemma: "~~- like aste";
    stem noun_gen_sg: "~k~- like kate";
    stem noun_par_pl: "~k~i- like kate";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    ),
    // Khmer: 5 words (sandwich, blackjack, khmer, hi-tech, preesens). Like Keri, except in 2 cells: nom sg, possessive_ill_pl _1sg.
    declension!("khmer", 5, "~", like Keri;
    nom: "~", _;
    possessive_ill_pl: "~eihini", _, _, _, _;
    ),
    // Syy: 5 words (syy, kyy, tekosyy, peltopyy, menyy). Like Aste with `` after each pattern (`y` after ~), except in 33 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("syy", 5, "~y", like Aste with "", "y" after "~";
    nom: _, "~yt";
    par: "~ytA", _;
    ine: "~yssA", _;
    ela: "~ystA", _;
    ill: "~yhVn", "~ihin";
    ade: "~yllA", _;
    abl: "~yltA", _;
    ess: "~ynA", _;
    abe: "~yttA", _;
    possessive_nom_sg: "~yni", _, _, _, "~ynsA";
    possessive_nom_pl: _, "~ysi", "~ymme", "~ynne", _;
    possessive_gen_sg: _, "~ysi", "~ymme", "~ynne", _;
    possessive_par_sg: _, _, _, _, "~ytAVn|~ytAnsV";
    possessive_ine_sg: _, _, _, _, "~yssAVn|~yssAnsV";
    possessive_ela_sg: _, _, _, _, "~ystAVn|~ystAnsV";
    possessive_ill_sg: _, _, _, _, "~yhVnsA";
    possessive_ill_pl: "~ihini", derived, derived, derived, "~ihinsA";
    possessive_ade_sg: _, _, _, _, "~yllAVn|~yllAnsV";
    possessive_abl_sg: _, _, _, _, "~yltAVn|~yltAnsV";
    possessive_all_sg: _, _, _, _, "~ylleen|~yllensA";
    possessive_ess_sg: _, _, _, _, "~ynAVn|~ynAnsV";
    possessive_tra_sg: _, _, _, _, "~ykseen|~yksensA";
    possessive_abe_sg: _, _, _, _, "~yttAVn|~yttAnsV";
    ),
    // Askel: 5 words (sammal, askel, lehtisammal, kivisammal, harmosammal).
    declension!("askel", 5, "~";
    stem noun_gen_sg: "~- like askel";
    stem noun_par_pl: "~- like askel";
    nom: "~|~e", _;
    par: "~tA|~etta", _;
    possessive_nom_pl: _, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    possessive_par_sg: _, _, _, _, "~tAVn|~ettAVn|~tansa|~ettansa";
    ),
    // Alkeet: 5 words (alkeet, lauteet, työvaatteet, kalusteet, puitteet). Like Aste with `` after each pattern (`et` after ~), except in 28 cells: nom pl, par sg, ine sg, ela sg, ill sg, ade sg, ….
    declension!("alkeet", 5, "~et", like Aste with "", "et" after "~";
    nom: _, "~et";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    ess: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, _, _, _, derived;
    possessive_nom_pl: "~Vni", _, _, _, "~VnsA";
    possessive_gen_sg: _, derived, derived, derived, _;
    possessive_gen_pl: _, _, _, _, "~idensA|~ittensa";
    possessive_par_sg: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ill_pl: _, _, _, _, "~isVinsA|~ihinsa";
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Lupa: 4 words (kompa, sopa, soopa, huopa). Like Siipi, except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("lupa", 4, "~pa", like Siipi;
    stem lemma: "~pa- like osa";
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
    possessive_ine_sg: _, _, _, _, "~vAssVVn|~vAssVnsV";
    possessive_ela_sg: _, _, _, _, "~vAstVVn|~vAstVnsV";
    possessive_ill_sg: _, _, _, _, "~paVnsV";
    possessive_ade_sg: _, _, _, _, "~vAllVVn|~vAllVnsV";
    possessive_abl_sg: _, _, _, _, "~vAltVVn|~vAltVnsV";
    possessive_all_sg: _, _, _, _, "~vAlleen|~vAllensA";
    possessive_ess_sg: _, _, _, _, "~panVVn|~panVnsV";
    possessive_tra_sg: _, _, _, _, "~vAkseen|~vAksensA";
    possessive_abe_sg: _, _, _, _, "~vAttVVn|~vAttVnsV";
    ),
    // Tee: 4 words (tee, gee, lee, kaksoisvee). Like Piste with `` after each pattern (`e` after ~), except in 40 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("tee", 4, "~e", like Piste with "", "e" after "~";
    nom: _, "~et";
    par: "~etÄ", _;
    ine: "~essÄ", _;
    ela: "~estÄ", _;
    ill: "~ehVn", "~ihin";
    ade: "~ellÄ", _;
    abl: "~eltÄ", _;
    ess: "~enÄ", _;
    abe: "~ettÄ", _;
    possessive_nom_sg: "~eni", _, _, _, "~ensÄ";
    possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    possessive_par_sg: _, _, _, _, "~etÄVn|~etÄnsV";
    possessive_par_pl: _, _, _, _, "~itÄVn|~itänsV";
    possessive_ine_sg: _, _, _, _, "~essÄVn|~essÄnsV";
    possessive_ine_pl: _, _, _, _, "~issÄVn|~issänsV";
    possessive_ela_sg: _, _, _, _, "~estÄVn|~estÄnsV";
    possessive_ela_pl: _, _, _, _, "~istÄVn|~istänsV";
    possessive_ill_sg: _, _, _, _, "~ehVnsÄ";
    possessive_ill_pl: "~ihini", derived, derived, derived, "~ihinsÄ";
    possessive_ade_sg: _, _, _, _, "~ellÄVn|~ellÄnsV";
    possessive_ade_pl: _, _, _, _, "~illÄVn|~illänsV";
    possessive_abl_sg: _, _, _, _, "~eltÄVn|~eltÄnsV";
    possessive_abl_pl: _, _, _, _, "~iltÄVn|~iltänsV";
    possessive_all_sg: _, _, _, _, "~elleen|~ellensÄ";
    possessive_ess_sg: _, _, _, _, "~enÄVn|~enÄnsV";
    possessive_ess_pl: _, _, _, _, "~inÄVn|~inänsV";
    possessive_tra_sg: _, _, _, _, "~ekseen|~eksensÄ";
    possessive_abe_sg: _, _, _, _, "~ettÄVn|~ettÄnsV";
    possessive_abe_pl: _, _, _, _, "~ittÄVn|~ittänsV";
    ),
    // Poika: 4 words (poika, talonpoika, rattopoika, sulhaspoika). Like Kärki, except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("poika", 4, "~ika", like Kärki;
    stem lemma: "~ika- like aika";
    stem noun_par_pl: "~ik- like poika";
    nom: _, "~jAt";
    ine: "~jAssV", _;
    ela: "~jAstV", _;
    ill: "~ikaan", _;
    ade: "~jAllV", _;
    abl: "~jAltV", _;
    ess: "~ikana", _;
    abe: "~jAttV", _;
    possessive_nom_sg: "~ikani", _, _, _, "~ikansa";
    possessive_nom_pl: _, "~ikasi", "~ikamme", "~ikanne", _;
    possessive_gen_sg: _, "~ikasi", "~ikamme", "~ikanne", _;
    possessive_ine_sg: _, _, _, _, "~jAssVVn|~jAssVnsV";
    possessive_ela_sg: _, _, _, _, "~jAstVVn|~jAstVnsV";
    possessive_ill_sg: _, _, _, _, "~ikaansa";
    possessive_ade_sg: _, _, _, _, "~jAllVVn|~jAllVnsV";
    possessive_abl_sg: _, _, _, _, "~jAltVVn|~jAltVnsV";
    possessive_all_sg: _, _, _, _, "~jAlleen|~jAllensA";
    possessive_ess_sg: _, _, _, _, "~ikanaan|~ikanansa";
    possessive_tra_sg: _, _, _, _, "~jAkseen|~jAksensA";
    possessive_abe_sg: _, _, _, _, "~jAttVVn|~jAttVnsV";
    ),
    // Hopea: 4 words (valkea, elohopea, hopea, kulovalkea). Like Aste with `` after each pattern (`a` after ~), except in 35 cells: nom pl, par sg, ine sg, ela sg, ill sg, ade sg, ….
    declension!("hopea", 4, "~a", like Aste with "", "a" after "~";
    nom: _, "~at";
    par: "~aV|~atV", _;
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
    possessive_par_sg: _, _, _, _, "~atVVn|~aansa|~atVnsV";
    possessive_par_pl: "~atVni|~itVni", "~atVsi|~itVsi", "~atVmme|~itVmme", "~atVnne|~itVnne", _;
    possessive_ine_sg: _, _, _, _, "~assVVn|~assVnsV";
    possessive_ela_sg: _, _, _, _, "~astVVn|~astVnsV";
    possessive_ill_sg: _, _, _, _, "~aVnsV";
    possessive_ill_pl: "~isVVni|~ihini", "~isVVsi|~ihisi", "~isVVmme|~ihimme", "~isVVnne|~ihinne", _;
    possessive_ade_sg: _, _, _, _, "~allVVn|~allVnsV";
    possessive_abl_sg: _, _, _, _, "~altVVn|~altVnsV";
    possessive_all_sg: _, _, _, _, "~alleen|~allensA";
    possessive_ess_sg: _, _, _, _, "~anVVn|~anVnsV";
    possessive_tra_sg: _, _, _, _, "~akseen|~aksensA";
    possessive_abe_sg: _, _, _, _, "~attVVn|~attVnsV";
    ),
    // Ydin: 4 words (ydin, säädin, vedin, ulosvedin). Like Osoitin, except in 18 cells: gen_ pl, par pl, ess pl, possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, ….
    declension!("ydin", 4, "~d~n", like Osoitin;
    stem lemma: "~d~n- like etumies";
    stem noun_gen_sg: "~t~m- like ydin";
    gen_: _, "~t~mVen|~d~nten";
    par: _, "~t~mVÄ";
    ess: _, "~t~minÄ";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    possessive_gen_pl: "~t~mVeni|~d~nteni", "~t~mVesi|~d~ntesi", "~t~mVemme|~d~ntemme", "~t~mVenne|~d~ntenne", "~t~mVensÄ|~d~ntensÄ";
    possessive_par_pl: "~t~mVÄni", _, _, _, "~t~mVÄVn|~t~mVÄnsV";
    possessive_ill_pl: _, _, _, _, "~t~miinsÄ";
    possessive_ess_pl: "~t~minÄni", _, _, _, "~t~minÄVn|~t~minÄnsV";
    possessive_com_pl: _, _, _, _, "~t~mineen|~t~minensÄ";
    ),
    // Metsäkauris: 4 words (kauris, saalis, ruumis, metsäkauris). Like Siemen with `` after each pattern (`s` after ~), except in 18 cells: gen_ pl, par sg, par pl, ill sg, ill pl, possessive_gen_pl _1sg, ….
    declension!("metsäkauris", 4, "~s", like Siemen with "", "s" after "~";
    gen_: _, "~Vden|~Vtten";
    par: "~stA", "~VtA";
    ill: "~VseVn", "~VsVin";
    possessive_gen_pl: "~Vdeni|~Vtteni", "~Vdesi|~Vttesi", "~Vdemme|~Vttemme", "~Vdenne|~Vttenne", "~VdensA|~VttensA";
    possessive_par_sg: _, _, _, _, "~stAVn|~stAnsV";
    possessive_par_pl: "~VtAni", _, _, _, "~VtAVn|~VtAnsV";
    possessive_ill_sg: _, _, _, _, "~VseVnsA";
    possessive_ill_pl: "~VsVVni", _, _, _, "~VsVinsA";
    possessive_all_sg: _, _, _, _, "~Vlleen|~VllensA";
    possessive_tra_sg: _, _, _, _, "~Vkseen|~VksensA";
    ),
    // Rättänä: 4 words (äpärä, käpälä, kissankäpälä, rättänä). Like Erä, except in 43 cells: ine pl, ela pl, ade pl, abl pl, all pl, tra pl, ….
    declension!("rättänä", 4, "~ä", like Erä;
    stem noun_par_pl: "~- like rättänä";
    ine: _, "~öissA|~issA";
    ela: _, "~öistA|~istA";
    ade: _, "~öillA|~illA";
    abl: _, "~öiltA|~iltA";
    all: _, "~öille|~ille";
    tra: _, "~öiksi|~iksi";
    abe: _, "~öittA|~ittA";
    ins: _, "~öin|~in";
    possessive_ine_pl: "~öissAni|~issAni", "~öissAsi|~issAsi", "~öissAmme|~issAmme", "~öissAnne|~issAnne", "~öissAVn|~issAVn|~öissAnsV|~issänsä";
    possessive_ela_pl: "~öistAni|~istAni", "~öistAsi|~istAsi", "~öistAmme|~istAmme", "~öistAnne|~istAnne", "~öistAVn|~istAVn|~öistAnsV|~istänsä";
    possessive_ade_pl: "~öillAni|~illAni", "~öillAsi|~illAsi", "~öillAmme|~illAmme", "~öillAnne|~illAnne", "~öillAVn|~illAVn|~öillAnsV|~illänsä";
    possessive_abl_pl: "~öiltAni|~iltAni", "~öiltAsi|~iltAsi", "~öiltAmme|~iltAmme", "~öiltAnne|~iltAnne", "~öiltAVn|~iltAVn|~öiltAnsV|~iltänsä";
    possessive_all_pl: "~öilleni|~illeni", "~öillesi|~illesi", "~öillemme|~illemme", "~öillenne|~illenne", "~öilleen|~illeen|~öillensA|~illensä";
    possessive_tra_pl: "~öikseni|~ikseni", "~öiksesi|~iksesi", "~öiksemme|~iksemme", "~öiksenne|~iksenne", "~öikseen|~ikseen|~öiksensA|~iksensä";
    possessive_abe_pl: "~öittAni|~ittAni", "~öittAsi|~ittAsi", "~öittAmme|~ittAmme", "~öittAnne|~ittAnne", "~öittAVn|~ittAVn|~öittAnsV|~ittänsä";
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
    possessive_par_sg: _, _, _, _, "~utAVn|~utAnsV";
    possessive_ine_sg: _, _, _, _, "~ussAVn|~ussAnsV";
    possessive_ela_sg: _, _, _, _, "~ustAVn|~ustAnsV";
    possessive_ill_sg: _, _, _, _, "~useVnsA";
    possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    possessive_ade_sg: _, _, _, _, "~ullAVn|~ullAnsV";
    possessive_abl_sg: _, _, _, _, "~ultAVn|~ultAnsV";
    possessive_all_sg: _, _, _, _, "~ulleen|~ullensA";
    possessive_ess_sg: _, _, _, _, "~unAVn|~unAnsV";
    possessive_tra_sg: _, _, _, _, "~ukseen|~uksensA";
    possessive_abe_sg: _, _, _, _, "~uttAVn|~uttAnsV";
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
    // Onki: 4 words (henki, onki, kanki, kiertokanki). Like Väki, except in 32 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("onki", 4, "~ki", like Väki;
    nom: _, "~get";
    ine: "~gessA", "~gissA";
    ela: "~gestA", "~gistA";
    ade: "~gellA", "~gillA";
    abl: "~geltA", "~giltA";
    all: _, "~gille";
    abe: "~gettA", "~gittA";
    possessive_ine_sg: _, _, _, _, "~gessAVn|~gessAnsV";
    possessive_ine_pl: "~gissAni", _, _, _, "~gissAVn|~gissAnsV";
    possessive_ela_sg: _, _, _, _, "~gestAVn|~gestAnsV";
    possessive_ela_pl: "~gistAni", _, _, _, "~gistAVn|~gistAnsV";
    possessive_ade_sg: _, _, _, _, "~gellAVn|~gellAnsV";
    possessive_ade_pl: "~gillAni", _, _, _, "~gillAVn|~gillAnsV";
    possessive_abl_sg: _, _, _, _, "~geltAVn|~geltAnsV";
    possessive_abl_pl: "~giltAni", _, _, _, "~giltAVn|~giltAnsV";
    possessive_all_sg: _, _, _, _, "~gelleen|~gellensA";
    possessive_all_pl: "~gilleni", _, _, _, "~gilleen|~gillensA";
    possessive_tra_sg: _, _, _, _, "~gekseen|~geksensA";
    possessive_tra_pl: _, _, _, _, "~gikseen|~giksensA";
    possessive_abe_sg: _, _, _, _, "~gettAVn|~gettAnsV";
    possessive_abe_pl: "~gittAni", _, _, _, "~gittAVn|~gittAnsV";
    ),
    // Vehje: 4 words (hylje, vehje, sulje, rahje). Like Koe, except in 24 cells: possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_par_pl _3, possessive_ine_sg _3, possessive_ine_pl _3, ….
    declension!("vehje", 4, "~j~", like Koe;
    stem lemma: "~j~- like aste";
    stem noun_par_pl: "~k~i- like vehje";
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
    // Piennar: 4 words (manner, tanner, kinner, piennar). Like Askel with `ˇ` after each pattern.
    declension!("piennar", 4, "~ˇ", like Askel with "ˇ";
    ),
    // Lapa: 3 words (kampa, salpa, kalpa). Like Lupa, except in 32 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("lapa", 3, "~pa", like Lupa;
    stem noun_par_pl: "~po- like taso";
    nom: _, "~vVt";
    ine: "~vVssV", "~voissA";
    ela: "~vVstV", "~voistA";
    ade: "~vVllV", "~voillA";
    abl: "~vVltV", "~voiltA";
    all: _, "~voille";
    abe: "~vVttV", "~voittA";
    possessive_ine_sg: _, _, _, _, "~vVssVVn|~vVssVnsV";
    possessive_ine_pl: "~voissAni", _, _, _, "~voissAVn|~voissAnsV";
    possessive_ela_sg: _, _, _, _, "~vVstVVn|~vVstVnsV";
    possessive_ela_pl: "~voistAni", _, _, _, "~voistAVn|~voistAnsV";
    possessive_ade_sg: _, _, _, _, "~vVllVVn|~vVllVnsV";
    possessive_ade_pl: "~voillAni", _, _, _, "~voillAVn|~voillAnsV";
    possessive_abl_sg: _, _, _, _, "~vVltVVn|~vVltVnsV";
    possessive_abl_pl: "~voiltAni", _, _, _, "~voiltAVn|~voiltAnsV";
    possessive_all_sg: _, _, _, _, "~vVlleen|~vVllensA";
    possessive_all_pl: "~voilleni", _, _, _, "~voilleen|~voillensA";
    possessive_tra_sg: _, _, _, _, "~vVkseen|~vVksensA";
    possessive_tra_pl: _, _, _, _, "~voikseen|~voiksensA";
    possessive_abe_sg: _, _, _, _, "~vVttVVn|~vVttVnsV";
    possessive_abe_pl: "~voittAni", _, _, _, "~voittAVn|~voittAnsV";
    ),
    // Leipä: 3 words (syöpä, verisyöpä, sepä). Like Siipi, except in 42 cells: nom pl, ine sg, ine pl, ela sg, ela pl, ill sg, ….
    declension!("leipä", 3, "~pä", like Siipi;
    stem lemma: "~pä- like osa";
    stem noun_par_pl: "~pi- like lehti";
    nom: _, "~vÄt";
    ine: "~vÄssV", "~vissÄ";
    ela: "~vÄstV", "~vistÄ";
    ill: "~päVn", _;
    ade: "~vÄllV", "~villÄ";
    abl: "~vÄltV", "~viltÄ";
    ess: "~pänV", _;
    abe: "~vÄttV", "~vittÄ";
    possessive_nom_sg: "~päni", _, _, _, "~pänsV";
    possessive_nom_pl: _, "~päsi", "~pämme", "~pänne", _;
    possessive_gen_sg: _, "~päsi", "~pämme", "~pänne", _;
    possessive_ine_sg: _, _, _, _, "~vÄssVVn|~vÄssVnsV";
    possessive_ine_pl: "~vissÄni", _, _, _, "~vissÄVn|~vissÄnsV";
    possessive_ela_sg: _, _, _, _, "~vÄstVVn|~vÄstVnsV";
    possessive_ela_pl: "~vistÄni", _, _, _, "~vistÄVn|~vistÄnsV";
    possessive_ill_sg: _, _, _, _, "~päVnsV";
    possessive_ade_sg: _, _, _, _, "~vÄllVVn|~vÄllVnsV";
    possessive_ade_pl: "~villÄni", _, _, _, "~villÄVn|~villÄnsV";
    possessive_abl_sg: _, _, _, _, "~vÄltVVn|~vÄltVnsV";
    possessive_abl_pl: "~viltÄni", _, _, _, "~viltÄVn|~viltÄnsV";
    possessive_all_sg: _, _, _, _, "~vÄlleen|~vÄllensA";
    possessive_all_pl: _, _, _, _, "~villeen|~villensÄ";
    possessive_ess_sg: _, _, _, _, "~pänVVn|~pänVnsV";
    possessive_tra_sg: _, _, _, _, "~vÄkseen|~vÄksensA";
    possessive_tra_pl: _, _, _, _, "~vikseen|~viksensÄ";
    possessive_abe_sg: _, _, _, _, "~vÄttVVn|~vÄttVnsV";
    possessive_abe_pl: "~vittÄni", _, _, _, "~vittÄVn|~vittÄnsV";
    ),
    // Maahockey: 3 words (body, curry, copy). Like Taso, except in 44 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("maahockey", 3, "~", like Taso;
    par: "~I", "~jI";
    ine: "~ssI", "~issa";
    ela: "~stI", "~ista";
    ade: "~llI", "~illa";
    abl: "~ltI", "~ilta";
    ess: "~nI", "~ina";
    abe: "~ttI", "~itta";
    possessive_nom_sg: _, _, _, _, "~nsI";
    possessive_gen_pl: _, _, _, _, "~jensI";
    possessive_par_sg: _, _, _, _, "~IVn|~InsV";
    possessive_par_pl: "~jIni", _, _, _, "~jIVn|~jInsV";
    possessive_ine_sg: _, _, _, _, "~ssIVn|~ssInsV";
    possessive_ine_pl: "~issani", _, _, _, "~issaVn|~issansV";
    possessive_ela_sg: _, _, _, _, "~stIVn|~stInsV";
    possessive_ela_pl: "~istani", _, _, _, "~istaVn|~istansV";
    possessive_ill_sg: _, _, _, _, "~VnsI";
    possessive_ill_pl: _, _, _, _, "~ihinsa";
    possessive_ade_sg: _, _, _, _, "~llIVn|~llInsV";
    possessive_ade_pl: "~illani", _, _, _, "~illaVn|~illansV";
    possessive_abl_sg: _, _, _, _, "~ltIVn|~ltInsV";
    possessive_abl_pl: "~iltani", _, _, _, "~iltaVn|~iltansV";
    possessive_all_sg: _, _, _, _, "~lleen|~llensI";
    possessive_all_pl: _, _, _, _, "~illeen|~illensa";
    possessive_ess_sg: _, _, _, _, "~nIVn|~nInsV";
    possessive_ess_pl: "~inani", _, _, _, "~inaVn|~inansV";
    possessive_tra_sg: _, _, _, _, "~kseen|~ksensI";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensa";
    possessive_abe_sg: _, _, _, _, "~ttIVn|~ttInsV";
    possessive_abe_pl: "~ittani", _, _, _, "~ittaVn|~ittansV";
    possessive_com_pl: _, _, _, _, "~ineen|~inensa";
    ),
    // Hammas: 3 words (lammas, hammas, kulmahammas). Like Kuve, except in 34 cells: nom sg, nom pl, par sg, ine sg, ela sg, ill sg, ….
    declension!("hammas", 3, "~m~s", like Kuve;
    nom: "~m~s", "~p~Vt";
    par: "~m~sta", _;
    ine: "~p~VssV", _;
    ela: "~p~VstV", _;
    ill: "~p~Vseen", "~p~isiin";
    ade: "~p~VllV", _;
    abl: "~p~VltV", _;
    ess: "~p~VnV", _;
    abe: "~p~VttV", _;
    possessive_nom_sg: "~p~Vni", _, _, _, "~p~VnsV";
    possessive_nom_pl: _, "~p~Vsi", "~p~Vmme", "~p~Vnne", _;
    possessive_gen_sg: _, "~p~Vsi", "~p~Vmme", "~p~Vnne", _;
    possessive_par_sg: _, _, _, _, "~m~staan|~m~stansa";
    possessive_ine_sg: _, _, _, _, "~p~VssVVn|~p~VssVnsV";
    possessive_ela_sg: _, _, _, _, "~p~VstVVn|~p~VstVnsV";
    possessive_ill_sg: _, _, _, _, "~p~VseensA";
    possessive_ill_pl: "~p~isiini", derived, derived, derived, "~p~isiinsA";
    possessive_ade_sg: _, _, _, _, "~p~VllVVn|~p~VllVnsV";
    possessive_abl_sg: _, _, _, _, "~p~VltVVn|~p~VltVnsV";
    possessive_all_sg: _, _, _, _, "~p~Vlleen|~p~VllensA";
    possessive_ess_sg: _, _, _, _, "~p~VnVVn|~p~VnVnsV";
    possessive_tra_sg: _, _, _, _, "~p~Vkseen|~p~VksensA";
    possessive_abe_sg: _, _, _, _, "~p~VttVVn|~p~VttVnsV";
    ),
    // Komme: 3 words (komme, lumme, ommel). Like Kuve, except in 27 cells: nom sg, par sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_par_sg _3, ….
    declension!("komme", 3, "~m~", like Kuve;
    stem noun_par_pl: "~p~i- like vehje";
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
    // Bokserit: 3 words (polttarit, kroaatit, bokserit). Like Kaanon with `` after each pattern (`it` after ~), except in 31 cells: par sg, par pl, ine sg, ela sg, ill sg, ade sg, ….
    declension!("bokserit", 3, "~it", like Kaanon with "", "it" after "~";
    par: derived, "~eitA|~eja";
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
    possessive_gen_pl: _, _, _, _, "~iensA|~eidensa|~eittensa";
    possessive_par_sg: _, _, _, _, derived;
    possessive_par_pl: "~eitAni|~ejani", "~eitAsi|~ejasi", "~eitAmme|~ejamme", "~eitAnne|~ejanne", _;
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
    // Opas: 3 words (opas, lipas, matkaopas). Like Hammas, except in 13 cells: par pl, ess pl, possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3, ….
    declension!("opas", 3, "~~s", like Hammas;
    stem lemma: "~~s- like tarvas";
    stem noun_gen_sg: "~p~- like opas";
    par: _, "~p~ita";
    ess: _, "~p~ina";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    possessive_gen_pl: _, _, _, _, "~p~idensa|~p~ittensa";
    possessive_par_pl: "~p~itani", _, _, _, "~p~itaan|~p~itansa";
    possessive_ill_pl: _, _, _, _, "~p~isiinsa";
    possessive_ess_pl: "~p~inani", _, _, _, "~p~inaan|~p~inansa";
    possessive_com_pl: _, _, _, _, "~p~ineen|~p~inensa";
    ),
    // Lapsi: 3 words (lapsi, opetuslapsi, sotalapsi). Like Kuve, except in 20 cells: nom sg, gen_ pl, par sg, par pl, ill sg, ill pl, ….
    declension!("lapsi", 3, "~p~i", like Kuve;
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
    // Markkinat: 3 words (kakkulat, talvikisat, markkinat). Like Nappula with `` after each pattern (`t` after a), except in 29 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
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
    possessive_par_sg: _, _, _, _, derived;
    possessive_ine_sg: _, _, _, _, derived;
    possessive_ela_sg: _, _, _, _, derived;
    possessive_ill_sg: _, _, _, _, derived;
    possessive_ill_pl: "~oihini", _, _, _, _;
    possessive_ade_sg: _, _, _, _, derived;
    possessive_abl_sg: _, _, _, _, derived;
    possessive_all_sg: _, _, _, _, derived;
    possessive_ess_sg: _, _, _, _, derived;
    possessive_tra_sg: _, _, _, _, derived;
    possessive_abe_sg: _, _, _, _, derived;
    ),
    // Ahdin: 3 words (kudin, johdin, ahdin). Like Osoitin, except in 10 cells: gen_ pl, possessive_nom_pl _1sg, possessive_nom_pl _3, possessive_gen_sg _1sg, possessive_gen_sg _3, possessive_gen_pl _1sg, ….
    declension!("ahdin", 3, "~d~n", like Osoitin;
    stem lemma: "~d~n- like tunnus";
    stem noun_gen_sg: "~t~m- like purkaminen";
    gen_: _, "~t~mVen|~d~nten";
    possessive_nom_pl: derived, _, _, _, derived;
    possessive_gen_sg: derived, _, _, _, derived;
    possessive_gen_pl: "~t~mVeni|~d~nteni", "~t~mVesi|~d~ntesi", "~t~mVemme|~d~ntemme", "~t~mVenne|~d~ntenne", "~t~mVensA|~d~ntensA";
    ),
    // Hässäkkä: 3 words (lehdykkä, hässäkkä, räpäkkä). Like Tekijä, except in 25 cells: nom pl, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("hässäkkä", 3, "~kä", like Tekijä;
    stem lemma: "~kä- like osa";
    stem noun_par_pl: "~- like hässäkkä";
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
    possessive_ine_sg: _, _, _, _, "~AssVVn|~AssVnsV";
    possessive_ela_sg: _, _, _, _, "~AstVVn|~AstVnsV";
    possessive_ill_sg: _, _, _, _, "~käVnsV";
    possessive_ade_sg: _, _, _, _, "~AllVVn|~AllVnsV";
    possessive_abl_sg: _, _, _, _, "~AltVVn|~AltVnsV";
    possessive_all_sg: _, _, _, _, "~Alleen|~AllensA";
    possessive_ess_sg: _, _, _, _, "~känVVn|~känVnsV";
    possessive_tra_sg: _, _, _, _, "~Akseen|~AksensA";
    possessive_abe_sg: _, _, _, _, "~AttVVn|~AttVnsV";
    ),
];
