//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~aaˇ`, `polttaa`: the stem and `aaˇ`). A stem in two pieces (`~~ta`, `leikata`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `olisimm% ~taneet`, of
//! `puhaltaa`), as the copy laws in `laws.rs` change it. A capital (`U`, `A`, `O`, `Ä`, `V`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~aaˇ`: `polttaa`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem indicative_pres_aff_1sg: \"~- like voida\"`: the slot's cells are `voida`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
/// `pres_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `pres_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `past_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `past_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `perf_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `perf_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `plup_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `plup_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `conditional_pres_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `conditional_pres_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `conditional_perf_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `conditional_perf_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `imperative_aff` (_2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `imperative_neg` (_2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `perfect_imperative_aff` (_3sg, _3pl, pass)
/// `perfect_imperative_neg` (_3sg, _3pl, pass)
/// `potential_pres_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `potential_pres_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `potential_perf_aff` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `potential_perf_neg` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl, pass)
/// `infinitive`
/// `long_infinitive` (_1sg, _2sg, _1pl, _2pl, _3)
/// `second_infinitive` (act, pass)
/// `second_infinitive_possessive` (_1sg, _2sg, _1pl, _2pl, _3)
/// `second_infinitive_instructive`
/// `third_infinitive` (ine, ela, ill, ade, abe, ins)
/// `third_infinitive_passive`
/// `fourth_infinitive`
/// `fifth_infinitive` (_1sg, _2sg, _1pl, _2pl, _3)
/// `participle` (pres_act, pres_pass, past_act, past_pass, agent, neg)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem indicative_pres_aff_1sg: $s0:tt;)?
     $(stem indicative_pres_aff_3sg: $s1:tt;)?
     $(stem participle_past_pass: $s2:tt;)?
     $(stem indicative_past_aff_3sg: $s3:tt;)?
     $(stem lemma: $s4:tt;)?
     $(pres_aff: $c0:tt, $c1:tt, $c2:tt, $c3:tt, $c4:tt, $c5:tt, $c6:tt;)?
     $(pres_neg: $c7:tt, $c8:tt, $c9:tt, $c10:tt, $c11:tt, $c12:tt, $c13:tt;)?
     $(past_aff: $c14:tt, $c15:tt, $c16:tt, $c17:tt, $c18:tt, $c19:tt, $c20:tt;)?
     $(past_neg: $c21:tt, $c22:tt, $c23:tt, $c24:tt, $c25:tt, $c26:tt, $c27:tt;)?
     $(perf_aff: $c28:tt, $c29:tt, $c30:tt, $c31:tt, $c32:tt, $c33:tt, $c34:tt;)?
     $(perf_neg: $c35:tt, $c36:tt, $c37:tt, $c38:tt, $c39:tt, $c40:tt, $c41:tt;)?
     $(plup_aff: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt, $c48:tt;)?
     $(plup_neg: $c49:tt, $c50:tt, $c51:tt, $c52:tt, $c53:tt, $c54:tt, $c55:tt;)?
     $(conditional_pres_aff: $c56:tt, $c57:tt, $c58:tt, $c59:tt, $c60:tt, $c61:tt, $c62:tt;)?
     $(conditional_pres_neg: $c63:tt, $c64:tt, $c65:tt, $c66:tt, $c67:tt, $c68:tt, $c69:tt;)?
     $(conditional_perf_aff: $c70:tt, $c71:tt, $c72:tt, $c73:tt, $c74:tt, $c75:tt, $c76:tt;)?
     $(conditional_perf_neg: $c77:tt, $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt, $c83:tt;)?
     $(imperative_aff: $c84:tt, $c85:tt, $c86:tt, $c87:tt, $c88:tt, $c89:tt;)?
     $(imperative_neg: $c90:tt, $c91:tt, $c92:tt, $c93:tt, $c94:tt, $c95:tt;)?
     $(perfect_imperative_aff: $c96:tt, $c97:tt, $c98:tt;)?
     $(perfect_imperative_neg: $c99:tt, $c100:tt, $c101:tt;)?
     $(potential_pres_aff: $c102:tt, $c103:tt, $c104:tt, $c105:tt, $c106:tt, $c107:tt, $c108:tt;)?
     $(potential_pres_neg: $c109:tt, $c110:tt, $c111:tt, $c112:tt, $c113:tt, $c114:tt, $c115:tt;)?
     $(potential_perf_aff: $c116:tt, $c117:tt, $c118:tt, $c119:tt, $c120:tt, $c121:tt, $c122:tt;)?
     $(potential_perf_neg: $c123:tt, $c124:tt, $c125:tt, $c126:tt, $c127:tt, $c128:tt, $c129:tt;)?
     $(infinitive: $c130:tt;)?
     $(long_infinitive: $c131:tt, $c132:tt, $c133:tt, $c134:tt, $c135:tt;)?
     $(second_infinitive: $c136:tt, $c137:tt;)?
     $(second_infinitive_possessive: $c138:tt, $c139:tt, $c140:tt, $c141:tt, $c142:tt;)?
     $(second_infinitive_instructive: $c143:tt;)?
     $(third_infinitive: $c144:tt, $c145:tt, $c146:tt, $c147:tt, $c148:tt, $c149:tt;)?
     $(third_infinitive_passive: $c150:tt;)?
     $(fourth_infinitive: $c151:tt;)?
     $(fifth_infinitive: $c152:tt, $c153:tt, $c154:tt, $c155:tt, $c156:tt;)?
     $(participle: $c157:tt, $c158:tt, $c159:tt, $c160:tt, $c161:tt, $c162:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// The cells a conjugation reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a conjugation adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `polttaa`: 1,745 words.
    Polttaa,
    /// Like `purkautua`: 1,259 words.
    Purkautua,
    /// Like `voida`: 935 words.
    Voida,
    /// Like `hiihtää`: 892 words.
    Hiihtää,
    /// Like `ohjata`: 771 words.
    Ohjata,
    /// Like `tulla`: 667 words.
    Tulla,
    /// Like `nostaa`: 562 words.
    Nostaa,
    /// Like `ajatella`: 512 words.
    Ajatella,
    /// Like `sammua`: 478 words.
    Sammua,
    /// Like `jäätyä`: 462 words.
    Jäätyä,
    /// Like `epäillä`: 278 words.
    Epäillä,
    /// Like `tiivistää`: 237 words.
    Tiivistää,
    /// Like `käsitellä`: 211 words.
    Käsitellä,
    /// Like `kypsyä`: 200 words.
    Kypsyä,
    /// Like `kipata`: 200 words.
    Kipata,
    /// Like `nousta`: 185 words.
    Nousta,
    /// Like `kuoria`: 158 words.
    Kuoria,
    /// Like `johtaa`: 149 words.
    Johtaa,
    /// Like `määrätä`: 140 words.
    Määrätä,
    /// Like `päästä`: 139 words.
    Päästä,
    /// Like `leikata`: 137 words.
    Leikata,
    /// Like `pyöriä`: 136 words.
    Pyöriä,
    /// Like `hajota`: 124 words.
    Hajota,
    /// Like `katsoa`: 88 words.
    Katsoa,
    /// Like `pätevöidä`: 84 words.
    Pätevöidä,
    /// Like `kääntää`: 82 words.
    Kääntää,
    /// Like `värähdellä`: 74 words.
    Värähdellä,
    /// Like `hylätä`: 71 words.
    Hylätä,
    /// Like `parata`: 56 words.
    Parata,
    /// Like `murtaa`: 51 words.
    Murtaa,
    /// Like `tuomita`: 45 words.
    Tuomita,
    /// Like `asioida`: 44 words.
    Asioida,
    /// Like `yletä`: 42 words.
    Yletä,
    /// Like `hävitä`: 41 words.
    Hävitä,
    /// Like `kiertää`: 41 words.
    Kiertää,
    /// Like `aueta`: 39 words.
    Aueta,
    /// Like `liukua`: 36 words.
    Liukua,
    /// Like `sotia`: 34 words.
    Sotia,
    /// Like `pelehtiä`: 34 words.
    Pelehtiä,
    /// Like `paeta`: 32 words.
    Paeta,
    /// Like `kieltää`: 31 words.
    Kieltää,
    /// Like `sykkiä`: 30 words.
    Sykkiä,
    /// Like `puhaltaa`: 28 words.
    Puhaltaa,
    /// Like `ajaa`: 27 words.
    Ajaa,
    /// Like `hankkia`: 25 words.
    Hankkia,
    /// Like `sitoa`: 24 words.
    Sitoa,
    /// Like `haikaa`: 23 words.
    Haikaa,
    /// Like `rikkoa`: 22 words.
    Rikkoa,
    /// Like `isännöidä`: 19 words.
    Isännöidä,
    /// Like `rundaa`: 18 words.
    Rundaa,
    /// Like `ojentaa`: 17 words.
    Ojentaa,
    /// Like `merkitä`: 15 words.
    Merkitä,
    /// Like `ähkää`: 14 words.
    Ähkää,
    /// Like `kaataa`: 14 words.
    Kaataa,
    /// Like `taittaa`: 13 words.
    Taittaa,
    /// Like `kiivetä`: 13 words.
    Kiivetä,
    /// Like `jyrketä`: 12 words.
    Jyrketä,
    /// Like `tarpoa`: 12 words.
    Tarpoa,
    /// Like `laueta`: 10 words.
    Laueta,
    /// Like `hiipiä`: 10 words.
    Hiipiä,
    /// Like `hyppiä`: 10 words.
    Hyppiä,
    /// Like `laskea`: 9 words.
    Laskea,
    /// Like `harppia`: 9 words.
    Harppia,
    /// Like `kääntää`: 8 words.
    Kääntää2,
    /// Like `taannehtia`: 8 words.
    Taannehtia,
    /// Like `iskeä`: 7 words.
    Iskeä,
    /// Like `kenkiä`: 7 words.
    Kenkiä,
    /// Like `lukea`: 7 words.
    Lukea,
    /// Like `luoda`: 7 words.
    Luoda,
    /// Like `ruveta`: 7 words.
    Ruveta,
    /// Like `julkipanna`: 6 words.
    Julkipanna,
    /// Like `ladata`: 6 words.
    Ladata,
    /// Like `virittää`: 5 words.
    Virittää,
    /// Like `kulkea`: 5 words.
    Kulkea,
    /// Like `sopia`: 5 words.
    Sopia,
    /// Like `upota`: 5 words.
    Upota,
    /// Like `lyödä`: 5 words.
    Lyödä,
    /// Like `tuultaa`: 5 words.
    Tuultaa,
    /// Like `kontia`: 5 words.
    Kontia,
    /// Like `empiä`: 5 words.
    Empiä,
    /// Like `siitä`: 4 words.
    Siitä,
    /// Like `ampua`: 4 words.
    Ampua,
    /// Like `purkaa`: 4 words.
    Purkaa,
    /// Like `tenttiä`: 4 words.
    Tenttiä,
    /// Like `dubata`: 4 words.
    Dubata,
    /// Like `tonkia`: 4 words.
    Tonkia,
    /// Like `lähteä`: 4 words.
    Lähteä,
    /// Like `heläjää`: 4 words.
    Heläjää,
    /// Like `jydätä`: 4 words.
    Jydätä,
    /// Like `heiketä`: 3 words.
    Heiketä,
    /// Like `häipyä`: 3 words.
    Häipyä,
    /// Like `takuta`: 3 words.
    Takuta,
    /// Like `myydä`: 3 words.
    Myydä,
    /// Like `nylkeä`: 3 words.
    Nylkeä,
    /// Like `lappaa`: 3 words.
    Lappaa,
    /// Like `pinkoa`: 3 words.
    Pinkoa,
    /// Like `halajaa`: 3 words.
    Halajaa,
    /// Like `nappoa`: 3 words.
    Nappoa,
    /// Like `vaa'ata`: 2 words.
    Vaaata,
    /// Like `tarvii`: 1 words.
    Tarvii,
    /// Like `tarttee`: 1 words.
    Tarttee,
    /// Like `bygee`: 1 words.
    Bygee,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 102] = [
        Conjugation::Polttaa,
        Conjugation::Purkautua,
        Conjugation::Voida,
        Conjugation::Hiihtää,
        Conjugation::Ohjata,
        Conjugation::Tulla,
        Conjugation::Nostaa,
        Conjugation::Ajatella,
        Conjugation::Sammua,
        Conjugation::Jäätyä,
        Conjugation::Epäillä,
        Conjugation::Tiivistää,
        Conjugation::Käsitellä,
        Conjugation::Kypsyä,
        Conjugation::Kipata,
        Conjugation::Nousta,
        Conjugation::Kuoria,
        Conjugation::Johtaa,
        Conjugation::Määrätä,
        Conjugation::Päästä,
        Conjugation::Leikata,
        Conjugation::Pyöriä,
        Conjugation::Hajota,
        Conjugation::Katsoa,
        Conjugation::Pätevöidä,
        Conjugation::Kääntää,
        Conjugation::Värähdellä,
        Conjugation::Hylätä,
        Conjugation::Parata,
        Conjugation::Murtaa,
        Conjugation::Tuomita,
        Conjugation::Asioida,
        Conjugation::Yletä,
        Conjugation::Hävitä,
        Conjugation::Kiertää,
        Conjugation::Aueta,
        Conjugation::Liukua,
        Conjugation::Sotia,
        Conjugation::Pelehtiä,
        Conjugation::Paeta,
        Conjugation::Kieltää,
        Conjugation::Sykkiä,
        Conjugation::Puhaltaa,
        Conjugation::Ajaa,
        Conjugation::Hankkia,
        Conjugation::Sitoa,
        Conjugation::Haikaa,
        Conjugation::Rikkoa,
        Conjugation::Isännöidä,
        Conjugation::Rundaa,
        Conjugation::Ojentaa,
        Conjugation::Merkitä,
        Conjugation::Ähkää,
        Conjugation::Kaataa,
        Conjugation::Taittaa,
        Conjugation::Kiivetä,
        Conjugation::Jyrketä,
        Conjugation::Tarpoa,
        Conjugation::Laueta,
        Conjugation::Hiipiä,
        Conjugation::Hyppiä,
        Conjugation::Laskea,
        Conjugation::Harppia,
        Conjugation::Kääntää2,
        Conjugation::Taannehtia,
        Conjugation::Iskeä,
        Conjugation::Kenkiä,
        Conjugation::Lukea,
        Conjugation::Luoda,
        Conjugation::Ruveta,
        Conjugation::Julkipanna,
        Conjugation::Ladata,
        Conjugation::Virittää,
        Conjugation::Kulkea,
        Conjugation::Sopia,
        Conjugation::Upota,
        Conjugation::Lyödä,
        Conjugation::Tuultaa,
        Conjugation::Kontia,
        Conjugation::Empiä,
        Conjugation::Siitä,
        Conjugation::Ampua,
        Conjugation::Purkaa,
        Conjugation::Tenttiä,
        Conjugation::Dubata,
        Conjugation::Tonkia,
        Conjugation::Lähteä,
        Conjugation::Heläjää,
        Conjugation::Jydätä,
        Conjugation::Heiketä,
        Conjugation::Häipyä,
        Conjugation::Takuta,
        Conjugation::Myydä,
        Conjugation::Nylkeä,
        Conjugation::Lappaa,
        Conjugation::Pinkoa,
        Conjugation::Halajaa,
        Conjugation::Nappoa,
        Conjugation::Vaaata,
        Conjugation::Tarvii,
        Conjugation::Tarttee,
        Conjugation::Bygee,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 102] = [
    // Polttaa: 1,745 words (kirjoittaa, ottaa, erottaa, muuttaa, soittaa, maattaa). Like Nostaa with `ˇ` after each pattern.
    conjugation!("polttaa", 1745, "~aaˇ", like Nostaa with "ˇ";
    ),
    // Purkautua: 1,259 words (purra, muuttua, tapahtua, surra, osoittautua, suuttua). Like Sammua with `ˇ` after each pattern.
    conjugation!("purkautua", 1259, "~aˇ", like Sammua with "ˇ";
    ),
    // Voida: 935 words (naida, uida, saada, kuukkeloida, vandalisoida, imuroida).
    conjugation!("voida", 935, "~da";
    stem indicative_pres_aff_1sg: "~- like voida";
    stem indicative_pres_aff_3sg: "~- like voida";
    stem participle_past_pass: "~- like voida";
    stem indicative_past_aff_3sg: "~- like voida";
    stem lemma: "~- like voida";
    ),
    // Hiihtää: 892 words (hiihtää, heittää, käyttää, pitää, jättää, näyttää). Like Tiivistää with `ˇ` after each pattern.
    conjugation!("hiihtää", 892, "~ääˇ", like Tiivistää with "ˇ";
    ),
    // Ohjata: 771 words (seurata, vastata, sahata, pelata, testata, googlata).
    conjugation!("ohjata", 771, "~ta";
    stem indicative_pres_aff_1sg: "~- like ohjata";
    stem indicative_pres_aff_3sg: "~A- like ohjata";
    stem participle_past_pass: "~t- like ohjata";
    stem indicative_past_aff_3sg: "~- like ohjata";
    stem lemma: "~- like ohjata";
    ),
    // Tulla: 667 words (tuulla, tulla, kuulla, arvella, olla, lomailla).
    conjugation!("tulla", 667, "~la";
    stem indicative_pres_aff_1sg: "~- like olla";
    stem indicative_pres_aff_3sg: "~e- like olla";
    stem participle_past_pass: "~- like olla";
    stem indicative_past_aff_3sg: "~i- like voida";
    stem lemma: "~- like olla";
    ),
    // Nostaa: 562 words (valmistaa, ostaa, muistaa, rakastaa, nostaa, poistaa).
    conjugation!("nostaa", 562, "~aa";
    stem indicative_pres_aff_1sg: "~- like nostaa";
    stem indicative_pres_aff_3sg: "~- like nostaa";
    stem participle_past_pass: "~et- like nostaa";
    stem indicative_past_aff_3sg: "~- like nostaa";
    stem lemma: "~- like nostaa";
    ),
    // Ajatella: 512 words (ajatella, valehdella, kuunnella, kuvitella, suudella, homotella). Like Tulla with `ˇ` after each pattern.
    conjugation!("ajatella", 512, "~laˇ", like Tulla with "ˇ";
    ),
    // Sammua: 478 words (kiehua, istua, kutsua, kuivua, puhua, lipsua). Like Voida.
    conjugation!("sammua", 478, "~a", like Voida;
    stem indicative_pres_aff_1sg: "~- like sammua";
    stem indicative_pres_aff_3sg: "~- like sammua";
    stem participle_past_pass: "~t- like nostaa";
    stem indicative_past_aff_3sg: "~i- like voida";
    stem lemma: "~- like sammua";
    ),
    // Jäätyä: 462 words (jäätyä, mennä, mädäntyä, käyttäytyä, syntyä, keskittyä). Like Kypsyä with `ˇ` after each pattern.
    conjugation!("jäätyä", 462, "~äˇ", like Kypsyä with "ˇ";
    ),
    // Epäillä: 278 words (kävellä, hymyillä, niellä, viljellä, hilseillä, epäillä). Like Tulla.
    conjugation!("epäillä", 278, "~lä", like Tulla;
    stem participle_past_pass: "~- like epäillä";
    stem lemma: "~- like epäillä";
    ),
    // Tiivistää: 237 words (pyydystää, estää, pistää, elää, äänestää, yhdistää). Like Nostaa.
    conjugation!("tiivistää", 237, "~ää", like Nostaa;
    stem indicative_pres_aff_3sg: "~- like tiivistää";
    stem lemma: "~- like tiivistää";
    ),
    // Käsitellä: 211 words (ihmetellä, teititellä, näytellä, vältellä, vietellä, äännellä). Like Epäillä with `ˇ` after each pattern.
    conjugation!("käsitellä", 211, "~läˇ", like Epäillä with "ˇ";
    ),
    // Kypsyä: 200 words (kysyä, hyväksyä, pysyä, syöksyä, väsyä, ilmestyä). Like Sammua.
    conjugation!("kypsyä", 200, "~ä", like Sammua;
    stem lemma: "~- like kypsyä";
    ),
    // Kipata: 200 words (maata, ladata, tavata, matata, taata, kammata). Like Ohjata with `ˇ` after each pattern.
    conjugation!("kipata", 200, "~taˇ", like Ohjata with "ˇ";
    ),
    // Nousta: 185 words (haista, valaista, turista, nousta, lukaista, lakaista). Like Tulla, except in 61 cells: pres_neg pass, past_neg _1sg, past_neg _1pl, past_neg _3pl, perf_aff _1sg, perf_aff _2sg, ….
    conjugation!("nousta", 185, "~ta", like Tulla;
    pres_neg: _, _, _, _, _, _, "ei ~ta";
    past_neg: "en ~sUt", _, _, "emme ~seet", _, "eivät ~seet", _;
    perf_aff: "olen ~sUt", "olet ~sUt", _, _, "olette ~seet", "ovat ~seet", _;
    perf_neg: "en ole ~sUt", _, _, "emme ole ~seet", _, "eivät ole ~seet", _;
    plup_aff: _, _, _, _, _, "olivat ~seet", _;
    plup_neg: "en ollut ~sUt", _, _, "emme olleet ~seet", _, "eivät olleet ~seet", _;
    conditional_pres_aff: _, _, _, _, _, _, "~taisiin";
    conditional_pres_neg: _, _, _, _, _, _, "ei ~taisi";
    conditional_perf_aff: "olisin ~sUt", "olisit ~sUt", "olisi ~sUt", "olisimme ~seet", "olisitte ~seet", "olisivat ~seet", _;
    conditional_perf_neg: _, _, _, "emme olisi ~seet", _, "eivät olisi ~seet", _;
    imperative_aff: _, _, _, _, _, "~takOOn";
    imperative_neg: _, _, _, _, _, "älköön ~takO";
    perfect_imperative_aff: "olkoon ~sUt", "olkoot ~seet", _;
    perfect_imperative_neg: "älköön olko ~sUt", "älkööt olko ~seet", _;
    potential_pres_aff: "~sen", _, _, _, _, "~sevAt", "~taneen";
    potential_pres_neg: _, _, _, _, _, _, "ei ~tane";
    potential_perf_aff: "lienen ~sUt", "lienet ~sUt", "lienee ~sUt", "lienemme ~seet", "lienette ~seet", "lienevät ~seet", _;
    potential_perf_neg: "en liene ~sUt", _, _, "emme liene ~seet", _, "eivät liene ~seet", _;
    infinitive: "~ta";
    long_infinitive: "~takseni", "~taksesi", "~taksemme", "~taksenne", "~takseen|~taksensA";
    second_infinitive: "~tessA", "~taessA";
    second_infinitive_possessive: "~tessAni", "~tessAsi", "~tessAmme", "~tessAnne", "~tessAAn|~tessAnsA";
    second_infinitive_instructive: "~ten";
    third_infinitive_passive: "~tamAn";
    participle: _, "~tavA", "~sUt", _, _, _;
    ),
    // Kuoria: 158 words (potkia, tutkia, nussia, toimia, funtsia, matkia). Like Sammua.
    conjugation!("kuoria", 158, "~a", like Sammua;
    stem indicative_pres_aff_1sg: "~- like voida";
    stem indicative_pres_aff_3sg: "~- like kuoria";
    stem indicative_past_aff_3sg: "~- like kuoria";
    ),
    // Johtaa: 149 words (huutaa, suomentaa, ruotsintaa, parantaa, oksentaa, rakentaa).
    conjugation!("johtaa", 149, "~taa";
    stem indicative_pres_aff_1sg: "~d- like nostaa";
    stem indicative_pres_aff_3sg: "~t- like nostaa";
    stem participle_past_pass: "~det- like nostaa";
    stem indicative_past_aff_3sg: "~t- like nostaa";
    stem lemma: "~t- like nostaa";
    ),
    // Määrätä: 140 words (kerjätä, levätä, värjätä, herätä, löyhkätä, lämmetä). Like Ohjata, except in 7 cells: pres_neg pass, infinitive, long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, ….
    conjugation!("määrätä", 140, "~tä", like Ohjata;
    pres_neg: _, _, _, _, _, _, "ei ~tä";
    infinitive: "~tä";
    long_infinitive: "~täkseni", "~täksesi", "~täksemme", "~täksenne", "~täkseen|~täksensA";
    ),
    // Päästä: 139 words (pestä, vikistä, syöstä, piestä, läpäistä, seistä). Like Nousta, except in 16 cells: pres_neg pass, conditional_pres_aff pass, conditional_pres_neg pass, imperative_aff pass, imperative_neg pass, potential_pres_aff pass, ….
    conjugation!("päästä", 139, "~tä", like Nousta;
    pres_neg: _, _, _, _, _, _, "ei ~tä";
    conditional_pres_aff: _, _, _, _, _, _, "~täisiin";
    conditional_pres_neg: _, _, _, _, _, _, "ei ~täisi";
    imperative_aff: _, _, _, _, _, "~täkOOn";
    imperative_neg: _, _, _, _, _, "älköön ~täkO";
    potential_pres_aff: _, _, _, _, _, _, "~täneen";
    potential_pres_neg: _, _, _, _, _, _, "ei ~täne";
    infinitive: "~tä";
    long_infinitive: "~täkseni", "~täksesi", "~täksemme", "~täksenne", "~täkseen|~täksensA";
    second_infinitive: _, "~täessA";
    third_infinitive_passive: "~tämAn";
    participle: _, "~tävA", _, _, _, _;
    ),
    // Leikata: 137 words (juosta, kolkata, leikata, rokata, hakata, pakata).
    conjugation!("leikata", 137, "~~ta";
    stem indicative_pres_aff_1sg: "~k~- like ohjata";
    stem indicative_pres_aff_3sg: "~k~A- like ohjata";
    stem participle_past_pass: "~~t- like ohjata";
    stem indicative_past_aff_3sg: "~k~- like ohjata";
    stem lemma: "~~- like ohjata";
    ),
    // Pyöriä: 136 words (etsiä, hässiä, vieriä, sääliä, kärsiä, siistiä). Like Kuoria.
    conjugation!("pyöriä", 136, "~ä", like Kuoria;
    stem lemma: "~- like kypsyä";
    ),
    // Hajota: 124 words (haluta, erota, kohota, tuhota, kirota, tajuta). Like Ohjata.
    conjugation!("hajota", 124, "~ta", like Ohjata;
    stem indicative_pres_aff_3sg: "~A- like hajota";
    stem indicative_past_aff_3sg: "~- like hajota";
    ),
    // Katsoa: 88 words (anoa, sanoa, katsoa, kertoa, toivoa, seisoa). Like Sammua.
    conjugation!("katsoa", 88, "~a", like Sammua;
    stem indicative_pres_aff_3sg: "~- like katsoa";
    ),
    // Pätevöidä: 84 words (nähdä, jäädä, tehdä, neliöidä, käydä, viedä). Like Voida, except in 1 cell: pres_neg pass.
    conjugation!("pätevöidä", 84, "~dä", like Voida;
    stem lemma: "~- like viedä";
    pres_neg: _, _, _, _, _, _, "ei ~dä";
    ),
    // Kääntää: 82 words (pyytää, löytää, tietää, lentää, lyhentää, kääntää). Like Johtaa.
    conjugation!("kääntää", 82, "~tää", like Johtaa;
    stem indicative_pres_aff_3sg: "~t- like tiivistää";
    stem lemma: "~t- like tiivistää";
    ),
    // Värähdellä: 74 words (pahoinpidellä, säädellä, räjähdellä, pidellä, riidellä, vedellä).
    conjugation!("värähdellä", 74, "~d~lä";
    stem indicative_pres_aff_3sg: "~t~- like värähdellä";
    stem participle_past_pass: "~d~- like epäillä";
    stem indicative_past_aff_3sg: "~t~i- like voida";
    stem lemma: "~d~- like epäillä";
    pres_aff: "~t~Vn", _, _, "~t~Vmme", _, _, _;
    pres_neg: _, _, _, "emme ~t~V", _, "eivät ~t~V", _;
    past_aff: "~t~in", _, _, "~t~imme", _, _, _;
    ),
    // Hylätä: 71 words (hypätä, pelätä, hyökätä, pyykätä, ryöpätä, töötätä). Like Määrätä with `ˇ` after each pattern.
    conjugation!("hylätä", 71, "~täˇ", like Määrätä with "ˇ";
    ),
    // Parata: 56 words (kometa, parata, someta, aleta, oheta, nuoreta). Like Ohjata.
    conjugation!("parata", 56, "~ta", like Ohjata;
    stem indicative_pres_aff_1sg: "~n- like olla";
    stem indicative_pres_aff_3sg: "~ne- like parata";
    stem indicative_past_aff_3sg: "~ni- like voida";
    ),
    // Murtaa: 51 words (ahertaa, kumartaa, murtaa, saartaa, kaivertaa, vaikertaa). Like Johtaa, except in 2 cells: past_aff _3sg, past_aff _3pl.
    conjugation!("murtaa", 51, "~taa", like Johtaa;
    stem indicative_pres_aff_1sg: "~- like murtaa";
    stem participle_past_pass: "~ret- like nostaa";
    past_aff: _, _, "~si", _, _, "~sivAt", _;
    ),
    // Tuomita: 45 words (ansaita, tarvita, valita, lukita, hallita, palkita). Like Ohjata.
    conjugation!("tuomita", 45, "~ta", like Ohjata;
    stem indicative_pres_aff_1sg: "~ts- like olla";
    stem indicative_pres_aff_3sg: "~tse- like parata";
    stem indicative_past_aff_3sg: "~tsi- like voida";
    ),
    // Asioida: 44 words (kuutioida, tupakoida, aateloida, ahkeroida, kapinoida, asioida). Like Voida, except in 20 cells: long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, long_infinitive _3, second_infinitive act, ….
    conjugation!("asioida", 44, "~da", like Voida;
    stem indicative_pres_aff_1sg: "~- like asioida";
    stem indicative_pres_aff_3sg: "~- like asioida";
    stem indicative_past_aff_3sg: "~- like asioida";
    long_infinitive: "~tAvA", "~daksemme", "~dakseni", "~daksesi", "~daksenne|~dakseen";
    second_infinitive: "~tU", "~dessA";
    second_infinitive_possessive: "~mA", "~dessAni", "~tsemA", "~dessAmme", "~dessAsi|~dessAnne";
    second_infinitive_instructive: "~dessAnsA";
    third_infinitive: _, derived, _, _, derived, _;
    third_infinitive_passive: derived;
    fifth_infinitive: _, _, derived, _, _;
    participle: _, "~tsevA", "~daksensA", "~nUt", _, _;
    ),
    // Yletä: 42 words (yletä, tyhjetä, enetä, vähetä, kylmetä, syvetä). Like Parata, except in 7 cells: pres_neg pass, infinitive, long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, ….
    conjugation!("yletä", 42, "~tä", like Parata;
    pres_neg: _, _, _, _, _, _, "ei ~tä";
    infinitive: "~tä";
    long_infinitive: "~täkseni", "~täksesi", "~täksemme", "~täksenne", "~täkseen|~täksensA";
    ),
    // Hävitä: 41 words (hävitä, yrjötä, älytä, nimetä, öljytä, hirvetä). Like Määrätä.
    conjugation!("hävitä", 41, "~tä", like Määrätä;
    stem indicative_pres_aff_3sg: "~A- like hajota";
    stem indicative_past_aff_3sg: "~- like hävitä";
    ),
    // Kiertää: 41 words (piirtää, ymmärtää, siirtää, kiertää, hiertää, hämärtää). Like Kääntää, except in 2 cells: past_aff _3sg, past_aff _3pl.
    conjugation!("kiertää", 41, "~tää", like Kääntää;
    stem indicative_pres_aff_1sg: "~- like murtaa";
    stem participle_past_pass: "~ret- like nostaa";
    past_aff: _, _, "~si", _, _, "~sivAt", _;
    ),
    // Aueta: 39 words (kimmota, aueta, haljeta, pudota, lotota, kadota). Like Hajota with `ˇ` after each pattern.
    conjugation!("aueta", 39, "~taˇ", like Hajota with "ˇ";
    ),
    // Liukua: 36 words (liikkua, nukkua, hukkua, kirkua, raakkua, kukkua).
    conjugation!("liukua", 36, "~k~a";
    stem indicative_pres_aff_1sg: "~~- like sammua";
    stem indicative_pres_aff_3sg: "~k~- like sammua";
    stem participle_past_pass: "~~t- like nostaa";
    stem indicative_past_aff_3sg: "~k~i- like voida";
    stem lemma: "~k~- like sammua";
    ),
    // Sotia: 34 words (purjehtia, luutia, ahnehtia, pohtia, vaatia, sotia).
    conjugation!("sotia", 34, "~t~a";
    stem indicative_pres_aff_1sg: "~d~- like voida";
    stem indicative_pres_aff_3sg: "~t~- like kuoria";
    stem participle_past_pass: "~d~t- like nostaa";
    stem indicative_past_aff_3sg: "~t~- like voida";
    stem lemma: "~t~- like sotia";
    ),
    // Pelehtiä: 34 words (vyyhtiä, ehtiä, mätiä, märehtiä, kyynelehtiä, ilvehtiä). Like Sotia.
    conjugation!("pelehtiä", 34, "~t~ä", like Sotia;
    stem lemma: "~t~- like kypsyä";
    ),
    // Paeta: 32 words (paeta, vaieta, saeta, taeta, valjeta, oieta). Like Parata with `ˇ` after each pattern.
    conjugation!("paeta", 32, "~taˇ", like Parata with "ˇ";
    ),
    // Kieltää: 31 words (kieltää, sisältää, edeltää, yltää, viheltää, kiiltää). Like Kiertää.
    conjugation!("kieltää", 31, "~tää", like Kiertää;
    stem indicative_pres_aff_1sg: "~- like kieltää";
    stem participle_past_pass: "~let- like nostaa";
    ),
    // Sykkiä: 30 words (leikkiä, sykkiä, pykiä, märkiä, pyrkiä, pyyhkiä). Like Liukua.
    conjugation!("sykkiä", 30, "~k~ä", like Liukua;
    stem indicative_pres_aff_1sg: "~~- like voida";
    stem indicative_pres_aff_3sg: "~k~- like kuoria";
    stem indicative_past_aff_3sg: "~k~- like sykkiä";
    stem lemma: "~k~- like kypsyä";
    ),
    // Puhaltaa: 28 words (sukeltaa, uskaltaa, suoltaa, puoltaa, kuultaa, kimaltaa). Like Murtaa, except in 2 cells: conditional_perf_aff _1pl, conditional_perf_aff _3pl.
    conjugation!("puhaltaa", 28, "~taa", like Murtaa;
    stem indicative_pres_aff_1sg: "~- like kieltää";
    stem participle_past_pass: "~let- like nostaa";
    conditional_perf_aff: _, _, _, "olisimm% ~taneet", _, "olisiva% ~taneet", _;
    ),
    // Ajaa: 27 words (ajaa, haistaa, jatkaa, jaksaa, maistaa, nauraa). Like Nostaa, except in 4 cells: past_aff _3sg, past_aff _3pl, conditional_perf_aff _1pl, conditional_perf_aff _3pl.
    conjugation!("ajaa", 27, "~aa", like Nostaa;
    stem indicative_pres_aff_1sg: "~- like ajaa";
    past_aff: _, _, "~Oi", _, _, "~OivAt", _;
    conditional_perf_aff: _, _, _, "olisimm% ~Aneet", _, "olisiva% ~aneet", _;
    ),
    // Hankkia: 25 words (kukkia, kakkia, sukia, noukkia, ruokkia, tukkia). Like Sykkiä.
    conjugation!("hankkia", 25, "~k~a", like Sykkiä;
    stem lemma: "~k~- like sammua";
    ),
    // Sitoa: 24 words (kutoa, sitoa, kutea, nitoa, kiehtoa, huuhtoa). Like Sotia.
    conjugation!("sitoa", 24, "~t~a", like Sotia;
    stem indicative_pres_aff_1sg: "~d~- like sammua";
    stem indicative_pres_aff_3sg: "~t~- like katsoa";
    stem indicative_past_aff_3sg: "~t~i- like voida";
    stem lemma: "~t~- like sammua";
    ),
    // Haikaa: 23 words (naksaa, napsaa, niksaa, hohkaa, tuikkaa, paukkaa).
    conjugation!("haikaa", 23, "~a";
    pres_aff: _, _, "~a", _, _, "~avAt", _;
    pres_neg: _, _, "ei ~a", _, _, "eivät ~a", _;
    conditional_pres_aff: _, _, "~isi", _, _, "~isivAt", _;
    conditional_pres_neg: _, _, "ei ~isi", _, _, "eivät ~isi", _;
    infinitive: "~a";
    ),
    // Rikkoa: 22 words (taikoa, rikkoa, takoa, aikoa, pilkkoa, oikoa). Like Liukua.
    conjugation!("rikkoa", 22, "~k~a", like Liukua;
    stem indicative_pres_aff_3sg: "~k~- like katsoa";
    ),
    // Isännöidä: 19 words (käräjöidä, käpälöidä, kipinöidä, seppelöidä, ikävöidä, isännöidä). Like Asioida, except in 7 cells: pres_neg pass, infinitive, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, long_infinitive _3, ….
    conjugation!("isännöidä", 19, "~dä", like Asioida;
    pres_neg: _, _, _, _, _, _, "ei ~dä";
    infinitive: "~dä";
    long_infinitive: _, "~däksemme", "~däkseni", "~däksesi", "~däksenne|~däkseen";
    participle: _, _, "~däksensA", _, _, _;
    ),
    // Rundaa: 18 words (bonjaa, köraa, dallaa, stikkaa, föraa, griinaa). Like Haikaa, except in 15 cells: pres_aff _1sg, pres_aff _1pl, pres_aff _3pl, pres_neg _3sg, pres_neg _1pl, pres_neg _2pl, ….
    conjugation!("rundaa", 18, "~a", like Haikaa;
    stem participle_past_pass: "~- like rundaa";
    stem indicative_past_aff_3sg: "~- like rundaa";
    stem lemma: "~- like rundaa";
    pres_aff: "~an", _, _, "~tAAn|~amme", _, "~a", _;
    pres_neg: _, _, derived, "ei ~ta|emme ~a", "ette ~a", "ei ~a", _;
    past_aff: "~sin", _, _, "~ttiin|~simme", _, _, _;
    third_infinitive: "~a", "~amAAn", "~amAl", _, "~amatta", _;
    participle: "~tA", _, _, _, _, "~amAst";
    ),
    // Ojentaa: 17 words (hienontaa, huonontaa, isontaa, jalontaa, juontaa, virontaa). Like Murtaa.
    conjugation!("ojentaa", 17, "~taa", like Murtaa;
    stem indicative_pres_aff_1sg: "~- like ojentaa";
    stem participle_past_pass: "~net- like nostaa";
    ),
    // Merkitä: 15 words (merkitä, hillitä, häiritä, kestitä, kyyditä, lääkitä). Like Tuomita.
    conjugation!("merkitä", 15, "~tä", like Tuomita;
    stem indicative_pres_aff_3sg: "~tse- like merkitä";
    stem participle_past_pass: "~t- like merkitä";
    stem indicative_past_aff_3sg: "~tsi- like merkitä";
    stem lemma: "~- like merkitä";
    ),
    // Ähkää: 14 words (ähkää, jyskää, räikää, helkkää, helskää, läpsää). Like Haikaa, except in 5 cells: pres_aff _3sg, pres_aff _3pl, pres_neg _3sg, pres_neg _3pl, infinitive.
    conjugation!("ähkää", 14, "~ä", like Haikaa;
    pres_aff: _, _, "~ä", _, _, "~ävAt", _;
    pres_neg: _, _, "ei ~ä", _, _, "eivät ~ä", _;
    infinitive: "~ä";
    ),
    // Kaataa: 14 words (antaa, sataa, kantaa, mahtaa, kaataa, ahtaa). Like Johtaa, except in 2 cells: past_aff _1sg, past_aff _1pl.
    conjugation!("kaataa", 14, "~taa", like Johtaa;
    stem indicative_past_aff_3sg: "~t- like kaataa";
    stem lemma: "~t- like kaataa";
    past_aff: "~doin", _, _, "~doimme", _, _, _;
    ),
    // Taittaa: 13 words (auttaa, karttaa, laittaa, saattaa, kattaa, maittaa). Like Johtaa.
    conjugation!("taittaa", 13, "~taa", like Johtaa;
    stem indicative_pres_aff_1sg: "~- like ajaa";
    stem participle_past_pass: "~et- like nostaa";
    stem indicative_past_aff_3sg: "~t- like kaataa";
    ),
    // Kiivetä: 13 words (kiivetä, iljetä, kerjetä, ängetä, herjetä, pietä). Like Hävitä with `ˇ` after each pattern.
    conjugation!("kiivetä", 13, "~täˇ", like Hävitä with "ˇ";
    ),
    // Jyrketä: 12 words (jyrketä, heiketä, lietä, seljetä, pidetä, herketä). Like Yletä with `ˇ` after each pattern.
    conjugation!("jyrketä", 12, "~täˇ", like Yletä with "ˇ";
    ),
    // Tarpoa: 12 words (leipoa, saapua, uupua, arpoa, silpoa, virpoa).
    conjugation!("tarpoa", 12, "~p~a";
    stem indicative_pres_aff_1sg: "~v~- like sammua";
    stem indicative_pres_aff_3sg: "~p~- like katsoa";
    stem participle_past_pass: "~v~t- like nostaa";
    stem indicative_past_aff_3sg: "~p~i- like voida";
    stem lemma: "~p~- like sotia";
    ),
    // Laueta: 10 words (virota, seota, kaikota, karkota, koukuta, liuota). Like Leikata.
    conjugation!("laueta", 10, "~~ta", like Leikata;
    stem indicative_pres_aff_3sg: "~k~A- like hajota";
    stem indicative_past_aff_3sg: "~k~- like hajota";
    stem lemma: "~~- like laueta";
    ),
    // Hiipiä: 10 words (kylpeä, hiipiä, kyrpiä, repiä, rypeä, silpiä). Like Tarpoa, except in 6 cells: infinitive, long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, long_infinitive _3.
    conjugation!("hiipiä", 10, "~p~ä", like Tarpoa;
    stem indicative_pres_aff_1sg: "~v~- like voida";
    stem indicative_pres_aff_3sg: "~p~- like kuoria";
    stem indicative_past_aff_3sg: "~p~- like sykkiä";
    infinitive: "~p~ä";
    long_infinitive: "~p~äkseni", "~p~äksesi", "~p~äksemme", "~p~äksenne", "~p~äkseen|~p~äksensA";
    ),
    // Hyppiä: 10 words (nyppiä, hyppiä, hörppiä, jelppiä, tärppiä, lääppiä). Like Pyöriä with `ˇ` after each pattern.
    conjugation!("hyppiä", 10, "~äˇ", like Pyöriä with "ˇ";
    ),
    // Laskea: 9 words (koskea, laskea, asea, sotkea, suitsea, puskea). Like Tulla.
    conjugation!("laskea", 9, "~ea", like Tulla;
    stem participle_past_pass: "~et- like nostaa";
    stem lemma: "~- like laskea";
    ),
    // Harppia: 9 words (oppia, nauttia, tuuppia, moittia, pomppia, kalppia). Like Kuoria with `ˇ` after each pattern.
    conjugation!("harppia", 9, "~aˇ", like Kuoria with "ˇ";
    ),
    // Kääntää2: 8 words (iirintää, filmintää, siintää, tekstintää, fäärintää, fidžintää). Like Kiertää.
    conjugation!("kääntää", 8, "~tää", like Kiertää;
    stem indicative_pres_aff_1sg: "~- like ojentaa";
    stem participle_past_pass: "~net- like nostaa";
    ),
    // Taannehtia: 8 words (oirehtia, tohtia, tuutia, tutia, talvehtia, tuikehtia). Like Sotia, except in 1 cell: conditional_perf_aff _1pl.
    conjugation!("taannehtia", 8, "~t~a", like Sotia;
    conditional_perf_aff: _, _, _, "olisimme ~t~neet", _, _, _;
    ),
    // Iskeä: 7 words (käskeä, itkeä, imeä, kätkeä, kytkeä, iskeä). Like Laskea, except in 8 cells: conditional_perf_aff _1pl, conditional_perf_aff _3pl, infinitive, long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, ….
    conjugation!("iskeä", 7, "~eä", like Laskea;
    conditional_perf_aff: _, _, _, "olisimme ~eneet", _, "olisivat ~eneet", _;
    infinitive: "~eä";
    long_infinitive: "~eäkseni", "~eäksesi", "~eäksemme", "~eäksenne", "~eäkseen|~eäksensA";
    ),
    // Kenkiä: 7 words (kenkiä, mönkiä, tinkiä, henkiä, änkeä, hönkiä). Like Sykkiä, except in 2 cells: conditional_perf_aff _1pl, conditional_perf_aff _3pl.
    conjugation!("kenkiä", 7, "~k~ä", like Sykkiä;
    stem indicative_pres_aff_1sg: "~g~- like voida";
    stem participle_past_pass: "~g~t- like nostaa";
    conditional_perf_aff: _, _, _, "olisimm% ~k~neet", _, "olisiva% ~k~neet", _;
    ),
    // Lukea: 7 words (lukea, kokea, tukea, pukea, hokea, hakea).
    conjugation!("lukea", 7, "~kea";
    stem indicative_pres_aff_1sg: "~- like olla";
    stem indicative_pres_aff_3sg: "~ke- like parata";
    stem participle_past_pass: "~et- like nostaa";
    stem indicative_past_aff_3sg: "~ki- like voida";
    stem lemma: "~k- like lukea";
    ),
    // Luoda: 7 words (juoda, tuoda, luoda, suoda, ilmituoda, julkituoda).
    conjugation!("luoda", 7, "~u~da";
    stem indicative_pres_aff_3sg: "~u~- like luoda";
    stem participle_past_pass: "~u~- like luoda";
    stem indicative_past_aff_3sg: "~~i- like voida";
    stem lemma: "~u~- like luoda";
    pres_aff: "~u~n", _, _, "~u~mme", _, _, _;
    pres_neg: _, _, _, "emme ~u~", _, "eivät ~u~", _;
    past_aff: "~~in", _, _, "~~imme", _, _, _;
    ),
    // Ruveta: 7 words (luvata, huovata, ruveta, turvota, kirvota, hervota).
    conjugation!("ruveta", 7, "~v~ta";
    stem indicative_pres_aff_1sg: "~p~- like ohjata";
    stem indicative_pres_aff_3sg: "~p~A- like hajota";
    stem participle_past_pass: "~v~t- like ohjata";
    stem indicative_past_aff_3sg: "~p~- like ruveta";
    stem lemma: "~v~- like laueta";
    ),
    // Julkipanna: 6 words (panna, julkipanna, kokoonpanna, maksuunpanna, muistiinpanna, toimeenpanna). Like Voida, except in 16 cells: pres_neg pass, conditional_perf_aff _1pl, conditional_perf_aff _3pl, infinitive, long_infinitive _1sg, long_infinitive _2sg, ….
    conjugation!("julkipanna", 6, "~na", like Voida;
    stem indicative_pres_aff_1sg: "~- like olla";
    stem indicative_pres_aff_3sg: "~e- like olla";
    stem indicative_past_aff_3sg: "~i- like voida";
    pres_neg: _, _, _, _, _, _, "ei ~na";
    conditional_perf_aff: _, _, _, "olisimm% ~neet", _, "olisiva% ~neet", _;
    infinitive: "~na";
    long_infinitive: "~nakseni", "~naksesi", "~naksemme", "~naksenne", "~nakseen|~naksensA";
    second_infinitive: "~nessA", _;
    second_infinitive_possessive: "~nessAni", "~nessAsi", "~nessAmme", "~nessAnne", "~nessAAn|~nessAnsA";
    second_infinitive_instructive: "~nen";
    ),
    // Ladata: 6 words (todeta, luodata, vedota, pedata, muodota, liiduta).
    conjugation!("ladata", 6, "~d~ta";
    stem indicative_pres_aff_1sg: "~t~- like ohjata";
    stem indicative_pres_aff_3sg: "~t~A- like ohjata";
    stem participle_past_pass: "~d~t- like ohjata";
    stem indicative_past_aff_3sg: "~t~- like ohjata";
    stem lemma: "~d~- like laueta";
    ),
    // Virittää: 5 words (pikaviestittää, postimerkittää, hajakeskittää, kantaesittää, jatkokehittää). Like Kääntää.
    conjugation!("virittää", 5, "~tää", like Kääntää;
    stem indicative_pres_aff_1sg: "~- like nostaa";
    stem participle_past_pass: "~et- like nostaa";
    ),
    // Kulkea: 5 words (sulkea, kulkea, polkea, oikosulkea, poissulkea). Like Lukea.
    conjugation!("kulkea", 5, "~kea", like Lukea;
    stem indicative_pres_aff_1sg: "~j- like olla";
    stem participle_past_pass: "~jet- like nostaa";
    stem lemma: "~k- like laskea";
    ),
    // Sopia: 5 words (sopia, raapia, kaapia, kuopia, ruopia). Like Hiipiä.
    conjugation!("sopia", 5, "~p~a", like Hiipiä;
    stem lemma: "~p~- like sotia";
    ),
    // Upota: 5 words (upota, pulputa, pumputa, lipota, silputa). Like Laueta.
    conjugation!("upota", 5, "~~ta", like Laueta;
    stem indicative_pres_aff_1sg: "~p~- like ohjata";
    stem indicative_pres_aff_3sg: "~p~- like nostaa";
    stem indicative_past_aff_3sg: "~p~- like upota";
    ),
    // Lyödä: 5 words (syödä, lyödä, laiminlyödä, myödä, ylensyödä).
    conjugation!("lyödä", 5, "~y~dä";
    stem indicative_pres_aff_3sg: "~y~- like lyödä";
    stem participle_past_pass: "~y~- like lyödä";
    stem indicative_past_aff_3sg: "~~i- like voida";
    stem lemma: "~y~- like lyödä";
    pres_aff: "~y~n", _, _, "~y~mme", _, _, _;
    pres_neg: _, _, _, "emme ~y~", _, "eivät ~y~", _;
    past_aff: "~~in", _, _, "~~imme", _, _, _;
    ),
    // Tuultaa: 5 words (oivaltaa, taivaltaa, toheltaa, tuultaa, takeltaa). Like Puhaltaa, except in 2 cells: conditional_perf_aff _1pl, conditional_perf_aff _3pl.
    conjugation!("tuultaa", 5, "~taa", like Puhaltaa;
    conditional_perf_aff: _, _, _, "olisimme ~tAneet", _, "olisiva% ~tAneet", _;
    ),
    // Kontia: 5 words (tuntea, suuntia, sontia, kontia, runtoa). Like Sotia.
    conjugation!("kontia", 5, "~t~a", like Sotia;
    stem indicative_pres_aff_1sg: "~n~- like voida";
    stem participle_past_pass: "~n~t- like nostaa";
    ),
    // Empiä: 5 words (empiä, lempiä, rämpiä, kömpiä, tympiä). Like Hiipiä.
    conjugation!("empiä", 5, "~p~ä", like Hiipiä;
    stem indicative_pres_aff_1sg: "~m~- like voida";
    stem participle_past_pass: "~m~t- like nostaa";
    ),
    // Siitä: 4 words (siitä, keritä, selitä, kehitä). Like Leikata, except in 9 cells: pres_neg pass, conditional_perf_aff _1pl, conditional_perf_aff _3pl, infinitive, long_infinitive _1sg, long_infinitive _2sg, ….
    conjugation!("siitä", 4, "~~tä", like Leikata;
    stem indicative_pres_aff_3sg: "~k~A- like hajota";
    stem indicative_past_aff_3sg: "~k~- like hävitä";
    pres_neg: _, _, _, _, _, _, "ei ~~tä";
    conditional_perf_aff: _, _, _, "olisimm% ~~nneet", _, "olisiva% ~~nneet", _;
    infinitive: "~~tä";
    long_infinitive: "~~täkseni", "~~täksesi", "~~täksemme", "~~täksenne", "~~täkseen|~~täksensA";
    ),
    // Ampua: 4 words (ampua, tempoa, yliampua, kompia). Like Empiä.
    conjugation!("ampua", 4, "~p~a", like Empiä;
    stem indicative_pres_aff_1sg: "~m~- like sammua";
    stem indicative_pres_aff_3sg: "~p~- like sammua";
    stem indicative_past_aff_3sg: "~p~i- like voida";
    stem lemma: "~p~- like sotia";
    ),
    // Purkaa: 4 words (alkaa, jakaa, purkaa, virkkaa).
    conjugation!("purkaa", 4, "~kaa";
    stem indicative_pres_aff_3sg: "~ka- like purkaa";
    stem participle_past_pass: "~et- like nostaa";
    stem indicative_past_aff_3sg: "~k- like purkaa";
    stem lemma: "~k- like purkaa";
    pres_aff: "~an", _, _, "~amme", _, _, _;
    pres_neg: _, _, _, "emme ~a", _, "eivät ~a", _;
    past_aff: "~oin", _, _, "~oimme", _, _, _;
    ),
    // Tenttiä: 4 words (miettiä, sättiä, tenttiä, ettiä). Like Pelehtiä, except in 2 cells: conditional_perf_aff _1pl, conditional_perf_aff _3pl.
    conjugation!("tenttiä", 4, "~t~ä", like Pelehtiä;
    stem indicative_pres_aff_1sg: "~~- like voida";
    stem participle_past_pass: "~~t- like nostaa";
    conditional_perf_aff: _, _, _, "olisimm% ~t~neet", _, "olisiva% ~t~neet", _;
    ),
    // Dubata: 4 words (lobata, dubata, mobata, jobata). Like Leikata, except in 6 cells: pres_aff _1sg, pres_aff _1pl, pres_neg _1pl, pres_neg _3pl, past_aff _1sg, past_aff _1pl.
    conjugation!("dubata", 4, "~~ta", like Leikata;
    stem indicative_pres_aff_3sg: "~b~a- like luoda";
    stem indicative_past_aff_3sg: "~b~- like dubata";
    stem lemma: "~~- like laueta";
    pres_aff: "~b~an", _, _, "~b~amme", _, _, _;
    pres_neg: _, _, _, "emme ~b~a", _, "eivät ~b~a", _;
    past_aff: "~b~sin", _, _, "~b~simme", _, _, _;
    ),
    // Tonkia: 4 words (onkia, tunkea, tonkia, sonkia). Like Kenkiä, except in 7 cells: conditional_perf_aff _1pl, infinitive, long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, ….
    conjugation!("tonkia", 4, "~k~a", like Kenkiä;
    conditional_perf_aff: _, _, _, "olisimme ~k~neet", _, _, _;
    infinitive: "~k~a";
    long_infinitive: "~k~akseni", "~k~aksesi", "~k~aksemme", "~k~aksenne", "~k~akseen|~k~aksensA";
    ),
    // Lähteä: 4 words (lähteä, kyteä, päteä, syteä).
    conjugation!("lähteä", 4, "~teä";
    stem indicative_pres_aff_1sg: "~d- like olla";
    stem indicative_pres_aff_3sg: "~te- like parata";
    stem participle_past_pass: "~det- like nostaa";
    stem indicative_past_aff_3sg: "~ti- like voida";
    stem lemma: "~t- like lähteä";
    ),
    // Heläjää: 4 words (heläjää, täräjää, väräjää, pöläjää). Like Haikaa, except in 24 cells: pres_aff _3sg, pres_aff _3pl, pres_neg _3sg, pres_neg _3pl, conditional_pres_aff pass, conditional_pres_neg pass, ….
    conjugation!("heläjää", 4, "~jää", like Haikaa;
    stem indicative_past_aff_3sg: "~jAisi- like heläjää";
    stem lemma: "~- like heläjää";
    pres_aff: _, _, derived, _, _, derived, _;
    pres_neg: _, _, derived, _, _, derived, _;
    conditional_pres_aff: _, _, _, _, _, _, "~jAttAisiin";
    conditional_pres_neg: _, _, _, _, _, _, "ei ~ttAisi";
    conditional_perf_aff: _, _, _, _, _, _, "olisi ~ttU";
    imperative_aff: "~jA", _, _, _, _, "~ttAkOOn";
    imperative_neg: "älä ~jA", _, _, _, _, "älköön ~ttAkO";
    perfect_imperative_aff: _, _, "olkoon ~ttU";
    perfect_imperative_neg: _, _, "älköön olko ~ttU";
    potential_pres_aff: _, _, _, _, _, _, "~ttAneen";
    potential_pres_neg: _, _, _, _, _, _, "ei ~ttAne";
    potential_perf_aff: _, _, _, _, _, _, "lienee ~ttU";
    potential_perf_neg: _, _, _, _, _, _, "ei liene ~ttU";
    second_infinitive: _, "~jAmAtOn";
    third_infinitive: "~jAmAn", "~jAminen", _, _, _, _;
    participle: "~tA", "~jAvA", _, "~ten", "~jAmAssA", _;
    ),
    // Jydätä: 4 words (jydätä, pöydätä, myödätä, töydätä). Like Ladata, except in 7 cells: pres_neg pass, infinitive, long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, ….
    conjugation!("jydätä", 4, "~d~tä", like Ladata;
    pres_neg: _, _, _, _, _, _, "ei ~d~tä";
    infinitive: "~d~tä";
    long_infinitive: "~d~täkseni", "~d~täksesi", "~d~täksemme", "~d~täksenne", "~d~täkseen|~d~täksensA";
    ),
    // Heiketä: 3 words (kyetä, käetä, märätä). Like Siitä.
    conjugation!("heiketä", 3, "~~tä", like Siitä;
    stem indicative_pres_aff_1sg: "~k~n- like olla";
    stem indicative_pres_aff_3sg: "~k~ne- like parata";
    stem indicative_past_aff_3sg: "~k~ni- like voida";
    ),
    // Häipyä: 3 words (syöpyä, yöpyä, elpyä). Like Tarpoa, except in 6 cells: infinitive, long_infinitive _1sg, long_infinitive _2sg, long_infinitive _1pl, long_infinitive _2pl, long_infinitive _3.
    conjugation!("häipyä", 3, "~p~ä", like Tarpoa;
    stem indicative_pres_aff_3sg: "~p~- like sammua";
    infinitive: "~p~ä";
    long_infinitive: "~p~äkseni", "~p~äksesi", "~p~äksemme", "~p~äksenne", "~p~äkseen|~p~äksensA";
    ),
    // Takuta: 3 words (koota, takuta, tauota). Like Laueta, except in 1 cell: conditional_perf_aff _1pl.
    conjugation!("takuta", 3, "~~ta", like Laueta;
    conditional_perf_aff: _, _, _, "olisimme ~~nneet", _, _, _;
    ),
    // Myydä: 3 words (myydä, loppuunmyydä, alimyydä).
    conjugation!("myydä", 3, "~ydä";
    stem indicative_pres_aff_3sg: "~U- like ohjata";
    stem participle_past_pass: "~- like myydä";
    stem indicative_past_aff_3sg: "~i- like voida";
    stem lemma: "~- like myydä";
    pres_aff: "~Un", _, _, "~Umme", _, _, _;
    pres_neg: _, _, _, "emme ~U", _, "eivät ~U", _;
    past_aff: "~in", _, _, "~imme", _, _, _;
    ),
    // Nylkeä: 3 words (sylkeä, särkeä, nylkeä). Like Kulkea.
    conjugation!("nylkeä", 3, "~keä", like Kulkea;
    stem lemma: "~k- like lähteä";
    ),
    // Lappaa: 3 words (tappaa, appaa, lappaa).
    conjugation!("lappaa", 3, "~paa";
    stem indicative_pres_aff_1sg: "~- like ajaa";
    stem indicative_pres_aff_3sg: "~p- like nostaa";
    stem participle_past_pass: "~et- like nostaa";
    stem indicative_past_aff_3sg: "~p- like kaataa";
    stem lemma: "~p- like kaataa";
    ),
    // Pinkoa: 3 words (penkoa, pinkoa, rankoa). Like Tonkia.
    conjugation!("pinkoa", 3, "~k~a", like Tonkia;
    stem indicative_pres_aff_1sg: "~g~- like sammua";
    stem indicative_pres_aff_3sg: "~k~- like katsoa";
    stem indicative_past_aff_3sg: "~k~i- like voida";
    stem lemma: "~k~- like sotia";
    ),
    // Halajaa: 3 words (kumajaa, halajaa, vipajaa). Like Heläjää, except in 1 cell: infinitive.
    conjugation!("halajaa", 3, "~jaa", like Heläjää;
    infinitive: "~jaa";
    ),
    // Nappoa: 3 words (harppoa, loppea, nappoa). Like Tarpoa.
    conjugation!("nappoa", 3, "~p~a", like Tarpoa;
    stem indicative_pres_aff_1sg: "~~- like sammua";
    stem participle_past_pass: "~~t- like nostaa";
    stem lemma: "~p~- like sammua";
    ),
    // Vaaata: 2 words (vaa'ata, laa'ata). Like Leikata, except in 78 cells: pres_neg pass, past_aff pass, past_neg _1sg, past_neg _1pl, past_neg _3pl, past_neg pass, ….
    conjugation!("vaa'ata", 2, "~'~ta", like Leikata;
    pres_neg: _, _, _, _, _, _, derived;
    past_aff: _, _, _, _, _, _, derived;
    past_neg: derived, _, _, derived, _, derived, derived;
    perf_aff: derived, derived, _, _, derived, derived, _;
    perf_neg: derived, _, _, derived, _, derived, derived;
    plup_aff: _, _, _, _, _, derived, _;
    plup_neg: derived, _, _, derived, _, derived, derived;
    conditional_pres_aff: _, _, _, _, _, _, derived;
    conditional_pres_neg: _, _, _, _, _, _, derived;
    conditional_perf_aff: derived, derived, derived, derived, derived, derived, derived;
    conditional_perf_neg: _, _, _, derived, _, derived, _;
    imperative_aff: _, derived, derived, derived, _, derived;
    imperative_neg: _, derived, derived, derived, derived, derived;
    perfect_imperative_aff: derived, derived, derived;
    perfect_imperative_neg: derived, derived, derived;
    potential_pres_aff: derived, _, _, _, _, derived, derived;
    potential_pres_neg: _, _, _, _, _, _, derived;
    potential_perf_aff: derived, derived, derived, derived, derived, derived, derived;
    potential_perf_neg: derived, _, _, derived, _, derived, derived;
    infinitive: "~'~ta";
    long_infinitive: derived, derived, derived, derived, derived;
    second_infinitive: derived, derived;
    second_infinitive_possessive: derived, derived, derived, derived, derived;
    second_infinitive_instructive: derived;
    third_infinitive_passive: derived;
    participle: _, derived, derived, derived, _, _;
    ),
    // Tarvii: 1 words (tarvii). Like Haikaa, except in 13 cells: pres_aff _1sg, pres_aff _3sg, pres_aff _1pl, pres_aff _3pl, pres_neg _1sg, pres_neg _2sg, ….
    conjugation!("tarvii", 1, "~i", like Haikaa;
    stem indicative_past_aff_3sg: "~- like tarvii";
    pres_aff: "~in", _, "~i", "~imme", _, "~ivAt", _;
    pres_neg: "en ~i|~", "et ~i|~", "ei ~i|~", "emme ~i|~", "ette ~i|~", "eivät ~i|~", _;
    imperative_aff: "~i|~", _, _, _, _, _;
    imperative_neg: "älä ~i|~", _, _, _, _, _;
    infinitive: "~i";
    ),
    // Tarttee: 1 words (tarttee). Like Haikaa, except in 21 cells: pres_aff _3sg, pres_aff _3pl, pres_neg _3sg, pres_neg pass, past_aff pass, past_neg _1sg, ….
    conjugation!("tarttee", 1, "~ee", like Haikaa;
    stem indicative_pres_aff_1sg: "~- like olla";
    stem indicative_past_aff_3sg: "~i- like voida";
    pres_aff: _, _, "~ee", _, _, "~evAt", _;
    pres_neg: _, _, derived, _, _, _, "ei ~etA";
    past_aff: _, _, _, _, _, _, "~ettiin";
    past_neg: "en ~enUt", _, _, _, _, _, _;
    perf_aff: "olen ~enUt", "olet ~enUt", _, _, _, _, _;
    perf_neg: "en ole ~enUt", _, _, _, _, _, _;
    plup_neg: "en ollut ~enUt", _, _, _, _, _, _;
    conditional_perf_aff: "olisin ~enUt", "olisit ~enUt", "olisi ~enUt", _, _, _, _;
    perfect_imperative_aff: "olkoon ~enUt", _, _;
    perfect_imperative_neg: "älköön olko ~enUt", _, _;
    potential_perf_aff: "lienen ~enUt", "lienet ~enUt", "lienee ~enUt", _, _, _, _;
    potential_perf_neg: "en liene ~enUt", _, _, _, _, _, _;
    infinitive: "~ee";
    long_infinitive: _, _, "~enut", _, _;
    ),
    // Bygee: 1 words (bygee). Like Haikaa, except in 16 cells: pres_aff _1sg, pres_aff _3sg, pres_aff _3pl, pres_neg _3sg, pres_neg _3pl, past_aff _1sg, ….
    conjugation!("bygee", 1, "~e", like Haikaa;
    pres_aff: "~en", _, "~e", _, _, derived, _;
    pres_neg: _, _, derived, _, _, derived, _;
    past_aff: "~sin", _, "~s", _, _, _, _;
    past_neg: "en ~nnU", _, _, _, _, _, _;
    conditional_pres_aff: "~isin", _, "~is", _, _, derived, _;
    conditional_pres_neg: "en ~is", "et ~is", "ei ~is", _, _, derived, _;
    infinitive: "~e";
    ),
];
