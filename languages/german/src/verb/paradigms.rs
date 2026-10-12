//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's auxiliary says):
/// `present_first` (sg, pl)
/// `present_second` (sg, pl)
/// `present_third` (sg, pl)
/// `preterite_first` (sg, pl)
/// `preterite_second` (sg, pl)
/// `preterite_third` (sg, pl)
/// `subjunctive_i_first` (sg, pl)
/// `subjunctive_i_second` (sg, pl)
/// `subjunctive_i_third` (sg, pl)
/// `subjunctive_i_i_first` (sg, pl)
/// `subjunctive_i_i_second` (sg, pl)
/// `subjunctive_i_i_third` (sg, pl)
/// `subordinate_present_first` (sg, pl)
/// `subordinate_present_second` (sg, pl)
/// `subordinate_present_third` (sg, pl)
/// `subordinate_preterite_first` (sg, pl)
/// `subordinate_preterite_second` (sg, pl)
/// `subordinate_preterite_third` (sg, pl)
/// `subordinate_subjunctive_i_first` (sg, pl)
/// `subordinate_subjunctive_i_second` (sg, pl)
/// `subordinate_subjunctive_i_third` (sg, pl)
/// `subordinate_subjunctive_i_i_first` (sg, pl)
/// `subordinate_subjunctive_i_i_second` (sg, pl)
/// `subordinate_subjunctive_i_i_third` (sg, pl)
/// `imperative` (sg, pl)
/// `infinitive`
/// `zu_infinitive`
/// `participle` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_first: $c0:tt, $c1:tt;
     present_second: $c2:tt, $c3:tt;
     present_third: $c4:tt, $c5:tt;
     preterite_first: $c6:tt, $c7:tt;
     preterite_second: $c8:tt, $c9:tt;
     preterite_third: $c10:tt, $c11:tt;
     subjunctive_i_first: $c12:tt, $c13:tt;
     subjunctive_i_second: $c14:tt, $c15:tt;
     subjunctive_i_third: $c16:tt, $c17:tt;
     subjunctive_i_i_first: $c18:tt, $c19:tt;
     subjunctive_i_i_second: $c20:tt, $c21:tt;
     subjunctive_i_i_third: $c22:tt, $c23:tt;
     subordinate_present_first: $c24:tt, $c25:tt;
     subordinate_present_second: $c26:tt, $c27:tt;
     subordinate_present_third: $c28:tt, $c29:tt;
     subordinate_preterite_first: $c30:tt, $c31:tt;
     subordinate_preterite_second: $c32:tt, $c33:tt;
     subordinate_preterite_third: $c34:tt, $c35:tt;
     subordinate_subjunctive_i_first: $c36:tt, $c37:tt;
     subordinate_subjunctive_i_second: $c38:tt, $c39:tt;
     subordinate_subjunctive_i_third: $c40:tt, $c41:tt;
     subordinate_subjunctive_i_i_first: $c42:tt, $c43:tt;
     subordinate_subjunctive_i_i_second: $c44:tt, $c45:tt;
     subordinate_subjunctive_i_i_third: $c46:tt, $c47:tt;
     imperative: $c48:tt, $c49:tt;
     infinitive: $c50:tt;
     zu_infinitive: $c51:tt;
     participle: $c52:tt, $c53:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
                flexion::form!($c48),
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
                flexion::form!($c52),
                flexion::form!($c53),
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `machen`: 1,407 words.
    Machen,
    /// Like `kassieren`: 1,247 words.
    Kassieren,
    /// Like `nudeln`: 812 words.
    Nudeln,
    /// Like `richten`: 324 words.
    Richten,
    /// Like `lösen`: 313 words.
    Lösen,
    /// Like `lagen`: 33 words.
    Lagen,
    /// Like `schreiben`: 32 words.
    Schreiben,
    /// Like `nehmen`: 30 words.
    Nehmen,
    /// Like `sein`: 29 words.
    Sein,
    /// Like `langmachen`: 24 words.
    Langmachen,
    /// Like `befördern`: 19 words.
    Befördern,
    /// Like `dagegenhalten`: 19 words.
    Dagegenhalten,
    /// Like `schwingen`: 18 words.
    Schwingen,
    /// Like `bereiten`: 18 words.
    Bereiten,
    /// Like `zeigen`: 16 words.
    Zeigen,
    /// Like `werden`: 12 words.
    Werden,
    /// Like `drangehen`: 12 words.
    Drangehen,
    /// Like `fliegen`: 11 words.
    Fliegen,
    /// Like `wahrnehmen`: 11 words.
    Wahrnehmen,
    /// Like `vorfallen`: 10 words.
    Vorfallen,
    /// Like `schwimmen`: 9 words.
    Schwimmen,
    /// Like `schmelzen`: 9 words.
    Schmelzen,
    /// Like `gelten`: 8 words.
    Gelten,
    /// Like `schmeißen`: 8 words.
    Schmeißen,
    /// Like `halten`: 8 words.
    Halten,
    /// Like `bieten`: 8 words.
    Bieten,
    /// Like `schießen`: 8 words.
    Schießen,
    /// Like `gegenüberstellen`: 8 words.
    Gegenüberstellen,
    /// Like `wehtun`: 8 words.
    Wehtun,
    /// Like `tun`: 7 words.
    Tun,
    /// Like `zetteln`: 7 words.
    Zetteln,
    /// Like `drankommen`: 7 words.
    Drankommen,
    /// Like `versetzen`: 7 words.
    Versetzen,
    /// Like `stehen`: 7 words.
    Stehen,
    /// Like `beissen`: 6 words.
    Beissen,
    /// Like `binden`: 6 words.
    Binden,
    /// Like `gucken`: 6 words.
    Gucken,
    /// Like `ziehen`: 6 words.
    Ziehen,
    /// Like `verwenden`: 6 words.
    Verwenden,
    /// Like `greifen`: 5 words.
    Greifen,
    /// Like `schleichen`: 5 words.
    Schleichen,
    /// Like `glänzen`: 5 words.
    Glänzen,
    /// Like `gabeln`: 5 words.
    Gabeln,
    /// Like `darüberliegen`: 5 words.
    Darüberliegen,
    /// Like `bekennen`: 5 words.
    Bekennen,
    /// Like `beklimmen`: 5 words.
    Beklimmen,
    /// Like `brennen`: 4 words.
    Brennen,
    /// Like `reiten`: 4 words.
    Reiten,
    /// Like `vierteilen`: 4 words.
    Vierteilen,
    /// Like `eilen`: 4 words.
    Eilen,
    /// Like `notlanden`: 4 words.
    Notlanden,
    /// Like `gegenüberstehen`: 4 words.
    Gegenüberstehen,
    /// Like `fechten`: 4 words.
    Fechten,
    /// Like `umhinkönnen`: 4 words.
    Umhinkönnen,
    /// Like `behalten`: 4 words.
    Behalten,
    /// Like `begegnen`: 4 words.
    Begegnen,
    /// Like `reien`: 3 words.
    Reien,
    /// Like `scheren`: 3 words.
    Scheren,
    /// Like `pflegen`: 3 words.
    Pflegen,
    /// Like `zucken`: 3 words.
    Zucken,
    /// Like `ekeln`: 3 words.
    Ekeln,
    /// Like `schlechtmachen`: 3 words.
    Schlechtmachen,
    /// Like `lüsten`: 3 words.
    Lüsten,
    /// Like `endlagern`: 3 words.
    Endlagern,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 64] = [
        Conjugation::Machen,
        Conjugation::Kassieren,
        Conjugation::Nudeln,
        Conjugation::Richten,
        Conjugation::Lösen,
        Conjugation::Lagen,
        Conjugation::Schreiben,
        Conjugation::Nehmen,
        Conjugation::Sein,
        Conjugation::Langmachen,
        Conjugation::Befördern,
        Conjugation::Dagegenhalten,
        Conjugation::Schwingen,
        Conjugation::Bereiten,
        Conjugation::Zeigen,
        Conjugation::Werden,
        Conjugation::Drangehen,
        Conjugation::Fliegen,
        Conjugation::Wahrnehmen,
        Conjugation::Vorfallen,
        Conjugation::Schwimmen,
        Conjugation::Schmelzen,
        Conjugation::Gelten,
        Conjugation::Schmeißen,
        Conjugation::Halten,
        Conjugation::Bieten,
        Conjugation::Schießen,
        Conjugation::Gegenüberstellen,
        Conjugation::Wehtun,
        Conjugation::Tun,
        Conjugation::Zetteln,
        Conjugation::Drankommen,
        Conjugation::Versetzen,
        Conjugation::Stehen,
        Conjugation::Beissen,
        Conjugation::Binden,
        Conjugation::Gucken,
        Conjugation::Ziehen,
        Conjugation::Verwenden,
        Conjugation::Greifen,
        Conjugation::Schleichen,
        Conjugation::Glänzen,
        Conjugation::Gabeln,
        Conjugation::Darüberliegen,
        Conjugation::Bekennen,
        Conjugation::Beklimmen,
        Conjugation::Brennen,
        Conjugation::Reiten,
        Conjugation::Vierteilen,
        Conjugation::Eilen,
        Conjugation::Notlanden,
        Conjugation::Gegenüberstehen,
        Conjugation::Fechten,
        Conjugation::Umhinkönnen,
        Conjugation::Behalten,
        Conjugation::Begegnen,
        Conjugation::Reien,
        Conjugation::Scheren,
        Conjugation::Pflegen,
        Conjugation::Zucken,
        Conjugation::Ekeln,
        Conjugation::Schlechtmachen,
        Conjugation::Lüsten,
        Conjugation::Endlagern,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 64] = [
    // Machen: 1,407 words (raven, stricken, nerven, legen, glauben, wägen).
    conjugation!("machen", 1407, "~en";
        present_first: "~e", "~en";
        present_second: "~st", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~e", "~en";
        subordinate_present_second: "~st", "~t";
        subordinate_present_third: "~t", "~en";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: referred;
        zu_infinitive: "zu~en";
        participle: "~end", "ge~t";
    ),
    // Kassieren: 1,247 words (protegieren, faschieren, urinieren, mikturieren, miauen, gratulieren).
    conjugation!("kassieren", 1247, "~en";
        present_first: "~e", "~en";
        present_second: "~st", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~e", "~en";
        subordinate_present_second: "~st", "~t";
        subordinate_present_third: "~t", "~en";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: referred;
        zu_infinitive: "zu~en";
        participle: "~end", "~t";
    ),
    // Nudeln: 812 words (nebeln, modern, klettern, pinkeln, steuern, tafeln).
    conjugation!("nudeln", 812, "~e~n";
        present_first: "~~e|~e~e|~e~", "~e~n";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~n";
        preterite_first: "~e~te", "~e~ten";
        preterite_second: "~e~test", "~e~tet";
        preterite_third: "~e~te", "~e~ten";
        subjunctive_i_first: "~e~e|~~e", "~e~n";
        subjunctive_i_second: "~e~est|~~est", "~e~et|~~et";
        subjunctive_i_third: "~e~e|~~e", "~e~n";
        subjunctive_i_i_first: "~e~te", "~e~ten";
        subjunctive_i_i_second: "~e~test", "~e~tet";
        subjunctive_i_i_third: "~e~te", "~e~ten";
        subordinate_present_first: "~~e|~e~e|~e~", "~e~n";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~n";
        subordinate_preterite_first: "~e~te", "~e~ten";
        subordinate_preterite_second: "~e~test", "~e~tet";
        subordinate_preterite_third: "~e~te", "~e~ten";
        subordinate_subjunctive_i_first: "~e~e|~~e", "~e~n";
        subordinate_subjunctive_i_second: "~e~est|~~est", "~e~et|~~et";
        subordinate_subjunctive_i_third: "~e~e|~~e", "~e~n";
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~~e|~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~n";
        participle: "~e~nd", "ge~e~t";
    ),
    // Richten: 324 words (listen, bitten, rotten, landen, baden, arbeiten).
    conjugation!("richten", 324, "~en";
        present_first: "~e", "~en";
        present_second: "~est", "~et";
        present_third: "~et", "~en";
        preterite_first: "~ete", "~eten";
        preterite_second: "~etest", "~etet";
        preterite_third: "~ete", "~eten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~ete", "~eten";
        subjunctive_i_i_second: "~etest", "~etet";
        subjunctive_i_i_third: "~ete", "~eten";
        subordinate_present_first: "~e", "~en";
        subordinate_present_second: "~est", "~et";
        subordinate_present_third: "~et", "~en";
        subordinate_preterite_first: "~ete", "~eten";
        subordinate_preterite_second: "~etest", "~etet";
        subordinate_preterite_third: "~ete", "~eten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~ete", "~eten";
        subordinate_subjunctive_i_i_second: "~etest", "~etet";
        subordinate_subjunctive_i_i_third: "~ete", "~eten";
        imperative: "~|~e", "~et";
        infinitive: referred;
        zu_infinitive: "zu~en";
        participle: "~end", "ge~et";
    ),
    // Lösen: 313 words (pflanzen, aasen, pissen, boxen, fotzen, sitzen).
    conjugation!("lösen", 313, "~en";
        present_first: "~e", "~en";
        present_second: "~t", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~e", "~en";
        subordinate_present_second: "~t", "~t";
        subordinate_present_third: "~t", "~en";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: referred;
        zu_infinitive: "zu~en";
        participle: "~end", "ge~t";
    ),
    // Lagen: 33 words (fallen, fangen, laden, fahren, laufen, lassen).
    conjugation!("lagen", 33, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~t";
        present_third: "~ä~t", "~a~en";
        preterite_first: "~u~", "~u~en";
        preterite_second: "~u~st", "~u~t";
        preterite_third: "~u~", "~u~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", "~a~en";
        subjunctive_i_i_first: "~ü~e", "~ü~en";
        subjunctive_i_i_second: "~ü~est", "~ü~et";
        subjunctive_i_i_third: "~ü~e", "~ü~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~st", "~a~t";
        subordinate_present_third: "~ä~t", "~a~en";
        subordinate_preterite_first: "~u~", "~u~en";
        subordinate_preterite_second: "~u~st", "~u~t";
        subordinate_preterite_third: "~u~", "~u~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", "~a~en";
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est", "~ü~et";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~a~", "~a~t";
        infinitive: referred;
        zu_infinitive: "zu~a~en";
        participle: "~a~end", "ge~a~en";
    ),
    // Schreiben: 32 words (heißen, schreiben, scheinen, meiden, bleiben, scheiden).
    conjugation!("schreiben", 32, "~e~~en";
        present_first: "~e~~e", "~e~~en";
        present_second: "~e~~st", "~e~~t";
        present_third: "~e~~t", "~e~~en";
        preterite_first: "~~e~", "~~e~en";
        preterite_second: "~~e~st", "~~e~t";
        preterite_third: "~~e~", "~~e~en";
        subjunctive_i_first: "~e~~e", "~e~~en";
        subjunctive_i_second: "~e~~est", "~e~~et";
        subjunctive_i_third: "~e~~e", "~e~~en";
        subjunctive_i_i_first: "~~e~e", "~~e~en";
        subjunctive_i_i_second: "~~e~est", "~~e~et|~~e~t";
        subjunctive_i_i_third: "~~e~e", "~~e~en";
        subordinate_present_first: "~e~~e", "~e~~en";
        subordinate_present_second: "~e~~st", "~e~~t";
        subordinate_present_third: "~e~~t", "~e~~en";
        subordinate_preterite_first: "~~e~", "~~e~en";
        subordinate_preterite_second: "~~e~st", "~~e~t";
        subordinate_preterite_third: "~~e~", "~~e~en";
        subordinate_subjunctive_i_first: "~e~~e", "~e~~en";
        subordinate_subjunctive_i_second: "~e~~est", "~e~~et";
        subordinate_subjunctive_i_third: "~e~~e", "~e~~en";
        subordinate_subjunctive_i_i_first: "~~e~e", "~~e~en";
        subordinate_subjunctive_i_i_second: "~~e~est", "~~e~et|~~e~t";
        subordinate_subjunctive_i_i_third: "~~e~e", "~~e~en";
        imperative: "~e~~|~e~~e", "~e~~t";
        infinitive: referred;
        zu_infinitive: "zu~e~~en";
        participle: "~e~~end", "ge~~e~en";
    ),
    // Nehmen: 30 words (sehen, geben, nehmen, lesen, vergessen, treten).
    conjugation!("nehmen", 30, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "~e~en";
    ),
    // Sein: 29 words (regnen, sein, öffnen, atmen, rechnen, leugnen).
    conjugation!("sein", 29, "~n";
        present_first: "~", "~n";
        present_second: "~st", "~t";
        present_third: "~t", "~n";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~", "~n";
        subjunctive_i_second: "~st", "~t";
        subjunctive_i_third: "~", "~n";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~", "~n";
        subordinate_present_second: "~st", "~t";
        subordinate_present_third: "~t", "~n";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~", "~n";
        subordinate_subjunctive_i_second: "~st", "~t";
        subordinate_subjunctive_i_third: "~", "~n";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~", "~t";
        infinitive: referred;
        zu_infinitive: "zu~n";
        participle: "~nd", "ge~t";
    ),
    // Langmachen: 24 words (wiedergutmachen, wettmachen, kaltmachen, mobilmachen, bekanntmachen, weismachen).
    conjugation!("langmachen", 24, "~machen";
        present_first: "mache ~", "machen ~";
        present_second: "machst ~", "macht ~";
        present_third: "macht ~", "machen ~";
        preterite_first: "machte ~", "machten ~";
        preterite_second: "machtest ~", "machtet ~";
        preterite_third: "machte ~", "machten ~";
        subjunctive_i_first: "mache ~", "machen ~";
        subjunctive_i_second: "machest ~", "machet ~";
        subjunctive_i_third: "mache ~", "machen ~";
        subjunctive_i_i_first: "machte ~", "machten ~";
        subjunctive_i_i_second: "machtest ~", "machtet ~";
        subjunctive_i_i_third: "machte ~", "machten ~";
        subordinate_present_first: "~mache", "~machen";
        subordinate_present_second: "~machst", "~macht";
        subordinate_present_third: "~macht", "~machen";
        subordinate_preterite_first: "~machte", "~machten";
        subordinate_preterite_second: "~machtest", "~machtet";
        subordinate_preterite_third: "~machte", "~machten";
        subordinate_subjunctive_i_first: "~mache", "~machen";
        subordinate_subjunctive_i_second: "~machest", "~machet";
        subordinate_subjunctive_i_third: "~mache", "~machen";
        subordinate_subjunctive_i_i_first: "~machte", "~machten";
        subordinate_subjunctive_i_i_second: "~machtest", "~machtet";
        subordinate_subjunctive_i_i_third: "~machte", "~machten";
        imperative: "mach ~", "macht ~";
        infinitive: referred;
        zu_infinitive: "~zumachen";
        participle: "~machend", "~gemacht";
    ),
    // Befördern: 19 words (begeistern, kalfatern, containern, vergewissern, misgendern, menetekeln).
    conjugation!("befördern", 19, "~e~n";
        present_first: "~~e|~e~e|~e~", "~e~n";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~n";
        preterite_first: "~e~te", "~e~ten";
        preterite_second: "~e~test", "~e~tet";
        preterite_third: "~e~te", "~e~ten";
        subjunctive_i_first: "~e~e|~~e", "~e~n";
        subjunctive_i_second: "~e~est|~~est", "~e~et|~~et";
        subjunctive_i_third: "~e~e|~~e", "~e~n";
        subjunctive_i_i_first: "~e~te", "~e~ten";
        subjunctive_i_i_second: "~e~test", "~e~tet";
        subjunctive_i_i_third: "~e~te", "~e~ten";
        subordinate_present_first: "~~e|~e~e|~e~", "~e~n";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~n";
        subordinate_preterite_first: "~e~te", "~e~ten";
        subordinate_preterite_second: "~e~test", "~e~tet";
        subordinate_preterite_third: "~e~te", "~e~ten";
        subordinate_subjunctive_i_first: "~e~e|~~e", "~e~n";
        subordinate_subjunctive_i_second: "~e~est|~~est", "~e~et|~~et";
        subordinate_subjunctive_i_third: "~e~e|~~e", "~e~n";
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~~e|~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~n";
        participle: "~e~nd", "~e~t";
    ),
    // Dagegenhalten: 19 words (standhalten, warmhalten, maßhalten, masshalten, aneinanderhalten, beieinanderhalten).
    conjugation!("dagegenhalten", 19, "~halten";
        present_first: "halte ~", "halten ~";
        present_second: "hältst ~", "haltet ~";
        present_third: "hält ~", "halten ~";
        preterite_first: "hielt ~", "hielten ~";
        preterite_second: "hieltest ~|hieltst ~", "hieltet ~";
        preterite_third: "hielt ~", "hielten ~";
        subjunctive_i_first: "halte ~", "halten ~";
        subjunctive_i_second: "haltest ~", "haltet ~";
        subjunctive_i_third: "halte ~", "halten ~";
        subjunctive_i_i_first: "hielte ~", "hielten ~";
        subjunctive_i_i_second: "hieltest ~", "hieltet ~";
        subjunctive_i_i_third: "hielte ~", "hielten ~";
        subordinate_present_first: "~halte", "~halten";
        subordinate_present_second: "~hältst", "~haltet";
        subordinate_present_third: "~hält", "~halten";
        subordinate_preterite_first: "~hielt", "~hielten";
        subordinate_preterite_second: "~hieltest|~hieltst", "~hieltet";
        subordinate_preterite_third: "~hielt", "~hielten";
        subordinate_subjunctive_i_first: "~halte", "~halten";
        subordinate_subjunctive_i_second: "~haltest", "~haltet";
        subordinate_subjunctive_i_third: "~halte", "~halten";
        subordinate_subjunctive_i_i_first: "~hielte", "~hielten";
        subordinate_subjunctive_i_i_second: "~hieltest", "~hieltet";
        subordinate_subjunctive_i_i_third: "~hielte", "~hielten";
        imperative: "halt ~|halte ~", "haltet ~";
        infinitive: referred;
        zu_infinitive: "~zuhalten";
        participle: "~haltend", "~gehalten";
    ),
    // Schwingen: 18 words (trinken, springen, singen, wringen, dringen, klingen).
    conjugation!("schwingen", 18, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~st", "~i~t";
        present_third: "~i~t", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", "~i~en";
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~i~e", "~i~en";
        subordinate_present_second: "~i~st", "~i~t";
        subordinate_present_third: "~i~t", "~i~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_second: "~i~est", "~i~et";
        subordinate_subjunctive_i_third: "~i~e", "~i~en";
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~|~i~e", "~i~t";
        infinitive: referred;
        zu_infinitive: "zu~i~en";
        participle: "~i~end", "ge~u~en";
    ),
    // Bereiten: 18 words (begleiten, ballasten, bereiten, überbewerten, entgelten, obwalten).
    conjugation!("bereiten", 18, "~en";
        present_first: "~e", "~en";
        present_second: "~est", "~et";
        present_third: "~et", "~en";
        preterite_first: "~ete", "~eten";
        preterite_second: "~etest", "~etet";
        preterite_third: "~ete", "~eten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~ete", "~eten";
        subjunctive_i_i_second: "~etest", "~etet";
        subjunctive_i_i_third: "~ete", "~eten";
        subordinate_present_first: "~e", "~en";
        subordinate_present_second: "~est", "~et";
        subordinate_present_third: "~et", "~en";
        subordinate_preterite_first: "~ete", "~eten";
        subordinate_preterite_second: "~etest", "~etet";
        subordinate_preterite_third: "~ete", "~eten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~ete", "~eten";
        subordinate_subjunctive_i_i_second: "~etest", "~etet";
        subordinate_subjunctive_i_i_third: "~ete", "~eten";
        imperative: "~|~e", "~et";
        infinitive: referred;
        zu_infinitive: "zu~en";
        participle: "~end", "~et";
    ),
    // Zeigen: 16 words (zahlen, zählen, zeigen, zwängen, zocken, zielen).
    conjugation!("zeigen", 16, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", "~~en";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~~e", "~~en";
        subordinate_present_second: "~~st", "~~t";
        subordinate_present_third: "~~t", "~~en";
        subordinate_preterite_first: "~~te", "~~ten";
        subordinate_preterite_second: "~~test", "~~tet";
        subordinate_preterite_third: "~~te", "~~ten";
        subordinate_subjunctive_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_third: "~~e", "~~en";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: referred;
        zu_infinitive: "~uz~en";
        participle: "~~end", "ge~~t";
    ),
    // Werden: 12 words (helfen, werden, fressen, werfen, sterben, messen).
    conjugation!("werden", 12, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~ü~e", "~ü~en";
        subjunctive_i_i_second: "~ü~est", "~ü~et";
        subjunctive_i_i_third: "~ü~e", "~ü~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est", "~ü~et";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~i~", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~o~en";
    ),
    // Drangehen: 12 words (spazierengehen, vorübergehen, sichergehen, drangehen, fremdgehen, verlorengehen).
    conjugation!("drangehen", 12, "~gehen";
        present_first: "gehe ~", "gehen ~";
        present_second: "gehst ~", "geht ~";
        present_third: "geht ~", "gehen ~";
        preterite_first: "ging ~", "gingen ~";
        preterite_second: "gingst ~", "gingt ~";
        preterite_third: "ging ~", "gingen ~";
        subjunctive_i_first: "gehe ~", "gehen ~";
        subjunctive_i_second: "gehest ~", "gehet ~";
        subjunctive_i_third: "gehe ~", "gehen ~";
        subjunctive_i_i_first: "ginge ~", "gingen ~";
        subjunctive_i_i_second: "gingest ~", "ginget ~";
        subjunctive_i_i_third: "ginge ~", "gingen ~";
        subordinate_present_first: "~gehe", "~gehen";
        subordinate_present_second: "~gehst", "~geht";
        subordinate_present_third: "~geht", "~gehen";
        subordinate_preterite_first: "~ging", "~gingen";
        subordinate_preterite_second: "~gingst", "~gingt";
        subordinate_preterite_third: "~ging", "~gingen";
        subordinate_subjunctive_i_first: "~gehe", "~gehen";
        subordinate_subjunctive_i_second: "~gehest", "~gehet";
        subordinate_subjunctive_i_third: "~gehe", "~gehen";
        subordinate_subjunctive_i_i_first: "~ginge", "~gingen";
        subordinate_subjunctive_i_i_second: "~gingest", "~ginget";
        subordinate_subjunctive_i_i_third: "~ginge", "~gingen";
        imperative: "geh ~", "geht ~";
        infinitive: referred;
        zu_infinitive: "~zugehen";
        participle: "~gehend", "~gegangen";
    ),
    // Fliegen: 11 words (biegen, wiegen, fliehen, fliegen, riechen, frieren).
    conjugation!("fliegen", 11, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~st", "~ie~t";
        present_third: "~ie~t", "~ie~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", "~ie~en";
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~ie~e", "~ie~en";
        subordinate_present_second: "~ie~st", "~ie~t";
        subordinate_present_third: "~ie~t", "~ie~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~st", "~o~t";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_third: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ie~|~ie~e", "~ie~t";
        infinitive: referred;
        zu_infinitive: "zu~ie~en";
        participle: "~ie~end", "ge~o~en";
    ),
    // Wahrnehmen: 11 words (teilnehmen, wahrnehmen, vorwegnehmen, vorliebnehmen, wundernehmen, überhandnehmen).
    conjugation!("wahrnehmen", 11, "~nehmen";
        present_first: "nehme ~", "nehmen ~";
        present_second: "nimmst ~", "nehmt ~";
        present_third: "nimmt ~", "nehmen ~";
        preterite_first: "nahm ~", "nahmen ~";
        preterite_second: "nahmst ~", "nahmt ~";
        preterite_third: "nahm ~", "nahmen ~";
        subjunctive_i_first: "nehme ~", "nehmen ~";
        subjunctive_i_second: "nehmest ~", "nehmet ~";
        subjunctive_i_third: "nehme ~", "nehmen ~";
        subjunctive_i_i_first: "nähme ~", "nähmen ~";
        subjunctive_i_i_second: "nähmest ~", "nähmet ~";
        subjunctive_i_i_third: "nähme ~", "nähmen ~";
        subordinate_present_first: "~nehme", "~nehmen";
        subordinate_present_second: "~nimmst", "~nehmt";
        subordinate_present_third: "~nimmt", "~nehmen";
        subordinate_preterite_first: "~nahm", "~nahmen";
        subordinate_preterite_second: "~nahmst", "~nahmt";
        subordinate_preterite_third: "~nahm", "~nahmen";
        subordinate_subjunctive_i_first: "~nehme", "~nehmen";
        subordinate_subjunctive_i_second: "~nehmest", "~nehmet";
        subordinate_subjunctive_i_third: "~nehme", "~nehmen";
        subordinate_subjunctive_i_i_first: "~nähme", "~nähmen";
        subordinate_subjunctive_i_i_second: "~nähmest", "~nähmet";
        subordinate_subjunctive_i_i_third: "~nähme", "~nähmen";
        imperative: "nimm ~", "nehmt ~";
        infinitive: referred;
        zu_infinitive: "~zunehmen";
        participle: "~nehmend", "~genommen";
    ),
    // Vorfallen: 10 words (lenzen, einleuchten, schwerfallen, ziemen, vorfallen, ehebrechen).
    conjugation!("vorfallen", 10, "~en";
        present_first: "~e", "~en";
        present_second: "~st", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~e", "~en";
        subordinate_present_second: "~st", "~t";
        subordinate_present_third: "~t", "~en";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~", "~t";
        infinitive: "~en";
        zu_infinitive: "zu~en";
        participle: "~end", "ge~t";
    ),
    // Schwimmen: 9 words (klimmen, schwimmen, spinnen, glimmen, sinnen, rinnen).
    conjugation!("schwimmen", 9, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~st", "~i~t";
        present_third: "~i~t", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", "~i~en";
        subjunctive_i_i_first: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subjunctive_i_i_second: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
        subjunctive_i_i_third: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subordinate_present_first: "~i~e", "~i~en";
        subordinate_present_second: "~i~st", "~i~t";
        subordinate_present_third: "~i~t", "~i~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_second: "~i~est", "~i~et";
        subordinate_subjunctive_i_third: "~i~e", "~i~en";
        subordinate_subjunctive_i_i_first: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ä~e|~ö~e", "~ä~en|~ö~en";
        imperative: "~i~|~i~e", "~i~t";
        infinitive: referred;
        zu_infinitive: "zu~i~en";
        participle: "~i~end", "ge~o~en";
    ),
    // Schmelzen: 9 words (schmelzen, dreschen, melken, schwellen, quellen, umschmelzen).
    conjugation!("schmelzen", 9, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est", "~ö~et|~ö~t";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~st", "~o~t";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~i~", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~o~en";
    ),
    // Gelten: 8 words (bergen, sprechen, stehlen, brechen, bersten, gelten).
    conjugation!("gelten", 8, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~o~en";
    ),
    // Schmeißen: 8 words (scheißen, beißen, schleißen, reißen, schmeißen, spleißen).
    conjugation!("schmeißen", 8, "~e~ßen";
        present_first: "~e~ße", "~e~ßen";
        present_second: "~e~ßt", "~e~ßt";
        present_third: "~e~ßt", "~e~ßen";
        preterite_first: "~~ss", "~~ssen";
        preterite_second: "~~ssest|~~sst", "~~sst";
        preterite_third: "~~ss", "~~ssen";
        subjunctive_i_first: "~e~ße", "~e~ßen";
        subjunctive_i_second: "~e~ßest", "~e~ßet";
        subjunctive_i_third: "~e~ße", "~e~ßen";
        subjunctive_i_i_first: "~~sse", "~~ssen";
        subjunctive_i_i_second: "~~ssest|~~sst", "~~sset|~~sst";
        subjunctive_i_i_third: "~~sse", "~~ssen";
        subordinate_present_first: "~e~ße", "~e~ßen";
        subordinate_present_second: "~e~ßt", "~e~ßt";
        subordinate_present_third: "~e~ßt", "~e~ßen";
        subordinate_preterite_first: "~~ss", "~~ssen";
        subordinate_preterite_second: "~~ssest|~~sst", "~~sst";
        subordinate_preterite_third: "~~ss", "~~ssen";
        subordinate_subjunctive_i_first: "~e~ße", "~e~ßen";
        subordinate_subjunctive_i_second: "~e~ßest", "~e~ßet";
        subordinate_subjunctive_i_third: "~e~ße", "~e~ßen";
        subordinate_subjunctive_i_i_first: "~~sse", "~~ssen";
        subordinate_subjunctive_i_i_second: "~~ssest|~~sst", "~~sset|~~sst";
        subordinate_subjunctive_i_i_third: "~~sse", "~~ssen";
        imperative: "~e~ß|~e~ße", "~e~ßt";
        infinitive: referred;
        zu_infinitive: "zu~e~ßen";
        participle: "~e~ßend", "ge~~ssen";
    ),
    // Halten: 8 words (braten, halten, blasen, schlafen, raten, rathen).
    conjugation!("halten", 8, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~et";
        present_third: "~ä~", "~a~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~est", "~ie~et";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", "~a~en";
        subjunctive_i_i_first: "~ie~e", "~ie~en";
        subjunctive_i_i_second: "~ie~est", "~ie~et";
        subjunctive_i_i_third: "~ie~e", "~ie~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~st", "~a~et";
        subordinate_present_third: "~ä~", "~a~en";
        subordinate_preterite_first: "~ie~", "~ie~en";
        subordinate_preterite_second: "~ie~est", "~ie~et";
        subordinate_preterite_third: "~ie~", "~ie~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", "~a~en";
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~a~|~a~e", "~a~et";
        infinitive: referred;
        zu_infinitive: "zu~a~en";
        participle: "~a~end", "ge~a~en";
    ),
    // Bieten: 8 words (schliessen, giessen, fliessen, bieten, schiessen, niessen).
    conjugation!("bieten", 8, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~t", "~ie~t";
        present_third: "~ie~t", "~ie~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~est|~o~t", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", "~ie~en";
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est|~ö~t", "~ö~et|~ö~t";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~ie~e", "~ie~en";
        subordinate_present_second: "~ie~t", "~ie~t";
        subordinate_present_third: "~ie~t", "~ie~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~est|~o~t", "~o~t";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_third: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~t", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ie~|~ie~e", "~ie~t";
        infinitive: referred;
        zu_infinitive: "zu~ie~en";
        participle: "~ie~end", "ge~o~en";
    ),
    // Schießen: 8 words (fließen, gießen, schließen, schießen, sprießen, nießen).
    conjugation!("schießen", 8, "~ießen";
        present_first: "~ieße", "~ießen";
        present_second: "~ießt", "~ießt";
        present_third: "~ießt", "~ießen";
        preterite_first: "~oss", "~ossen";
        preterite_second: "~ossest|~osst", "~osst";
        preterite_third: "~oss", "~ossen";
        subjunctive_i_first: "~ieße", "~ießen";
        subjunctive_i_second: "~ießest", "~ießet";
        subjunctive_i_third: "~ieße", "~ießen";
        subjunctive_i_i_first: "~össe", "~össen";
        subjunctive_i_i_second: "~össest|~össt", "~össet|~össt";
        subjunctive_i_i_third: "~össe", "~össen";
        subordinate_present_first: "~ieße", "~ießen";
        subordinate_present_second: "~ießt", "~ießt";
        subordinate_present_third: "~ießt", "~ießen";
        subordinate_preterite_first: "~oss", "~ossen";
        subordinate_preterite_second: "~ossest|~osst", "~osst";
        subordinate_preterite_third: "~oss", "~ossen";
        subordinate_subjunctive_i_first: "~ieße", "~ießen";
        subordinate_subjunctive_i_second: "~ießest", "~ießet";
        subordinate_subjunctive_i_third: "~ieße", "~ießen";
        subordinate_subjunctive_i_i_first: "~össe", "~össen";
        subordinate_subjunctive_i_i_second: "~össest|~össt", "~össet|~össt";
        subordinate_subjunctive_i_i_third: "~össe", "~össen";
        imperative: "~ieß|~ieße", "~ießt";
        infinitive: referred;
        zu_infinitive: "zu~ießen";
        participle: "~ießend", "ge~ossen";
    ),
    // Gegenüberstellen: 8 words (zufriedenstellen, wiederherstellen, sicherstellen, gegenüberstellen, bloßstellen, besserstellen).
    conjugation!("gegenüberstellen", 8, "~stellen";
        present_first: "stelle ~", "stellen ~";
        present_second: "stellst ~", "stellt ~";
        present_third: "stellt ~", "stellen ~";
        preterite_first: "stellte ~", "stellten ~";
        preterite_second: "stelltest ~", "stelltet ~";
        preterite_third: "stellte ~", "stellten ~";
        subjunctive_i_first: "stelle ~", "stellen ~";
        subjunctive_i_second: "stellest ~", "stellet ~";
        subjunctive_i_third: "stelle ~", "stellen ~";
        subjunctive_i_i_first: "stellte ~", "stellten ~";
        subjunctive_i_i_second: "stelltest ~", "stelltet ~";
        subjunctive_i_i_third: "stellte ~", "stellten ~";
        subordinate_present_first: "~stelle", "~stellen";
        subordinate_present_second: "~stellst", "~stellt";
        subordinate_present_third: "~stellt", "~stellen";
        subordinate_preterite_first: "~stellte", "~stellten";
        subordinate_preterite_second: "~stelltest", "~stelltet";
        subordinate_preterite_third: "~stellte", "~stellten";
        subordinate_subjunctive_i_first: "~stelle", "~stellen";
        subordinate_subjunctive_i_second: "~stellest", "~stellet";
        subordinate_subjunctive_i_third: "~stelle", "~stellen";
        subordinate_subjunctive_i_i_first: "~stellte", "~stellten";
        subordinate_subjunctive_i_i_second: "~stelltest", "~stelltet";
        subordinate_subjunctive_i_i_third: "~stellte", "~stellten";
        imperative: "stell ~", "stellt ~";
        infinitive: referred;
        zu_infinitive: "~zustellen";
        participle: "~stellend", "~gestellt";
    ),
    // Wehtun: 8 words (wehtun, leidtun, heimlichtun, schwertun, nottun, wichtigtun).
    conjugation!("wehtun", 8, "~tun";
        present_first: "tue ~", "tun ~";
        present_second: "tust ~", "tut ~";
        present_third: "tut ~", "tun ~";
        preterite_first: "tat ~", "taten ~";
        preterite_second: "tatest ~|tatst ~", "tatet ~";
        preterite_third: "tat ~", "taten ~";
        subjunctive_i_first: "tue ~", "tuen ~";
        subjunctive_i_second: "tuest ~", "tuet ~";
        subjunctive_i_third: "tue ~", "tuen ~";
        subjunctive_i_i_first: "täte ~", "täten ~";
        subjunctive_i_i_second: "tätest ~", "tätet ~";
        subjunctive_i_i_third: "täte ~", "täten ~";
        subordinate_present_first: "~tue", "~tun";
        subordinate_present_second: "~tust", "~tut";
        subordinate_present_third: "~tut", "~tun";
        subordinate_preterite_first: "~tat", "~taten";
        subordinate_preterite_second: "~tatest|~tatst", "~tatet";
        subordinate_preterite_third: "~tat", "~taten";
        subordinate_subjunctive_i_first: "~tue", "~tuen";
        subordinate_subjunctive_i_second: "~tuest", "~tuet";
        subordinate_subjunctive_i_third: "~tue", "~tuen";
        subordinate_subjunctive_i_i_first: "~täte", "~täten";
        subordinate_subjunctive_i_i_second: "~tätest", "~tätet";
        subordinate_subjunctive_i_i_third: "~täte", "~täten";
        imperative: "tu ~|tue ~", "tut ~";
        infinitive: referred;
        zu_infinitive: "~zutun";
        participle: "~tuend", "~getan";
    ),
    // Tun: 7 words (tun, fensterln, thun, äußerln, stierln, flankerln).
    conjugation!("tun", 7, "~n";
        present_first: "~e", "~n";
        present_second: "~st", "~t";
        present_third: "~t", "~n";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~e", "~n";
        subordinate_present_second: "~st", "~t";
        subordinate_present_third: "~t", "~n";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: referred;
        zu_infinitive: "zu~n";
        participle: "~nd", "ge~t";
    ),
    // Zetteln: 7 words (zögern, zaubern, zwinkern, zweifeln, zetteln, zimmern).
    conjugation!("zetteln", 7, "~~e~n";
        present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        present_second: "~~e~st", "~~e~t";
        present_third: "~~e~t", "~~e~n";
        preterite_first: "~~e~te", "~~e~ten";
        preterite_second: "~~e~test", "~~e~tet";
        preterite_third: "~~e~te", "~~e~ten";
        subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subjunctive_i_i_third: "~~e~te", "~~e~ten";
        subordinate_present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        subordinate_present_second: "~~e~st", "~~e~t";
        subordinate_present_third: "~~e~t", "~~e~n";
        subordinate_preterite_first: "~~e~te", "~~e~ten";
        subordinate_preterite_second: "~~e~test", "~~e~tet";
        subordinate_preterite_third: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subordinate_subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subordinate_subjunctive_i_i_third: "~~e~te", "~~e~ten";
        imperative: "~~~e|~~e~|~~e~e", "~~e~t";
        infinitive: referred;
        zu_infinitive: "~uz~e~n";
        participle: "~~e~nd", "ge~~e~t";
    ),
    // Drankommen: 7 words (drankommen, zupasskommen, abhandenkommen, zugutekommen, hierherkommen, dahinterkommen).
    conjugation!("drankommen", 7, "~kommen";
        present_first: "komme ~", "kommen ~";
        present_second: "kommst ~", "kommt ~";
        present_third: "kommt ~", "kommen ~";
        preterite_first: "kam ~", "kamen ~";
        preterite_second: "kamst ~", "kamt ~";
        preterite_third: "kam ~", "kamen ~";
        subjunctive_i_first: "komme ~", "kommen ~";
        subjunctive_i_second: "kommest ~", "kommet ~";
        subjunctive_i_third: "komme ~", "kommen ~";
        subjunctive_i_i_first: "käme ~", "kämen ~";
        subjunctive_i_i_second: "kämest ~|kämst ~", "kämet ~|kämt ~";
        subjunctive_i_i_third: "käme ~", "kämen ~";
        subordinate_present_first: "~komme", "~kommen";
        subordinate_present_second: "~kommst", "~kommt";
        subordinate_present_third: "~kommt", "~kommen";
        subordinate_preterite_first: "~kam", "~kamen";
        subordinate_preterite_second: "~kamst", "~kamt";
        subordinate_preterite_third: "~kam", "~kamen";
        subordinate_subjunctive_i_first: "~komme", "~kommen";
        subordinate_subjunctive_i_second: "~kommest", "~kommet";
        subordinate_subjunctive_i_third: "~komme", "~kommen";
        subordinate_subjunctive_i_i_first: "~käme", "~kämen";
        subordinate_subjunctive_i_i_second: "~kämest|~kämst", "~kämet|~kämt";
        subordinate_subjunctive_i_i_third: "~käme", "~kämen";
        imperative: "komm ~|komme ~", "kommt ~";
        infinitive: referred;
        zu_infinitive: "~zukommen";
        participle: "~kommend", "~gekommen";
    ),
    // Versetzen: 7 words (stibitzen, veranlassen, schmarotzen, kredenzen, ergetzen, besetzen).
    conjugation!("versetzen", 7, "~en";
        present_first: "~e", "~en";
        present_second: "~t", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", "~en";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~e", "~en";
        subordinate_present_second: "~t", "~t";
        subordinate_present_third: "~t", "~en";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~e", "~en";
        subordinate_subjunctive_i_second: "~est", "~et";
        subordinate_subjunctive_i_third: "~e", "~en";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: referred;
        zu_infinitive: "zu~en";
        participle: "~end", "~t";
    ),
    // Stehen: 7 words (stehen, missverstehen, mißverstehen, wiederauferstehen, erstehen, gestehen).
    conjugation!("stehen", 7, "~ehen";
        present_first: "~ehe", "~ehen";
        present_second: "~ehst", "~eht";
        present_third: "~eht", "~ehen";
        preterite_first: "~and", "~anden";
        preterite_second: "~andest|~andst", "~andet";
        preterite_third: "~and", "~anden";
        subjunctive_i_first: "~ehe", "~ehen";
        subjunctive_i_second: "~ehest", "~ehet";
        subjunctive_i_third: "~ehe", "~ehen";
        subjunctive_i_i_first: "~ände|~ünde", "~änden|~ünden";
        subjunctive_i_i_second: "~ändest|~ündest", "~ändet|~ündet";
        subjunctive_i_i_third: "~ände|~ünde", "~änden|~ünden";
        subordinate_present_first: "~ehe", "~ehen";
        subordinate_present_second: "~ehst", "~eht";
        subordinate_present_third: "~eht", "~ehen";
        subordinate_preterite_first: "~and", "~anden";
        subordinate_preterite_second: "~andest|~andst", "~andet";
        subordinate_preterite_third: "~and", "~anden";
        subordinate_subjunctive_i_first: "~ehe", "~ehen";
        subordinate_subjunctive_i_second: "~ehest", "~ehet";
        subordinate_subjunctive_i_third: "~ehe", "~ehen";
        subordinate_subjunctive_i_i_first: "~ände|~ünde", "~änden|~ünden";
        subordinate_subjunctive_i_i_second: "~ändest|~ündest", "~ändet|~ündet";
        subordinate_subjunctive_i_i_third: "~ände|~ünde", "~änden|~ünden";
        imperative: "~eh|~ehe", "~eht";
        infinitive: referred;
        zu_infinitive: "zu~ehen";
        participle: "~ehend", "~anden";
    ),
    // Beissen: 6 words (scheissen, reissen, beissen, schmeissen, fleissen, leissen).
    conjugation!("beissen", 6, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~t", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~", "~~en";
        preterite_second: "~~est|~~t", "~~t";
        preterite_third: "~~", "~~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~~e", "~~en";
        subjunctive_i_i_second: "~~est|~~t", "~~et|~~t";
        subjunctive_i_i_third: "~~e", "~~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~t", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~~", "~~en";
        subordinate_preterite_second: "~~est|~~t", "~~t";
        subordinate_preterite_third: "~~", "~~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_i_second: "~~est|~~t", "~~et|~~t";
        subordinate_subjunctive_i_i_third: "~~e", "~~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~~en";
    ),
    // Binden: 6 words (finden, binden, winden, schwinden, empfinden, schinden).
    conjugation!("binden", 6, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~est", "~i~et";
        present_third: "~i~et", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~est|~a~st", "~a~et";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", "~i~en";
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~i~e", "~i~en";
        subordinate_present_second: "~i~est", "~i~et";
        subordinate_present_third: "~i~et", "~i~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~est|~a~st", "~a~et";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_second: "~i~est", "~i~et";
        subordinate_subjunctive_i_third: "~i~e", "~i~en";
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~|~i~e", "~i~et";
        infinitive: referred;
        zu_infinitive: "zu~i~en";
        participle: "~i~end", "ge~u~en";
    ),
    // Gucken: 6 words (gähnen, glühen, grauen, grillen, gaffen, gucken).
    conjugation!("gucken", 6, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", "~~en";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~~e", "~~en";
        subordinate_present_second: "~~st", "~~t";
        subordinate_present_third: "~~t", "~~en";
        subordinate_preterite_first: "~~te", "~~ten";
        subordinate_preterite_second: "~~test", "~~tet";
        subordinate_preterite_third: "~~te", "~~ten";
        subordinate_subjunctive_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_third: "~~e", "~~en";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: referred;
        zu_infinitive: "zu~~en";
        participle: "~~end", "~eg~t";
    ),
    // Ziehen: 6 words (ziehen, großziehen, blankziehen, vollziehen, beziehen, erziehen).
    conjugation!("ziehen", 6, "~iehen";
        present_first: "~iehe", "~iehen";
        present_second: "~iehst", "~ieht";
        present_third: "~ieht", "~iehen";
        preterite_first: "~og", "~ogen";
        preterite_second: "~ogst", "~ogt";
        preterite_third: "~og", "~ogen";
        subjunctive_i_first: "~iehe", "~iehen";
        subjunctive_i_second: "~iehest", "~iehet";
        subjunctive_i_third: "~iehe", "~iehen";
        subjunctive_i_i_first: "~öge", "~ögen";
        subjunctive_i_i_second: "~ögest|~ögst", "~öget|~ögt";
        subjunctive_i_i_third: "~öge", "~ögen";
        subordinate_present_first: "~iehe", "~iehen";
        subordinate_present_second: "~iehst", "~ieht";
        subordinate_present_third: "~ieht", "~iehen";
        subordinate_preterite_first: "~og", "~ogen";
        subordinate_preterite_second: "~ogst", "~ogt";
        subordinate_preterite_third: "~og", "~ogen";
        subordinate_subjunctive_i_first: "~iehe", "~iehen";
        subordinate_subjunctive_i_second: "~iehest", "~iehet";
        subordinate_subjunctive_i_third: "~iehe", "~iehen";
        subordinate_subjunctive_i_i_first: "~öge", "~ögen";
        subordinate_subjunctive_i_i_second: "~ögest|~ögst", "~öget|~ögt";
        subordinate_subjunctive_i_i_third: "~öge", "~ögen";
        imperative: "~ieh|~iehe", "~ieht";
        infinitive: referred;
        zu_infinitive: "zu~iehen";
        participle: "~iehend", "~ogen";
    ),
    // Verwenden: 6 words (verleiden, vergeuden, afterreden, bemitleiden, zweckentfremden, verwenden).
    conjugation!("verwenden", 6, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~est", "~e~et";
        present_third: "~e~et", "~e~en";
        preterite_first: "~e~ete", "~e~eten";
        preterite_second: "~e~etest", "~e~etet";
        preterite_third: "~e~ete", "~e~eten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~e~ete", "~e~eten";
        subjunctive_i_i_second: "~e~etest", "~e~etet";
        subjunctive_i_i_third: "~e~ete", "~e~eten";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~est", "~e~et";
        subordinate_present_third: "~e~et", "~e~en";
        subordinate_preterite_first: "~e~ete", "~e~eten";
        subordinate_preterite_second: "~e~etest", "~e~etet";
        subordinate_preterite_third: "~e~ete", "~e~eten";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~e~ete", "~e~eten";
        subordinate_subjunctive_i_i_second: "~e~etest", "~e~etet";
        subordinate_subjunctive_i_i_third: "~e~ete", "~e~eten";
        imperative: "~e~", "~e~et";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "~e~et";
    ),
    // Greifen: 5 words (pfeifen, greifen, kneifen, schleifen, begreifen).
    conjugation!("greifen", 5, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~f", "~~fen";
        preterite_second: "~~fst", "~~ft";
        preterite_third: "~~f", "~~fen";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~~fe", "~~fen";
        subjunctive_i_i_second: "~~fest|~~fst", "~~fet|~~ft";
        subjunctive_i_i_third: "~~fe", "~~fen";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~~f", "~~fen";
        subordinate_preterite_second: "~~fst", "~~ft";
        subordinate_preterite_third: "~~f", "~~fen";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~~fe", "~~fen";
        subordinate_subjunctive_i_i_second: "~~fest|~~fst", "~~fet|~~ft";
        subordinate_subjunctive_i_i_third: "~~fe", "~~fen";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~~fen";
    ),
    // Schleichen: 5 words (weichen, schleichen, streichen, gleichen, verbleichen).
    conjugation!("schleichen", 5, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~", "~~en";
        preterite_second: "~~st", "~~t";
        preterite_third: "~~", "~~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~~e", "~~en";
        subjunctive_i_i_second: "~~est|~~st", "~~et|~~t";
        subjunctive_i_i_third: "~~e", "~~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~~", "~~en";
        subordinate_preterite_second: "~~st", "~~t";
        subordinate_preterite_third: "~~", "~~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_i_second: "~~est|~~st", "~~et|~~t";
        subordinate_subjunctive_i_i_third: "~~e", "~~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~~en";
    ),
    // Glänzen: 5 words (glänzen, grunzen, grenzen, glotzen, gleisen).
    conjugation!("glänzen", 5, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~t", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", "~~en";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~~e", "~~en";
        subordinate_present_second: "~~t", "~~t";
        subordinate_present_third: "~~t", "~~en";
        subordinate_preterite_first: "~~te", "~~ten";
        subordinate_preterite_second: "~~test", "~~tet";
        subordinate_preterite_third: "~~te", "~~ten";
        subordinate_subjunctive_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_third: "~~e", "~~en";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: referred;
        zu_infinitive: "zu~~en";
        participle: "~~end", "~eg~t";
    ),
    // Gabeln: 5 words (gabeln, gliedern, gammeln, gaukeln, ganseln).
    conjugation!("gabeln", 5, "~~e~n";
        present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        present_second: "~~e~st", "~~e~t";
        present_third: "~~e~t", "~~e~n";
        preterite_first: "~~e~te", "~~e~ten";
        preterite_second: "~~e~test", "~~e~tet";
        preterite_third: "~~e~te", "~~e~ten";
        subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subjunctive_i_i_third: "~~e~te", "~~e~ten";
        subordinate_present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        subordinate_present_second: "~~e~st", "~~e~t";
        subordinate_present_third: "~~e~t", "~~e~n";
        subordinate_preterite_first: "~~e~te", "~~e~ten";
        subordinate_preterite_second: "~~e~test", "~~e~tet";
        subordinate_preterite_third: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subordinate_subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subordinate_subjunctive_i_i_third: "~~e~te", "~~e~ten";
        imperative: "~~~e|~~e~|~~e~e", "~~e~t";
        infinitive: referred;
        zu_infinitive: "zu~~e~n";
        participle: "~~e~nd", "~eg~e~t";
    ),
    // Darüberliegen: 5 words (darüberliegen, darniederliegen, falschliegen, richtigliegen, brachliegen).
    conjugation!("darüberliegen", 5, "~liegen";
        present_first: "liege ~", "liegen ~";
        present_second: "liegst ~", "liegt ~";
        present_third: "liegt ~", "liegen ~";
        preterite_first: "lag ~", "lagen ~";
        preterite_second: "lagst ~", "lagt ~";
        preterite_third: "lag ~", "lagen ~";
        subjunctive_i_first: "liege ~", "liegen ~";
        subjunctive_i_second: "liegest ~", "lieget ~";
        subjunctive_i_third: "liege ~", "liegen ~";
        subjunctive_i_i_first: "läge ~", "lägen ~";
        subjunctive_i_i_second: "lägest ~|lägst ~", "läget ~|lägt ~";
        subjunctive_i_i_third: "läge ~", "lägen ~";
        subordinate_present_first: "~liege", "~liegen";
        subordinate_present_second: "~liegst", "~liegt";
        subordinate_present_third: "~liegt", "~liegen";
        subordinate_preterite_first: "~lag", "~lagen";
        subordinate_preterite_second: "~lagst", "~lagt";
        subordinate_preterite_third: "~lag", "~lagen";
        subordinate_subjunctive_i_first: "~liege", "~liegen";
        subordinate_subjunctive_i_second: "~liegest", "~lieget";
        subordinate_subjunctive_i_third: "~liege", "~liegen";
        subordinate_subjunctive_i_i_first: "~läge", "~lägen";
        subordinate_subjunctive_i_i_second: "~lägest|~lägst", "~läget|~lägt";
        subordinate_subjunctive_i_i_third: "~läge", "~lägen";
        imperative: "lieg ~|liege ~", "liegt ~";
        infinitive: referred;
        zu_infinitive: "~zuliegen";
        participle: "~liegend", "~gelegen";
    ),
    // Bekennen: 5 words (versenden, entsenden, erkennen, benennen, bekennen).
    conjugation!("bekennen", 5, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~a~te", "~a~ten";
        preterite_second: "~a~test", "~a~tet";
        preterite_third: "~a~te", "~a~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~e~te", "~e~ten";
        subjunctive_i_i_second: "~e~test", "~e~tet";
        subjunctive_i_i_third: "~e~te", "~e~ten";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~a~te", "~a~ten";
        subordinate_preterite_second: "~a~test", "~a~tet";
        subordinate_preterite_third: "~a~te", "~a~ten";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "~a~t";
    ),
    // Beklimmen: 5 words (beklimmen, verschwimmen, liebgewinnen, gewinnen, besinnen).
    conjugation!("beklimmen", 5, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~st", "~i~t";
        present_third: "~i~t", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", "~i~en";
        subjunctive_i_i_first: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subjunctive_i_i_second: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
        subjunctive_i_i_third: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subordinate_present_first: "~i~e", "~i~en";
        subordinate_present_second: "~i~st", "~i~t";
        subordinate_present_third: "~i~t", "~i~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_second: "~i~est", "~i~et";
        subordinate_subjunctive_i_third: "~i~e", "~i~en";
        subordinate_subjunctive_i_i_first: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ä~e|~ö~e", "~ä~en|~ö~en";
        imperative: "~i~|~i~e", "~i~t";
        infinitive: referred;
        zu_infinitive: "zu~i~en";
        participle: "~i~end", "~o~en";
    ),
    // Brennen: 4 words (rennen, nennen, kennen, brennen).
    conjugation!("brennen", 4, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~a~te", "~a~ten";
        preterite_second: "~a~test", "~a~tet";
        preterite_third: "~a~te", "~a~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~e~te", "~e~ten";
        subjunctive_i_i_second: "~e~test", "~e~tet";
        subjunctive_i_i_third: "~e~te", "~e~ten";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~a~te", "~a~ten";
        subordinate_preterite_second: "~a~test", "~a~tet";
        subordinate_preterite_third: "~a~te", "~a~ten";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~a~t";
    ),
    // Reiten: 4 words (gleiten, reiten, streiten, schreiten).
    conjugation!("reiten", 4, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~est", "~e~et";
        present_third: "~e~et", "~e~en";
        preterite_first: "~~t", "~~ten";
        preterite_second: "~~test|~~tst", "~~tet";
        preterite_third: "~~t", "~~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~est", "~e~et";
        subordinate_present_third: "~e~et", "~e~en";
        subordinate_preterite_first: "~~t", "~~ten";
        subordinate_preterite_second: "~~test|~~tst", "~~tet";
        subordinate_preterite_third: "~~t", "~~ten";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~e~|~e~e", "~e~et";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~~ten";
    ),
    // Vierteilen: 4 words (wechselwirken, lobsagen, vierteilen, machinieren).
    conjugation!("vierteilen", 4, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", "~~en";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~~e", "~~en";
        subordinate_present_second: "~~st", "~~t";
        subordinate_present_third: "~~t", "~~en";
        subordinate_preterite_first: "~~te", "~~ten";
        subordinate_preterite_second: "~~test", "~~tet";
        subordinate_preterite_third: "~~te", "~~ten";
        subordinate_subjunctive_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_third: "~~e", "~~en";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: referred;
        zu_infinitive: "~uz~en";
        participle: "~~end", "~ge~t";
    ),
    // Eilen: 4 words (eilen, engen, englischen, ecken).
    conjugation!("eilen", 4, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", "~~en";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~~e", "~~en";
        subordinate_present_second: "~~st", "~~t";
        subordinate_present_third: "~~t", "~~en";
        subordinate_preterite_first: "~~te", "~~ten";
        subordinate_preterite_second: "~~test", "~~tet";
        subordinate_preterite_third: "~~te", "~~ten";
        subordinate_subjunctive_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_third: "~~e", "~~en";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: referred;
        zu_infinitive: "zu~~en";
        participle: "~~end", "g~e~t";
    ),
    // Notlanden: 4 words (notlanden, bauchreden, stoßlüften, stosslüften).
    conjugation!("notlanden", 4, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~est", "~~et";
        present_third: "~~et", "~~en";
        preterite_first: "~~ete", "~~eten";
        preterite_second: "~~etest", "~~etet";
        preterite_third: "~~ete", "~~eten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", "~~en";
        subjunctive_i_i_first: "~~ete", "~~eten";
        subjunctive_i_i_second: "~~etest", "~~etet";
        subjunctive_i_i_third: "~~ete", "~~eten";
        subordinate_present_first: "~~e", "~~en";
        subordinate_present_second: "~~est", "~~et";
        subordinate_present_third: "~~et", "~~en";
        subordinate_preterite_first: "~~ete", "~~eten";
        subordinate_preterite_second: "~~etest", "~~etet";
        subordinate_preterite_third: "~~ete", "~~eten";
        subordinate_subjunctive_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_third: "~~e", "~~en";
        subordinate_subjunctive_i_i_first: "~~ete", "~~eten";
        subordinate_subjunctive_i_i_second: "~~etest", "~~etet";
        subordinate_subjunctive_i_i_third: "~~ete", "~~eten";
        imperative: "~~|~~e", "~~et";
        infinitive: referred;
        zu_infinitive: "~uz~en";
        participle: "~~end", "~ge~et";
    ),
    // Gegenüberstehen: 4 words (strammstehen, bevorstehen, gegenüberstehen, umeinanderstehen).
    conjugation!("gegenüberstehen", 4, "~stehen";
        present_first: "stehe ~", "stehen ~";
        present_second: "stehst ~", "steht ~";
        present_third: "steht ~", "stehen ~";
        preterite_first: "stand ~", "standen ~";
        preterite_second: "standest ~", "standet ~";
        preterite_third: "stand ~", "standen ~";
        subjunctive_i_first: "stehe ~", "stehen ~";
        subjunctive_i_second: "stehest ~", "stehet ~";
        subjunctive_i_third: "stehe ~", "stehen ~";
        subjunctive_i_i_first: "stände ~", "ständen ~";
        subjunctive_i_i_second: "ständest ~", "ständet ~";
        subjunctive_i_i_third: "stände ~", "ständen ~";
        subordinate_present_first: "~stehe", "~stehen";
        subordinate_present_second: "~stehst", "~steht";
        subordinate_present_third: "~steht", "~stehen";
        subordinate_preterite_first: "~stand", "~standen";
        subordinate_preterite_second: "~standest", "~standet";
        subordinate_preterite_third: "~stand", "~standen";
        subordinate_subjunctive_i_first: "~stehe", "~stehen";
        subordinate_subjunctive_i_second: "~stehest", "~stehet";
        subordinate_subjunctive_i_third: "~stehe", "~stehen";
        subordinate_subjunctive_i_i_first: "~stände", "~ständen";
        subordinate_subjunctive_i_i_second: "~ständest", "~ständet";
        subordinate_subjunctive_i_i_third: "~stände", "~ständen";
        imperative: "steh ~", "steht ~";
        infinitive: referred;
        zu_infinitive: "~zustehen";
        participle: "~stehend", "~gestanden";
    ),
    // Fechten: 4 words (fechten, verfechten, verflechten, entflechten).
    conjugation!("fechten", 4, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~et";
        present_third: "~i~", "~e~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~est|~o~st", "~o~et";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est", "~ö~et";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~et";
        subordinate_present_third: "~i~", "~e~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~est|~o~st", "~o~et";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est", "~ö~et";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~i~", "~e~et";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "~o~en";
    ),
    // Umhinkönnen: 4 words (umhinkönnen, drüberkönnen, drunterkönnen, hinüberkönnen).
    conjugation!("umhinkönnen", 4, "~können";
        present_first: "kann ~", "können ~";
        present_second: "kannst ~", "könnt ~";
        present_third: "kann ~", "können ~";
        preterite_first: "konnte ~", "konnten ~";
        preterite_second: "konntest ~", "konntet ~";
        preterite_third: "konnte ~", "konnten ~";
        subjunctive_i_first: "könne ~", "können ~";
        subjunctive_i_second: "könnest ~", "könnet ~";
        subjunctive_i_third: "könne ~", "können ~";
        subjunctive_i_i_first: "könnte ~", "könnten ~";
        subjunctive_i_i_second: "könntest ~", "könntet ~";
        subjunctive_i_i_third: "könnte ~", "könnten ~";
        subordinate_present_first: "~kann", "~können";
        subordinate_present_second: "~kannst", "~könnt";
        subordinate_present_third: "~kann", "~können";
        subordinate_preterite_first: "~konnte", "~konnten";
        subordinate_preterite_second: "~konntest", "~konntet";
        subordinate_preterite_third: "~konnte", "~konnten";
        subordinate_subjunctive_i_first: "~könne", "~können";
        subordinate_subjunctive_i_second: "~könnest", "~könnet";
        subordinate_subjunctive_i_third: "~könne", "~können";
        subordinate_subjunctive_i_i_first: "~könnte", "~könnten";
        subordinate_subjunctive_i_i_second: "~könntest", "~könntet";
        subordinate_subjunctive_i_i_third: "~könnte", "~könnten";
        imperative: "~", "~t";
        infinitive: referred;
        zu_infinitive: "~zukönnen";
        participle: "~könnend", "~gekonnt";
    ),
    // Behalten: 4 words (aufrechterhalten, geraten, behalten, enthalten).
    conjugation!("behalten", 4, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~et";
        present_third: "~ä~", "~a~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~est|~ie~st", "~ie~et";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", "~a~en";
        subjunctive_i_i_first: "~ie~e", "~ie~en";
        subjunctive_i_i_second: "~ie~est", "~ie~et";
        subjunctive_i_i_third: "~ie~e", "~ie~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~st", "~a~et";
        subordinate_present_third: "~ä~", "~a~en";
        subordinate_preterite_first: "~ie~", "~ie~en";
        subordinate_preterite_second: "~ie~est|~ie~st", "~ie~et";
        subordinate_preterite_third: "~ie~", "~ie~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", "~a~en";
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~a~|~a~e", "~a~et";
        infinitive: referred;
        zu_infinitive: "zu~a~en";
        participle: "~a~end", "~a~en";
    ),
    // Begegnen: 4 words (begegnen, entgegnen, zwangsenteignen, unterzeichnen).
    conjugation!("begegnen", 4, "~~n";
        present_first: "~~", "~~n";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~n";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~", "~~n";
        subjunctive_i_second: "~~st", "~~t";
        subjunctive_i_third: "~~", "~~n";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~~", "~~n";
        subordinate_present_second: "~~st", "~~t";
        subordinate_present_third: "~~t", "~~n";
        subordinate_preterite_first: "~~te", "~~ten";
        subordinate_preterite_second: "~~test", "~~tet";
        subordinate_preterite_third: "~~te", "~~ten";
        subordinate_subjunctive_i_first: "~~", "~~n";
        subordinate_subjunctive_i_second: "~~st", "~~t";
        subordinate_subjunctive_i_third: "~~", "~~n";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~", "~~t";
        infinitive: referred;
        zu_infinitive: "z~u~n";
        participle: "~~nd", "~~t";
    ),
    // Reien: 3 words (schreien, speien, reien).
    conjugation!("reien", 3, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~e", "~~en";
        preterite_second: "~~est", "~~et";
        preterite_third: "~~e", "~~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~~e", "~~en";
        subjunctive_i_i_second: "~~est", "~~et";
        subjunctive_i_i_third: "~~e", "~~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~~e", "~~en";
        subordinate_preterite_second: "~~est", "~~et";
        subordinate_preterite_third: "~~e", "~~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_i_third: "~~e", "~~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~~en";
    ),
    // Scheren: 3 words (scheren, heben, wegen).
    conjugation!("scheren", 3, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~st", "~o~t";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~o~en";
    ),
    // Pflegen: 3 words (bellen, pflegen, bleichen).
    conjugation!("pflegen", 3, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~e~te", "~e~ten";
        preterite_second: "~e~test", "~e~tet";
        preterite_third: "~e~te", "~e~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", "~e~en";
        subjunctive_i_i_first: "~e~te|~ö~e", "~e~ten|~ö~en";
        subjunctive_i_i_second: "~e~test|~ö~est|~ö~st", "~e~tet|~ö~et|~ö~t";
        subjunctive_i_i_third: "~e~te|~ö~e", "~e~ten|~ö~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~st", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~e~te", "~e~ten";
        subordinate_preterite_second: "~e~test", "~e~tet";
        subordinate_preterite_third: "~e~te", "~e~ten";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", "~e~en";
        subordinate_subjunctive_i_i_first: "~e~te|~ö~e", "~e~ten|~ö~en";
        subordinate_subjunctive_i_i_second: "~e~test|~ö~est|~ö~st", "~e~tet|~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~e~te|~ö~e", "~e~ten|~ö~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: referred;
        zu_infinitive: "zu~e~en";
        participle: "~e~end", "ge~e~t|ge~o~en";
    ),
    // Zucken: 3 words (zucken, zurren, zupfen).
    conjugation!("zucken", 3, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", "~~en";
        subjunctive_i_i_first: "~~te", "~~ten";
        subjunctive_i_i_second: "~~test", "~~tet";
        subjunctive_i_i_third: "~~te", "~~ten";
        subordinate_present_first: "~~e", "~~en";
        subordinate_present_second: "~~st", "~~t";
        subordinate_present_third: "~~t", "~~en";
        subordinate_preterite_first: "~~te", "~~ten";
        subordinate_preterite_second: "~~test", "~~tet";
        subordinate_preterite_third: "~~te", "~~ten";
        subordinate_subjunctive_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_third: "~~e", "~~en";
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: referred;
        zu_infinitive: "~zu~en";
        participle: "~~end", "ge~~t";
    ),
    // Ekeln: 3 words (eiern, ekeln, eifern).
    conjugation!("ekeln", 3, "~~e~n";
        present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        present_second: "~~e~st", "~~e~t";
        present_third: "~~e~t", "~~e~n";
        preterite_first: "~~e~te", "~~e~ten";
        preterite_second: "~~e~test", "~~e~tet";
        preterite_third: "~~e~te", "~~e~ten";
        subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subjunctive_i_i_third: "~~e~te", "~~e~ten";
        subordinate_present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        subordinate_present_second: "~~e~st", "~~e~t";
        subordinate_present_third: "~~e~t", "~~e~n";
        subordinate_preterite_first: "~~e~te", "~~e~ten";
        subordinate_preterite_second: "~~e~test", "~~e~tet";
        subordinate_preterite_third: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subordinate_subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subordinate_subjunctive_i_i_third: "~~e~te", "~~e~ten";
        imperative: "~~~e|~~e~|~~e~e", "~~e~t";
        infinitive: referred;
        zu_infinitive: "zu~~e~n";
        participle: "~~e~nd", "g~e~e~t";
    ),
    // Schlechtmachen: 3 words (schlechtmachen, saubermachen, schlappmachen).
    conjugation!("schlechtmachen", 3, "~~machen";
        present_first: "mache ~~", "machen ~~";
        present_second: "mach~t s~", "macht ~~";
        present_third: "macht ~~", "machen ~~";
        preterite_first: "machte ~~", "machten ~~";
        preterite_second: "machte~t s~", "machtet ~~";
        preterite_third: "machte ~~", "machten ~~";
        subjunctive_i_first: "mache ~~", "machen ~~";
        subjunctive_i_second: "mache~t s~", "machet ~~";
        subjunctive_i_third: "mache ~~", "machen ~~";
        subjunctive_i_i_first: "machte ~~", "machten ~~";
        subjunctive_i_i_second: "machte~t s~", "machtet ~~";
        subjunctive_i_i_third: "machte ~~", "machten ~~";
        subordinate_present_first: "~~mache", "~~machen";
        subordinate_present_second: "~~machst", "~~macht";
        subordinate_present_third: "~~macht", "~~machen";
        subordinate_preterite_first: "~~machte", "~~machten";
        subordinate_preterite_second: "~~machtest", "~~machtet";
        subordinate_preterite_third: "~~machte", "~~machten";
        subordinate_subjunctive_i_first: "~~mache", "~~machen";
        subordinate_subjunctive_i_second: "~~machest", "~~machet";
        subordinate_subjunctive_i_third: "~~mache", "~~machen";
        subordinate_subjunctive_i_i_first: "~~machte", "~~machten";
        subordinate_subjunctive_i_i_second: "~~machtest", "~~machtet";
        subordinate_subjunctive_i_i_third: "~~machte", "~~machten";
        imperative: "mach ~~|mache ~~", "macht ~~";
        infinitive: referred;
        zu_infinitive: "~~zumachen";
        participle: "~~machend", "~~gemacht";
    ),
    // Lüsten: 3 words (fisseln, weihnachten, lüsten).
    conjugation!("lüsten", 3, "~n";
        present_first: "~", "~n";
        present_second: "~st", "~t";
        present_third: "~t", "~n";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~", "~n";
        subjunctive_i_second: "~st", "~t";
        subjunctive_i_third: "~", "~n";
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "~", "~n";
        subordinate_present_second: "~st", "~t";
        subordinate_present_third: "~t", "~n";
        subordinate_preterite_first: "~te", "~ten";
        subordinate_preterite_second: "~test", "~tet";
        subordinate_preterite_third: "~te", "~ten";
        subordinate_subjunctive_i_first: "~", "~n";
        subordinate_subjunctive_i_second: "~st", "~t";
        subordinate_subjunctive_i_third: "~", "~n";
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~", "~t";
        infinitive: "~n";
        zu_infinitive: "zu~n";
        participle: "~nd", "ge~t";
    ),
    // Endlagern: 3 words (endlagern, maßschneidern, massschneidern).
    conjugation!("endlagern", 3, "~~e~n";
        present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        present_second: "~~e~st", "~~e~t";
        present_third: "~~e~t", "~~e~n";
        preterite_first: "~~e~te", "~~e~ten";
        preterite_second: "~~e~test", "~~e~tet";
        preterite_third: "~~e~te", "~~e~ten";
        subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subjunctive_i_i_third: "~~e~te", "~~e~ten";
        subordinate_present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        subordinate_present_second: "~~e~st", "~~e~t";
        subordinate_present_third: "~~e~t", "~~e~n";
        subordinate_preterite_first: "~~e~te", "~~e~ten";
        subordinate_preterite_second: "~~e~test", "~~e~tet";
        subordinate_preterite_third: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subordinate_subjunctive_i_third: "~~e~e|~~~e", "~~e~n";
        subordinate_subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subordinate_subjunctive_i_i_third: "~~e~te", "~~e~ten";
        imperative: "~~~e|~~e~|~~e~e", "~~e~t";
        infinitive: referred;
        zu_infinitive: "~uz~e~n";
        participle: "~~e~nd", "~ge~e~t";
    ),
];
