//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~a`, `makedonia`: the stem and `a`). A stem in two pieces (`~~`, `johannes`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A capital (`A`, `V`, `E`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `tokelau`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem gen_sg: \"~a- like makedonia\"`: the slot's cells are `makedonia`'s table's patterns
/// with `~a` for their `~`) (see `stems.rs`), then a row of patterns per line
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
/// `proper_noun_possessive_nom_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_nom_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_gen_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_gen_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_par_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_par_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ine_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ine_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ela_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ela_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ill_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ill_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ade_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ade_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_abl_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_abl_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_all_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_all_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ess_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ess_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_tra_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_tra_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_abe_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_abe_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ins_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_ins_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_com_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `proper_noun_possessive_com_pl` (_1sg, _2sg, _1pl, _2pl, _3)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem gen_sg: $s1:tt;)?
     $(stem par_pl: $s2:tt;)?
     $(stem par_sg: $s3:tt;)?
     $(stem ine_sg: $s4:tt;)?
     $(stem ill_sg: $s5:tt;)?
     $(stem ade_sg: $s6:tt;)?
     $(stem abl_sg: $s7:tt;)?
     $(stem all_sg: $s8:tt;)?
     $(stem ess_sg: $s9:tt;)?
     $(stem tra_sg: $s10:tt;)?
     $(stem possessive_gen_sg: $s11:tt;)?
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
     $(proper_noun_possessive_nom_sg: $c28:tt, $c29:tt, $c30:tt, $c31:tt, $c32:tt;)?
     $(proper_noun_possessive_nom_pl: $c33:tt, $c34:tt, $c35:tt, $c36:tt, $c37:tt;)?
     $(proper_noun_possessive_gen_sg: $c38:tt, $c39:tt, $c40:tt, $c41:tt, $c42:tt;)?
     $(proper_noun_possessive_gen_pl: $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(proper_noun_possessive_par_sg: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt;)?
     $(proper_noun_possessive_par_pl: $c53:tt, $c54:tt, $c55:tt, $c56:tt, $c57:tt;)?
     $(proper_noun_possessive_ine_sg: $c58:tt, $c59:tt, $c60:tt, $c61:tt, $c62:tt;)?
     $(proper_noun_possessive_ine_pl: $c63:tt, $c64:tt, $c65:tt, $c66:tt, $c67:tt;)?
     $(proper_noun_possessive_ela_sg: $c68:tt, $c69:tt, $c70:tt, $c71:tt, $c72:tt;)?
     $(proper_noun_possessive_ela_pl: $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(proper_noun_possessive_ill_sg: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt;)?
     $(proper_noun_possessive_ill_pl: $c83:tt, $c84:tt, $c85:tt, $c86:tt, $c87:tt;)?
     $(proper_noun_possessive_ade_sg: $c88:tt, $c89:tt, $c90:tt, $c91:tt, $c92:tt;)?
     $(proper_noun_possessive_ade_pl: $c93:tt, $c94:tt, $c95:tt, $c96:tt, $c97:tt;)?
     $(proper_noun_possessive_abl_sg: $c98:tt, $c99:tt, $c100:tt, $c101:tt, $c102:tt;)?
     $(proper_noun_possessive_abl_pl: $c103:tt, $c104:tt, $c105:tt, $c106:tt, $c107:tt;)?
     $(proper_noun_possessive_all_sg: $c108:tt, $c109:tt, $c110:tt, $c111:tt, $c112:tt;)?
     $(proper_noun_possessive_all_pl: $c113:tt, $c114:tt, $c115:tt, $c116:tt, $c117:tt;)?
     $(proper_noun_possessive_ess_sg: $c118:tt, $c119:tt, $c120:tt, $c121:tt, $c122:tt;)?
     $(proper_noun_possessive_ess_pl: $c123:tt, $c124:tt, $c125:tt, $c126:tt, $c127:tt;)?
     $(proper_noun_possessive_tra_sg: $c128:tt, $c129:tt, $c130:tt, $c131:tt, $c132:tt;)?
     $(proper_noun_possessive_tra_pl: $c133:tt, $c134:tt, $c135:tt, $c136:tt, $c137:tt;)?
     $(proper_noun_possessive_abe_sg: $c138:tt, $c139:tt, $c140:tt, $c141:tt, $c142:tt;)?
     $(proper_noun_possessive_abe_pl: $c143:tt, $c144:tt, $c145:tt, $c146:tt, $c147:tt;)?
     $(proper_noun_possessive_ins_sg: $c148:tt, $c149:tt, $c150:tt, $c151:tt, $c152:tt;)?
     $(proper_noun_possessive_ins_pl: $c153:tt, $c154:tt, $c155:tt, $c156:tt, $c157:tt;)?
     $(proper_noun_possessive_com_sg: $c158:tt, $c159:tt, $c160:tt, $c161:tt, $c162:tt;)?
     $(proper_noun_possessive_com_pl: $c163:tt, $c164:tt, $c165:tt, $c166:tt, $c167:tt;)?
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
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
                $(flexion::cell!($s6),)? "\t",
                $(flexion::cell!($s7),)? "\t",
                $(flexion::cell!($s8),)? "\t",
                $(flexion::cell!($s9),)? "\t",
                $(flexion::cell!($s10),)? "\t",
                $(flexion::cell!($s11),)? "\t",
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
    /// Like `makedonia`: 1,340 words.
    Makedonia,
    /// Like `ilmarinen`: 1,180 words.
    Ilmarinen,
    /// Like `salo`: 949 words.
    Salo,
    /// Like `soini`: 520 words.
    Soini,
    /// Like `hydra`: 397 words.
    Hydra,
    /// Like `luxemburg`: 386 words.
    Luxemburg,
    /// Like `tokelau`: 332 words.
    Tokelau,
    /// Like `kärpänen`: 330 words.
    Kärpänen,
    /// Like `leivonmäki`: 283 words.
    Leivonmäki,
    /// Like `sammatti`: 255 words.
    Sammatti,
    /// Like `olhava`: 229 words.
    Olhava,
    /// Like `johannes`: 229 words.
    Johannes,
    /// Like `kestilä`: 227 words.
    Kestilä,
    /// Like `inari`: 223 words.
    Inari,
    /// Like `pyhäjärvi`: 193 words.
    Pyhäjärvi,
    /// Like `rita`: 174 words.
    Rita,
    /// Like `tulimaa`: 161 words.
    Tulimaa,
    /// Like `hietaniemi`: 158 words.
    Hietaniemi,
    /// Like `lahti`: 146 words.
    Lahti,
    /// Like `borneo`: 120 words.
    Borneo,
    /// Like `saari`: 113 words.
    Saari,
    /// Like `härmä`: 109 words.
    Härmä,
    /// Like `salmi`: 106 words.
    Salmi,
    /// Like `nutikka`: 74 words.
    Nutikka,
    /// Like `opas`: 70 words.
    Opas,
    /// Like `lotta`: 58 words.
    Lotta,
    /// Like `kakskerta`: 57 words.
    Kakskerta,
    /// Like `järvenpää`: 46 words.
    Järvenpää,
    /// Like `jäämeri`: 43 words.
    Jäämeri,
    /// Like `aaron`: 41 words.
    Aaron,
    /// Like `roine`: 32 words.
    Roine,
    /// Like `kärppä`: 31 words.
    Kärppä,
    /// Like `kaksoset`: 28 words.
    Kaksoset,
    /// Like `häme`: 26 words.
    Häme,
    /// Like `oinas`: 23 words.
    Oinas,
    /// Like `plejadit`: 20 words.
    Plejadit,
    /// Like `luukas`: 19 words.
    Luukas,
    /// Like `rinne`: 18 words.
    Rinne,
    /// Like `orivesi`: 17 words.
    Orivesi,
    /// Like `vesimies`: 14 words.
    Vesimies,
    /// Like `vehmaa`: 14 words.
    Vehmaa,
    /// Like `joensuu`: 13 words.
    Joensuu,
    /// Like `ahava`: 12 words.
    Ahava,
    /// Like `altai`: 11 words.
    Altai,
    /// Like `heinisuo`: 9 words.
    Heinisuo,
    /// Like `delfoi`: 9 words.
    Delfoi,
    /// Like `okko`: 9 words.
    Okko,
    /// Like `herkules`: 8 words.
    Herkules,
    /// Like `pohjantähti`: 7 words.
    Pohjantähti,
    /// Like `komorit`: 7 words.
    Komorit,
    /// Like `hölttä`: 7 words.
    Hölttä,
    /// Like `korppoo`: 7 words.
    Korppoo,
    /// Like `kaukomieli`: 7 words.
    Kaukomieli,
    /// Like `leijonat`: 7 words.
    Leijonat,
    /// Like `seem`: 6 words.
    Seem,
    /// Like `ilkka`: 6 words.
    Ilkka,
    /// Like `kristillisdemokraatit`: 6 words.
    Kristillisdemokraatit,
    /// Like `kainuu`: 6 words.
    Kainuu,
    /// Like `räty`: 6 words.
    Räty,
    /// Like `kalat`: 5 words.
    Kalat,
    /// Like `hermes`: 5 words.
    Hermes,
    /// Like `ilmatar`: 5 words.
    Ilmatar,
    /// Like `föglö`: 5 words.
    Föglö,
    /// Like `ärväs`: 5 words.
    Ärväs,
    /// Like `salokannel`: 5 words.
    Salokannel,
    /// Like `ahde`: 5 words.
    Ahde,
    /// Like `kauris`: 5 words.
    Kauris,
    /// Like `paasi`: 5 words.
    Paasi,
    /// Like `viisut`: 5 words.
    Viisut,
    /// Like `lepo`: 4 words.
    Lepo,
    /// Like `marjatta`: 4 words.
    Marjatta,
    /// Like `yrjänä`: 4 words.
    Yrjänä,
    /// Like `luopa`: 4 words.
    Luopa,
    /// Like `lohi`: 4 words.
    Lohi,
    /// Like `haukipudas`: 4 words.
    Haukipudas,
    /// Like `korkee`: 4 words.
    Korkee,
    /// Like `tehtaankatu`: 4 words.
    Tehtaankatu,
    /// Like `suvitie`: 4 words.
    Suvitie,
    /// Like `hyväkkä`: 4 words.
    Hyväkkä,
    /// Like `alavus`: 3 words.
    Alavus,
    /// Like `pyy`: 3 words.
    Pyy,
    /// Like `kurki`: 3 words.
    Kurki,
    /// Like `lähi-itä`: 3 words.
    Lähiitä,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 83] = [
        Declension::Makedonia,
        Declension::Ilmarinen,
        Declension::Salo,
        Declension::Soini,
        Declension::Hydra,
        Declension::Luxemburg,
        Declension::Tokelau,
        Declension::Kärpänen,
        Declension::Leivonmäki,
        Declension::Sammatti,
        Declension::Olhava,
        Declension::Johannes,
        Declension::Kestilä,
        Declension::Inari,
        Declension::Pyhäjärvi,
        Declension::Rita,
        Declension::Tulimaa,
        Declension::Hietaniemi,
        Declension::Lahti,
        Declension::Borneo,
        Declension::Saari,
        Declension::Härmä,
        Declension::Salmi,
        Declension::Nutikka,
        Declension::Opas,
        Declension::Lotta,
        Declension::Kakskerta,
        Declension::Järvenpää,
        Declension::Jäämeri,
        Declension::Aaron,
        Declension::Roine,
        Declension::Kärppä,
        Declension::Kaksoset,
        Declension::Häme,
        Declension::Oinas,
        Declension::Plejadit,
        Declension::Luukas,
        Declension::Rinne,
        Declension::Orivesi,
        Declension::Vesimies,
        Declension::Vehmaa,
        Declension::Joensuu,
        Declension::Ahava,
        Declension::Altai,
        Declension::Heinisuo,
        Declension::Delfoi,
        Declension::Okko,
        Declension::Herkules,
        Declension::Pohjantähti,
        Declension::Komorit,
        Declension::Hölttä,
        Declension::Korppoo,
        Declension::Kaukomieli,
        Declension::Leijonat,
        Declension::Seem,
        Declension::Ilkka,
        Declension::Kristillisdemokraatit,
        Declension::Kainuu,
        Declension::Räty,
        Declension::Kalat,
        Declension::Hermes,
        Declension::Ilmatar,
        Declension::Föglö,
        Declension::Ärväs,
        Declension::Salokannel,
        Declension::Ahde,
        Declension::Kauris,
        Declension::Paasi,
        Declension::Viisut,
        Declension::Lepo,
        Declension::Marjatta,
        Declension::Yrjänä,
        Declension::Luopa,
        Declension::Lohi,
        Declension::Haukipudas,
        Declension::Korkee,
        Declension::Tehtaankatu,
        Declension::Suvitie,
        Declension::Hyväkkä,
        Declension::Alavus,
        Declension::Pyy,
        Declension::Kurki,
        Declension::Lähiitä,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 83] = [
    // Makedonia: 1,340 words (latvia, algeria, tunisia, libya, andorra, albania).
    declension!("makedonia", 1340, "~a";
    stem gen_sg: "~a- like makedonia";
    stem par_pl: "~- like makedonia";
    stem par_sg: "~aV- like makedonia";
    stem ine_sg: "~- like makedonia";
    stem ill_sg: "~aVn- like makedonia";
    stem ade_sg: "~allV- like makedonia";
    stem abl_sg: "~altV- like makedonia";
    stem all_sg: "~alle- like makedonia";
    stem ess_sg: "~anV- like makedonia";
    stem tra_sg: "~aks- like makedonia";
    nom: "~a", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~asi", "~amme", "~anne", _;
    ),
    // Ilmarinen: 1,180 words (yksisarvinen, peukaloinen, virtanen, koskinen, korhonen, patronen).
    declension!("ilmarinen", 1180, "~nen";
    stem gen_sg: "~se- like ilmarinen";
    stem par_pl: "~s- like ilmarinen";
    stem par_sg: "~stA- like ilmarinen";
    stem ine_sg: "~s- like ilmarinen";
    stem ill_sg: "~seen- like ilmarinen";
    stem ade_sg: "~sellA- like makedonia";
    stem abl_sg: "~seltA- like makedonia";
    stem all_sg: "~selle- like makedonia";
    stem ess_sg: "~senA- like makedonia";
    stem tra_sg: "~seks- like makedonia";
    nom: "~nen", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~sesi", "~semme", "~senne", _;
    ),
    // Salo: 949 words (belize, chile, monaco, nauru, peru, singapore).
    declension!("salo", 949, "~";
    stem gen_sg: "~- like ilmarinen";
    stem par_pl: "~- like salo";
    stem par_sg: "~A- like ilmarinen";
    stem ine_sg: "~- like salo";
    stem ill_sg: "~Vn- like ilmarinen";
    stem ade_sg: "~llA- like makedonia";
    stem abl_sg: "~ltA- like makedonia";
    stem all_sg: "~lle- like makedonia";
    stem ess_sg: "~nA- like makedonia";
    stem tra_sg: "~ks- like makedonia";
    nom: "~", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~si", "~mme", "~nne", _;
    ),
    // Soini: 520 words (burundi, djibouti, haiti, kiribati, malawi, havaiji).
    declension!("soini", 520, "~i";
    stem gen_sg: "~i- like ilmarinen";
    stem par_pl: "~- like soini";
    stem par_sg: "~iA- like ilmarinen";
    stem ine_sg: "~- like soini";
    stem ill_sg: "~iVn- like ilmarinen";
    stem ade_sg: "~illA- like makedonia";
    stem abl_sg: "~iltA- like makedonia";
    stem all_sg: "~ille- like makedonia";
    stem ess_sg: "~inA- like makedonia";
    stem tra_sg: "~iks- like makedonia";
    nom: "~i", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~isi", "~imme", "~inne", _;
    ),
    // Hydra: 397 words (laura, enna, rauma, amanda, saksa, ala). Like Makedonia, except in 9 cells: gen_ pl, par pl, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, proper_noun_possessive_gen_pl _1pl, proper_noun_possessive_gen_pl _2pl, ….
    declension!("hydra", 397, "~a", like Makedonia;
    gen_: _, "~ojen";
    par: _, "~ojA";
    proper_noun_possessive_gen_pl: "~ojeni", "~ojesi", "~ojemme", "~ojenne", "~ojensA";
    proper_noun_possessive_par_pl: "~ojAni", _, _, _, "~ojAVn|~ojAnsV";
    ),
    // Luxemburg: 386 words (israel, bangladesh, bahrain, bhutan, ecuador, gabon). Like Soini, except in 12 cells: nom sg, proper_noun_possessive_par_pl _3, proper_noun_possessive_ine_pl _3, proper_noun_possessive_ela_pl _3, proper_noun_possessive_ill_pl _1sg, proper_noun_possessive_ade_pl _3, ….
    declension!("luxemburg", 386, "~", like Soini;
    stem ill_sg: "~iin- like ilmarinen";
    nom: "~", _;
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~eistAVn";
    proper_noun_possessive_ill_pl: "~eihini", _, _, _, _;
    proper_noun_possessive_ade_pl: _, _, _, _, "~eillAVn";
    proper_noun_possessive_abl_pl: _, _, _, _, "~eiltAVn";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~eittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~eineen";
    ),
    // Tokelau: 332 words (tokelau, palau, guinea-bissau, chisinau, raamattu, ahto). Like Salo with `ˇ` after each pattern.
    declension!("tokelau", 332, "~ˇ", like Salo with "ˇ";
    ),
    // Kärpänen: 330 words (kyyhkynen, ääninen, väinämöinen, kärpänen, nieminen, mäkinen). Like Ilmarinen.
    declension!("kärpänen", 330, "~nen", like Ilmarinen;
    ),
    // Leivonmäki: 283 words (keltainenjoki, kilpi, tyynimeri, öljymäki, mäki, kivimäki). Like Pyhäjärvi with `ˇ` after each pattern.
    declension!("leivonmäki", 283, "~iˇ", like Pyhäjärvi with "ˇ";
    ),
    // Sammatti: 255 words (helsinki, ahti, mississippi, tšekki, englanti, islanti). Like Soini with `ˇ` after each pattern.
    declension!("sammatti", 255, "~iˇ", like Soini with "ˇ";
    ),
    // Olhava: 229 words (aurora, leijona, luka, luna, ora, rosa). Like Salo with `` after each pattern (`a` after ~), except in 41 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("olhava", 229, "~a", like Salo with "", "a" after "~";
    nom: _, "~at";
    gen_: _, "~ien";
    par: "~aV", "~iA";
    ine: "~assV", _;
    ela: "~astV", _;
    ill: "~aVn", "~iin";
    ade: "~allV", _;
    abl: "~altV", _;
    all: "~alle", _;
    ess: "~anV", _;
    tra: "~aksi", _;
    abe: "~attV", _;
    proper_noun_possessive_nom_sg: "~ani", _, _, _, "~ansV";
    proper_noun_possessive_nom_pl: _, "~asi", "~amme", "~anne", _;
    proper_noun_possessive_gen_sg: _, "~asi", "~amme", "~anne", _;
    proper_noun_possessive_gen_pl: "~ieni", "~iesi", "~iemme", "~ienne", "~iensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~aVnsV";
    proper_noun_possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~assVVn|~assVnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~astVVn|~astVnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~aVnsV";
    proper_noun_possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~allVVn|~allVnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~altVVn|~altVnsV";
    proper_noun_possessive_all_sg: _, _, _, _, "~alleen|~allensA";
    proper_noun_possessive_ess_sg: _, _, _, _, "~anVVn|~anVnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, "~akseen|~aksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, "~attVVn|~attVnsV";
    ),
    // Johannes: 229 words (mauritius, barbados, nikolas, venus, uranus, andreas).
    declension!("johannes", 229, "~~";
    stem gen_sg: "~k~e- like ilmarinen";
    stem par_sg: "~~tA- like ilmarinen";
    stem ine_sg: "~k~- like ilmarinen";
    stem ill_sg: "~k~een- like ilmarinen";
    stem ade_sg: "~k~ellA- like makedonia";
    stem abl_sg: "~k~eltA- like makedonia";
    stem all_sg: "~k~elle- like makedonia";
    stem ess_sg: "~k~enA- like makedonia";
    stem tra_sg: "~k~eks- like makedonia";
    nom: "~~", _;
    gen_: _, "~~ten|~k~ien";
    par: _, "~k~iA";
    ill: _, "~k~iin";
    ess: _, "~k~inA";
    proper_noun_possessive_nom_sg: "~k~eni", _, _, _, "~k~ensA";
    proper_noun_possessive_nom_pl: _, "~k~esi", "~k~emme", "~k~enne", _;
    proper_noun_possessive_gen_sg: _, "~k~esi", "~k~emme", "~k~enne", _;
    proper_noun_possessive_gen_pl: "~~teni|~k~ieni", "~~tesi|~k~iesi", "~~temme|~k~iemme", "~~tenne|~k~ienne", "~~tensA|~k~iensA";
    proper_noun_possessive_par_pl: "~k~iAni", _, _, _, "~k~iAVn|~k~iAnsV";
    proper_noun_possessive_ill_pl: "~k~iini", _, _, _, "~k~iinsA";
    proper_noun_possessive_ess_pl: "~k~inAni", "~k~inAsi", "~k~inAmme", "~k~inAnne", "~k~inAVn|~k~inAnsV";
    proper_noun_possessive_com_pl: "~k~ineni", _, _, _, "~k~ineen|~k~inensA";
    ),
    // Kestilä: 227 words (perähikiä, jenkkilä, helinä, luoteisväylä, siviä, rysälä).
    declension!("kestilä", 227, "~ä";
    stem gen_sg: "~ä- like makedonia";
    stem par_pl: "~- like kestilä";
    stem par_sg: "~äV- like makedonia";
    stem ine_sg: "~- like kestilä";
    stem ill_sg: "~äVn- like makedonia";
    stem ade_sg: "~ällV- like makedonia";
    stem abl_sg: "~ältV- like makedonia";
    stem all_sg: "~älle- like makedonia";
    stem ess_sg: "~änV- like makedonia";
    stem tra_sg: "~äks- like makedonia";
    nom: "~ä", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~äsi", "~ämme", "~änne", _;
    ),
    // Inari: 223 words (mali, bali, bengali, portugali, pietari, inari). Like Luxemburg with `` after each pattern (`i` after ~), except in 24 cells: gen_ pl, par pl, ill sg, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, proper_noun_possessive_gen_pl _1pl, ….
    declension!("inari", 223, "~i", like Luxemburg with "", "i" after "~";
    gen_: _, "~ien|~eiden|~eitten";
    par: _, "~eitA|~ejA";
    ill: "~iVn", _;
    proper_noun_possessive_gen_pl: "~ieni|~eideni|~eitteni", "~iesi|~eidesi|~eittesi", "~iemme|~eidemme|~eittemme", "~ienne|~eidenne|~eittenne", "~iensA|~eidensA|~eittensA";
    proper_noun_possessive_par_pl: "~eitAni|~ejAni", "~eitAsi|~ejAsi", "~eitAmme|~ejAmme", "~eitAnne|~ejAnne", "~eitAVn|~ejAVn|~eitAnsV|~ejansa";
    proper_noun_possessive_ine_pl: _, _, _, _, "~eissAVn|~eissAnsV";
    proper_noun_possessive_ela_pl: _, _, _, _, "~eistAVn|~eistAnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~iVnsA";
    proper_noun_possessive_ill_pl: "~eihVni", _, _, _, _;
    proper_noun_possessive_ade_pl: _, _, _, _, "~eillAVn|~eillAnsV";
    proper_noun_possessive_abl_pl: _, _, _, _, "~eiltAVn|~eiltAnsV";
    proper_noun_possessive_all_pl: _, _, _, _, "~eilleen|~eillensA";
    proper_noun_possessive_ess_pl: _, _, _, _, "~einAVn|~einAnsV";
    proper_noun_possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensA";
    proper_noun_possessive_abe_pl: _, _, _, _, "~eittAVn|~eittAnsV";
    proper_noun_possessive_com_pl: _, _, _, _, "~eineen|~einensA";
    ),
    // Pyhäjärvi: 193 words (ilmajärvi, yläjärvi, ancylusjärvi, araljärvi, pohjois-suomi, etelä-suomi). Like Olhava, except in 53 cells: nom sg, par pl, ine sg, ine pl, ela pl, ade pl, ….
    declension!("pyhäjärvi", 193, "~i", like Olhava;
    stem gen_sg: "~e- like ilmarinen";
    stem par_sg: "~eA- like ilmarinen";
    stem ill_sg: "~eVn- like ilmarinen";
    stem ade_sg: "~ellA- like makedonia";
    stem abl_sg: "~eltA- like makedonia";
    stem all_sg: "~elle- like makedonia";
    stem ess_sg: "~enA- like makedonia";
    stem tra_sg: "~eks- like makedonia";
    nom: "~i", _;
    par: _, "~iä";
    ine: "~essA", "~issä";
    ela: _, "~istä";
    ade: _, "~illä";
    abl: _, "~iltä";
    ess: _, "~inä";
    abe: _, "~ittä";
    proper_noun_possessive_nom_sg: "~eni", _, _, _, "~ensA";
    proper_noun_possessive_nom_pl: _, "~esi", "~emme", "~enne", "~ensä";
    proper_noun_possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_pl: _, _, _, _, "~iensä";
    proper_noun_possessive_par_pl: "~iäni", _, _, _, "~iäVn|~iAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    proper_noun_possessive_ine_pl: "~issäni", "~issäsi", "~issämme", "~issänne", "~issäVn|~issAnsV";
    proper_noun_possessive_ela_pl: "~istäni", "~istäsi", "~istämme", "~istänne", "~istäVn|~istAnsV";
    proper_noun_possessive_ill_pl: _, _, _, _, "~iinsä";
    proper_noun_possessive_ade_pl: "~illäni", "~illäsi", "~illämme", "~illänne", "~illäVn|~illAnsV";
    proper_noun_possessive_abl_pl: "~iltäni", "~iltäsi", "~iltämme", "~iltänne", "~iltäVn|~iltAnsV";
    proper_noun_possessive_ess_pl: "~inäni", "~inäsi", "~inämme", "~inänne", "~inäVn|~inAnsV";
    proper_noun_possessive_abe_pl: "~ittäni", "~ittäsi", "~ittämme", "~ittänne", "~ittäVn|~ittAnsV";
    ),
    // Rita: 174 words (rita, pirkka, pekka, haaparanta, anitta, birgitta). Like Hydra with `ˇ` after each pattern.
    declension!("rita", 174, "~aˇ", like Hydra with "ˇ";
    ),
    // Tulimaa: 161 words (ahvenanmaa, maa, thaimaa, swazimaa, tulimaa, hiidenmaa). Like Salo with `` after each pattern (`a` after ~), except in 39 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("tulimaa", 161, "~a", like Salo with "", "a" after "~";
    nom: _, "~at";
    gen_: _, "~iden|~itten";
    par: "~atV", "~itA";
    ine: "~assV", _;
    ela: "~astV", _;
    ill: "~ahVn", _;
    ade: "~allV", _;
    abl: "~altV", _;
    all: "~alle", _;
    ess: "~anV", _;
    tra: "~aksi", _;
    abe: "~attV", _;
    proper_noun_possessive_nom_sg: "~ani", _, _, _, "~ansV";
    proper_noun_possessive_nom_pl: _, "~asi", "~amme", "~anne", _;
    proper_noun_possessive_gen_sg: _, "~asi", "~amme", "~anne", _;
    proper_noun_possessive_gen_pl: "~ideni|~itteni", "~idesi|~ittesi", "~idemme|~ittemme", "~idenne|~ittenne", "~idensA|~ittensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~atVVn|~atVnsV";
    proper_noun_possessive_par_pl: "~itAni", _, _, _, "~itAVn|~itAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~assVVn|~assVnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~astVVn|~astVnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~ahVnsV";
    proper_noun_possessive_ill_pl: "~ihini", _, _, _, _;
    proper_noun_possessive_ade_sg: _, _, _, _, "~allVVn|~allVnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~altVVn|~altVnsV";
    proper_noun_possessive_all_sg: _, _, _, _, "~alleen|~allensA";
    proper_noun_possessive_ess_sg: _, _, _, _, "~anVVn|~anVnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, "~akseen|~aksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, "~attVVn|~attVnsV";
    ),
    // Hietaniemi: 158 words (hyväntoivonniemi, rovaniemi, kirkkoniemi, koivuniemi, kangasniemi, myllyniemi).
    declension!("hietaniemi", 158, "~mi";
    stem gen_sg: "~mV- like hietaniemi";
    stem par_pl: "~- like hietaniemi";
    stem ine_sg: "~m- like hietaniemi";
    stem ill_sg: "~meVn- like hietaniemi";
    stem ade_sg: "~mVllä- like makedonia";
    stem abl_sg: "~mVltä- like makedonia";
    stem all_sg: "~mVlle- like hietaniemi";
    stem ess_sg: "~m- like hietaniemi";
    stem tra_sg: "~mVks- like hietaniemi";
    nom: "~mi", _;
    par: "~ntä|~meä", _;
    proper_noun_possessive_gen_sg: _, "~mesi", "~memme", "~menne", _;
    proper_noun_possessive_par_sg: _, _, _, _, "~ntäVn|~meäVn|~ntänsä|~meänsä";
    ),
    // Lahti: 146 words (lahti, suomenlahti, pohjanlahti, persianlahti, meksikonlahti, hudsoninlahti). Like Salmi with `ˇ` after each pattern.
    declension!("lahti", 146, "~iˇ", like Salmi with "ˇ";
    ),
    // Borneo: 120 words (borneo, kuopio, tapio, ensio, päiviö, kallio). Like Salo, except in 2 cells: proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3.
    declension!("borneo", 120, "~", like Salo;
    stem par_pl: "~- like tapio";
    stem par_sg: "~tA- like ilmarinen";
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    ),
    // Saari: 113 words (mansaari, eteläsaari, pohjoissaari, nuoli, vesisaari, pöytävuori). Like Borneo with `` after each pattern (`i` after ~), except in 39 cells: nom pl, gen_ pl, par pl, ine sg, ela sg, ill sg, ….
    declension!("saari", 113, "~i", like Borneo with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ten|~ien";
    par: _, "~iA";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~eVn", "~iin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    all: "~elle", _;
    ess: "~enA", _;
    tra: "~eksi", _;
    abe: "~ettA", _;
    proper_noun_possessive_nom_sg: "~eni", _, _, _, "~ensA";
    proper_noun_possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensA|~iensA";
    proper_noun_possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~eVnsA";
    proper_noun_possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    proper_noun_possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    proper_noun_possessive_ess_sg: _, _, _, _, "~enAVn|~enAnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    ),
    // Härmä: 109 words (venäjä, isä, valko-venäjä, jyväskylä, kuvanveistäjä, ryssä). Like Pyhäjärvi, except in 22 cells: nom sg, ine sg, proper_noun_possessive_nom_sg _1sg, proper_noun_possessive_nom_sg _3, proper_noun_possessive_nom_pl _2sg, proper_noun_possessive_nom_pl _1pl, ….
    declension!("härmä", 109, "~ä", like Pyhäjärvi;
    stem gen_sg: "~ä- like makedonia";
    stem par_sg: "~äV- like makedonia";
    stem ill_sg: "~äVn- like makedonia";
    stem ade_sg: "~ällV- like makedonia";
    stem abl_sg: "~ältV- like makedonia";
    stem all_sg: "~älle- like makedonia";
    stem ess_sg: "~änV- like makedonia";
    stem tra_sg: "~äks- like makedonia";
    nom: "~ä", _;
    ine: "~ässV", _;
    proper_noun_possessive_nom_sg: "~äni", _, _, _, "~änsV";
    proper_noun_possessive_nom_pl: _, "~äsi", "~ämme", "~änne", derived;
    proper_noun_possessive_gen_sg: _, "~äsi", "~ämme", "~änne", _;
    proper_noun_possessive_par_pl: _, _, _, _, "~iäVn|~iänsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~ässVVn|~ässVnsV";
    proper_noun_possessive_ine_pl: _, _, _, _, "~issäVn|~issänsV";
    proper_noun_possessive_ela_pl: _, _, _, _, "~istäVn|~istänsV";
    proper_noun_possessive_ade_pl: _, _, _, _, "~illäVn|~illänsV";
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltäVn|~iltänsV";
    proper_noun_possessive_all_pl: _, _, _, _, "~illeen|~illensä";
    proper_noun_possessive_ess_pl: _, _, _, _, "~inäVn|~inänsV";
    proper_noun_possessive_tra_pl: _, _, _, _, "~ikseen|~iksensä";
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittäVn|~ittänsV";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen|~inensä";
    ),
    // Salmi: 106 words (suomi, salmi, kaukoputki, vuoksi, talvi, nurmi). Like Salo with `` after each pattern (`i` after ~), except in 41 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("salmi", 106, "~i", like Salo with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ien";
    par: "~eA", "~iA";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~eVn", "~iin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    all: "~elle", _;
    ess: "~enA", _;
    tra: "~eksi", _;
    abe: "~ettA", _;
    proper_noun_possessive_nom_sg: "~eni", _, _, _, "~ensA";
    proper_noun_possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_pl: "~ieni", "~iesi", "~iemme", "~ienne", "~iensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~eAVn|~eAnsV";
    proper_noun_possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~eVnsA";
    proper_noun_possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    proper_noun_possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    proper_noun_possessive_ess_sg: _, _, _, _, "~enAVn|~enAnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    ),
    // Nutikka: 74 words (amerikka, riika, rebekka, annukka, marjukka, henriikka). Like Makedonia, except in 6 cells: nom sg, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_gen_sg _2sg, proper_noun_possessive_gen_sg _1pl, proper_noun_possessive_gen_sg _2pl.
    declension!("nutikka", 74, "~ka", like Makedonia;
    stem par_pl: "~- like nutikka";
    stem par_sg: "~kaV- like makedonia";
    stem ill_sg: "~kaVn- like makedonia";
    stem ess_sg: "~kanV- like makedonia";
    nom: "~ka", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~kasi", "~kamme", "~kanne", _;
    ),
    // Opas: 70 words (opas, kangas, saarikangas, isokangas, porras, haapakangas). Like Oinas with `ˇ` after each pattern.
    declension!("opas", 70, "~sˇ", like Oinas with "ˇ";
    ),
    // Lotta: 58 words (jukka, lotta, jutta, kolkka, mutta, kuikka). Like Vehmaa with `ˇ` after each pattern, except in 33 cells: gen_ pl, par sg, par pl, ill sg, ill pl, proper_noun_possessive_gen_pl _1sg, ….
    declension!("lotta", 58, "~aˇ", like Vehmaa with "ˇ";
    gen_: _, "~ienˇ";
    par: "~aVˇ", "~iAˇ";
    ill: "~aVnˇ", "~iinˇ";
    proper_noun_possessive_gen_pl: "~ieniˇ", "~iesiˇ", "~iemmeˇ", "~ienneˇ", "~iensAˇ";
    proper_noun_possessive_par_sg: _, _, _, _, "~aVnsVˇ";
    proper_noun_possessive_par_pl: "~iAniˇ", _, _, _, "~iAVnˇ|~iAnsVˇ";
    proper_noun_possessive_ine_sg: _, _, _, _, "~assVVnˇ|~assVnsVˇ";
    proper_noun_possessive_ine_pl: _, _, _, _, "~issAVnˇ|~issAnsVˇ";
    proper_noun_possessive_ela_sg: _, _, _, _, "~astVVnˇ|~astVnsVˇ";
    proper_noun_possessive_ela_pl: _, _, _, _, "~istAVnˇ|~istAnsVˇ";
    proper_noun_possessive_ill_sg: _, _, _, _, "~aVnsVˇ";
    proper_noun_possessive_ill_pl: "~iiniˇ", _, _, _, "~iinsAˇ";
    proper_noun_possessive_ade_sg: _, _, _, _, "~allVVnˇ|~allVnsVˇ";
    proper_noun_possessive_ade_pl: _, _, _, _, "~illAVnˇ|~illAnsVˇ";
    proper_noun_possessive_abl_sg: _, _, _, _, "~altVVnˇ|~altVnsVˇ";
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltAVnˇ|~iltAnsVˇ";
    proper_noun_possessive_all_sg: _, _, _, _, "~alleenˇ|~allensAˇ";
    proper_noun_possessive_all_pl: _, _, _, _, "~illeenˇ|~illensAˇ";
    proper_noun_possessive_ess_sg: _, _, _, _, "~anVVnˇ|~anVnsVˇ";
    proper_noun_possessive_ess_pl: _, _, _, _, "~inAVnˇ|~inAnsVˇ";
    proper_noun_possessive_tra_sg: _, _, _, _, "~akseenˇ|~aksensAˇ";
    proper_noun_possessive_tra_pl: _, _, _, _, "~ikseenˇ|~iksensAˇ";
    proper_noun_possessive_abe_sg: _, _, _, _, "~attVVnˇ|~attVnsVˇ";
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittAVnˇ|~ittAnsVˇ";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineenˇ|~inensAˇ";
    ),
    // Kakskerta: 57 words (itävalta, eurooppa, kreikka, etelä-afrikka, pohjois-amerikka, etelä-amerikka). Like Makedonia with `ˇ` after each pattern.
    declension!("kakskerta", 57, "~aˇ", like Makedonia with "ˇ";
    ),
    // Järvenpää: 46 words (mäenpää, järvenpää, tähkäpää, sillanpää, kankaanpää, pyhtää). Like Roine with `` after each pattern (`ä` after ~), except in 35 cells: nom pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("järvenpää", 46, "~ä", like Roine with "", "ä" after "~";
    nom: _, "~ät";
    par: "~ätV", _;
    ine: "~ässV", _;
    ela: "~ästV", _;
    ill: "~ähVn", "~ihin";
    ade: "~ällV", _;
    abl: "~ältV", _;
    all: "~älle", _;
    ess: "~änV", _;
    tra: "~äksi", _;
    abe: "~ättV", _;
    proper_noun_possessive_nom_sg: "~äni", _, _, _, "~änsV";
    proper_noun_possessive_nom_pl: _, "~äsi", "~ämme", "~änne", _;
    proper_noun_possessive_gen_sg: _, "~äsi", "~ämme", "~änne", _;
    proper_noun_possessive_par_sg: _, _, _, _, "~ätVVn|~ätVnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~ässVVn|~ässVnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~ästVVn|~ästVnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~ähVnsV";
    proper_noun_possessive_ill_pl: "~ihini", derived, derived, derived, "~ihinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~ällVVn|~ällVnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~ältVVn|~ältVnsV";
    proper_noun_possessive_all_sg: _, _, _, _, "~älleen|~ällensA";
    proper_noun_possessive_ess_sg: _, _, _, _, "~änVVn|~änVnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, "~äkseen|~äksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, "~ättVVn|~ättVnsV";
    ),
    // Jäämeri: 43 words (kaspianmeri, mustameri, punainenmeri, vienanmeri, itämeri, pohjanmeri). Like Salmi, except in 21 cells: ine sg, proper_noun_possessive_nom_sg _1sg, proper_noun_possessive_nom_sg _3, proper_noun_possessive_nom_pl _2sg, proper_noun_possessive_nom_pl _1pl, proper_noun_possessive_nom_pl _2pl, ….
    declension!("jäämeri", 43, "~i", like Salmi;
    stem gen_sg: "~V- like hietaniemi";
    stem par_sg: "~ta- like ilmarinen";
    stem ill_sg: "~VVn- like hietaniemi";
    stem ade_sg: "~Vllä- like makedonia";
    stem abl_sg: "~Vltä- like makedonia";
    stem all_sg: "~Vlle- like hietaniemi";
    stem ess_sg: "~Vnä- like makedonia";
    stem tra_sg: "~Vks- like hietaniemi";
    ine: "~Vssä", _;
    proper_noun_possessive_nom_sg: "~Vni", _, _, _, "~Vnsä";
    proper_noun_possessive_nom_pl: _, "~Vsi", "~Vmme", "~Vnne", "~VnsA";
    proper_noun_possessive_gen_sg: _, "~Vsi", "~Vmme", "~Vnne", _;
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~VssäVn|~VssänsV";
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~istAVn";
    proper_noun_possessive_ade_pl: _, _, _, _, "~illAVn";
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltAVn";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen";
    ),
    // Aaron: 41 words (jupiter, daniel, aaron, simon, boston, erin). Like Luxemburg, except in 2 cells: par pl, proper_noun_possessive_par_pl _1sg.
    declension!("aaron", 41, "~", like Luxemburg;
    par: _, "~eitA";
    proper_noun_possessive_par_pl: "~eitAni", _, _, _, _;
    ),
    // Roine: 32 words (tampere, joutsen, laine, purje, helle, halme). Like Tulimaa, except in 8 cells: nom sg, ine sg, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_gen_sg _2sg, proper_noun_possessive_gen_sg _1pl, ….
    declension!("roine", 32, "~", like Tulimaa;
    stem gen_sg: "~V- like ilmarinen";
    stem par_pl: "~- like roine";
    stem par_sg: "~ttA- like ilmarinen";
    stem ill_sg: "~eseen- like ilmarinen";
    stem ade_sg: "~VllA- like makedonia";
    stem abl_sg: "~VltA- like makedonia";
    stem all_sg: "~Vlle- like makedonia";
    stem ess_sg: "~VnA- like makedonia";
    stem tra_sg: "~Vks- like makedonia";
    nom: "~", _;
    ine: "~VssA", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~Vsi", "~Vmme", "~Vnne", _;
    proper_noun_possessive_ine_sg: _, _, _, _, "~VssAVn|~VssAnsV";
    ),
    // Kärppä: 31 words (kärppä, pyhäselkä, maanselkä, suomenselkä, pyhäntä, salpausselkä). Like Härmä with `ˇ` after each pattern.
    declension!("kärppä", 31, "~äˇ", like Härmä with "ˇ";
    ),
    // Kaksoset: 28 words (färsaaret, kalliovuoret, huippuvuoret, kaksoset, seulaset, atlasvuoret). Like Saari, except in 32 cells: nom sg, par sg, par pl, ine sg, ela sg, ill sg, ….
    declension!("kaksoset", 28, "~et", like Saari;
    nom: "~et", _;
    par: derived, "~ia";
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    all: derived, _;
    ess: derived, _;
    tra: derived, _;
    abe: derived, _;
    proper_noun_possessive_nom_sg: derived, _, _, _, derived;
    proper_noun_possessive_nom_pl: "~eni", _, _, _, "~ensA";
    proper_noun_possessive_gen_sg: _, derived, derived, derived, _;
    proper_noun_possessive_par_sg: _, _, _, _, derived;
    proper_noun_possessive_par_pl: "~iani", _, _, _, "~iaan|~iAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, derived;
    proper_noun_possessive_ill_sg: _, _, _, _, derived;
    proper_noun_possessive_ill_pl: _, _, _, _, "~iinsa";
    proper_noun_possessive_ade_sg: _, _, _, _, derived;
    proper_noun_possessive_abl_sg: _, _, _, _, derived;
    proper_noun_possessive_all_sg: _, _, _, _, derived;
    proper_noun_possessive_ess_sg: _, _, _, _, derived;
    proper_noun_possessive_tra_sg: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, derived;
    ),
    // Häme: 26 words (käärme, pere, lohikäärme, vesikäärme, häme, filippiläiskirje). Like Härmä, except in 32 cells: nom sg, gen_ pl, par pl, ine sg, ill pl, proper_noun_possessive_nom_sg _1sg, ….
    declension!("häme", 26, "~", like Härmä;
    stem gen_sg: "~V- like hietaniemi";
    stem par_sg: "~ttä- like ilmarinen";
    stem ill_sg: "~eseen- like hietaniemi";
    stem ade_sg: "~Vllä- like makedonia";
    stem abl_sg: "~Vltä- like makedonia";
    stem all_sg: "~Vlle- like hietaniemi";
    stem ess_sg: "~Vnä- like makedonia";
    stem tra_sg: "~Vks- like hietaniemi";
    nom: "~", _;
    gen_: _, "~iden";
    par: _, "~itä";
    ine: "~Vssä", _;
    ill: _, "~isVin";
    proper_noun_possessive_nom_sg: "~Vni", _, _, _, "~Vnsä";
    proper_noun_possessive_nom_pl: _, "~Vsi", "~Vmme", "~Vnne", _;
    proper_noun_possessive_gen_sg: _, "~Vsi", "~Vmme", "~Vnne", _;
    proper_noun_possessive_gen_pl: "~ideni", "~idesi", "~idemme", "~idenne", "~idensä";
    proper_noun_possessive_par_pl: "~itäni", _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~VssäVn|~VssänsV";
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~istäVn";
    proper_noun_possessive_ill_pl: "~isVini", _, _, _, "~isVinsä";
    proper_noun_possessive_ade_pl: _, _, _, _, "~illäVn";
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltäVn";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittäVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen";
    ),
    // Oinas: 23 words (valas, oinas, pythagoras, tursas, jeremias, barnabas). Like Roine with `` after each pattern (`s` after ~), except in 39 cells: gen_ pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("oinas", 23, "~s", like Roine with "", "s" after "~";
    gen_: _, "~iden";
    par: "~stV", _;
    ine: "~VssV", _;
    ela: "~VstV", _;
    ill: "~VseVn", "~isVin";
    ade: "~VllV", _;
    abl: "~VltV", _;
    ess: "~VnV", _;
    abe: "~VttV", _;
    proper_noun_possessive_nom_sg: _, _, _, _, "~VnsV";
    proper_noun_possessive_gen_pl: "~ideni", "~idesi", "~idemme", "~idenne", "~idensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~stVVn|~stVnsV";
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~VssVVn|~VssVnsV";
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, "~VstVVn|~VstVnsV";
    proper_noun_possessive_ela_pl: _, _, _, _, "~istAVn";
    proper_noun_possessive_ill_sg: _, _, _, _, "~VseVnsA";
    proper_noun_possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~VllVVn|~VllVnsV";
    proper_noun_possessive_ade_pl: _, _, _, _, "~illAVn";
    proper_noun_possessive_abl_sg: _, _, _, _, "~VltVVn|~VltVnsV";
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltAVn";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_sg: _, _, _, _, "~VnVVn|~VnVnsV";
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, "~VttVVn|~VttVnsV";
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen";
    ),
    // Plejadit: 20 words (andit, alpit, skandit, molukit, plejadit, seychellit). Like Luxemburg with `` after each pattern (`it` after ~), except in 37 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("plejadit", 20, "~it", like Luxemburg with "", "it" after "~";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    all: derived, _;
    ess: derived, _;
    tra: derived, _;
    abe: derived, _;
    proper_noun_possessive_nom_sg: derived, _, _, _, derived;
    proper_noun_possessive_nom_pl: "~ini", _, _, _, "~insA";
    proper_noun_possessive_gen_sg: _, derived, derived, derived, _;
    proper_noun_possessive_par_sg: _, _, _, _, derived;
    proper_noun_possessive_par_pl: _, _, _, _, "~ejAVn|~ejAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, "~eissAVn|~eissAnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~eistAVn|~eistAnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, derived;
    proper_noun_possessive_ade_sg: _, _, _, _, derived;
    proper_noun_possessive_ade_pl: _, _, _, _, "~eillAVn|~eillAnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, derived;
    proper_noun_possessive_abl_pl: _, _, _, _, "~eiltAVn|~eiltAnsV";
    proper_noun_possessive_all_sg: _, _, _, _, derived;
    proper_noun_possessive_all_pl: _, _, _, _, "~eilleen|~eillensA";
    proper_noun_possessive_ess_sg: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, "~einAVn|~einAnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~eittAVn|~eittAnsV";
    proper_noun_possessive_com_pl: _, _, _, _, "~eineen|~einensA";
    ),
    // Luukas: 19 words (luukas, ahokas, laukas, asukas, äikäs, kansikas). Like Johannes, except in 30 cells: nom sg, nom pl, ine sg, ela sg, ade sg, abl sg, ….
    declension!("luukas", 19, "~~s", like Johannes;
    stem par_pl: "~k~- like luukas";
    stem par_sg: "~~stV- like ilmarinen";
    stem ill_sg: "~k~Vseen- like ilmarinen";
    nom: "~~s", "~k~Vt";
    ine: "~k~VssV", _;
    ela: "~k~VstV", _;
    ade: "~k~VllV", _;
    abl: "~k~VltV", _;
    all: "~k~Vlle", _;
    ess: "~k~VnV", _;
    tra: "~k~Vksi", _;
    abe: "~k~VttV", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~k~Vsi", "~k~Vmme", "~k~Vnne", _;
    proper_noun_possessive_ine_sg: _, _, _, _, "~k~VssVVn|~k~assansa";
    proper_noun_possessive_ine_pl: _, _, _, _, "~k~issAVn|~k~issansa";
    proper_noun_possessive_ela_sg: _, _, _, _, "~k~VstVVn|~k~astansa";
    proper_noun_possessive_ela_pl: _, _, _, _, "~k~istAVn|~k~istansa";
    proper_noun_possessive_ade_sg: _, _, _, _, "~k~VllVVn|~k~allansa";
    proper_noun_possessive_ade_pl: _, _, _, _, "~k~illAVn|~k~illansa";
    proper_noun_possessive_abl_sg: _, _, _, _, "~k~VltVVn|~k~altansa";
    proper_noun_possessive_abl_pl: _, _, _, _, "~k~iltAVn|~k~iltansa";
    proper_noun_possessive_all_sg: _, _, _, _, "~k~Vlleen|~k~allensa";
    proper_noun_possessive_all_pl: _, _, _, _, "~k~illeen|~k~illensa";
    proper_noun_possessive_ess_sg: _, _, _, _, "~k~VnVVn|~k~anansa";
    proper_noun_possessive_tra_sg: _, _, _, _, "~k~Vkseen|~k~aksensa";
    proper_noun_possessive_tra_pl: _, _, _, _, "~k~ikseen|~k~iksensa";
    proper_noun_possessive_abe_sg: _, _, _, _, "~k~VttVVn|~k~attansa";
    proper_noun_possessive_abe_pl: _, _, _, _, "~k~ittAVn|~k~ittansa";
    ),
    // Rinne: 18 words (rinne, päijänne, selänne, alanne, ylänne, kaarre). Like Roine with `ˇ` after each pattern.
    declension!("rinne", 18, "~ˇ", like Roine with "ˇ";
    ),
    // Orivesi: 17 words (haapavesi, kuorevesi, orivesi, ruovesi, kiuruvesi, heinävesi). Like Ilmarinen, except in 73 cells: nom sg, nom pl, gen_ pl, par pl, ine sg, ine pl, ….
    declension!("orivesi", 17, "~si", like Ilmarinen;
    stem par_sg: "~ttä- like ilmarinen";
    stem ill_sg: "~teVn- like hietaniemi";
    stem ess_sg: "~tenä- like makedonia";
    nom: "~si", "~dVt";
    gen_: _, "~sien";
    par: _, "~siä";
    ine: "~dVssä", "~sissä";
    ela: "~dVstä", "~sistä";
    ade: "~dVllä", "~sillä";
    abl: "~dVltä", "~siltä";
    all: "~dVlle", _;
    ess: _, "~sinä";
    tra: "~dVksi", _;
    abe: "~dVttä", "~sittä";
    proper_noun_possessive_nom_sg: "~teni", _, _, _, "~tensä";
    proper_noun_possessive_nom_pl: _, "~tesi", "~temme", "~tenne", _;
    proper_noun_possessive_gen_sg: _, "~tesi", "~temme", "~tenne", _;
    proper_noun_possessive_gen_pl: "~sieni", "~siesi", "~siemme", "~sienne", "~siensä";
    proper_noun_possessive_par_pl: "~siäni", _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~dVssäVn|~dessänsV";
    proper_noun_possessive_ine_pl: "~sissäni", "~sissäsi", "~sissämme", "~sissänne", derived;
    proper_noun_possessive_ela_sg: _, _, _, _, "~dVstäVn|~destänsV";
    proper_noun_possessive_ela_pl: "~sistäni", "~sistäsi", "~sistämme", "~sistänne", "~sistäVn";
    proper_noun_possessive_ill_pl: _, _, _, _, "~siinsä";
    proper_noun_possessive_ade_sg: _, _, _, _, "~dVlläVn|~dellänsV";
    proper_noun_possessive_ade_pl: "~silläni", "~silläsi", "~sillämme", "~sillänne", "~silläVn";
    proper_noun_possessive_abl_sg: _, _, _, _, "~dVltäVn|~deltänsV";
    proper_noun_possessive_abl_pl: "~siltäni", "~siltäsi", "~siltämme", "~siltänne", "~siltäVn";
    proper_noun_possessive_all_sg: _, _, _, _, "~dVlleen|~dellensä";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: "~sinäni", "~sinäsi", "~sinämme", "~sinänne", derived;
    proper_noun_possessive_tra_sg: _, _, _, _, "~dVkseen|~deksensä";
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, "~dVttäVn|~dettänsV";
    proper_noun_possessive_abe_pl: "~sittäni", "~sittäsi", "~sittämme", "~sittänne", "~sittäVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~sineen";
    ),
    // Vesimies: 14 words (mies, teräsmies, vesimies, ajomies, jousimies, häkämies).
    declension!("vesimies", 14, "~s";
    stem gen_sg: "~he- like hietaniemi";
    stem par_sg: "~stä- like ilmarinen";
    stem ine_sg: "~h- like vesimies";
    stem ill_sg: "~hVVn- like hietaniemi";
    stem ade_sg: "~hellä- like makedonia";
    stem abl_sg: "~heltä- like makedonia";
    stem all_sg: "~helle- like hietaniemi";
    stem ess_sg: "~henä- like makedonia";
    stem tra_sg: "~heks- like hietaniemi";
    nom: "~s", _;
    gen_: _, "~sten|~hien";
    par: _, "~hiä";
    ill: _, "~hiin";
    ess: _, "~hinä";
    proper_noun_possessive_nom_sg: "~heni", _, _, _, "~hensä";
    proper_noun_possessive_nom_pl: _, "~hesi", "~hemme", "~henne", _;
    proper_noun_possessive_gen_sg: _, "~hesi", "~hemme", "~henne", _;
    proper_noun_possessive_gen_pl: "~steni|~hieni", "~stesi|~hiesi", "~stemme|~hiemme", "~stenne|~hienne", "~stensä|~hiensä";
    proper_noun_possessive_par_pl: "~hiäni", _, _, _, "~hiäVn|~hiänsV";
    proper_noun_possessive_ill_pl: "~hiini", _, _, _, "~hiinsä";
    proper_noun_possessive_ess_pl: "~hinäni", "~hinäsi", "~hinämme", "~hinänne", "~hinäVn|~hinänsV";
    proper_noun_possessive_com_pl: "~hineni", _, _, _, "~hineen|~hinensä";
    ),
    // Vehmaa: 14 words (saimaa, vantaa, loimaa, karjaa, akaa, vehmaa). Like Tulimaa, except in 27 cells: gen_ pl, ill pl, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, proper_noun_possessive_gen_pl _1pl, proper_noun_possessive_gen_pl _2pl, ….
    declension!("vehmaa", 14, "~a", like Tulimaa;
    stem par_sg: "~atV- like vehmaa";
    stem ill_sg: "~aseVn- like ilmarinen";
    gen_: _, "~iden";
    ill: _, "~isVin";
    proper_noun_possessive_gen_pl: "~ideni", "~idesi", "~idemme", "~idenne", "~idensA";
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~istAVn";
    proper_noun_possessive_ill_pl: "~isVVni", _, _, _, "~isVinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, derived;
    proper_noun_possessive_ade_pl: _, _, _, _, "~illAVn";
    proper_noun_possessive_abl_sg: _, _, _, _, derived;
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltAVn";
    proper_noun_possessive_all_sg: _, _, _, _, derived;
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_sg: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_sg: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen";
    ),
    // Joensuu: 13 words (kuu, joensuu, chișinău, pielisensuu, ojansuu, kujansuu). Like Vehmaa, except in 18 cells: nom sg, ine sg, ill pl, ade sg, abl sg, all sg, ….
    declension!("joensuu", 13, "~u", like Vehmaa;
    stem gen_sg: "~u- like joensuu";
    stem par_sg: "~utA- like vehmaa";
    stem ill_sg: "~uhVn- like ilmarinen";
    nom: "~u", _;
    ine: "~ussA", _;
    ill: _, "~ihin";
    ade: "~ullA", _;
    abl: "~ultA", _;
    all: "~ulle", _;
    ess: "~unA", _;
    tra: "~uksi", _;
    proper_noun_possessive_nom_sg: "~uni", _, _, _, "~unsA";
    proper_noun_possessive_nom_pl: _, "~usi", "~umme", "~unne", _;
    proper_noun_possessive_gen_sg: _, "~usi", "~umme", "~unne", _;
    proper_noun_possessive_ill_pl: "~ihini", _, _, _, "~ihinsA";
    ),
    // Ahava: 12 words (joanna, juhana, ilona, mimosa, jesaja, ahava). Like Makedonia, except in 29 cells: gen_ pl, par pl, ill pl, ess pl, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, ….
    declension!("ahava", 12, "~a", like Makedonia;
    stem ine_sg: "~- like ahava";
    gen_: _, "~ien|~oiden|~oitten";
    par: _, "~iA|~oitA";
    ill: _, "~iin|~oihin";
    ess: _, "~oinA|~inA";
    proper_noun_possessive_gen_pl: "~ieni|~oideni|~oitteni", "~iesi|~oidesi|~oittesi", "~iemme|~oidemme|~oittemme", "~ienne|~oidenne|~oittenne", "~iensA|~oidensA|~oittensA";
    proper_noun_possessive_par_pl: "~iAni|~oitAni", "~iAsi|~oitAsi", "~iAmme|~oitAmme", "~iAnne|~oitAnne", "~iAVn|~oitAVn|~iAnsV|~oitAnsV";
    proper_noun_possessive_ill_pl: "~iini|~oihini", "~iisi|~oihisi", "~iimme|~oihimme", "~iinne|~oihinne", "~iinsA|~oihinsA";
    proper_noun_possessive_ess_pl: "~oinAni|~inAni", "~oinAsi|~inAsi", "~oinAmme|~inAmme", "~oinAnne|~inAnne", "~oinAVn|~inAVn|~oinAnsV|~inansa";
    proper_noun_possessive_com_pl: "~oineni|~ineni", "~oinesi|~inesi", "~oinemme|~inemme", "~oinenne|~inenne", "~oineen|~ineen|~oinensA|~inensa";
    ),
    // Altai: 11 words (paraguay, uruguay, brunei, mumbai, kai, aleksei). Like Salo, except in 29 cells: gen_ pl, par pl, ela sg, ill pl, ess pl, abe sg, ….
    declension!("altai", 11, "~", like Salo;
    stem par_sg: "~ta- like ilmarinen";
    stem ine_sg: "~- like altai";
    stem ill_sg: "~hin- like altai";
    stem ade_sg: "~lla- like makedonia";
    stem abl_sg: "~lta- like makedonia";
    stem all_sg: "~lle- like altai";
    stem ess_sg: "~na- like makedonia";
    stem tra_sg: "~ks- like altai";
    gen_: _, "~den";
    par: _, "~tA";
    ela: "~sta", _;
    ill: _, "~hin";
    ess: _, "~nA";
    abe: "~tta", _;
    proper_noun_possessive_nom_sg: _, _, _, _, "~nsa";
    proper_noun_possessive_nom_pl: _, _, _, _, "~nsA";
    proper_noun_possessive_gen_pl: "~deni", "~desi", "~demme", "~denne", "~densA";
    proper_noun_possessive_par_pl: "~tAni", _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, "~staVn|~stansV";
    proper_noun_possessive_ill_pl: "~hini", _, _, _, "~hinsA";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: "~nAni", "~nAsi", "~nAmme", "~nAnne", derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, "~ttaVn|~ttansV";
    proper_noun_possessive_com_pl: "~neni", _, _, _, "~neen";
    ),
    // Heinisuo: 9 words (kallasvuo, salmensuo, heinisuo, lahdensuo, ruskeasuo, honkasuo).
    declension!("heinisuo", 9, "~u~";
    stem ill_sg: "~u~hon- like altai";
    nom: "~u~", "~u~t";
    gen_: _, "~~iden";
    par: "~u~ta", "~~itA";
    ine: "~u~ssa", "~~issA";
    ela: "~u~sta", "~~istA";
    ill: _, "~~ihin";
    ade: "~u~lla", "~~illA";
    abl: "~u~lta", "~~iltA";
    all: "~u~lle", "~~ille";
    ess: "~u~na", "~~inA";
    tra: "~u~ksi", _;
    abe: "~u~tta", "~~ittA";
    proper_noun_possessive_nom_sg: "~u~ni", _, _, _, "~u~nsa";
    proper_noun_possessive_nom_pl: _, "~u~si", "~u~mme", "~u~nne", _;
    proper_noun_possessive_gen_sg: _, "~u~si", "~u~mme", "~u~nne", _;
    proper_noun_possessive_gen_pl: "~~ideni", "~~idesi", "~~idemme", "~~idenne", "~~idensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~u~taan";
    proper_noun_possessive_par_pl: "~~itAni", _, _, _, _;
    proper_noun_possessive_ine_pl: "~~issAni", "~~issAsi", "~~issAmme", "~~issAnne", _;
    proper_noun_possessive_ela_pl: "~~istAni", "~~istAsi", "~~istAmme", "~~istAnne", "~~istAVn";
    proper_noun_possessive_ill_pl: "~~ihini", _, _, _, "~~ihinsA";
    proper_noun_possessive_ade_pl: "~~illAni", "~~illAsi", "~~illAmme", "~~illAnne", "~~illAVn";
    proper_noun_possessive_abl_pl: "~~iltAni", "~~iltAsi", "~~iltAmme", "~~iltAnne", "~~iltAVn";
    proper_noun_possessive_all_pl: "~~illeni", "~~illesi", "~~illemme", "~~illenne", _;
    proper_noun_possessive_ess_pl: "~~inAni", "~~inAsi", "~~inAmme", "~~inAnne", _;
    proper_noun_possessive_tra_pl: "~~ikseni", "~~iksesi", "~~iksemme", "~~iksenne", _;
    proper_noun_possessive_abe_pl: "~~ittAni", "~~ittAsi", "~~ittAmme", "~~ittAnne", "~~ittAVn";
    proper_noun_possessive_com_pl: "~~ineni", _, _, _, "~~ineen";
    ),
    // Delfoi: 9 words (delfoi, jenisei, filippoi, petroskoi, tokoi, keto-tokoi). Like Altai, except in 5 cells: ine sg, proper_noun_possessive_nom_sg _3, proper_noun_possessive_nom_pl _3, proper_noun_possessive_ine_sg _3, proper_noun_possessive_ill_pl _1sg.
    declension!("delfoi", 9, "~", like Altai;
    stem gen_sg: "~- like ilmarinen";
    stem par_sg: "~tA- like ilmarinen";
    stem ill_sg: "~hin- like ilmarinen";
    stem ade_sg: "~llA- like makedonia";
    stem abl_sg: "~ltA- like makedonia";
    stem all_sg: "~lle- like makedonia";
    stem ess_sg: "~nA- like makedonia";
    stem tra_sg: "~ks- like makedonia";
    ine: "~ssA", _;
    proper_noun_possessive_nom_sg: _, _, _, _, "~nsA";
    proper_noun_possessive_nom_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~ssAVn|~ssAnsV";
    proper_noun_possessive_ill_pl: "~seVni", _, _, _, _;
    ),
    // Okko: 9 words (turku, kouki, kurko, haiko, pitko, louko). Like Heinisuo, except in 6 cells: nom sg, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_gen_sg _2sg, proper_noun_possessive_gen_sg _1pl, proper_noun_possessive_gen_sg _2pl.
    declension!("okko", 9, "~k~", like Heinisuo;
    stem gen_sg: "~~- like ilmarinen";
    stem par_pl: "~k~- like okko";
    stem par_sg: "~k~A- like ilmarinen";
    stem ine_sg: "~~- like salo";
    stem ill_sg: "~k~Vn- like ilmarinen";
    stem ade_sg: "~~llA- like makedonia";
    stem abl_sg: "~~ltA- like makedonia";
    stem all_sg: "~~lle- like makedonia";
    stem ess_sg: "~k~nA- like makedonia";
    stem tra_sg: "~~ks- like makedonia";
    nom: "~k~", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~k~si", "~k~mme", "~k~nne", _;
    ),
    // Herkules: 8 words (sokrates, aristoteles, herakles, hippokrates, herkules, orestes). Like Roine with `` after each pattern (`s` after ~), except in 26 cells: gen_ pl, par sg, ill sg, ill pl, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, ….
    declension!("herkules", 8, "~s", like Roine with "", "s" after "~";
    gen_: _, "~iden";
    par: "~stA", _;
    ill: "~VsVVn", "~isVin";
    proper_noun_possessive_gen_pl: "~ideni", "~idesi", "~idemme", "~idenne", "~idensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~stAVn|~stAnsV";
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~istAVn";
    proper_noun_possessive_ill_sg: _, _, _, _, "~VsVVnsA";
    proper_noun_possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    proper_noun_possessive_ade_pl: _, _, _, _, "~illAVn";
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltAVn";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen";
    ),
    // Pohjantähti: 7 words (pohjantähti, seitsentähti, aamulehti, lehti, joulutähti, tähti). Like Orivesi, except in 21 cells: nom sg, gen_ pl, par pl, ill pl, ess pl, proper_noun_possessive_gen_pl _1sg, ….
    declension!("pohjantähti", 7, "~ti", like Orivesi;
    stem gen_sg: "~de- like hietaniemi";
    stem par_sg: "~teä- like ilmarinen";
    stem ine_sg: "~d- like vesimies";
    stem ade_sg: "~dellä- like makedonia";
    stem abl_sg: "~deltä- like makedonia";
    stem all_sg: "~delle- like hietaniemi";
    stem tra_sg: "~deks- like hietaniemi";
    nom: "~ti", _;
    gen_: _, "~tien";
    par: _, "~tiä";
    ill: _, "~tiin";
    ess: _, "~tinä";
    proper_noun_possessive_gen_pl: "~tieni", "~tiesi", "~tiemme", "~tienne", "~tiensä";
    proper_noun_possessive_par_pl: "~tiäni", _, _, _, "~tiäVn|~tiänsV";
    proper_noun_possessive_ill_pl: "~tiini", _, _, _, "~tiinsä";
    proper_noun_possessive_ess_pl: "~tinäni", "~tinäsi", "~tinämme", "~tinänne", "~tinäVn|~tinänsV";
    proper_noun_possessive_com_pl: "~tineni", _, _, _, "~tineen|~tinensä";
    ),
    // Komorit: 7 words (komorit, antillit, azorit, kuriilit, helmarit, lattarit). Like Plejadit, except in 12 cells: gen_ pl, par pl, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, proper_noun_possessive_gen_pl _1pl, proper_noun_possessive_gen_pl _2pl, ….
    declension!("komorit", 7, "~it", like Plejadit;
    gen_: _, "~ien|~eiden|~eitten";
    par: _, "~eitA|~eja";
    proper_noun_possessive_gen_pl: "~ieni|~eideni|~eitteni", "~iesi|~eidesi|~eittesi", "~iemme|~eidemme|~eittemme", "~ienne|~eidenne|~eittenne", "~iensA|~eidensA|~eittensa";
    proper_noun_possessive_par_pl: "~eitAni|~ejani", "~eitAsi|~ejasi", "~eitAmme|~ejamme", "~eitAnne|~ejanne", "~eitAVn|~ejAVn|~eitAnsV|~ejansa";
    ),
    // Hölttä: 7 words (määttä, mänttä, kenttä, hölttä, skyttä, röyttä). Like Salo, except in 6 cells: nom sg, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_gen_sg _2sg, proper_noun_possessive_gen_sg _1pl, proper_noun_possessive_gen_sg _2pl.
    declension!("hölttä", 7, "~tä", like Salo;
    stem gen_sg: "~A- like makedonia";
    stem par_pl: "~t- like hölttä";
    stem par_sg: "~täV- like makedonia";
    stem ine_sg: "~- like hölttä";
    stem ill_sg: "~täVn- like makedonia";
    stem ade_sg: "~AllV- like makedonia";
    stem abl_sg: "~AltV- like makedonia";
    stem all_sg: "~Alle- like makedonia";
    stem ess_sg: "~tänV- like makedonia";
    stem tra_sg: "~Aks- like makedonia";
    nom: "~tä", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~täsi", "~tämme", "~tänne", _;
    ),
    // Korppoo: 7 words (espoo, lontoo, porvoo, suur-lontoo, inkoo, sipoo). Like Vehmaa, except in 16 cells: nom sg, ine sg, ade sg, abl sg, all sg, ess sg, ….
    declension!("korppoo", 7, "~o", like Vehmaa;
    stem gen_sg: "~o- like joensuu";
    stem par_sg: "~otA- like vehmaa";
    stem ill_sg: "~oseVn- like ilmarinen";
    nom: "~o", _;
    ine: "~ossA", _;
    ade: "~ollA", _;
    abl: "~oltA", _;
    all: "~olle", _;
    ess: "~onA", _;
    tra: "~oksi", _;
    proper_noun_possessive_nom_sg: "~oni", _, _, _, "~onsA";
    proper_noun_possessive_nom_pl: _, "~osi", "~omme", "~onne", "~onsE";
    proper_noun_possessive_gen_sg: _, "~osi", "~omme", "~onne", _;
    ),
    // Kaukomieli: 7 words (kaukomieli, vieri, kallivieri, ääri, kangasvieri, tienvieri). Like Pyhäjärvi, except in 11 cells: gen_ pl, proper_noun_possessive_nom_sg _3, proper_noun_possessive_nom_pl _3, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, proper_noun_possessive_gen_pl _1pl, ….
    declension!("kaukomieli", 7, "~i", like Pyhäjärvi;
    stem gen_sg: "~e- like hietaniemi";
    stem par_sg: "~tä- like ilmarinen";
    stem ine_sg: "~- like vesimies";
    stem ill_sg: "~eVn- like hietaniemi";
    stem ade_sg: "~ellä- like makedonia";
    stem abl_sg: "~eltä- like makedonia";
    stem all_sg: "~elle- like hietaniemi";
    stem ess_sg: "~enä- like makedonia";
    stem tra_sg: "~eks- like hietaniemi";
    gen_: _, "~ten|~ien";
    proper_noun_possessive_nom_sg: _, _, _, _, "~ensä";
    proper_noun_possessive_nom_pl: _, _, _, _, derived;
    proper_noun_possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensä|~iensä";
    proper_noun_possessive_par_pl: _, _, _, _, "~iäVn|~iänsV";
    proper_noun_possessive_ess_pl: _, _, _, _, "~inäVn|~inänsV";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen|~inensä";
    ),
    // Leijonat: 7 words (alankomaat, pohjoismaat, ajokoirat, arabiemiirikunnat, huuhkajat, leijonat). Like Kaksoset, except in 13 cells: nom sg, nom pl, gen_ pl, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _2sg, proper_noun_possessive_nom_pl _1pl, ….
    declension!("leijonat", 7, "~at", like Kaksoset;
    nom: "~at", "~at";
    gen_: _, "~ien";
    proper_noun_possessive_nom_pl: "~Ani", "~Asi", "~Amme", "~Anne", "~AnsV";
    proper_noun_possessive_gen_pl: "~ieni", "~iesi", "~iemme", "~ienne", "~iensA";
    ),
    // Seem: 6 words (antwerpen, pohjois-jemen, tšeljabinsk, mannerheim, lakselv, pönttö-mac). Like Luxemburg, except in 41 cells: par pl, ine sg, ine pl, ela pl, ade pl, abl pl, ….
    declension!("seem", 6, "~", like Luxemburg;
    stem gen_sg: "~i- like hietaniemi";
    stem par_sg: "~iä- like ilmarinen";
    stem ill_sg: "~iin- like hietaniemi";
    stem ade_sg: "~illä- like makedonia";
    stem abl_sg: "~iltä- like makedonia";
    stem all_sg: "~ille- like hietaniemi";
    stem ess_sg: "~inä- like makedonia";
    stem tra_sg: "~iks- like hietaniemi";
    par: _, "~ejä";
    ine: "~issä", "~eissä";
    ela: _, "~eistä";
    ade: _, "~eillä";
    abl: _, "~eiltä";
    ess: _, "~einä";
    abe: _, "~eittä";
    proper_noun_possessive_nom_sg: _, _, _, _, "~insä";
    proper_noun_possessive_gen_pl: _, _, _, _, "~iensä";
    proper_noun_possessive_par_pl: "~ejäni", _, _, _, _;
    proper_noun_possessive_ine_sg: _, _, _, _, "~issäVn|~issänsV";
    proper_noun_possessive_ine_pl: "~eissäni", "~eissäsi", "~eissämme", "~eissänne", _;
    proper_noun_possessive_ela_pl: "~eistäni", "~eistäsi", "~eistämme", "~eistänne", "~eistäVn";
    proper_noun_possessive_ill_pl: _, _, _, _, "~eihinsä";
    proper_noun_possessive_ade_pl: "~eilläni", "~eilläsi", "~eillämme", "~eillänne", "~eilläVn";
    proper_noun_possessive_abl_pl: "~eiltäni", "~eiltäsi", "~eiltämme", "~eiltänne", "~eiltäVn";
    proper_noun_possessive_ess_pl: "~einäni", "~einäsi", "~einämme", "~einänne", _;
    proper_noun_possessive_abe_pl: "~eittäni", "~eittäsi", "~eittämme", "~eittänne", "~eittäVn";
    ),
    // Ilkka: 6 words (vaaka, haka, vatka, kruununhaka, kivihaka, koivuhaka). Like Nutikka, except in 18 cells: gen_ pl, par pl, ill pl, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, proper_noun_possessive_gen_pl _1pl, ….
    declension!("ilkka", 6, "~ka", like Nutikka;
    gen_: _, "~kojen";
    par: _, "~kojA";
    ill: _, "~koihin";
    proper_noun_possessive_gen_pl: "~kojeni", "~kojesi", "~kojemme", "~kojenne", "~kojensA";
    proper_noun_possessive_par_pl: "~kojAni", derived, derived, derived, "~kojAVn|~kojAnsV";
    proper_noun_possessive_ill_pl: "~koihini", derived, derived, derived, "~koihinsA";
    ),
    // Kristillisdemokraatit: 6 words (kristillisdemokraatit, aleutit, karpaatit, lofootit, emiraatit, arabiemiraatit). Like Plejadit.
    declension!("kristillisdemokraatit", 6, "~it", like Plejadit;
    stem par_pl: "~t- like plejadit";
    ),
    // Kainuu: 6 words (kainuu, hyvinkää, ylikainuu, keuruu, lappee, vuolee). Like Delfoi, except in 60 cells: nom pl, gen_ pl, par pl, ine pl, ela pl, ill pl, ….
    declension!("kainuu", 6, "~", like Delfoi;
    stem gen_sg: "~- like kainuu";
    stem ill_sg: "~seen- like altai";
    nom: _, derived;
    gen_: _, derived;
    par: _, derived;
    ine: _, derived;
    ela: _, derived;
    ill: _, derived;
    ade: _, derived;
    abl: _, derived;
    all: _, derived;
    ess: _, derived;
    abe: _, derived;
    proper_noun_possessive_nom_pl: _, derived, derived, derived, _;
    proper_noun_possessive_gen_pl: derived, derived, derived, derived, derived;
    proper_noun_possessive_par_pl: derived, _, _, _, _;
    proper_noun_possessive_ine_pl: derived, derived, derived, derived, _;
    proper_noun_possessive_ela_pl: derived, derived, derived, derived, derived;
    proper_noun_possessive_ill_pl: derived, _, _, _, derived;
    proper_noun_possessive_ade_pl: derived, derived, derived, derived, derived;
    proper_noun_possessive_abl_pl: derived, derived, derived, derived, derived;
    proper_noun_possessive_all_pl: derived, derived, derived, derived, _;
    proper_noun_possessive_ess_pl: derived, derived, derived, derived, _;
    proper_noun_possessive_tra_pl: derived, derived, derived, derived, _;
    proper_noun_possessive_abe_pl: derived, derived, derived, derived, derived;
    proper_noun_possessive_com_pl: derived, _, _, _, derived;
    ),
    // Räty: 6 words (pöytä, räty, ruutu, vihti, kytö, käyhty).
    declension!("räty", 6, "~t~";
    stem gen_sg: "~d~- like hietaniemi";
    stem par_pl: "~t~- like kytö";
    stem par_sg: "~t~ä- like ilmarinen";
    stem ine_sg: "~d~- like kytö";
    stem ade_sg: "~d~llä- like makedonia";
    stem abl_sg: "~d~ltä- like makedonia";
    stem all_sg: "~d~lle- like hietaniemi";
    stem ess_sg: "~t~nä- like makedonia";
    stem tra_sg: "~d~ks- like hietaniemi";
    nom: "~t~", _;
    ill: "~t~Vn", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~t~si", "~t~mme", "~t~nne", _;
    proper_noun_possessive_ill_sg: _, _, _, _, "~t~VnsE";
    ),
    // Kalat: 5 words (yhdysvallat, kalat, virrat, aikakirjat, linjat). Like Hydra with `` after each pattern (`t` after a), except in 28 cells: par sg, ine sg, ela sg, ill sg, ade sg, abl sg, ….
    declension!("kalat", 5, "~at", like Hydra with "", "t" after "a";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    all: derived, _;
    ess: derived, _;
    tra: derived, _;
    abe: derived, _;
    proper_noun_possessive_nom_sg: derived, _, _, _, derived;
    proper_noun_possessive_nom_pl: "~ani", _, _, _, "~ansV";
    proper_noun_possessive_gen_sg: _, derived, derived, derived, _;
    proper_noun_possessive_par_sg: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, derived;
    proper_noun_possessive_ill_sg: _, _, _, _, derived;
    proper_noun_possessive_ill_pl: "~oihini", _, _, _, _;
    proper_noun_possessive_ade_sg: _, _, _, _, derived;
    proper_noun_possessive_abl_sg: _, _, _, _, derived;
    proper_noun_possessive_all_sg: _, _, _, _, derived;
    proper_noun_possessive_ess_sg: _, _, _, _, derived;
    proper_noun_possessive_tra_sg: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, derived;
    ),
    // Hermes: 5 words (hermes, arkhimedes, kserkses, prokrustes, kirves). Like Roine with `` after each pattern (`s` after ~), except in 42 cells: gen_ pl, par sg, ine sg, ela sg, ill sg, ill pl, ….
    declension!("hermes", 5, "~s", like Roine with "", "s" after "~";
    gen_: _, "~iden";
    par: "~stä", _;
    ine: "~Vssä", _;
    ela: "~Vstä", _;
    ill: "~VsVVn", "~isVin";
    ade: "~Vllä", _;
    abl: "~Vltä", _;
    ess: "~Vnä", _;
    abe: "~Vttä", _;
    proper_noun_possessive_nom_sg: _, _, _, _, "~Vnsä";
    proper_noun_possessive_nom_pl: _, _, _, _, "~VnsA";
    proper_noun_possessive_gen_pl: "~ideni", "~idesi", "~idemme", "~idenne", "~idensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~stäVn|~stänsV";
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~VssäVn|~VssänsV";
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, "~VstäVn|~VstänsV";
    proper_noun_possessive_ela_pl: _, _, _, _, "~istAVn";
    proper_noun_possessive_ill_sg: _, _, _, _, "~VsVVnsä";
    proper_noun_possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~VlläVn|~VllänsV";
    proper_noun_possessive_ade_pl: _, _, _, _, "~illAVn";
    proper_noun_possessive_abl_sg: _, _, _, _, "~VltäVn|~VltänsV";
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltAVn";
    proper_noun_possessive_all_sg: _, _, _, _, "~Vlleen|~Vllensä";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_sg: _, _, _, _, "~VnäVn|~VnänsV";
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_sg: _, _, _, _, "~Vkseen|~Vksensä";
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, "~VttäVn|~VttänsV";
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen";
    ),
    // Ilmatar: 5 words (aallotar, ilmatar, suometar, korte, ilpotar).
    declension!("ilmatar", 5, "~~";
    stem gen_sg: "~t~e- like ilmatar";
    stem ine_sg: "~t~- like ilmatar";
    stem ill_sg: "~t~een- like altai";
    stem ade_sg: "~t~ella- like ilmatar";
    stem abl_sg: "~t~elta- like ilmatar";
    stem all_sg: "~t~elle- like altai";
    stem ess_sg: "~t~ena- like ilmatar";
    stem tra_sg: "~t~eks- like altai";
    nom: "~~", _;
    gen_: _, "~t~ien|~~ten";
    par: "~~tV", "~t~ia";
    ill: _, "~t~iin";
    ess: _, "~t~ina";
    proper_noun_possessive_nom_sg: "~t~eni", _, _, _, "~t~ensa";
    proper_noun_possessive_nom_pl: _, "~t~esi", "~t~emme", "~t~enne", _;
    proper_noun_possessive_gen_sg: _, "~t~esi", "~t~emme", "~t~enne", _;
    proper_noun_possessive_gen_pl: "~t~ieni|~~teni", "~t~iesi|~~tesi", "~t~iemme|~~temme", "~t~ienne|~~tenne", "~t~iensa|~~tensa";
    proper_noun_possessive_par_sg: _, _, _, _, "~~tVVn|~~tAnsV";
    proper_noun_possessive_par_pl: "~t~iani", _, _, _, "~t~iaan|~t~iansa";
    proper_noun_possessive_ill_pl: "~t~iini", _, _, _, "~t~iinsa";
    proper_noun_possessive_ess_pl: "~t~inani", "~t~inasi", "~t~inamme", "~t~inanne", "~t~inaan|~t~inansa";
    proper_noun_possessive_com_pl: "~t~ineni", _, _, _, "~t~ineen|~t~inensa";
    ),
    // Föglö: 5 words (nönnönnöö, brändö, eckerö, föglö, vårdö). Like Joensuu, except in 16 cells: nom sg, ine sg, ade sg, abl sg, all sg, ess sg, ….
    declension!("föglö", 5, "~ö", like Joensuu;
    stem gen_sg: "~ö- like joensuu";
    stem par_sg: "~ötA- like vehmaa";
    stem ill_sg: "~öhVn- like ilmarinen";
    nom: "~ö", _;
    ine: "~össA", _;
    ade: "~öllA", _;
    abl: "~öltA", _;
    all: "~ölle", _;
    ess: "~önA", _;
    tra: "~öksi", _;
    proper_noun_possessive_nom_sg: "~öni", _, _, _, "~önsA";
    proper_noun_possessive_nom_pl: _, "~ösi", "~ömme", "~önne", _;
    proper_noun_possessive_gen_sg: _, "~ösi", "~ömme", "~önne", _;
    proper_noun_possessive_ill_pl: "~öseVni", _, _, _, _;
    ),
    // Ärväs: 5 words (äyräs, keihäs, ärväs, pylväs, töyräs). Like Roine with `` after each pattern (`s` after ~), except in 23 cells: par sg, ine sg, ela sg, ill sg, ill pl, ade sg, ….
    declension!("ärväs", 5, "~s", like Roine with "", "s" after "~";
    par: "~stV", _;
    ine: "~VssV", _;
    ela: "~VstV", _;
    ill: "~VseVn", "~isVin";
    ade: "~VllV", _;
    abl: "~VltV", _;
    ess: "~VnV", _;
    abe: "~VttV", _;
    proper_noun_possessive_nom_sg: _, _, _, _, "~VnsV";
    proper_noun_possessive_par_sg: _, _, _, _, "~stVVn|~stVnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~VssVVn|~VssVnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~VstVVn|~VstVnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~VseVnsA";
    proper_noun_possessive_ill_pl: "~isVVni", derived, derived, derived, "~isVinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~VllVVn|~VllVnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~VltVVn|~VltVnsV";
    proper_noun_possessive_ess_sg: _, _, _, _, "~VnVVn|~VnVnsV";
    proper_noun_possessive_abe_sg: _, _, _, _, "~VttVVn|~VttVnsV";
    ),
    // Salokannel: 5 words (etelämanner, etelänapamanner, gondwanamanner, ahvenanmanner, salokannel). Like Ilmatar, except in 8 cells: nom sg, par sg, proper_noun_possessive_par_sg _3, proper_noun_possessive_par_pl _3, proper_noun_possessive_ine_pl _3, proper_noun_possessive_all_pl _3, ….
    declension!("salokannel", 5, "~n~", like Ilmatar;
    stem gen_sg: "~t~e- like salokannel";
    stem par_pl: "~t~- like salokannel";
    stem ine_sg: "~t~- like salokannel";
    stem ill_sg: "~t~eeseen- like altai";
    stem ade_sg: "~t~e- like salokannel";
    stem abl_sg: "~t~e- like salokannel";
    stem all_sg: "~t~e- like salokannel";
    stem ess_sg: "~t~e- like salokannel";
    stem tra_sg: "~t~e- like salokannel";
    nom: "~n~|~t~e", _;
    par: "~n~ta|~t~etta", _;
    proper_noun_possessive_par_sg: _, _, _, _, "~n~taan|~t~ettaan|~n~tansa|~t~ettansa";
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    ),
    // Ahde: 5 words (säde, ahde, kiviahde, luode, lähde). Like Ilmatar, except in 5 cells: nom sg, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_all_sg _3, proper_noun_possessive_tra_sg _3.
    declension!("ahde", 5, "~d~", like Ilmatar;
    stem gen_sg: "~t~e- like ilmarinen";
    stem par_pl: "~t~- like ahde";
    stem par_sg: "~d~ttA- like ilmarinen";
    stem ine_sg: "~t~- like ilmarinen";
    stem ill_sg: "~t~eseen- like ilmarinen";
    stem ade_sg: "~t~ellA- like makedonia";
    stem abl_sg: "~t~eltA- like makedonia";
    stem ess_sg: "~t~enA- like makedonia";
    nom: "~d~", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_all_sg: _, _, _, _, "~t~elleen|~t~ellVnsA";
    proper_noun_possessive_tra_sg: _, _, _, _, "~t~ekseen|~t~eksVnsA";
    ),
    // Kauris: 5 words (tigris, artemis, kauris, akropolis, khalkis). Like Roine with `` after each pattern (`s` after ~), except in 36 cells: gen_ pl, par sg, par pl, ill sg, ill pl, proper_noun_possessive_gen_pl _1sg, ….
    declension!("kauris", 5, "~s", like Roine with "", "s" after "~";
    gen_: _, "~Vden";
    par: "~stA", "~VtA";
    ill: "~VseVn", "~VsVin";
    proper_noun_possessive_gen_pl: "~Vdeni", "~Vdesi", "~Vdemme", "~Vdenne", "~VdensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~stAVn|~stAnsV";
    proper_noun_possessive_par_pl: "~VtAni", _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~istAVn";
    proper_noun_possessive_ill_sg: _, _, _, _, "~VseVnsA";
    proper_noun_possessive_ill_pl: "~VsVVni", derived, derived, derived, "~VsVinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, derived;
    proper_noun_possessive_ade_pl: _, _, _, _, "~illAVn";
    proper_noun_possessive_abl_sg: _, _, _, _, derived;
    proper_noun_possessive_abl_pl: _, _, _, _, "~iltAVn";
    proper_noun_possessive_all_sg: _, _, _, _, derived;
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_sg: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_sg: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~ittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~ineen";
    ),
    // Paasi: 5 words (susi, paasi, sengoku-kausi, käsivarsi, jokivarsi). Like Ilmarinen, except in 27 cells: nom sg, gen_ pl, ine sg, proper_noun_possessive_nom_sg _1sg, proper_noun_possessive_nom_sg _3, proper_noun_possessive_nom_pl _2sg, ….
    declension!("paasi", 5, "~si", like Ilmarinen;
    stem gen_sg: "~de- like ilmarinen";
    stem par_sg: "~ttE- like ilmarinen";
    stem ill_sg: "~teVn- like ilmarinen";
    stem ade_sg: "~dellA- like makedonia";
    stem abl_sg: "~deltA- like makedonia";
    stem all_sg: "~delle- like makedonia";
    stem ess_sg: "~tenA- like makedonia";
    stem tra_sg: "~deks- like makedonia";
    nom: "~si", _;
    gen_: _, "~sien";
    ine: "~dessA", _;
    proper_noun_possessive_nom_sg: "~teni", _, _, _, "~tensA";
    proper_noun_possessive_nom_pl: _, "~tesi", "~temme", "~tenne", _;
    proper_noun_possessive_gen_sg: _, "~tesi", "~temme", "~tenne", _;
    proper_noun_possessive_gen_pl: "~sieni", "~siesi", "~siemme", "~sienne", "~siensA";
    proper_noun_possessive_par_pl: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~dessAVn|~dessAnsV";
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~sistAVn";
    proper_noun_possessive_ade_pl: _, _, _, _, "~sillAVn";
    proper_noun_possessive_abl_pl: _, _, _, _, "~siltAVn";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~sittAVn";
    proper_noun_possessive_com_pl: _, _, _, _, "~sineen";
    ),
    // Viisut: 5 words (vihreät, euroviisut, luoteisterritoriot, viisut, kostet). Like Salo with `` after each pattern (`t` after ~), except in 30 cells: par sg, ine sg, ela sg, ill sg, ill pl, ade sg, ….
    declension!("viisut", 5, "~t", like Salo with "", "t" after "~";
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, "~Ehin";
    ade: derived, _;
    abl: derived, _;
    all: derived, _;
    ess: derived, _;
    tra: derived, _;
    abe: derived, _;
    proper_noun_possessive_nom_sg: derived, _, _, _, derived;
    proper_noun_possessive_nom_pl: "~ni", _, _, _, "~nsa";
    proper_noun_possessive_gen_sg: _, derived, derived, derived, _;
    proper_noun_possessive_par_sg: _, _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, derived;
    proper_noun_possessive_ela_sg: _, _, _, _, derived;
    proper_noun_possessive_ill_sg: _, _, _, _, derived;
    proper_noun_possessive_ill_pl: "~Ehini", _, _, _, "~EhinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, derived;
    proper_noun_possessive_abl_sg: _, _, _, _, derived;
    proper_noun_possessive_all_sg: _, _, _, _, derived;
    proper_noun_possessive_ess_sg: _, _, _, _, derived;
    proper_noun_possessive_tra_sg: _, _, _, _, derived;
    proper_noun_possessive_abe_sg: _, _, _, _, derived;
    ),
    // Lepo: 4 words (repo, siipo, lepo, ropo).
    declension!("lepo", 4, "~p~";
    stem gen_sg: "~v~- like ilmatar";
    stem par_pl: "~p~- like okko";
    stem par_sg: "~p~A- like ilmarinen";
    stem ine_sg: "~v~- like lepo";
    stem ill_sg: "~p~Vn- like ilmarinen";
    stem ade_sg: "~v~lla- like ilmatar";
    stem abl_sg: "~v~lta- like ilmatar";
    stem all_sg: "~v~lle- like altai";
    stem ess_sg: "~p~nA- like makedonia";
    stem tra_sg: "~v~ks- like altai";
    nom: "~p~", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~p~si", "~p~mme", "~p~nne", _;
    ),
    // Marjatta: 4 words (maritta, marjatta, marketta, piritta). Like Makedonia, except in 6 cells: nom sg, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_gen_sg _2sg, proper_noun_possessive_gen_sg _1pl, proper_noun_possessive_gen_sg _2pl.
    declension!("marjatta", 4, "~ta", like Makedonia;
    stem par_pl: "~- like marjatta";
    stem par_sg: "~taV- like makedonia";
    stem ill_sg: "~taVn- like makedonia";
    stem ess_sg: "~tanV- like makedonia";
    nom: "~ta", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~tasi", "~tamme", "~tanne", _;
    ),
    // Yrjänä: 4 words (yrjänä, kähärä, kälkäjä, kärsämä). Like Kestilä, except in 29 cells: gen_ pl, par pl, ill pl, ess pl, proper_noun_possessive_gen_pl _1sg, proper_noun_possessive_gen_pl _2sg, ….
    declension!("yrjänä", 4, "~ä", like Kestilä;
    stem ine_sg: "~- like yrjänä";
    gen_: _, "~ien|~öiden|~öitten";
    par: _, "~iA|~öitA";
    ill: _, "~iin|~öihin";
    ess: _, "~öinA|~inA";
    proper_noun_possessive_gen_pl: "~ieni|~öideni|~öitteni", "~iesi|~öidesi|~öittesi", "~iemme|~öidemme|~öittemme", "~ienne|~öidenne|~öittenne", "~iensA|~öidensA|~öittensA";
    proper_noun_possessive_par_pl: "~iAni|~öitAni", "~iAsi|~öitAsi", "~iAmme|~öitAmme", "~iAnne|~öitAnne", "~iAVn|~öitAVn|~iAnsV|~öitAnsV";
    proper_noun_possessive_ill_pl: "~iini|~öihini", "~iisi|~öihisi", "~iimme|~öihimme", "~iinne|~öihinne", "~iinsA|~öihinsA";
    proper_noun_possessive_ess_pl: "~öinAni|~inAni", "~öinAsi|~inAsi", "~öinAmme|~inAmme", "~öinAnne|~inAnne", "~öinAVn|~inAVn|~öinAnsV|~inänsä";
    proper_noun_possessive_com_pl: "~öineni|~ineni", "~öinesi|~inesi", "~öinemme|~inemme", "~öinenne|~inenne", "~öineen|~ineen|~öinensA|~inensä";
    ),
    // Luopa: 4 words (isotupa, uusitupa, luopa, alitupa).
    declension!("luopa", 4, "~pa";
    stem gen_sg: "~vA- like makedonia";
    stem par_pl: "~p- like olhava";
    stem par_sg: "~paV- like makedonia";
    stem ine_sg: "~v- like hölttä";
    stem ill_sg: "~paVn- like makedonia";
    stem ade_sg: "~vAllV- like makedonia";
    stem abl_sg: "~vAltV- like makedonia";
    stem all_sg: "~vAlle- like makedonia";
    stem ess_sg: "~panV- like makedonia";
    stem tra_sg: "~vAks- like makedonia";
    nom: "~pa", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~pasi", "~pamme", "~panne", _;
    ),
    // Lohi: 4 words (vuohi, kuusi, lohi, jousi). Like Salo with `` after each pattern (`i` after ~), except in 41 cells: nom pl, gen_ pl, par sg, par pl, ine sg, ela sg, ….
    declension!("lohi", 4, "~i", like Salo with "", "i" after "~";
    nom: _, "~et";
    gen_: _, "~ien";
    par: "~tE", "~iA";
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~eVn", "~iin";
    ade: "~ellA", _;
    abl: "~eltA", _;
    all: "~elle", _;
    ess: "~enA", _;
    tra: "~eksi", _;
    abe: "~ettA", _;
    proper_noun_possessive_nom_sg: "~eni", _, _, _, "~ensA";
    proper_noun_possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_pl: "~ieni", "~iesi", "~iemme", "~ienne", "~iensA";
    proper_noun_possessive_par_sg: _, _, _, _, "~tEVn|~tEnsV";
    proper_noun_possessive_par_pl: "~iAni", _, _, _, "~iAVn|~iAnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~essAVn|~essAnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~estAVn|~estAnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~eVnsA";
    proper_noun_possessive_ill_pl: "~iini", _, _, _, "~iinsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~ellAVn|~ellAnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~eltAVn|~eltAnsV";
    proper_noun_possessive_all_sg: _, _, _, _, "~elleen|~ellensA";
    proper_noun_possessive_ess_sg: _, _, _, _, "~enAVn|~enAnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, "~ekseen|~eksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, "~ettAVn|~ettAnsV";
    ),
    // Haukipudas: 4 words (pihtipudas, haukipudas, poropudas, pudas). Like Ilmatar, except in 34 cells: nom sg, gen_ pl, par sg, par pl, ine sg, ill pl, ….
    declension!("haukipudas", 4, "~d~s", like Ilmatar;
    stem gen_sg: "~t~a- like ilmatar";
    stem ill_sg: "~t~aseen- like altai";
    stem ade_sg: "~t~alla- like ilmatar";
    stem abl_sg: "~t~alta- like ilmatar";
    stem all_sg: "~t~alle- like altai";
    stem ess_sg: "~t~ana- like ilmatar";
    stem tra_sg: "~t~aks- like altai";
    nom: "~d~s", _;
    gen_: _, "~t~iden";
    par: "~d~sta", "~t~ita";
    ine: "~t~assa", _;
    ill: _, "~t~isiin";
    proper_noun_possessive_nom_sg: "~t~ani", _, _, _, "~t~ansa";
    proper_noun_possessive_nom_pl: _, "~t~asi", "~t~amme", "~t~anne", _;
    proper_noun_possessive_gen_sg: _, "~t~asi", "~t~amme", "~t~anne", _;
    proper_noun_possessive_gen_pl: "~t~ideni", "~t~idesi", "~t~idemme", "~t~idenne", "~t~idensa";
    proper_noun_possessive_par_sg: _, _, _, _, "~d~staan|~d~stansa";
    proper_noun_possessive_par_pl: "~t~itani", _, _, _, derived;
    proper_noun_possessive_ine_sg: _, _, _, _, "~t~assaan|~t~assansa";
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_ela_pl: _, _, _, _, "~t~istaan";
    proper_noun_possessive_ill_pl: "~t~isiini", _, _, _, "~t~isiinsa";
    proper_noun_possessive_ade_pl: _, _, _, _, "~t~illaan";
    proper_noun_possessive_abl_pl: _, _, _, _, "~t~iltaan";
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_ess_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    proper_noun_possessive_abe_pl: _, _, _, _, "~t~ittaan";
    proper_noun_possessive_com_pl: _, _, _, _, "~t~ineen";
    ),
    // Korkee: 4 words (tennessee, korkee, selkee, suontee). Like Joensuu, except in 16 cells: nom sg, ine sg, ade sg, abl sg, all sg, ess sg, ….
    declension!("korkee", 4, "~e", like Joensuu;
    stem gen_sg: "~e- like joensuu";
    stem par_sg: "~etA- like vehmaa";
    stem ill_sg: "~ehVn- like ilmarinen";
    nom: "~e", _;
    ine: "~essA", _;
    ade: "~ellA", _;
    abl: "~eltA", _;
    all: "~elle", _;
    ess: "~enA", _;
    tra: "~eksi", _;
    proper_noun_possessive_nom_sg: "~eni", _, _, _, "~ensA";
    proper_noun_possessive_nom_pl: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_gen_sg: _, "~esi", "~emme", "~enne", _;
    proper_noun_possessive_ill_pl: "~esVVni", _, _, _, _;
    ),
    // Tehtaankatu: 4 words (linnunrata, satu, tehtaankatu, laatu). Like Räty, except in 5 cells: proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_ine_pl _3, proper_noun_possessive_all_pl _3, proper_noun_possessive_tra_pl _3.
    declension!("tehtaankatu", 4, "~t~", like Räty;
    stem gen_sg: "~d~- like ilmarinen";
    stem par_pl: "~t~- like okko";
    stem par_sg: "~t~A- like ilmarinen";
    stem ine_sg: "~d~- like tehtaankatu";
    stem ill_sg: "~t~Vn- like ilmarinen";
    stem ade_sg: "~d~llA- like makedonia";
    stem abl_sg: "~d~ltA- like makedonia";
    stem all_sg: "~d~lle- like makedonia";
    stem ess_sg: "~t~nA- like makedonia";
    stem tra_sg: "~d~ks- like makedonia";
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_ine_pl: _, _, _, _, derived;
    proper_noun_possessive_all_pl: _, _, _, _, derived;
    proper_noun_possessive_tra_pl: _, _, _, _, derived;
    ),
    // Suvitie: 4 words (kangastie, suvitie, talvitie, jokipii). Like Heinisuo, except in 73 cells: nom sg, nom pl, gen_ pl, par sg, par pl, ine sg, ….
    declension!("suvitie", 4, "~i~", like Heinisuo;
    stem ill_sg: "~i~hen- like hietaniemi";
    stem all_sg: "~i~lle- like hietaniemi";
    stem tra_sg: "~i~ks- like hietaniemi";
    nom: "~i~", "~i~t";
    gen_: _, "~~iden|~~itten";
    par: "~i~tä", "~~itä";
    ine: "~i~ssä", "~~issä";
    ela: "~i~stä", "~~istä";
    ade: "~i~llä", "~~illä";
    abl: "~i~ltä", "~~iltä";
    ess: "~i~nä", "~~inä";
    abe: "~i~ttä", "~~ittä";
    proper_noun_possessive_nom_sg: "~i~ni", _, _, _, "~i~nsä";
    proper_noun_possessive_nom_pl: _, "~i~si", "~i~mme", "~i~nne", _;
    proper_noun_possessive_gen_sg: _, "~i~si", "~i~mme", "~i~nne", _;
    proper_noun_possessive_gen_pl: "~~ideni|~~itteni", "~~idesi|~~ittesi", "~~idemme|~~ittemme", "~~idenne|~~ittenne", "~~idensä|~~ittensä";
    proper_noun_possessive_par_sg: _, _, _, _, "~i~tään|~i~tänsä";
    proper_noun_possessive_par_pl: "~~itäni", _, _, _, "~~itäVn|~~itänsä";
    proper_noun_possessive_ine_sg: _, _, _, _, "~i~ssään|~i~ssänsä";
    proper_noun_possessive_ine_pl: "~~issäni", "~~issäsi", "~~issämme", "~~issänne", "~~issäVn|~~issänsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~i~stään|~i~stänsä";
    proper_noun_possessive_ela_pl: "~~istäni", "~~istäsi", "~~istämme", "~~istänne", "~~istäVn|~~istänsV";
    proper_noun_possessive_ill_pl: _, _, _, _, "~~ihinsä";
    proper_noun_possessive_ade_sg: _, _, _, _, "~i~llään|~i~llänsä";
    proper_noun_possessive_ade_pl: "~~illäni", "~~illäsi", "~~illämme", "~~illänne", "~~illäVn|~~illänsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~i~ltään|~i~ltänsä";
    proper_noun_possessive_abl_pl: "~~iltäni", "~~iltäsi", "~~iltämme", "~~iltänne", "~~iltäVn|~~iltänsV";
    proper_noun_possessive_all_pl: _, _, _, _, "~~illeen|~~illensä";
    proper_noun_possessive_ess_sg: _, _, _, _, "~i~nään|~i~nänsä";
    proper_noun_possessive_ess_pl: "~~inäni", "~~inäsi", "~~inämme", "~~inänne", "~~inäVn|~~inänsä";
    proper_noun_possessive_tra_pl: _, _, _, _, "~~ikseen|~~iksensä";
    proper_noun_possessive_abe_sg: _, _, _, _, "~i~ttään|~i~ttänsä";
    proper_noun_possessive_abe_pl: "~~ittäni", "~~ittäsi", "~~ittämme", "~~ittänne", "~~ittäVn|~~ittänsV";
    proper_noun_possessive_com_pl: _, _, _, _, "~~ineen|~~inensä";
    ),
    // Hyväkkä: 4 words (härkä, hyväkkä, hälikkä, lyytikkä). Like Kestilä, except in 8 cells: nom sg, ine sg, proper_noun_possessive_nom_pl _1sg, proper_noun_possessive_nom_pl _3, proper_noun_possessive_gen_sg _2sg, proper_noun_possessive_gen_sg _1pl, ….
    declension!("hyväkkä", 4, "~kä", like Kestilä;
    stem gen_sg: "~A- like makedonia";
    stem par_pl: "~- like hyväkkä";
    stem par_sg: "~käV- like makedonia";
    stem ill_sg: "~käVn- like makedonia";
    stem ade_sg: "~AllV- like makedonia";
    stem abl_sg: "~AltV- like makedonia";
    stem all_sg: "~Alle- like makedonia";
    stem ess_sg: "~känV- like makedonia";
    stem tra_sg: "~Aks- like makedonia";
    nom: "~kä", _;
    ine: "~AssV", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~käsi", "~kämme", "~känne", _;
    proper_noun_possessive_ine_sg: _, _, _, _, "~AssVVn|~AssVnsV";
    ),
    // Alavus: 3 words (varkaus, alavus, warkaus). Like Kainuu with `s` after each pattern, except in 26 cells: gen_ sg, par sg, ine sg, ela sg, ill sg, ade sg, ….
    declension!("alavus", 3, "~s", like Kainuu with "s";
    gen_: "~den", _;
    par: "~ttE", _;
    ine: "~dessA", _;
    ela: "~destA", _;
    ill: "~teVn", _;
    ade: "~dellA", _;
    abl: "~deltA", _;
    all: "~delle", _;
    ess: "~tenA", _;
    tra: "~deksi", _;
    abe: "~dettA", _;
    proper_noun_possessive_nom_sg: "~teni", _, _, _, "~tensA";
    proper_noun_possessive_gen_sg: _, "~tesi", "~temme", "~tenne", _;
    proper_noun_possessive_par_sg: _, _, _, _, "~ttEVn|~ttEnsV";
    proper_noun_possessive_ine_sg: _, _, _, _, "~dessAVn|~dessAnsV";
    proper_noun_possessive_ela_sg: _, _, _, _, "~destAVn|~destAnsV";
    proper_noun_possessive_ill_sg: _, _, _, _, "~teVnsA";
    proper_noun_possessive_ade_sg: _, _, _, _, "~dellAVn|~dellAnsV";
    proper_noun_possessive_abl_sg: _, _, _, _, "~deltAVn|~deltAnsV";
    proper_noun_possessive_all_sg: _, _, _, _, "~delleen|~dellensA";
    proper_noun_possessive_ess_sg: _, _, _, _, "~tenAVn|~tenAnsV";
    proper_noun_possessive_tra_sg: _, _, _, _, "~dekseen|~deksensA";
    proper_noun_possessive_abe_sg: _, _, _, _, "~dettAVn|~dettAnsV";
    ),
    // Pyy: 3 words (kruunupyy, siipyy, pyy). Like Joensuu, except in 15 cells: nom sg, ine sg, ade sg, abl sg, all sg, ess sg, ….
    declension!("pyy", 3, "~y", like Joensuu;
    stem gen_sg: "~y- like joensuu";
    stem par_sg: "~ytA- like vehmaa";
    stem ill_sg: "~yhVn- like ilmarinen";
    nom: "~y", _;
    ine: "~yssA", _;
    ade: "~yllA", _;
    abl: "~yltA", _;
    all: "~ylle", _;
    ess: "~ynA", _;
    tra: "~yksi", _;
    proper_noun_possessive_nom_sg: "~yni", _, _, _, "~ynsA";
    proper_noun_possessive_nom_pl: _, "~ysi", "~ymme", "~ynne", _;
    proper_noun_possessive_gen_sg: _, "~ysi", "~ymme", "~ynne", _;
    ),
    // Kurki: 3 words (kurki, kärki, savikurki).
    declension!("kurki", 3, "~ki";
    stem gen_sg: "~je- like ilmarinen";
    stem par_pl: "~k- like salmi";
    stem par_sg: "~keA- like ilmarinen";
    stem ine_sg: "~j- like ilmarinen";
    stem ill_sg: "~keVn- like ilmarinen";
    stem ade_sg: "~jellA- like makedonia";
    stem abl_sg: "~jeltA- like makedonia";
    stem all_sg: "~jelle- like makedonia";
    stem ess_sg: "~kenA- like makedonia";
    stem tra_sg: "~jeks- like makedonia";
    nom: "~ki", _;
    proper_noun_possessive_nom_pl: derived, _, _, _, derived;
    proper_noun_possessive_gen_sg: _, "~kesi", "~kemme", "~kenne", _;
    ),
    // Lähiitä: 3 words (lähi-itä, kaukoitä, keski-itä). Like Kestilä with `ˇ` after each pattern, except in 12 cells: gen_ sg, tra pl, ins pl, proper_noun_possessive_par_pl _2sg, proper_noun_possessive_par_pl _1pl, proper_noun_possessive_par_pl _2pl, ….
    declension!("lähi-itä", 3, "~äˇ", like Kestilä with "ˇ";
    gen_: "~änˇ", _;
    tra: _, "~öiksiˇ";
    ins: _, "~öinˇ";
    proper_noun_possessive_par_pl: _, "~öitAsiˇ", "~öitAmmeˇ", "~öitAnneˇ", _;
    proper_noun_possessive_ill_pl: _, "~öihVsiˇ", "~öihVmmeˇ", "~öihVnneˇ", _;
    proper_noun_possessive_com_pl: _, "~öinesiˇ", "~öinemmeˇ", "~öinenneˇ", _;
    ),
];
