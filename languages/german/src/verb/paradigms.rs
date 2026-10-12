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
    /// Like `mögen`: 2,273 words.
    Mögen,
    /// Like `machen`: 927 words.
    Machen,
    /// Like `nudeln`: 722 words.
    Nudeln,
    /// Like `verpflichten`: 327 words.
    Verpflichten,
    /// Like `erinnern`: 322 words.
    Erinnern,
    /// Like `lösen`: 270 words.
    Lösen,
    /// Like `richten`: 212 words.
    Richten,
    /// Like `abziehen`: 151 words.
    Abziehen,
    /// Like `versetzen`: 127 words.
    Versetzen,
    /// Like `einstellen`: 114 words.
    Einstellen,
    /// Like `aufbauen`: 108 words.
    Aufbauen,
    /// Like `anziehen`: 107 words.
    Anziehen,
    /// Like `ausmachen`: 101 words.
    Ausmachen,
    /// Like `abkommen`: 65 words.
    Abkommen,
    /// Like `einhalten`: 63 words.
    Einhalten,
    /// Like `abgehen`: 57 words.
    Abgehen,
    /// Like `zulegen`: 54 words.
    Zulegen,
    /// Like `abändern`: 52 words.
    Abändern,
    /// Like `geben`: 50 words.
    Geben,
    /// Like `beschreiben`: 49 words.
    Beschreiben,
    /// Like `überspannen`: 48 words.
    Überspannen,
    /// Like `wegrauchen`: 45 words.
    Wegrauchen,
    /// Like `verschlagen`: 44 words.
    Verschlagen,
    /// Like `fertigmachen`: 44 words.
    Fertigmachen,
    /// Like `ausbreiten`: 43 words.
    Ausbreiten,
    /// Like `nachhängen`: 39 words.
    Nachhängen,
    /// Like `abschneiden`: 38 words.
    Abschneiden,
    /// Like `durchbringen`: 37 words.
    Durchbringen,
    /// Like `abgeben`: 37 words.
    Abgeben,
    /// Like `mitkönnen`: 37 words.
    Mitkönnen,
    /// Like `anlagern`: 36 words.
    Anlagern,
    /// Like `erwerben`: 35 words.
    Erwerben,
    /// Like `vorstellen`: 34 words.
    Vorstellen,
    /// Like `einlassen`: 34 words.
    Einlassen,
    /// Like `absetzen`: 33 words.
    Absetzen,
    /// Like `vertrinken`: 33 words.
    Vertrinken,
    /// Like `umlegen`: 33 words.
    Umlegen,
    /// Like `auftragen`: 33 words.
    Auftragen,
    /// Like `abnehmen`: 32 words.
    Abnehmen,
    /// Like `zurückgreifen`: 32 words.
    Zurückgreifen,
    /// Like `ausmustern`: 31 words.
    Ausmustern,
    /// Like `anzünden`: 31 words.
    Anzünden,
    /// Like `ansetzen`: 30 words.
    Ansetzen,
    /// Like `anstehen`: 30 words.
    Anstehen,
    /// Like `einziehen`: 30 words.
    Einziehen,
    /// Like `glauben`: 29 words.
    Glauben,
    /// Like `einfädeln`: 29 words.
    Einfädeln,
    /// Like `sein`: 29 words.
    Sein,
    /// Like `aufrappeln`: 29 words.
    Aufrappeln,
    /// Like `ausspannen`: 29 words.
    Ausspannen,
    /// Like `begießen`: 28 words.
    Begießen,
    /// Like `einrichten`: 28 words.
    Einrichten,
    /// Like `auftun`: 28 words.
    Auftun,
    /// Like `ablaufen`: 28 words.
    Ablaufen,
    /// Like `aussetzen`: 27 words.
    Aussetzen,
    /// Like `auffahren`: 27 words.
    Auffahren,
    /// Like `verhalten`: 26 words.
    Verhalten,
    /// Like `einsetzen`: 26 words.
    Einsetzen,
    /// Like `ansehen`: 25 words.
    Ansehen,
    /// Like `aufsitzen`: 24 words.
    Aufsitzen,
    /// Like `gabeln`: 24 words.
    Gabeln,
    /// Like `aufarbeiten`: 23 words.
    Aufarbeiten,
    /// Like `zurückfallen`: 23 words.
    Zurückfallen,
    /// Like `aufhaben`: 23 words.
    Aufhaben,
    /// Like `geniessen`: 22 words.
    Geniessen,
    /// Like `entbinden`: 22 words.
    Entbinden,
    /// Like `eintreten`: 22 words.
    Eintreten,
    /// Like `herumhängen`: 21 words.
    Herumhängen,
    /// Like `zurücklegen`: 20 words.
    Zurücklegen,
    /// Like `auseinandersetzen`: 19 words.
    Auseinandersetzen,
    /// Like `anliegen`: 19 words.
    Anliegen,
    /// Like `fortführen`: 19 words.
    Fortführen,
    /// Like `öffnen`: 18 words.
    Öffnen,
    /// Like `verschreien`: 18 words.
    Verschreien,
    /// Like `umwenden`: 18 words.
    Umwenden,
    /// Like `zuschütten`: 18 words.
    Zuschütten,
    /// Like `einwerfen`: 18 words.
    Einwerfen,
    /// Like `verlieren`: 17 words.
    Verlieren,
    /// Like `verbeißen`: 17 words.
    Verbeißen,
    /// Like `fressen`: 17 words.
    Fressen,
    /// Like `festmachen`: 17 words.
    Festmachen,
    /// Like `darstellen`: 17 words.
    Darstellen,
    /// Like `hochfliegen`: 17 words.
    Hochfliegen,
    /// Like `gewinnen`: 16 words.
    Gewinnen,
    /// Like `bringen`: 16 words.
    Bringen,
    /// Like `schmelzen`: 16 words.
    Schmelzen,
    /// Like `hinhauen`: 16 words.
    Hinhauen,
    /// Like `durchnudeln`: 16 words.
    Durchnudeln,
    /// Like `aufreißen`: 16 words.
    Aufreißen,
    /// Like `herumnudeln`: 15 words.
    Herumnudeln,
    /// Like `auslesen`: 15 words.
    Auslesen,
    /// Like `niederhauen`: 15 words.
    Niederhauen,
    /// Like `benennen`: 14 words.
    Benennen,
    /// Like `mithören`: 14 words.
    Mithören,
    /// Like `schwingen`: 13 words.
    Schwingen,
    /// Like `heißen`: 13 words.
    Heißen,
    /// Like `abstoßen`: 13 words.
    Abstoßen,
    /// Like `mitreden`: 13 words.
    Mitreden,
    /// Like `eilen`: 12 words.
    Eilen,
    /// Like `erheben`: 12 words.
    Erheben,
    /// Like `stehen`: 12 words.
    Stehen,
    /// Like `glänzen`: 12 words.
    Glänzen,
    /// Like `aufrufen`: 12 words.
    Aufrufen,
    /// Like `bestreiten`: 12 words.
    Bestreiten,
    /// Like `anvertrauen`: 12 words.
    Anvertrauen,
    /// Like `herausstellen`: 12 words.
    Herausstellen,
    /// Like `beibringen`: 12 words.
    Beibringen,
    /// Like `zusammenscharren`: 12 words.
    Zusammenscharren,
    /// Like `zusetzen`: 12 words.
    Zusetzen,
    /// Like `abstossen`: 12 words.
    Abstossen,
    /// Like `erwachsen`: 11 words.
    Erwachsen,
    /// Like `schreiben`: 11 words.
    Schreiben,
    /// Like `schneiden`: 11 words.
    Schneiden,
    /// Like `zujubeln`: 11 words.
    Zujubeln,
    /// Like `hermachen`: 11 words.
    Hermachen,
    /// Like `runterkippen`: 11 words.
    Runterkippen,
    /// Like `laden`: 11 words.
    Laden,
    /// Like `ausradieren`: 11 words.
    Ausradieren,
    /// Like `raushauen`: 11 words.
    Raushauen,
    /// Like `abbinden`: 11 words.
    Abbinden,
    /// Like `unterbringen`: 11 words.
    Unterbringen,
    /// Like `rumbringen`: 11 words.
    Rumbringen,
    /// Like `abfahren`: 10 words.
    Abfahren,
    /// Like `giften`: 10 words.
    Giften,
    /// Like `abzapfen`: 10 words.
    Abzapfen,
    /// Like `reinhauen`: 10 words.
    Reinhauen,
    /// Like `heimführen`: 10 words.
    Heimführen,
    /// Like `herreisen`: 10 words.
    Herreisen,
    /// Like `verfallen`: 9 words.
    Verfallen,
    /// Like `fliegen`: 9 words.
    Fliegen,
    /// Like `tun`: 9 words.
    Tun,
    /// Like `lassen`: 9 words.
    Lassen,
    /// Like `aufpolieren`: 9 words.
    Aufpolieren,
    /// Like `gehen`: 9 words.
    Gehen,
    /// Like `gerben`: 9 words.
    Gerben,
    /// Like `sehen`: 9 words.
    Sehen,
    /// Like `blasen`: 9 words.
    Blasen,
    /// Like `abwerfen`: 9 words.
    Abwerfen,
    /// Like `treten`: 9 words.
    Treten,
    /// Like `begreifen`: 9 words.
    Begreifen,
    /// Like `hineinbringen`: 9 words.
    Hineinbringen,
    /// Like `übergreifen`: 9 words.
    Übergreifen,
    /// Like `nacharbeiten`: 9 words.
    Nacharbeiten,
    /// Like `abessen`: 9 words.
    Abessen,
    /// Like `anstreichen`: 9 words.
    Anstreichen,
    /// Like `gebieten`: 8 words.
    Gebieten,
    /// Like `durchsetzen`: 8 words.
    Durchsetzen,
    /// Like `begleichen`: 8 words.
    Begleichen,
    /// Like `denken`: 8 words.
    Denken,
    /// Like `fangen`: 8 words.
    Fangen,
    /// Like `einverleiben`: 8 words.
    Einverleiben,
    /// Like `beziehen`: 8 words.
    Beziehen,
    /// Like `einschlagen`: 8 words.
    Einschlagen,
    /// Like `ausschlagen`: 8 words.
    Ausschlagen,
    /// Like `geifern`: 8 words.
    Geifern,
    /// Like `hinauswollen`: 8 words.
    Hinauswollen,
    /// Like `weiterzahlen`: 8 words.
    Weiterzahlen,
    /// Like `speien`: 7 words.
    Speien,
    /// Like `absteigen`: 7 words.
    Absteigen,
    /// Like `anschreiben`: 7 words.
    Anschreiben,
    /// Like `schmeißen`: 7 words.
    Schmeißen,
    /// Like `rumeiern`: 7 words.
    Rumeiern,
    /// Like `kommen`: 7 words.
    Kommen,
    /// Like `scheiden`: 7 words.
    Scheiden,
    /// Like `schwimmen`: 7 words.
    Schwimmen,
    /// Like `betrügen`: 7 words.
    Betrügen,
    /// Like `auffliegen`: 7 words.
    Auffliegen,
    /// Like `abreagieren`: 7 words.
    Abreagieren,
    /// Like `auftreiben`: 7 words.
    Auftreiben,
    /// Like `nehmen`: 7 words.
    Nehmen,
    /// Like `austrinken`: 7 words.
    Austrinken,
    /// Like `zurückführen`: 7 words.
    Zurückführen,
    /// Like `vorverlegen`: 7 words.
    Vorverlegen,
    /// Like `verhauen`: 7 words.
    Verhauen,
    /// Like `nachsetzen`: 7 words.
    Nachsetzen,
    /// Like `durchleiten`: 7 words.
    Durchleiten,
    /// Like `totsagen`: 7 words.
    Totsagen,
    /// Like `vollstopfen`: 7 words.
    Vollstopfen,
    /// Like `fechten`: 7 words.
    Fechten,
    /// Like `stoßen`: 7 words.
    Stoßen,
    /// Like `laufen`: 6 words.
    Laufen,
    /// Like `ergeben`: 6 words.
    Ergeben,
    /// Like `lossagen`: 6 words.
    Lossagen,
    /// Like `umwandeln`: 6 words.
    Umwandeln,
    /// Like `aufspringen`: 6 words.
    Aufspringen,
    /// Like `umgewöhnen`: 6 words.
    Umgewöhnen,
    /// Like `aussiedeln`: 6 words.
    Aussiedeln,
    /// Like `abbauen`: 6 words.
    Abbauen,
    /// Like `einbiegen`: 6 words.
    Einbiegen,
    /// Like `vorsetzen`: 6 words.
    Vorsetzen,
    /// Like `anfahren`: 6 words.
    Anfahren,
    /// Like `herumgehen`: 6 words.
    Herumgehen,
    /// Like `nacheifern`: 6 words.
    Nacheifern,
    /// Like `saufen`: 6 words.
    Saufen,
    /// Like `vorankündigen`: 6 words.
    Vorankündigen,
    /// Like `sprechen`: 5 words.
    Sprechen,
    /// Like `fahren`: 5 words.
    Fahren,
    /// Like `werden`: 5 words.
    Werden,
    /// Like `binden`: 5 words.
    Binden,
    /// Like `abbröckeln`: 5 words.
    Abbröckeln,
    /// Like `pflegen`: 5 words.
    Pflegen,
    /// Like `schießen`: 5 words.
    Schießen,
    /// Like `ekeln`: 5 words.
    Ekeln,
    /// Like `anhalten`: 5 words.
    Anhalten,
    /// Like `ausschlachten`: 5 words.
    Ausschlachten,
    /// Like `notlanden`: 5 words.
    Notlanden,
    /// Like `empfehlen`: 5 words.
    Empfehlen,
    /// Like `einsteigen`: 5 words.
    Einsteigen,
    /// Like `hindeuten`: 5 words.
    Hindeuten,
    /// Like `aufzeigen`: 5 words.
    Aufzeigen,
    /// Like `hinsetzen`: 5 words.
    Hinsetzen,
    /// Like `nachtragen`: 5 words.
    Nachtragen,
    /// Like `aufführen`: 5 words.
    Aufführen,
    /// Like `freimachen`: 5 words.
    Freimachen,
    /// Like `vorsingen`: 5 words.
    Vorsingen,
    /// Like `hochrechnen`: 5 words.
    Hochrechnen,
    /// Like `umwälzen`: 5 words.
    Umwälzen,
    /// Like `austreiben`: 5 words.
    Austreiben,
    /// Like `abrechnen`: 5 words.
    Abrechnen,
    /// Like `vorauseilen`: 5 words.
    Vorauseilen,
    /// Like `nacherzählen`: 5 words.
    Nacherzählen,
    /// Like `emporsteigen`: 5 words.
    Emporsteigen,
    /// Like `antrinken`: 5 words.
    Antrinken,
    /// Like `abheben`: 5 words.
    Abheben,
    /// Like `vorbilden`: 5 words.
    Vorbilden,
    /// Like `wölken`: 5 words.
    Wölken,
    /// Like `nachempfinden`: 5 words.
    Nachempfinden,
    /// Like `niederspritzen`: 5 words.
    Niederspritzen,
    /// Like `hineinbewegen`: 5 words.
    Hineinbewegen,
    /// Like `akklimatisieren`: 5 words.
    Akklimatisieren,
    /// Like `brennen`: 4 words.
    Brennen,
    /// Like `greifen`: 4 words.
    Greifen,
    /// Like `halten`: 4 words.
    Halten,
    /// Like `abschießen`: 4 words.
    Abschießen,
    /// Like `abbiegen`: 4 words.
    Abbiegen,
    /// Like `giessen`: 4 words.
    Giessen,
    /// Like `reiten`: 4 words.
    Reiten,
    /// Like `hinausgehen`: 4 words.
    Hinausgehen,
    /// Like `schleichen`: 4 words.
    Schleichen,
    /// Like `hinwegfegen`: 4 words.
    Hinwegfegen,
    /// Like `anzeigen`: 4 words.
    Anzeigen,
    /// Like `vortragen`: 4 words.
    Vortragen,
    /// Like `rufen`: 4 words.
    Rufen,
    /// Like `zuweisen`: 4 words.
    Zuweisen,
    /// Like `liegen`: 4 words.
    Liegen,
    /// Like `einordnen`: 4 words.
    Einordnen,
    /// Like `zuschieben`: 4 words.
    Zuschieben,
    /// Like `durchzappen`: 4 words.
    Durchzappen,
    /// Like `einschießen`: 4 words.
    Einschießen,
    /// Like `dahinleben`: 4 words.
    Dahinleben,
    /// Like `abbüßen`: 4 words.
    Abbüßen,
    /// Like `herausragen`: 4 words.
    Herausragen,
    /// Like `klarmachen`: 4 words.
    Klarmachen,
    /// Like `herunterdrücken`: 4 words.
    Herunterdrücken,
    /// Like `miterleben`: 4 words.
    Miterleben,
    /// Like `anreißen`: 4 words.
    Anreißen,
    /// Like `besagen`: 4 words.
    Besagen,
    /// Like `exen`: 3 words.
    Exen,
    /// Like `zusammenkehren`: 3 words.
    Zusammenkehren,
    /// Like `erden`: 3 words.
    Erden,
    /// Like `schlechtmachen`: 3 words.
    Schlechtmachen,
    /// Like `verwenden`: 3 words.
    Verwenden,
    /// Like `ausschreiben`: 3 words.
    Ausschreiben,
    /// Like `umschreiben`: 3 words.
    Umschreiben,
    /// Like `einzahlen`: 3 words.
    Einzahlen,
    /// Like `abbeeren`: 3 words.
    Abbeeren,
    /// Like `ebnen`: 3 words.
    Ebnen,
    /// Like `mitklingen`: 3 words.
    Mitklingen,
    /// Like `überschreiben`: 3 words.
    Überschreiben,
    /// Like `auffordern`: 3 words.
    Auffordern,
    /// Like `abfliessen`: 3 words.
    Abfliessen,
    /// Like `aussortieren`: 3 words.
    Aussortieren,
    /// Like `außbauen`: 3 words.
    Außbauen,
    /// Like `zusammenleben`: 3 words.
    Zusammenleben,
    /// Like `angucken`: 3 words.
    Angucken,
    /// Like `aufgleisen`: 3 words.
    Aufgleisen,
    /// Like `weichkochen`: 3 words.
    Weichkochen,
    /// Like `herumfahren`: 3 words.
    Herumfahren,
    /// Like `zubewegen`: 3 words.
    Zubewegen,
    /// Like `heraushalten`: 3 words.
    Heraushalten,
    /// Like `zusammenräumen`: 3 words.
    Zusammenräumen,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 281] = [
        Conjugation::Mögen,
        Conjugation::Machen,
        Conjugation::Nudeln,
        Conjugation::Verpflichten,
        Conjugation::Erinnern,
        Conjugation::Lösen,
        Conjugation::Richten,
        Conjugation::Abziehen,
        Conjugation::Versetzen,
        Conjugation::Einstellen,
        Conjugation::Aufbauen,
        Conjugation::Anziehen,
        Conjugation::Ausmachen,
        Conjugation::Abkommen,
        Conjugation::Einhalten,
        Conjugation::Abgehen,
        Conjugation::Zulegen,
        Conjugation::Abändern,
        Conjugation::Geben,
        Conjugation::Beschreiben,
        Conjugation::Überspannen,
        Conjugation::Wegrauchen,
        Conjugation::Verschlagen,
        Conjugation::Fertigmachen,
        Conjugation::Ausbreiten,
        Conjugation::Nachhängen,
        Conjugation::Abschneiden,
        Conjugation::Durchbringen,
        Conjugation::Abgeben,
        Conjugation::Mitkönnen,
        Conjugation::Anlagern,
        Conjugation::Erwerben,
        Conjugation::Vorstellen,
        Conjugation::Einlassen,
        Conjugation::Absetzen,
        Conjugation::Vertrinken,
        Conjugation::Umlegen,
        Conjugation::Auftragen,
        Conjugation::Abnehmen,
        Conjugation::Zurückgreifen,
        Conjugation::Ausmustern,
        Conjugation::Anzünden,
        Conjugation::Ansetzen,
        Conjugation::Anstehen,
        Conjugation::Einziehen,
        Conjugation::Glauben,
        Conjugation::Einfädeln,
        Conjugation::Sein,
        Conjugation::Aufrappeln,
        Conjugation::Ausspannen,
        Conjugation::Begießen,
        Conjugation::Einrichten,
        Conjugation::Auftun,
        Conjugation::Ablaufen,
        Conjugation::Aussetzen,
        Conjugation::Auffahren,
        Conjugation::Verhalten,
        Conjugation::Einsetzen,
        Conjugation::Ansehen,
        Conjugation::Aufsitzen,
        Conjugation::Gabeln,
        Conjugation::Aufarbeiten,
        Conjugation::Zurückfallen,
        Conjugation::Aufhaben,
        Conjugation::Geniessen,
        Conjugation::Entbinden,
        Conjugation::Eintreten,
        Conjugation::Herumhängen,
        Conjugation::Zurücklegen,
        Conjugation::Auseinandersetzen,
        Conjugation::Anliegen,
        Conjugation::Fortführen,
        Conjugation::Öffnen,
        Conjugation::Verschreien,
        Conjugation::Umwenden,
        Conjugation::Zuschütten,
        Conjugation::Einwerfen,
        Conjugation::Verlieren,
        Conjugation::Verbeißen,
        Conjugation::Fressen,
        Conjugation::Festmachen,
        Conjugation::Darstellen,
        Conjugation::Hochfliegen,
        Conjugation::Gewinnen,
        Conjugation::Bringen,
        Conjugation::Schmelzen,
        Conjugation::Hinhauen,
        Conjugation::Durchnudeln,
        Conjugation::Aufreißen,
        Conjugation::Herumnudeln,
        Conjugation::Auslesen,
        Conjugation::Niederhauen,
        Conjugation::Benennen,
        Conjugation::Mithören,
        Conjugation::Schwingen,
        Conjugation::Heißen,
        Conjugation::Abstoßen,
        Conjugation::Mitreden,
        Conjugation::Eilen,
        Conjugation::Erheben,
        Conjugation::Stehen,
        Conjugation::Glänzen,
        Conjugation::Aufrufen,
        Conjugation::Bestreiten,
        Conjugation::Anvertrauen,
        Conjugation::Herausstellen,
        Conjugation::Beibringen,
        Conjugation::Zusammenscharren,
        Conjugation::Zusetzen,
        Conjugation::Abstossen,
        Conjugation::Erwachsen,
        Conjugation::Schreiben,
        Conjugation::Schneiden,
        Conjugation::Zujubeln,
        Conjugation::Hermachen,
        Conjugation::Runterkippen,
        Conjugation::Laden,
        Conjugation::Ausradieren,
        Conjugation::Raushauen,
        Conjugation::Abbinden,
        Conjugation::Unterbringen,
        Conjugation::Rumbringen,
        Conjugation::Abfahren,
        Conjugation::Giften,
        Conjugation::Abzapfen,
        Conjugation::Reinhauen,
        Conjugation::Heimführen,
        Conjugation::Herreisen,
        Conjugation::Verfallen,
        Conjugation::Fliegen,
        Conjugation::Tun,
        Conjugation::Lassen,
        Conjugation::Aufpolieren,
        Conjugation::Gehen,
        Conjugation::Gerben,
        Conjugation::Sehen,
        Conjugation::Blasen,
        Conjugation::Abwerfen,
        Conjugation::Treten,
        Conjugation::Begreifen,
        Conjugation::Hineinbringen,
        Conjugation::Übergreifen,
        Conjugation::Nacharbeiten,
        Conjugation::Abessen,
        Conjugation::Anstreichen,
        Conjugation::Gebieten,
        Conjugation::Durchsetzen,
        Conjugation::Begleichen,
        Conjugation::Denken,
        Conjugation::Fangen,
        Conjugation::Einverleiben,
        Conjugation::Beziehen,
        Conjugation::Einschlagen,
        Conjugation::Ausschlagen,
        Conjugation::Geifern,
        Conjugation::Hinauswollen,
        Conjugation::Weiterzahlen,
        Conjugation::Speien,
        Conjugation::Absteigen,
        Conjugation::Anschreiben,
        Conjugation::Schmeißen,
        Conjugation::Rumeiern,
        Conjugation::Kommen,
        Conjugation::Scheiden,
        Conjugation::Schwimmen,
        Conjugation::Betrügen,
        Conjugation::Auffliegen,
        Conjugation::Abreagieren,
        Conjugation::Auftreiben,
        Conjugation::Nehmen,
        Conjugation::Austrinken,
        Conjugation::Zurückführen,
        Conjugation::Vorverlegen,
        Conjugation::Verhauen,
        Conjugation::Nachsetzen,
        Conjugation::Durchleiten,
        Conjugation::Totsagen,
        Conjugation::Vollstopfen,
        Conjugation::Fechten,
        Conjugation::Stoßen,
        Conjugation::Laufen,
        Conjugation::Ergeben,
        Conjugation::Lossagen,
        Conjugation::Umwandeln,
        Conjugation::Aufspringen,
        Conjugation::Umgewöhnen,
        Conjugation::Aussiedeln,
        Conjugation::Abbauen,
        Conjugation::Einbiegen,
        Conjugation::Vorsetzen,
        Conjugation::Anfahren,
        Conjugation::Herumgehen,
        Conjugation::Nacheifern,
        Conjugation::Saufen,
        Conjugation::Vorankündigen,
        Conjugation::Sprechen,
        Conjugation::Fahren,
        Conjugation::Werden,
        Conjugation::Binden,
        Conjugation::Abbröckeln,
        Conjugation::Pflegen,
        Conjugation::Schießen,
        Conjugation::Ekeln,
        Conjugation::Anhalten,
        Conjugation::Ausschlachten,
        Conjugation::Notlanden,
        Conjugation::Empfehlen,
        Conjugation::Einsteigen,
        Conjugation::Hindeuten,
        Conjugation::Aufzeigen,
        Conjugation::Hinsetzen,
        Conjugation::Nachtragen,
        Conjugation::Aufführen,
        Conjugation::Freimachen,
        Conjugation::Vorsingen,
        Conjugation::Hochrechnen,
        Conjugation::Umwälzen,
        Conjugation::Austreiben,
        Conjugation::Abrechnen,
        Conjugation::Vorauseilen,
        Conjugation::Nacherzählen,
        Conjugation::Emporsteigen,
        Conjugation::Antrinken,
        Conjugation::Abheben,
        Conjugation::Vorbilden,
        Conjugation::Wölken,
        Conjugation::Nachempfinden,
        Conjugation::Niederspritzen,
        Conjugation::Hineinbewegen,
        Conjugation::Akklimatisieren,
        Conjugation::Brennen,
        Conjugation::Greifen,
        Conjugation::Halten,
        Conjugation::Abschießen,
        Conjugation::Abbiegen,
        Conjugation::Giessen,
        Conjugation::Reiten,
        Conjugation::Hinausgehen,
        Conjugation::Schleichen,
        Conjugation::Hinwegfegen,
        Conjugation::Anzeigen,
        Conjugation::Vortragen,
        Conjugation::Rufen,
        Conjugation::Zuweisen,
        Conjugation::Liegen,
        Conjugation::Einordnen,
        Conjugation::Zuschieben,
        Conjugation::Durchzappen,
        Conjugation::Einschießen,
        Conjugation::Dahinleben,
        Conjugation::Abbüßen,
        Conjugation::Herausragen,
        Conjugation::Klarmachen,
        Conjugation::Herunterdrücken,
        Conjugation::Miterleben,
        Conjugation::Anreißen,
        Conjugation::Besagen,
        Conjugation::Exen,
        Conjugation::Zusammenkehren,
        Conjugation::Erden,
        Conjugation::Schlechtmachen,
        Conjugation::Verwenden,
        Conjugation::Ausschreiben,
        Conjugation::Umschreiben,
        Conjugation::Einzahlen,
        Conjugation::Abbeeren,
        Conjugation::Ebnen,
        Conjugation::Mitklingen,
        Conjugation::Überschreiben,
        Conjugation::Auffordern,
        Conjugation::Abfliessen,
        Conjugation::Aussortieren,
        Conjugation::Außbauen,
        Conjugation::Zusammenleben,
        Conjugation::Angucken,
        Conjugation::Aufgleisen,
        Conjugation::Weichkochen,
        Conjugation::Herumfahren,
        Conjugation::Zubewegen,
        Conjugation::Heraushalten,
        Conjugation::Zusammenräumen,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 281] = [
    // Mögen: 2,273 words (wägen, backen, beseelen, zerstören, zusammenkleben, verabscheuen).
    conjugation!("mögen", 2273, "~en";
        present_first: "~e", "~en";
        present_second: "~st", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "~t";
    ),
    // Machen: 927 words (raven, stricken, nerven, legen, zahlen, zählen).
    conjugation!("machen", 927, "~en";
        present_first: "~e", "~en";
        present_second: "~st", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "ge~t";
    ),
    // Nudeln: 722 words (nebeln, modern, klettern, pinkeln, steuern, tafeln).
    conjugation!("nudeln", 722, "~e~n";
        present_first: "~~e|~e~e|~e~", "~e~n";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~n";
        preterite_first: "~e~te", "~e~ten";
        preterite_second: "~e~test", "~e~tet";
        preterite_third: "~e~te", "~e~ten";
        subjunctive_i_first: "~e~e|~~e", "~e~n";
        subjunctive_i_second: "~e~est|~~est", "~e~et|~~et";
        subjunctive_i_third: "~e~e|~~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e|~~e", referred;
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~~e|~e~|~e~e", "~e~t";
        infinitive: "~e~n";
        zu_infinitive: "abzu~e~n";
        participle: "~e~nd", "ge~e~t";
    ),
    // Verpflichten: 327 words (bitten, beenden, verachten, beschriften, bekoten, unterrichten).
    conjugation!("verpflichten", 327, "~en";
        present_first: "~e", "~en";
        present_second: "~est", "~et";
        present_third: "~et", "~en";
        preterite_first: "~ete", "~eten";
        preterite_second: "~etest", "~etet";
        preterite_third: "~ete", "~eten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~ete", "~eten";
        subordinate_subjunctive_i_i_second: "~etest", "~etet";
        subordinate_subjunctive_i_i_third: "~ete", "~eten";
        imperative: "~|~e", "~et";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "~et";
    ),
    // Erinnern: 322 words (erleichtern, erfordern, verscheißern, behandeln, verringern, verschleudern).
    conjugation!("erinnern", 322, "~e~n";
        present_first: "~~e|~e~e|~e~", "~e~n";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~n";
        preterite_first: "~e~te", "~e~ten";
        preterite_second: "~e~test", "~e~tet";
        preterite_third: "~e~te", "~e~ten";
        subjunctive_i_first: "~e~e|~~e", "~e~n";
        subjunctive_i_second: "~e~est|~~est", "~e~et|~~et";
        subjunctive_i_third: "~e~e|~~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e|~~e", referred;
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~~e|~e~|~e~e", "~e~t";
        infinitive: "~e~n";
        zu_infinitive: "abzu~e~n";
        participle: "~e~nd", "~e~t";
    ),
    // Lösen: 270 words (pflanzen, aasen, pissen, boxen, fotzen, sitzen).
    conjugation!("lösen", 270, "~en";
        present_first: "~e", "~en";
        present_second: "~t", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "ge~t";
    ),
    // Richten: 212 words (listen, rotten, landen, baden, arbeiten, kitten).
    conjugation!("richten", 212, "~en";
        present_first: "~e", "~en";
        present_second: "~est", "~et";
        present_third: "~et", "~en";
        preterite_first: "~ete", "~eten";
        preterite_second: "~etest", "~etet";
        preterite_third: "~ete", "~eten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~ete", "~eten";
        subordinate_subjunctive_i_i_second: "~etest", "~etet";
        subordinate_subjunctive_i_i_third: "~ete", "~eten";
        imperative: "~|~e", "~et";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "ge~et";
    ),
    // Abziehen: 151 words (abdanken, abdecken, abflachen, abholen, abkacken, absägen).
    conjugation!("abziehen", 151, "ab~en";
        present_first: "~e ab", "~en ab";
        present_second: "~st ab", "~t ab";
        present_third: "~t ab", "~en ab";
        preterite_first: "~te ab", "~ten ab";
        preterite_second: "~test ab", "~tet ab";
        preterite_third: "~te ab", "~ten ab";
        subjunctive_i_first: "~e ab", "~en ab";
        subjunctive_i_second: "~est ab", "~et ab";
        subjunctive_i_third: "~e ab", referred;
        subjunctive_i_i_first: "~te ab", "~ten ab";
        subjunctive_i_i_second: "~test ab", "~tet ab";
        subjunctive_i_i_third: "~te ab", "~ten ab";
        subordinate_present_first: "ab~e", "ab~en";
        subordinate_present_second: "ab~st", "ab~t";
        subordinate_present_third: "ab~t", "ab~en";
        subordinate_preterite_first: "ab~te", "ab~ten";
        subordinate_preterite_second: "ab~test", "ab~tet";
        subordinate_preterite_third: "ab~te", "ab~ten";
        subordinate_subjunctive_i_first: "ab~e", "ab~en";
        subordinate_subjunctive_i_second: "ab~est", "ab~et";
        subordinate_subjunctive_i_third: "ab~e", referred;
        subordinate_subjunctive_i_i_first: "ab~te", "ab~ten";
        subordinate_subjunctive_i_i_second: "ab~test", "ab~tet";
        subordinate_subjunctive_i_i_third: "ab~te", "ab~ten";
        imperative: "~ ab|~e ab", "~t ab";
        infinitive: "ab~en";
        zu_infinitive: "abzu~en";
        participle: "ab~end", "abge~t";
    ),
    // Versetzen: 127 words (übersetzen, unterstützen, umfassen, verwanzen, vereisen, zerkreuzen).
    conjugation!("versetzen", 127, "~en";
        present_first: "~e", "~en";
        present_second: "~t", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "~t";
    ),
    // Einstellen: 114 words (einsperren, eintreffen, einführen, einmachen, einfärben, einstellen).
    conjugation!("einstellen", 114, "ein~en";
        present_first: "~e ein", "~en ein";
        present_second: "~st ein", "~t ein";
        present_third: "~t ein", "~en ein";
        preterite_first: "~te ein", "~ten ein";
        preterite_second: "~test ein", "~tet ein";
        preterite_third: "~te ein", "~ten ein";
        subjunctive_i_first: "~e ein", "~en ein";
        subjunctive_i_second: "~est ein", "~et ein";
        subjunctive_i_third: "~e ein", referred;
        subjunctive_i_i_first: "~te ein", "~ten ein";
        subjunctive_i_i_second: "~test ein", "~tet ein";
        subjunctive_i_i_third: "~te ein", "~ten ein";
        subordinate_present_first: "ein~e", "ein~en";
        subordinate_present_second: "ein~st", "ein~t";
        subordinate_present_third: "ein~t", "ein~en";
        subordinate_preterite_first: "ein~te", "ein~ten";
        subordinate_preterite_second: "ein~test", "ein~tet";
        subordinate_preterite_third: "ein~te", "ein~ten";
        subordinate_subjunctive_i_first: "ein~e", "ein~en";
        subordinate_subjunctive_i_second: "ein~est", "ein~et";
        subordinate_subjunctive_i_third: "ein~e", referred;
        subordinate_subjunctive_i_i_first: "ein~te", "ein~ten";
        subordinate_subjunctive_i_i_second: "ein~test", "ein~tet";
        subordinate_subjunctive_i_i_third: "ein~te", "ein~ten";
        imperative: "~ ein|~e ein", "~t ein";
        infinitive: "ein~en";
        zu_infinitive: "einzu~en";
        participle: "ein~end", "einge~t";
    ),
    // Aufbauen: 108 words (aufmachen, aufwühlen, auftauen, aufwecken, aufwachen, aufsperren).
    conjugation!("aufbauen", 108, "auf~en";
        present_first: "~e auf", "~en auf";
        present_second: "~st auf", "~t auf";
        present_third: "~t auf", "~en auf";
        preterite_first: "~te auf", "~ten auf";
        preterite_second: "~test auf", "~tet auf";
        preterite_third: "~te auf", "~ten auf";
        subjunctive_i_first: "~e auf", "~en auf";
        subjunctive_i_second: "~est auf", "~et auf";
        subjunctive_i_third: "~e auf", referred;
        subjunctive_i_i_first: "~te auf", "~ten auf";
        subjunctive_i_i_second: "~test auf", "~tet auf";
        subjunctive_i_i_third: "~te auf", "~ten auf";
        subordinate_present_first: "auf~e", "auf~en";
        subordinate_present_second: "auf~st", "auf~t";
        subordinate_present_third: "auf~t", "auf~en";
        subordinate_preterite_first: "auf~te", "auf~ten";
        subordinate_preterite_second: "auf~test", "auf~tet";
        subordinate_preterite_third: "auf~te", "auf~ten";
        subordinate_subjunctive_i_first: "auf~e", "auf~en";
        subordinate_subjunctive_i_second: "auf~est", "auf~et";
        subordinate_subjunctive_i_third: "auf~e", referred;
        subordinate_subjunctive_i_i_first: "auf~te", "auf~ten";
        subordinate_subjunctive_i_i_second: "auf~test", "auf~tet";
        subordinate_subjunctive_i_i_third: "auf~te", "auf~ten";
        imperative: "~ auf|~e auf", "~t auf";
        infinitive: "auf~en";
        zu_infinitive: "aufzu~en";
        participle: "auf~end", "aufge~t";
    ),
    // Anziehen: 107 words (anerkennen, anziehen, ansenken, angreifen, anschmiegen, anbringen).
    conjugation!("anziehen", 107, "an~en";
        present_first: "~e an", "~en an";
        present_second: "~st an", "~t an";
        present_third: "~t an", "~en an";
        preterite_first: "~te an", "~ten an";
        preterite_second: "~test an", "~tet an";
        preterite_third: "~te an", "~ten an";
        subjunctive_i_first: "~e an", "~en an";
        subjunctive_i_second: "~est an", "~et an";
        subjunctive_i_third: "~e an", referred;
        subjunctive_i_i_first: "~te an", "~ten an";
        subjunctive_i_i_second: "~test an", "~tet an";
        subjunctive_i_i_third: "~te an", "~ten an";
        subordinate_present_first: "an~e", "an~en";
        subordinate_present_second: "an~st", "an~t";
        subordinate_present_third: "an~t", "an~en";
        subordinate_preterite_first: "an~te", "an~ten";
        subordinate_preterite_second: "an~test", "an~tet";
        subordinate_preterite_third: "an~te", "an~ten";
        subordinate_subjunctive_i_first: "an~e", "an~en";
        subordinate_subjunctive_i_second: "an~est", "an~et";
        subordinate_subjunctive_i_third: "an~e", referred;
        subordinate_subjunctive_i_i_first: "an~te", "an~ten";
        subordinate_subjunctive_i_i_second: "an~test", "an~tet";
        subordinate_subjunctive_i_i_third: "an~te", "an~ten";
        imperative: "~ an|~e an", "~t an";
        infinitive: "an~en";
        zu_infinitive: "anzu~en";
        participle: "an~end", "ange~t";
    ),
    // Ausmachen: 101 words (ausloggen, auslachen, auslöschen, ausdehnen, ausüben, ausgleichen).
    conjugation!("ausmachen", 101, "aus~en";
        present_first: "~e aus", "~en aus";
        present_second: "~st aus", "~t aus";
        present_third: "~t aus", "~en aus";
        preterite_first: "~te aus", "~ten aus";
        preterite_second: "~test aus", "~tet aus";
        preterite_third: "~te aus", "~ten aus";
        subjunctive_i_first: "~e aus", "~en aus";
        subjunctive_i_second: "~est aus", "~et aus";
        subjunctive_i_third: "~e aus", referred;
        subjunctive_i_i_first: "~te aus", "~ten aus";
        subjunctive_i_i_second: "~test aus", "~tet aus";
        subjunctive_i_i_third: "~te aus", "~ten aus";
        subordinate_present_first: "aus~e", "aus~en";
        subordinate_present_second: "aus~st", "aus~t";
        subordinate_present_third: "aus~t", "aus~en";
        subordinate_preterite_first: "aus~te", "aus~ten";
        subordinate_preterite_second: "aus~test", "aus~tet";
        subordinate_preterite_third: "aus~te", "aus~ten";
        subordinate_subjunctive_i_first: "aus~e", "aus~en";
        subordinate_subjunctive_i_second: "aus~est", "aus~et";
        subordinate_subjunctive_i_third: "aus~e", referred;
        subordinate_subjunctive_i_i_first: "aus~te", "aus~ten";
        subordinate_subjunctive_i_i_second: "aus~test", "aus~tet";
        subordinate_subjunctive_i_i_third: "aus~te", "aus~ten";
        imperative: "~ aus|~e aus", "~t aus";
        infinitive: "aus~en";
        zu_infinitive: "auszu~en";
        participle: "aus~end", "ausge~t";
    ),
    // Abkommen: 65 words (abkommen, ankommen, auskommen, nachkommen, zurückkommen, mitkommen).
    conjugation!("abkommen", 65, "~kommen";
        present_first: "komme ~", "kommen ~";
        present_second: "kommst ~", "kommt ~";
        present_third: "kommt ~", "kommen ~";
        preterite_first: "kam ~", "kamen ~";
        preterite_second: "kamst ~", "kamt ~";
        preterite_third: "kam ~", "kamen ~";
        subjunctive_i_first: "komme ~", "kommen ~";
        subjunctive_i_second: "kommest ~", "kommet ~";
        subjunctive_i_third: "komme ~", referred;
        subjunctive_i_i_first: "käme ~", "kämen ~";
        subjunctive_i_i_second: "kämest ~", "kämet ~";
        subjunctive_i_i_third: "käme ~", "kämen ~";
        subordinate_present_first: "~komme", "~kommen";
        subordinate_present_second: "~kommst", "~kommt";
        subordinate_present_third: "~kommt", "~kommen";
        subordinate_preterite_first: "~kam", "~kamen";
        subordinate_preterite_second: "~kamst", "~kamt";
        subordinate_preterite_third: "~kam", "~kamen";
        subordinate_subjunctive_i_first: "~komme", "~kommen";
        subordinate_subjunctive_i_second: "~kommest", "~kommet";
        subordinate_subjunctive_i_third: "~komme", referred;
        subordinate_subjunctive_i_i_first: "~käme", "~kämen";
        subordinate_subjunctive_i_i_second: "~kämest", "~kämet";
        subordinate_subjunctive_i_i_third: "~käme", "~kämen";
        imperative: "komm ~", "kommt ~";
        infinitive: "~kommen";
        zu_infinitive: "~zukommen";
        participle: "~kommend", "~gekommen";
    ),
    // Einhalten: 63 words (aushalten, innehalten, aufhalten, zurückhalten, hinhalten, mithalten).
    conjugation!("einhalten", 63, "~halten";
        present_first: "halte ~", "halten ~";
        present_second: "hältst ~", "haltet ~";
        present_third: "hält ~", "halten ~";
        preterite_first: "hielt ~", "hielten ~";
        preterite_second: "hieltest ~", "hieltet ~";
        preterite_third: "hielt ~", "hielten ~";
        subjunctive_i_first: "halte ~", "halten ~";
        subjunctive_i_second: "haltest ~", "haltet ~";
        subjunctive_i_third: "halte ~", referred;
        subjunctive_i_i_first: "hielte ~", "hielten ~";
        subjunctive_i_i_second: "hieltest ~", "hieltet ~";
        subjunctive_i_i_third: "hielte ~", "hielten ~";
        subordinate_present_first: "~halte", "~halten";
        subordinate_present_second: "~hältst", "~haltet";
        subordinate_present_third: "~hält", "~halten";
        subordinate_preterite_first: "~hielt", "~hielten";
        subordinate_preterite_second: "~hieltest", "~hieltet";
        subordinate_preterite_third: "~hielt", "~hielten";
        subordinate_subjunctive_i_first: "~halte", "~halten";
        subordinate_subjunctive_i_second: "~haltest", "~haltet";
        subordinate_subjunctive_i_third: "~halte", referred;
        subordinate_subjunctive_i_i_first: "~hielte", "~hielten";
        subordinate_subjunctive_i_i_second: "~hieltest", "~hieltet";
        subordinate_subjunctive_i_i_third: "~hielte", "~hielten";
        imperative: "halt ~", "haltet ~";
        infinitive: "~halten";
        zu_infinitive: "~zuhalten";
        participle: "~haltend", "~gehalten";
    ),
    // Abgehen: 57 words (vorgehen, hineingehen, spazierengehen, untergehen, weggehen, angehen).
    conjugation!("abgehen", 57, "~gehen";
        present_first: "gehe ~", "gehen ~";
        present_second: "gehst ~", "geht ~";
        present_third: "geht ~", "gehen ~";
        preterite_first: "ging ~", "gingen ~";
        preterite_second: "gingst ~", "gingt ~";
        preterite_third: "ging ~", "gingen ~";
        subjunctive_i_first: "gehe ~", "gehen ~";
        subjunctive_i_second: "gehest ~", "gehet ~";
        subjunctive_i_third: "gehe ~", referred;
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
        subordinate_subjunctive_i_third: "~gehe", referred;
        subordinate_subjunctive_i_i_first: "~ginge", "~gingen";
        subordinate_subjunctive_i_i_second: "~gingest", "~ginget";
        subordinate_subjunctive_i_i_third: "~ginge", "~gingen";
        imperative: "geh ~", "geht ~";
        infinitive: "~gehen";
        zu_infinitive: "~zugehen";
        participle: "~gehend", "~gegangen";
    ),
    // Zulegen: 54 words (zuhören, zuschauen, zumachen, zukleben, zustimmen, zulangen).
    conjugation!("zulegen", 54, "zu~en";
        present_first: "~e zu", "~en zu";
        present_second: "~st zu", "~t zu";
        present_third: "~t zu", "~en zu";
        preterite_first: "~te zu", "~ten zu";
        preterite_second: "~test zu", "~tet zu";
        preterite_third: "~te zu", "~ten zu";
        subjunctive_i_first: "~e zu", "~en zu";
        subjunctive_i_second: "~est zu", "~et zu";
        subjunctive_i_third: "~e zu", referred;
        subjunctive_i_i_first: "~te zu", "~ten zu";
        subjunctive_i_i_second: "~test zu", "~tet zu";
        subjunctive_i_i_third: "~te zu", "~ten zu";
        subordinate_present_first: "zu~e", "zu~en";
        subordinate_present_second: "zu~st", "zu~t";
        subordinate_present_third: "zu~t", "zu~en";
        subordinate_preterite_first: "zu~te", "zu~ten";
        subordinate_preterite_second: "zu~test", "zu~tet";
        subordinate_preterite_third: "zu~te", "zu~ten";
        subordinate_subjunctive_i_first: "zu~e", "zu~en";
        subordinate_subjunctive_i_second: "zu~est", "zu~et";
        subordinate_subjunctive_i_third: "zu~e", referred;
        subordinate_subjunctive_i_i_first: "zu~te", "zu~ten";
        subordinate_subjunctive_i_i_second: "zu~test", "zu~tet";
        subordinate_subjunctive_i_i_third: "zu~te", "zu~ten";
        imperative: "~ zu|~e zu", "~t zu";
        infinitive: "zu~en";
        zu_infinitive: "zuzu~en";
        participle: "zu~end", "zuge~t";
    ),
    // Abändern: 52 words (abändern, abmagern, abfackeln, abfeuern, absondern, abkapseln).
    conjugation!("abändern", 52, "ab~e~n";
        present_first: "~~e ab|~e~e ab|~e~ ab", "~e~n ab";
        present_second: "~e~st ab", "~e~t ab";
        present_third: "~e~t ab", "~e~n ab";
        preterite_first: "~e~te ab", "~e~ten ab";
        preterite_second: "~e~test ab", "~e~tet ab";
        preterite_third: "~e~te ab", "~e~ten ab";
        subjunctive_i_first: "~e~e ab|~~e ab", "~e~n ab";
        subjunctive_i_second: "~e~est ab|~~est ab", "~e~et ab|~~et ab";
        subjunctive_i_third: "~e~e ab|~~e ab", referred;
        subjunctive_i_i_first: "~e~te ab", "~e~ten ab";
        subjunctive_i_i_second: "~e~test ab", "~e~tet ab";
        subjunctive_i_i_third: "~e~te ab", "~e~ten ab";
        subordinate_present_first: "ab~~e|ab~e~e|ab~e~", "ab~e~n";
        subordinate_present_second: "ab~e~st", "ab~e~t";
        subordinate_present_third: "ab~e~t", "ab~e~n";
        subordinate_preterite_first: "ab~e~te", "ab~e~ten";
        subordinate_preterite_second: "ab~e~test", "ab~e~tet";
        subordinate_preterite_third: "ab~e~te", "ab~e~ten";
        subordinate_subjunctive_i_first: "ab~e~e|ab~~e", "ab~e~n";
        subordinate_subjunctive_i_second: "ab~e~est|ab~~est", "ab~e~et|ab~~et";
        subordinate_subjunctive_i_third: "ab~e~e|ab~~e", referred;
        subordinate_subjunctive_i_i_first: "ab~e~te", "ab~e~ten";
        subordinate_subjunctive_i_i_second: "ab~e~test", "ab~e~tet";
        subordinate_subjunctive_i_i_third: "ab~e~te", "ab~e~ten";
        imperative: "~~e ab|~e~ ab|~e~e ab", "~e~t ab";
        infinitive: "ab~e~n";
        zu_infinitive: "abzu~e~n";
        participle: "ab~e~nd", "abge~e~t";
    ),
    // Geben: 50 words (geben, widersprechen, verbrechen, erbrechen, unterbrechen, versprechen).
    conjugation!("geben", 50, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~o~en";
    ),
    // Beschreiben: 49 words (verschreiben, unterschreiben, verleihen, betreiben, erscheinen, beschreiben).
    conjugation!("beschreiben", 49, "~e~~en";
        present_first: "~e~~e", "~e~~en";
        present_second: "~e~~st", "~e~~t";
        present_third: "~e~~t", "~e~~en";
        preterite_first: "~~e~", "~~e~en";
        preterite_second: "~~e~st", "~~e~t";
        preterite_third: "~~e~", "~~e~en";
        subjunctive_i_first: "~e~~e", "~e~~en";
        subjunctive_i_second: "~e~~est", "~e~~et";
        subjunctive_i_third: "~e~~e", referred;
        subjunctive_i_i_first: "~~e~e", "~~e~en";
        subjunctive_i_i_second: "~~e~est", "~~e~et";
        subjunctive_i_i_third: "~~e~e", "~~e~en";
        subordinate_present_first: "~e~~e", "~e~~en";
        subordinate_present_second: "~e~~st", "~e~~t";
        subordinate_present_third: "~e~~t", "~e~~en";
        subordinate_preterite_first: "~~e~", "~~e~en";
        subordinate_preterite_second: "~~e~st", "~~e~t";
        subordinate_preterite_third: "~~e~", "~~e~en";
        subordinate_subjunctive_i_first: "~e~~e", "~e~~en";
        subordinate_subjunctive_i_second: "~e~~est", "~e~~et";
        subordinate_subjunctive_i_third: "~e~~e", referred;
        subordinate_subjunctive_i_i_first: "~~e~e", "~~e~en";
        subordinate_subjunctive_i_i_second: "~~e~est", "~~e~et";
        subordinate_subjunctive_i_i_third: "~~e~e", "~~e~en";
        imperative: "~e~~", "~e~~t";
        infinitive: "~e~~en";
        zu_infinitive: "~~~zuziehen";
        participle: "~e~~end", "~~e~en";
    ),
    // Überspannen: 48 words (überwachen, überqueren, überlegen, überraschen, überwältigen, überzeugen).
    conjugation!("überspannen", 48, "über~en";
        present_first: "über~e", "über~en";
        present_second: "über~st", "über~t";
        present_third: "über~t", "über~en";
        preterite_first: "über~te", "über~ten";
        preterite_second: "über~test", "über~tet";
        preterite_third: "über~te", "über~ten";
        subjunctive_i_first: "über~e", "über~en";
        subjunctive_i_second: "über~est", "über~et";
        subjunctive_i_third: "über~e", referred;
        subjunctive_i_i_first: "über~te", "über~ten";
        subjunctive_i_i_second: "über~test", "über~tet";
        subjunctive_i_i_third: "über~te", "über~ten";
        subordinate_present_first: "über~e", "über~en";
        subordinate_present_second: "über~st", "über~t";
        subordinate_present_third: "über~t", "über~en";
        subordinate_preterite_first: "über~te", "über~ten";
        subordinate_preterite_second: "über~test", "über~tet";
        subordinate_preterite_third: "über~te", "über~ten";
        subordinate_subjunctive_i_first: "über~e", "über~en";
        subordinate_subjunctive_i_second: "über~est", "über~et";
        subordinate_subjunctive_i_third: "über~e", referred;
        subordinate_subjunctive_i_i_first: "über~te", "über~ten";
        subordinate_subjunctive_i_i_second: "über~test", "über~tet";
        subordinate_subjunctive_i_i_third: "über~te", "über~ten";
        imperative: "über~", "über~t";
        infinitive: "über~en";
        zu_infinitive: "zu über~en";
        participle: "über~end", "über~t";
    ),
    // Wegrauchen: 45 words (wegschieben, wegfliegen, wegschießen, wegjagen, weglegen, wegdrängen).
    conjugation!("wegrauchen", 45, "weg~en";
        present_first: "~e weg", "~en weg";
        present_second: "~st weg", "~t weg";
        present_third: "~t weg", "~en weg";
        preterite_first: "~te weg", "~ten weg";
        preterite_second: "~test weg", "~tet weg";
        preterite_third: "~te weg", "~ten weg";
        subjunctive_i_first: "~e weg", "~en weg";
        subjunctive_i_second: "~est weg", "~et weg";
        subjunctive_i_third: "~e weg", referred;
        subjunctive_i_i_first: "~te weg", "~ten weg";
        subjunctive_i_i_second: "~test weg", "~tet weg";
        subjunctive_i_i_third: "~te weg", "~ten weg";
        subordinate_present_first: "weg~e", "weg~en";
        subordinate_present_second: "weg~st", "weg~t";
        subordinate_present_third: "weg~t", "weg~en";
        subordinate_preterite_first: "weg~te", "weg~ten";
        subordinate_preterite_second: "weg~test", "weg~tet";
        subordinate_preterite_third: "weg~te", "weg~ten";
        subordinate_subjunctive_i_first: "weg~e", "weg~en";
        subordinate_subjunctive_i_second: "weg~est", "weg~et";
        subordinate_subjunctive_i_third: "weg~e", referred;
        subordinate_subjunctive_i_i_first: "weg~te", "weg~ten";
        subordinate_subjunctive_i_i_second: "weg~test", "weg~tet";
        subordinate_subjunctive_i_i_third: "weg~te", "weg~ten";
        imperative: "~ weg|~e weg", "~t weg";
        infinitive: "weg~en";
        zu_infinitive: "wegzu~en";
        participle: "weg~end", "wegge~t";
    ),
    // Verschlagen: 44 words (beschlagen, übertragen, durchfahren, zuschlagen, erfahren, begraben).
    conjugation!("verschlagen", 44, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~t";
        present_third: "~ä~t", "~a~en";
        preterite_first: "~u~", "~u~en";
        preterite_second: "~u~st", "~u~t";
        preterite_third: "~u~", "~u~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
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
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est", "~ü~et";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~a~", "~a~t";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "~a~en";
    ),
    // Fertigmachen: 44 words (wiedergutmachen, fertigmachen, mitmachen, wettmachen, kaltmachen, mobilmachen).
    conjugation!("fertigmachen", 44, "~machen";
        present_first: "mache ~", "machen ~";
        present_second: "machst ~", "macht ~";
        present_third: "macht ~", "machen ~";
        preterite_first: "machte ~", "machten ~";
        preterite_second: "machtest ~", "machtet ~";
        preterite_third: "machte ~", "machten ~";
        subjunctive_i_first: "mache ~", "machen ~";
        subjunctive_i_second: "machest ~", "machet ~";
        subjunctive_i_third: "mache ~", referred;
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
        subordinate_subjunctive_i_third: "~mache", referred;
        subordinate_subjunctive_i_i_first: "~machte", "~machten";
        subordinate_subjunctive_i_i_second: "~machtest", "~machtet";
        subordinate_subjunctive_i_i_third: "~machte", "~machten";
        imperative: "mach ~", "macht ~";
        infinitive: "~machen";
        zu_infinitive: "~zumachen";
        participle: "~machend", "~gemacht";
    ),
    // Ausbreiten: 43 words (ausblenden, ausrotten, aushärten, ausrüsten, auslasten, ausweiden).
    conjugation!("ausbreiten", 43, "aus~en";
        present_first: "~e aus", "~en aus";
        present_second: "~est aus", "~et aus";
        present_third: "~et aus", "~en aus";
        preterite_first: "~ete aus", "~eten aus";
        preterite_second: "~etest aus", "~etet aus";
        preterite_third: "~ete aus", "~eten aus";
        subjunctive_i_first: "~e aus", "~en aus";
        subjunctive_i_second: "~est aus", "~et aus";
        subjunctive_i_third: "~e aus", referred;
        subjunctive_i_i_first: "~ete aus", "~eten aus";
        subjunctive_i_i_second: "~etest aus", "~etet aus";
        subjunctive_i_i_third: "~ete aus", "~eten aus";
        subordinate_present_first: "aus~e", "aus~en";
        subordinate_present_second: "aus~est", "aus~et";
        subordinate_present_third: "aus~et", "aus~en";
        subordinate_preterite_first: "aus~ete", "aus~eten";
        subordinate_preterite_second: "aus~etest", "aus~etet";
        subordinate_preterite_third: "aus~ete", "aus~eten";
        subordinate_subjunctive_i_first: "aus~e", "aus~en";
        subordinate_subjunctive_i_second: "aus~est", "aus~et";
        subordinate_subjunctive_i_third: "aus~e", referred;
        subordinate_subjunctive_i_i_first: "aus~ete", "aus~eten";
        subordinate_subjunctive_i_i_second: "aus~etest", "aus~etet";
        subordinate_subjunctive_i_i_third: "aus~ete", "aus~eten";
        imperative: "~ aus|~e aus", "~et aus";
        infinitive: "aus~en";
        zu_infinitive: "auszu~en";
        participle: "aus~end", "ausge~et";
    ),
    // Nachhängen: 39 words (nachfolgen, nachdenken, nachahmen, nachdrucken, nachvollziehen, nachäffen).
    conjugation!("nachhängen", 39, "nach~en";
        present_first: "~e nach", "~en nach";
        present_second: "~st nach", "~t nach";
        present_third: "~t nach", "~en nach";
        preterite_first: "~te nach", "~ten nach";
        preterite_second: "~test nach", "~tet nach";
        preterite_third: "~te nach", "~ten nach";
        subjunctive_i_first: "~e nach", "~en nach";
        subjunctive_i_second: "~est nach", "~et nach";
        subjunctive_i_third: "~e nach", referred;
        subjunctive_i_i_first: "~te nach", "~ten nach";
        subjunctive_i_i_second: "~test nach", "~tet nach";
        subjunctive_i_i_third: "~te nach", "~ten nach";
        subordinate_present_first: "nach~e", "nach~en";
        subordinate_present_second: "nach~st", "nach~t";
        subordinate_present_third: "nach~t", "nach~en";
        subordinate_preterite_first: "nach~te", "nach~ten";
        subordinate_preterite_second: "nach~test", "nach~tet";
        subordinate_preterite_third: "nach~te", "nach~ten";
        subordinate_subjunctive_i_first: "nach~e", "nach~en";
        subordinate_subjunctive_i_second: "nach~est", "nach~et";
        subordinate_subjunctive_i_third: "nach~e", referred;
        subordinate_subjunctive_i_i_first: "nach~te", "nach~ten";
        subordinate_subjunctive_i_i_second: "nach~test", "nach~tet";
        subordinate_subjunctive_i_i_third: "nach~te", "nach~ten";
        imperative: "~ nach|~e nach", "~t nach";
        infinitive: "nach~en";
        zu_infinitive: "nachzu~en";
        participle: "nach~end", "nachge~t";
    ),
    // Abschneiden: 38 words (abarbeiten, abtöten, abbilden, abblenden, abmelden, abschneiden).
    conjugation!("abschneiden", 38, "ab~en";
        present_first: "~e ab", "~en ab";
        present_second: "~est ab", "~et ab";
        present_third: "~et ab", "~en ab";
        preterite_first: "~ete ab", "~eten ab";
        preterite_second: "~etest ab", "~etet ab";
        preterite_third: "~ete ab", "~eten ab";
        subjunctive_i_first: "~e ab", "~en ab";
        subjunctive_i_second: "~est ab", "~et ab";
        subjunctive_i_third: "~e ab", referred;
        subjunctive_i_i_first: "~ete ab", "~eten ab";
        subjunctive_i_i_second: "~etest ab", "~etet ab";
        subjunctive_i_i_third: "~ete ab", "~eten ab";
        subordinate_present_first: "ab~e", "ab~en";
        subordinate_present_second: "ab~est", "ab~et";
        subordinate_present_third: "ab~et", "ab~en";
        subordinate_preterite_first: "ab~ete", "ab~eten";
        subordinate_preterite_second: "ab~etest", "ab~etet";
        subordinate_preterite_third: "ab~ete", "ab~eten";
        subordinate_subjunctive_i_first: "ab~e", "ab~en";
        subordinate_subjunctive_i_second: "ab~est", "ab~et";
        subordinate_subjunctive_i_third: "ab~e", referred;
        subordinate_subjunctive_i_i_first: "ab~ete", "ab~eten";
        subordinate_subjunctive_i_i_second: "ab~etest", "ab~etet";
        subordinate_subjunctive_i_i_third: "ab~ete", "ab~eten";
        imperative: "~ ab|~e ab", "~et ab";
        infinitive: "ab~en";
        zu_infinitive: "abzu~en";
        participle: "ab~end", "abge~et";
    ),
    // Durchbringen: 37 words (durchkämmen, durchdrehen, durchschauen, durchstreichen, durchführen, durchmachen).
    conjugation!("durchbringen", 37, "durch~en";
        present_first: "~e durch", "~en durch";
        present_second: "~st durch", "~t durch";
        present_third: "~t durch", "~en durch";
        preterite_first: "~te durch", "~ten durch";
        preterite_second: "~test durch", "~tet durch";
        preterite_third: "~te durch", "~ten durch";
        subjunctive_i_first: "~e durch", "~en durch";
        subjunctive_i_second: "~est durch", "~et durch";
        subjunctive_i_third: "~e durch", referred;
        subjunctive_i_i_first: "~te durch", "~ten durch";
        subjunctive_i_i_second: "~test durch", "~tet durch";
        subjunctive_i_i_third: "~te durch", "~ten durch";
        subordinate_present_first: "durch~e", "durch~en";
        subordinate_present_second: "durch~st", "durch~t";
        subordinate_present_third: "durch~t", "durch~en";
        subordinate_preterite_first: "durch~te", "durch~ten";
        subordinate_preterite_second: "durch~test", "durch~tet";
        subordinate_preterite_third: "durch~te", "durch~ten";
        subordinate_subjunctive_i_first: "durch~e", "durch~en";
        subordinate_subjunctive_i_second: "durch~est", "durch~et";
        subordinate_subjunctive_i_third: "durch~e", referred;
        subordinate_subjunctive_i_i_first: "durch~te", "durch~ten";
        subordinate_subjunctive_i_i_second: "durch~test", "durch~tet";
        subordinate_subjunctive_i_i_third: "durch~te", "durch~ten";
        imperative: "~ durch", "~t durch";
        infinitive: "durch~en";
        zu_infinitive: "durchzu~en";
        participle: "durch~end", "durchge~t";
    ),
    // Abgeben: 37 words (abgeben, angeben, aufgeben, zurückgeben, zugeben, nachgeben).
    conjugation!("abgeben", 37, "~geben";
        present_first: "gebe ~", "geben ~";
        present_second: "gibst ~", "gebt ~";
        present_third: "gibt ~", "geben ~";
        preterite_first: "gab ~", "gaben ~";
        preterite_second: "gabst ~", "gabt ~";
        preterite_third: "gab ~", "gaben ~";
        subjunctive_i_first: "gebe ~", "geben ~";
        subjunctive_i_second: "gebest ~", "gebet ~";
        subjunctive_i_third: "gebe ~", referred;
        subjunctive_i_i_first: "gäbe ~", "gäben ~";
        subjunctive_i_i_second: "gäbest ~", "gäbet ~";
        subjunctive_i_i_third: "gäbe ~", "gäben ~";
        subordinate_present_first: "~gebe", "~geben";
        subordinate_present_second: "~gibst", "~gebt";
        subordinate_present_third: "~gibt", "~geben";
        subordinate_preterite_first: "~gab", "~gaben";
        subordinate_preterite_second: "~gabst", "~gabt";
        subordinate_preterite_third: "~gab", "~gaben";
        subordinate_subjunctive_i_first: "~gebe", "~geben";
        subordinate_subjunctive_i_second: "~gebest", "~gebet";
        subordinate_subjunctive_i_third: "~gebe", referred;
        subordinate_subjunctive_i_i_first: "~gäbe", "~gäben";
        subordinate_subjunctive_i_i_second: "~gäbest", "~gäbet";
        subordinate_subjunctive_i_i_third: "~gäbe", "~gäben";
        imperative: "gib ~", "gebt ~";
        infinitive: "~geben";
        zu_infinitive: "~zugeben";
        participle: "~gebend", "~gegeben";
    ),
    // Mitkönnen: 37 words (abkönnen, ankönnen, umhinkönnen, aufkönnen, dafürkönnen, fortkönnen).
    conjugation!("mitkönnen", 37, "~können";
        present_first: "kann ~", "können ~";
        present_second: "kannst ~", "könnt ~";
        present_third: "kann ~", "können ~";
        preterite_first: "konnte ~", "konnten ~";
        preterite_second: "konntest ~", "konntet ~";
        preterite_third: "konnte ~", "konnten ~";
        subjunctive_i_first: "könne ~", "können ~";
        subjunctive_i_second: "könnest ~", "könnet ~";
        subjunctive_i_third: "könne ~", referred;
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
        subordinate_subjunctive_i_third: "~könne", referred;
        subordinate_subjunctive_i_i_first: "~könnte", "~könnten";
        subordinate_subjunctive_i_i_second: "~könntest", "~könntet";
        subordinate_subjunctive_i_i_third: "~könnte", "~könnten";
        imperative: "~", "~t";
        infinitive: "~können";
        zu_infinitive: "~zukönnen";
        participle: "~könnend", "~gekonnt";
    ),
    // Anlagern: 36 words (anstacheln, anprangern, ansteuern, annähern, anreichern, anzweifeln).
    conjugation!("anlagern", 36, "an~e~n";
        present_first: "~~e an|~e~e an|~e~ an", "~e~n an";
        present_second: "~e~st an", "~e~t an";
        present_third: "~e~t an", "~e~n an";
        preterite_first: "~e~te an", "~e~ten an";
        preterite_second: "~e~test an", "~e~tet an";
        preterite_third: "~e~te an", "~e~ten an";
        subjunctive_i_first: "~e~e an|~~e an", "~e~n an";
        subjunctive_i_second: "~e~est an|~~est an", "~e~et an|~~et an";
        subjunctive_i_third: "~e~e an|~~e an", referred;
        subjunctive_i_i_first: "~e~te an", "~e~ten an";
        subjunctive_i_i_second: "~e~test an", "~e~tet an";
        subjunctive_i_i_third: "~e~te an", "~e~ten an";
        subordinate_present_first: "an~~e|an~e~e|an~e~", "an~e~n";
        subordinate_present_second: "an~e~st", "an~e~t";
        subordinate_present_third: "an~e~t", "an~e~n";
        subordinate_preterite_first: "an~e~te", "an~e~ten";
        subordinate_preterite_second: "an~e~test", "an~e~tet";
        subordinate_preterite_third: "an~e~te", "an~e~ten";
        subordinate_subjunctive_i_first: "an~e~e|an~~e", "an~e~n";
        subordinate_subjunctive_i_second: "an~e~est|an~~est", "an~e~et|an~~et";
        subordinate_subjunctive_i_third: "an~e~e|an~~e", referred;
        subordinate_subjunctive_i_i_first: "an~e~te", "an~e~ten";
        subordinate_subjunctive_i_i_second: "an~e~test", "an~e~tet";
        subordinate_subjunctive_i_i_third: "an~e~te", "an~e~ten";
        imperative: "~~e an|~e~ an|~e~e an", "~e~t an";
        infinitive: "an~e~n";
        zu_infinitive: "anzu~e~n";
        participle: "an~e~nd", "ange~e~t";
    ),
    // Erwerben: 35 words (versterben, übertreffen, betreffen, erschrecken, unterwerfen, erwerben).
    conjugation!("erwerben", 35, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est", "~ü~et";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~i~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~o~en";
    ),
    // Vorstellen: 34 words (vorstellen, vorrücken, vorschreiben, vortäuschen, vorherrschen, vorlegen).
    conjugation!("vorstellen", 34, "vor~en";
        present_first: "~e vor", "~en vor";
        present_second: "~st vor", "~t vor";
        present_third: "~t vor", "~en vor";
        preterite_first: "~te vor", "~ten vor";
        preterite_second: "~test vor", "~tet vor";
        preterite_third: "~te vor", "~ten vor";
        subjunctive_i_first: "~e vor", "~en vor";
        subjunctive_i_second: "~est vor", "~et vor";
        subjunctive_i_third: "~e vor", referred;
        subjunctive_i_i_first: "~te vor", "~ten vor";
        subjunctive_i_i_second: "~test vor", "~tet vor";
        subjunctive_i_i_third: "~te vor", "~ten vor";
        subordinate_present_first: "vor~e", "vor~en";
        subordinate_present_second: "vor~st", "vor~t";
        subordinate_present_third: "vor~t", "vor~en";
        subordinate_preterite_first: "vor~te", "vor~ten";
        subordinate_preterite_second: "vor~test", "vor~tet";
        subordinate_preterite_third: "vor~te", "vor~ten";
        subordinate_subjunctive_i_first: "vor~e", "vor~en";
        subordinate_subjunctive_i_second: "vor~est", "vor~et";
        subordinate_subjunctive_i_third: "vor~e", referred;
        subordinate_subjunctive_i_i_first: "vor~te", "vor~ten";
        subordinate_subjunctive_i_i_second: "vor~test", "vor~tet";
        subordinate_subjunctive_i_i_third: "vor~te", "vor~ten";
        imperative: "~ vor|~e vor", "~t vor";
        infinitive: "vor~en";
        zu_infinitive: "vorzu~en";
        participle: "vor~end", "vorge~t";
    ),
    // Einlassen: 34 words (ablassen, loslassen, einlassen, auslassen, nachlassen, freilassen).
    conjugation!("einlassen", 34, "~lassen";
        present_first: "lasse ~", "lassen ~";
        present_second: "lässt ~", "lasst ~";
        present_third: "lässt ~", "lassen ~";
        preterite_first: "ließ ~", "ließen ~";
        preterite_second: "ließest ~", "ließt ~";
        preterite_third: "ließ ~", "ließen ~";
        subjunctive_i_first: "lasse ~", "lassen ~";
        subjunctive_i_second: "lassest ~", "lasset ~";
        subjunctive_i_third: "lasse ~", referred;
        subjunctive_i_i_first: "ließe ~", "ließen ~";
        subjunctive_i_i_second: "ließest ~", "ließet ~";
        subjunctive_i_i_third: "ließe ~", "ließen ~";
        subordinate_present_first: "~lasse", "~lassen";
        subordinate_present_second: "~lässt", "~lasst";
        subordinate_present_third: "~lässt", "~lassen";
        subordinate_preterite_first: "~ließ", "~ließen";
        subordinate_preterite_second: "~ließest", "~ließt";
        subordinate_preterite_third: "~ließ", "~ließen";
        subordinate_subjunctive_i_first: "~lasse", "~lassen";
        subordinate_subjunctive_i_second: "~lassest", "~lasset";
        subordinate_subjunctive_i_third: "~lasse", referred;
        subordinate_subjunctive_i_i_first: "~ließe", "~ließen";
        subordinate_subjunctive_i_i_second: "~ließest", "~ließet";
        subordinate_subjunctive_i_i_third: "~ließe", "~ließen";
        imperative: "lass ~", "lasst ~";
        infinitive: "~lassen";
        zu_infinitive: "~zulassen";
        participle: "~lassend", "~gelassen";
    ),
    // Absetzen: 33 words (abkürzen, abfräsen, abholzen, absetzen, abspritzen, abnutzen).
    conjugation!("absetzen", 33, "ab~en";
        present_first: "~e ab", "~en ab";
        present_second: "~t ab", "~t ab";
        present_third: "~t ab", "~en ab";
        preterite_first: "~te ab", "~ten ab";
        preterite_second: "~test ab", "~tet ab";
        preterite_third: "~te ab", "~ten ab";
        subjunctive_i_first: "~e ab", "~en ab";
        subjunctive_i_second: "~est ab", "~et ab";
        subjunctive_i_third: "~e ab", referred;
        subjunctive_i_i_first: "~te ab", "~ten ab";
        subjunctive_i_i_second: "~test ab", "~tet ab";
        subjunctive_i_i_third: "~te ab", "~ten ab";
        subordinate_present_first: "ab~e", "ab~en";
        subordinate_present_second: "ab~t", "ab~t";
        subordinate_present_third: "ab~t", "ab~en";
        subordinate_preterite_first: "ab~te", "ab~ten";
        subordinate_preterite_second: "ab~test", "ab~tet";
        subordinate_preterite_third: "ab~te", "ab~ten";
        subordinate_subjunctive_i_first: "ab~e", "ab~en";
        subordinate_subjunctive_i_second: "ab~est", "ab~et";
        subordinate_subjunctive_i_third: "ab~e", referred;
        subordinate_subjunctive_i_i_first: "ab~te", "ab~ten";
        subordinate_subjunctive_i_i_second: "ab~test", "ab~tet";
        subordinate_subjunctive_i_i_third: "ab~te", "ab~ten";
        imperative: "~ ab|~e ab", "~t ab";
        infinitive: "ab~en";
        zu_infinitive: "abzu~en";
        participle: "ab~end", "abge~t";
    ),
    // Vertrinken: 33 words (gelingen, bezwingen, verschlingen, entspringen, erklingen, erzwingen).
    conjugation!("vertrinken", 33, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~st", "~i~t";
        present_third: "~i~t", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", referred;
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
        subordinate_subjunctive_i_third: "~i~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~|~i~e", "~i~t";
        infinitive: "~i~en";
        zu_infinitive: "abzu~e~n";
        participle: "~i~end", "~u~en";
    ),
    // Umlegen: 33 words (umdrehen, umziehen, umtauschen, umbringen, umbauen, umbenennen).
    conjugation!("umlegen", 33, "um~en";
        present_first: "~e um", "~en um";
        present_second: "~st um", "~t um";
        present_third: "~t um", "~en um";
        preterite_first: "~te um", "~ten um";
        preterite_second: "~test um", "~tet um";
        preterite_third: "~te um", "~ten um";
        subjunctive_i_first: "~e um", "~en um";
        subjunctive_i_second: "~est um", "~et um";
        subjunctive_i_third: "~e um", referred;
        subjunctive_i_i_first: "~te um", "~ten um";
        subjunctive_i_i_second: "~test um", "~tet um";
        subjunctive_i_i_third: "~te um", "~ten um";
        subordinate_present_first: "um~e", "um~en";
        subordinate_present_second: "um~st", "um~t";
        subordinate_present_third: "um~t", "um~en";
        subordinate_preterite_first: "um~te", "um~ten";
        subordinate_preterite_second: "um~test", "um~tet";
        subordinate_preterite_third: "um~te", "um~ten";
        subordinate_subjunctive_i_first: "um~e", "um~en";
        subordinate_subjunctive_i_second: "um~est", "um~et";
        subordinate_subjunctive_i_third: "um~e", referred;
        subordinate_subjunctive_i_i_first: "um~te", "um~ten";
        subordinate_subjunctive_i_i_second: "um~test", "um~tet";
        subordinate_subjunctive_i_i_third: "um~te", "um~ten";
        imperative: "~ um|~e um", "~t um";
        infinitive: "um~en";
        zu_infinitive: "umzu~en";
        participle: "um~end", "umge~t";
    ),
    // Auftragen: 33 words (beitragen, auftragen, zutragen, mittragen, emportragen, beytragen).
    conjugation!("auftragen", 33, "~tragen";
        present_first: "trage ~", "tragen ~";
        present_second: "trägst ~", "tragt ~";
        present_third: "trägt ~", "tragen ~";
        preterite_first: "trug ~", "trugen ~";
        preterite_second: "trugst ~", "trugt ~";
        preterite_third: "trug ~", "trugen ~";
        subjunctive_i_first: "trage ~", "tragen ~";
        subjunctive_i_second: "tragest ~", "traget ~";
        subjunctive_i_third: "trage ~", referred;
        subjunctive_i_i_first: "trüge ~", "trügen ~";
        subjunctive_i_i_second: "trügest ~", "trüget ~";
        subjunctive_i_i_third: "trüge ~", "trügen ~";
        subordinate_present_first: "~trage", "~tragen";
        subordinate_present_second: "~trägst", "~tragt";
        subordinate_present_third: "~trägt", "~tragen";
        subordinate_preterite_first: "~trug", "~trugen";
        subordinate_preterite_second: "~trugst", "~trugt";
        subordinate_preterite_third: "~trug", "~trugen";
        subordinate_subjunctive_i_first: "~trage", "~tragen";
        subordinate_subjunctive_i_second: "~tragest", "~traget";
        subordinate_subjunctive_i_third: "~trage", referred;
        subordinate_subjunctive_i_i_first: "~trüge", "~trügen";
        subordinate_subjunctive_i_i_second: "~trügest", "~trüget";
        subordinate_subjunctive_i_i_third: "~trüge", "~trügen";
        imperative: "trag ~", "tragt ~";
        infinitive: "~tragen";
        zu_infinitive: "~zutragen";
        participle: "~tragend", "~getragen";
    ),
    // Abnehmen: 32 words (annehmen, einnehmen, teilnehmen, wahrnehmen, zunehmen, abnehmen).
    conjugation!("abnehmen", 32, "~nehmen";
        present_first: "nehme ~", "nehmen ~";
        present_second: "nimmst ~", "nehmt ~";
        present_third: "nimmt ~", "nehmen ~";
        preterite_first: "nahm ~", "nahmen ~";
        preterite_second: "nahmst ~", "nahmt ~";
        preterite_third: "nahm ~", "nahmen ~";
        subjunctive_i_first: "nehme ~", "nehmen ~";
        subjunctive_i_second: "nehmest ~", "nehmet ~";
        subjunctive_i_third: "nehme ~", referred;
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
        subordinate_subjunctive_i_third: "~nehme", referred;
        subordinate_subjunctive_i_i_first: "~nähme", "~nähmen";
        subordinate_subjunctive_i_i_second: "~nähmest", "~nähmet";
        subordinate_subjunctive_i_i_third: "~nähme", "~nähmen";
        imperative: "nimm ~", "nehmt ~";
        infinitive: "~nehmen";
        zu_infinitive: "~zunehmen";
        participle: "~nehmend", "~genommen";
    ),
    // Zurückgreifen: 32 words (zurückbringen, zurückweisen, zurückbleiben, zurückverfolgen, zurückschreiben, zurückwenden).
    conjugation!("zurückgreifen", 32, "zurück~en";
        present_first: "~e zurück", "~en zurück";
        present_second: "~st zurück", "~t zurück";
        present_third: "~t zurück", "~en zurück";
        preterite_first: "~te zurück", "~ten zurück";
        preterite_second: "~test zurück", "~tet zurück";
        preterite_third: "~te zurück", "~ten zurück";
        subjunctive_i_first: "~e zurück", "~en zurück";
        subjunctive_i_second: "~est zurück", "~et zurück";
        subjunctive_i_third: "~e zurück", referred;
        subjunctive_i_i_first: "~te zurück", "~ten zurück";
        subjunctive_i_i_second: "~test zurück", "~tet zurück";
        subjunctive_i_i_third: "~te zurück", "~ten zurück";
        subordinate_present_first: "zurück~e", "zurück~en";
        subordinate_present_second: "zurück~st", "zurück~t";
        subordinate_present_third: "zurück~t", "zurück~en";
        subordinate_preterite_first: "zurück~te", "zurück~ten";
        subordinate_preterite_second: "zurück~test", "zurück~tet";
        subordinate_preterite_third: "zurück~te", "zurück~ten";
        subordinate_subjunctive_i_first: "zurück~e", "zurück~en";
        subordinate_subjunctive_i_second: "zurück~est", "zurück~et";
        subordinate_subjunctive_i_third: "zurück~e", referred;
        subordinate_subjunctive_i_i_first: "zurück~te", "zurück~ten";
        subordinate_subjunctive_i_i_second: "zurück~test", "zurück~tet";
        subordinate_subjunctive_i_i_third: "zurück~te", "zurück~ten";
        imperative: "~ zurück", "~t zurück";
        infinitive: "zurück~en";
        zu_infinitive: "zurückzu~en";
        participle: "zurück~end", "zurückge~t";
    ),
    // Ausmustern: 31 words (ausräuchern, auswandern, ausliefern, ausmustern, auslagern, aushandeln).
    conjugation!("ausmustern", 31, "aus~e~n";
        present_first: "~~e aus|~e~e aus|~e~ aus", "~e~n aus";
        present_second: "~e~st aus", "~e~t aus";
        present_third: "~e~t aus", "~e~n aus";
        preterite_first: "~e~te aus", "~e~ten aus";
        preterite_second: "~e~test aus", "~e~tet aus";
        preterite_third: "~e~te aus", "~e~ten aus";
        subjunctive_i_first: "~e~e aus|~~e aus", "~e~n aus";
        subjunctive_i_second: "~e~est aus|~~est aus", "~e~et aus|~~et aus";
        subjunctive_i_third: "~e~e aus|~~e aus", referred;
        subjunctive_i_i_first: "~e~te aus", "~e~ten aus";
        subjunctive_i_i_second: "~e~test aus", "~e~tet aus";
        subjunctive_i_i_third: "~e~te aus", "~e~ten aus";
        subordinate_present_first: "aus~~e|aus~e~e|aus~e~", "aus~e~n";
        subordinate_present_second: "aus~e~st", "aus~e~t";
        subordinate_present_third: "aus~e~t", "aus~e~n";
        subordinate_preterite_first: "aus~e~te", "aus~e~ten";
        subordinate_preterite_second: "aus~e~test", "aus~e~tet";
        subordinate_preterite_third: "aus~e~te", "aus~e~ten";
        subordinate_subjunctive_i_first: "aus~e~e|aus~~e", "aus~e~n";
        subordinate_subjunctive_i_second: "aus~e~est|aus~~est", "aus~e~et|aus~~et";
        subordinate_subjunctive_i_third: "aus~e~e|aus~~e", referred;
        subordinate_subjunctive_i_i_first: "aus~e~te", "aus~e~ten";
        subordinate_subjunctive_i_i_second: "aus~e~test", "aus~e~tet";
        subordinate_subjunctive_i_i_third: "aus~e~te", "aus~e~ten";
        imperative: "~~e aus|~e~ aus|~e~e aus", "~e~t aus";
        infinitive: "aus~e~n";
        zu_infinitive: "auszu~e~n";
        participle: "aus~e~nd", "ausge~e~t";
    ),
    // Anzünden: 31 words (anbieten, anzünden, anbeten, anleiten, ankreiden, aneignen).
    conjugation!("anzünden", 31, "an~en";
        present_first: "~e an", "~en an";
        present_second: "~est an", "~et an";
        present_third: "~et an", "~en an";
        preterite_first: "~ete an", "~eten an";
        preterite_second: "~etest an", "~etet an";
        preterite_third: "~ete an", "~eten an";
        subjunctive_i_first: "~e an", "~en an";
        subjunctive_i_second: "~est an", "~et an";
        subjunctive_i_third: "~e an", referred;
        subjunctive_i_i_first: "~ete an", "~eten an";
        subjunctive_i_i_second: "~etest an", "~etet an";
        subjunctive_i_i_third: "~ete an", "~eten an";
        subordinate_present_first: "an~e", "an~en";
        subordinate_present_second: "an~est", "an~et";
        subordinate_present_third: "an~et", "an~en";
        subordinate_preterite_first: "an~ete", "an~eten";
        subordinate_preterite_second: "an~etest", "an~etet";
        subordinate_preterite_third: "an~ete", "an~eten";
        subordinate_subjunctive_i_first: "an~e", "an~en";
        subordinate_subjunctive_i_second: "an~est", "an~et";
        subordinate_subjunctive_i_third: "an~e", referred;
        subordinate_subjunctive_i_i_first: "an~ete", "an~eten";
        subordinate_subjunctive_i_i_second: "an~etest", "an~etet";
        subordinate_subjunctive_i_i_third: "an~ete", "an~eten";
        imperative: "~ an|~e an", "~et an";
        infinitive: "an~en";
        zu_infinitive: "anzu~en";
        participle: "an~end", "ange~et";
    ),
    // Ansetzen: 30 words (anfassen, anschließen, anreisen, anschwärzen, anpassen, ankratzen).
    conjugation!("ansetzen", 30, "an~en";
        present_first: "~e an", "~en an";
        present_second: "~t an", "~t an";
        present_third: "~t an", "~en an";
        preterite_first: "~te an", "~ten an";
        preterite_second: "~test an", "~tet an";
        preterite_third: "~te an", "~ten an";
        subjunctive_i_first: "~e an", "~en an";
        subjunctive_i_second: "~est an", "~et an";
        subjunctive_i_third: "~e an", referred;
        subjunctive_i_i_first: "~te an", "~ten an";
        subjunctive_i_i_second: "~test an", "~tet an";
        subjunctive_i_i_third: "~te an", "~ten an";
        subordinate_present_first: "an~e", "an~en";
        subordinate_present_second: "an~t", "an~t";
        subordinate_present_third: "an~t", "an~en";
        subordinate_preterite_first: "an~te", "an~ten";
        subordinate_preterite_second: "an~test", "an~tet";
        subordinate_preterite_third: "an~te", "an~ten";
        subordinate_subjunctive_i_first: "an~e", "an~en";
        subordinate_subjunctive_i_second: "an~est", "an~et";
        subordinate_subjunctive_i_third: "an~e", referred;
        subordinate_subjunctive_i_i_first: "an~te", "an~ten";
        subordinate_subjunctive_i_i_second: "an~test", "an~tet";
        subordinate_subjunctive_i_i_third: "an~te", "an~ten";
        imperative: "~ an|~e an", "~t an";
        infinitive: "an~en";
        zu_infinitive: "anzu~en";
        participle: "an~end", "ange~t";
    ),
    // Anstehen: 30 words (aufstehen, strammstehen, beistehen, ausstehen, bevorstehen, stillstehen).
    conjugation!("anstehen", 30, "~stehen";
        present_first: "stehe ~", "stehen ~";
        present_second: "stehst ~", "steht ~";
        present_third: "steht ~", "stehen ~";
        preterite_first: "stand ~", "standen ~";
        preterite_second: "standest ~", "standet ~";
        preterite_third: "stand ~", "standen ~";
        subjunctive_i_first: "stehe ~", "stehen ~";
        subjunctive_i_second: "stehest ~", "stehet ~";
        subjunctive_i_third: "stehe ~", referred;
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
        subordinate_subjunctive_i_third: "~stehe", referred;
        subordinate_subjunctive_i_i_first: "~stände", "~ständen";
        subordinate_subjunctive_i_i_second: "~ständest", "~ständet";
        subordinate_subjunctive_i_i_third: "~stände", "~ständen";
        imperative: "steh ~", "steht ~";
        infinitive: "~stehen";
        zu_infinitive: "~zustehen";
        participle: "~stehend", "~gestanden";
    ),
    // Einziehen: 30 words (durchziehen, aufziehen, ausziehen, zurückziehen, vorziehen, zusammenziehen).
    conjugation!("einziehen", 30, "~~~ziehen";
        present_first: "zie~~ he~", "zie~~n he~";
        present_second: "zie~st h~~", "zie~t h~~";
        present_third: "zie~t h~~", "zie~~n he~";
        preterite_first: "zog ~~~", "zogen ~~~";
        preterite_second: "zogst ~~~", "zogt ~~~";
        preterite_third: "zog ~~~", "zogen ~~~";
        subjunctive_i_first: "zie~~ he~", "zie~~n he~";
        subjunctive_i_second: "zie~~st he~", "zie~~t he~";
        subjunctive_i_third: "zie~~ he~", referred;
        subjunctive_i_i_first: "zöge ~~~", "zögen ~~~";
        subjunctive_i_i_second: "zögest ~~~", "zöget ~~~";
        subjunctive_i_i_third: "zöge ~~~", "zögen ~~~";
        subordinate_present_first: "~~~ziehe", "~~~ziehen";
        subordinate_present_second: "~~~ziehst", "~~~zieht";
        subordinate_present_third: "~~~zieht", "~~~ziehen";
        subordinate_preterite_first: "~~~zog", "~~~zogen";
        subordinate_preterite_second: "~~~zogst", "~~~zogt";
        subordinate_preterite_third: "~~~zog", "~~~zogen";
        subordinate_subjunctive_i_first: "~~~ziehe", "~~~ziehen";
        subordinate_subjunctive_i_second: "~~~ziehest", "~~~ziehet";
        subordinate_subjunctive_i_third: "~~~ziehe", referred;
        subordinate_subjunctive_i_i_first: "~~~zöge", "~~~zögen";
        subordinate_subjunctive_i_i_second: "~~~zögest", "~~~zöget";
        subordinate_subjunctive_i_i_third: "~~~zöge", "~~~zögen";
        imperative: "zie~ h~~", "zie~t h~~";
        infinitive: "~~~ziehen";
        zu_infinitive: "~~~zuziehen";
        participle: "~~~ziehend", "~~~gezogen";
    ),
    // Glauben: 29 words (glauben, gähnen, glühen, gönnen, grauen, garen).
    conjugation!("glauben", 29, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "~eg~t";
    ),
    // Einfädeln: 29 words (einschüchtern, einwandern, einkapseln, einschmeicheln, einwickeln, eintrudeln).
    conjugation!("einfädeln", 29, "ein~e~n";
        present_first: "~~e ein|~e~e ein|~e~ ein", "~e~n ein";
        present_second: "~e~st ein", "~e~t ein";
        present_third: "~e~t ein", "~e~n ein";
        preterite_first: "~e~te ein", "~e~ten ein";
        preterite_second: "~e~test ein", "~e~tet ein";
        preterite_third: "~e~te ein", "~e~ten ein";
        subjunctive_i_first: "~e~e ein|~~e ein", "~e~n ein";
        subjunctive_i_second: "~e~est ein|~~est ein", "~e~et ein|~~et ein";
        subjunctive_i_third: "~e~e ein|~~e ein", referred;
        subjunctive_i_i_first: "~e~te ein", "~e~ten ein";
        subjunctive_i_i_second: "~e~test ein", "~e~tet ein";
        subjunctive_i_i_third: "~e~te ein", "~e~ten ein";
        subordinate_present_first: "ein~~e|ein~e~e|ein~e~", "ein~e~n";
        subordinate_present_second: "ein~e~st", "ein~e~t";
        subordinate_present_third: "ein~e~t", "ein~e~n";
        subordinate_preterite_first: "ein~e~te", "ein~e~ten";
        subordinate_preterite_second: "ein~e~test", "ein~e~tet";
        subordinate_preterite_third: "ein~e~te", "ein~e~ten";
        subordinate_subjunctive_i_first: "ein~e~e|ein~~e", "ein~e~n";
        subordinate_subjunctive_i_second: "ein~e~est|ein~~est", "ein~e~et|ein~~et";
        subordinate_subjunctive_i_third: "ein~e~e|ein~~e", referred;
        subordinate_subjunctive_i_i_first: "ein~e~te", "ein~e~ten";
        subordinate_subjunctive_i_i_second: "ein~e~test", "ein~e~tet";
        subordinate_subjunctive_i_i_third: "ein~e~te", "ein~e~ten";
        imperative: "~~e ein|~e~ ein|~e~e ein", "~e~t ein";
        infinitive: "ein~e~n";
        zu_infinitive: "einzu~e~n";
        participle: "ein~e~nd", "einge~e~t";
    ),
    // Sein: 29 words (sein, berechnen, bezeichnen, bewaffnen, entwaffnen, eröffnen).
    conjugation!("sein", 29, "~n";
        present_first: "~", "~n";
        present_second: "~st", "~t";
        present_third: "~t", "~n";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~", "~n";
        subjunctive_i_second: "~st", "~t";
        subjunctive_i_third: "~", referred;
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
        subordinate_subjunctive_i_third: "~", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~", "~t";
        infinitive: "~n";
        zu_infinitive: "sich zu ~en";
        participle: "~nd", "~t";
    ),
    // Aufrappeln: 29 words (aufmuntern, aufbessern, aufdröseln, aufheitern, aufsatteln, aufopfern).
    conjugation!("aufrappeln", 29, "auf~e~n";
        present_first: "~~e auf|~e~e auf|~e~ auf", "~e~n auf";
        present_second: "~e~st auf", "~e~t auf";
        present_third: "~e~t auf", "~e~n auf";
        preterite_first: "~e~te auf", "~e~ten auf";
        preterite_second: "~e~test auf", "~e~tet auf";
        preterite_third: "~e~te auf", "~e~ten auf";
        subjunctive_i_first: "~e~e auf|~~e auf", "~e~n auf";
        subjunctive_i_second: "~e~est auf|~~est auf", "~e~et auf|~~et auf";
        subjunctive_i_third: "~e~e auf|~~e auf", referred;
        subjunctive_i_i_first: "~e~te auf", "~e~ten auf";
        subjunctive_i_i_second: "~e~test auf", "~e~tet auf";
        subjunctive_i_i_third: "~e~te auf", "~e~ten auf";
        subordinate_present_first: "auf~~e|auf~e~e|auf~e~", "auf~e~n";
        subordinate_present_second: "auf~e~st", "auf~e~t";
        subordinate_present_third: "auf~e~t", "auf~e~n";
        subordinate_preterite_first: "auf~e~te", "auf~e~ten";
        subordinate_preterite_second: "auf~e~test", "auf~e~tet";
        subordinate_preterite_third: "auf~e~te", "auf~e~ten";
        subordinate_subjunctive_i_first: "auf~e~e|auf~~e", "auf~e~n";
        subordinate_subjunctive_i_second: "auf~e~est|auf~~est", "auf~e~et|auf~~et";
        subordinate_subjunctive_i_third: "auf~e~e|auf~~e", referred;
        subordinate_subjunctive_i_i_first: "auf~e~te", "auf~e~ten";
        subordinate_subjunctive_i_i_second: "auf~e~test", "auf~e~tet";
        subordinate_subjunctive_i_i_third: "auf~e~te", "auf~e~ten";
        imperative: "~~e auf|~e~ auf|~e~e auf", "~e~t auf";
        infinitive: "auf~e~n";
        zu_infinitive: "aufzu~e~n";
        participle: "auf~e~nd", "aufge~e~t";
    ),
    // Ausspannen: 29 words (ausstellen, ausschenken, ausstrahlen, aussparen, ausstrecken, aussagen).
    conjugation!("ausspannen", 29, "au~s~en";
        present_first: "~~e aus", "~~en aus";
        present_second: "~~st aus", "~~t aus";
        present_third: "~~t aus", "~~en aus";
        preterite_first: "~~te aus", "~~ten aus";
        preterite_second: "~~test aus", "~~tet aus";
        preterite_third: "~~te aus", "~~ten aus";
        subjunctive_i_first: "~~e aus", "~~en aus";
        subjunctive_i_second: "~~est aus", "~~et aus";
        subjunctive_i_third: "~~e aus", referred;
        subjunctive_i_i_first: "~~te aus", "~~ten aus";
        subjunctive_i_i_second: "~~test aus", "~~tet aus";
        subjunctive_i_i_third: "~~te aus", "~~ten aus";
        subordinate_present_first: "au~s~e", "au~s~en";
        subordinate_present_second: "au~s~st", "au~s~t";
        subordinate_present_third: "au~s~t", "au~s~en";
        subordinate_preterite_first: "au~s~te", "au~s~ten";
        subordinate_preterite_second: "au~s~test", "au~s~tet";
        subordinate_preterite_third: "au~s~te", "au~s~ten";
        subordinate_subjunctive_i_first: "au~s~e", "au~s~en";
        subordinate_subjunctive_i_second: "au~s~est", "au~s~et";
        subordinate_subjunctive_i_third: "au~s~e", referred;
        subordinate_subjunctive_i_i_first: "au~s~te", "au~s~ten";
        subordinate_subjunctive_i_i_second: "au~s~test", "au~s~tet";
        subordinate_subjunctive_i_i_third: "au~s~te", "au~s~ten";
        imperative: "~~ aus|~~e aus", "~~t aus";
        infinitive: "au~s~en";
        zu_infinitive: "au~zus~en";
        participle: "au~s~end", "au~ges~t";
    ),
    // Begießen: 28 words (genießen, verdrießen, erschießen, beschließen, ergießen, verschließen).
    conjugation!("begießen", 28, "~ießen";
        present_first: "~ieße", "~ießen";
        present_second: "~ießt", "~ießt";
        present_third: "~ießt", "~ießen";
        preterite_first: "~oss", "~ossen";
        preterite_second: "~ossest|~osst", "~osst";
        preterite_third: "~oss", "~ossen";
        subjunctive_i_first: "~ieße", "~ießen";
        subjunctive_i_second: "~ießest", "~ießet";
        subjunctive_i_third: "~ieße", referred;
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
        subordinate_subjunctive_i_third: "~ieße", referred;
        subordinate_subjunctive_i_i_first: "~össe", "~össen";
        subordinate_subjunctive_i_i_second: "~össest|~össt", "~össet|~össt";
        subordinate_subjunctive_i_i_third: "~össe", "~össen";
        imperative: "~ieß|~ieße", "~ießt";
        infinitive: "~ießen";
        zu_infinitive: "abzu~en";
        participle: "~ießend", "~ossen";
    ),
    // Einrichten: 28 words (einbetten, einschalten, einmotten, einleiten, einrichten, einfinden).
    conjugation!("einrichten", 28, "ein~~n";
        present_first: "~~ ein", "~~n ein";
        present_second: "~~st ein", "~~t ein";
        present_third: "~~t ein", "~~n ein";
        preterite_first: "~~te ein", "~~ten ein";
        preterite_second: "~~test ein", "~~tet ein";
        preterite_third: "~~te ein", "~~ten ein";
        subjunctive_i_first: "~~ ein", "~~n ein";
        subjunctive_i_second: "~~st ein", "~~t ein";
        subjunctive_i_third: "~~ ein", referred;
        subjunctive_i_i_first: "~~te ein", "~~ten ein";
        subjunctive_i_i_second: "~~test ein", "~~tet ein";
        subjunctive_i_i_third: "~~te ein", "~~ten ein";
        subordinate_present_first: "ein~~", "ein~~n";
        subordinate_present_second: "ein~~st", "ein~~t";
        subordinate_present_third: "ein~~t", "ein~~n";
        subordinate_preterite_first: "ein~~te", "ein~~ten";
        subordinate_preterite_second: "ein~~test", "ein~~tet";
        subordinate_preterite_third: "ein~~te", "ein~~ten";
        subordinate_subjunctive_i_first: "ein~~", "ein~~n";
        subordinate_subjunctive_i_second: "ein~~st", "ein~~t";
        subordinate_subjunctive_i_third: "ein~~", referred;
        subordinate_subjunctive_i_i_first: "ein~~te", "ein~~ten";
        subordinate_subjunctive_i_i_second: "ein~~test", "ein~~tet";
        subordinate_subjunctive_i_i_third: "ein~~te", "ein~~ten";
        imperative: "~ ~in|~~ ein", "~~t ein";
        infinitive: "ein~~n";
        zu_infinitive: "einzu~~n";
        participle: "ein~~nd", "einge~~t";
    ),
    // Auftun: 28 words (betun, vertun, abtun, antun, wehtun, auftun).
    conjugation!("auftun", 28, "~tun";
        present_first: "tue ~", "tun ~";
        present_second: "tust ~", "tut ~";
        present_third: "tut ~", "tun ~";
        preterite_first: "tat ~", "taten ~";
        preterite_second: "tatest ~|tatst ~", "tatet ~";
        preterite_third: "tat ~", "taten ~";
        subjunctive_i_first: "tue ~", "tuen ~";
        subjunctive_i_second: "tuest ~", "tuet ~";
        subjunctive_i_third: "tue ~", referred;
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
        subordinate_subjunctive_i_third: "~tue", referred;
        subordinate_subjunctive_i_i_first: "~täte", "~täten";
        subordinate_subjunctive_i_i_second: "~tätest", "~tätet";
        subordinate_subjunctive_i_i_third: "~täte", "~täten";
        imperative: "tu ~|tue ~", "tut ~";
        infinitive: "~tun";
        zu_infinitive: "~zutun";
        participle: "~tuend", "~getan";
    ),
    // Ablaufen: 28 words (umlaufen, überlaufen, ablaufen, anlaufen, davonlaufen, auslaufen).
    conjugation!("ablaufen", 28, "~laufen";
        present_first: "laufe ~", "laufen ~";
        present_second: "läufst ~", "lauft ~";
        present_third: "läuft ~", "laufen ~";
        preterite_first: "lief ~", "liefen ~";
        preterite_second: "liefst ~", "lieft ~";
        preterite_third: "lief ~", "liefen ~";
        subjunctive_i_first: "laufe ~", "laufen ~";
        subjunctive_i_second: "laufest ~", "laufet ~";
        subjunctive_i_third: "laufe ~", referred;
        subjunctive_i_i_first: "liefe ~", "liefen ~";
        subjunctive_i_i_second: "liefest ~", "liefet ~";
        subjunctive_i_i_third: "liefe ~", "liefen ~";
        subordinate_present_first: "~laufe", "~laufen";
        subordinate_present_second: "~läufst", "~lauft";
        subordinate_present_third: "~läuft", "~laufen";
        subordinate_preterite_first: "~lief", "~liefen";
        subordinate_preterite_second: "~liefst", "~lieft";
        subordinate_preterite_third: "~lief", "~liefen";
        subordinate_subjunctive_i_first: "~laufe", "~laufen";
        subordinate_subjunctive_i_second: "~laufest", "~laufet";
        subordinate_subjunctive_i_third: "~laufe", referred;
        subordinate_subjunctive_i_i_first: "~liefe", "~liefen";
        subordinate_subjunctive_i_i_second: "~liefest", "~liefet";
        subordinate_subjunctive_i_i_third: "~liefe", "~liefen";
        imperative: "lauf ~", "lauft ~";
        infinitive: "~laufen";
        zu_infinitive: "~zulaufen";
        participle: "~laufend", "~gelaufen";
    ),
    // Aussetzen: 27 words (ausmerzen, aussetzen, auslösen, ausschließen, ausnutzen, ausreizen).
    conjugation!("aussetzen", 27, "aus~en";
        present_first: "~e aus", "~en aus";
        present_second: "~t aus", "~t aus";
        present_third: "~t aus", "~en aus";
        preterite_first: "~te aus", "~ten aus";
        preterite_second: "~test aus", "~tet aus";
        preterite_third: "~te aus", "~ten aus";
        subjunctive_i_first: "~e aus", "~en aus";
        subjunctive_i_second: "~est aus", "~et aus";
        subjunctive_i_third: "~e aus", referred;
        subjunctive_i_i_first: "~te aus", "~ten aus";
        subjunctive_i_i_second: "~test aus", "~tet aus";
        subjunctive_i_i_third: "~te aus", "~ten aus";
        subordinate_present_first: "aus~e", "aus~en";
        subordinate_present_second: "aus~t", "aus~t";
        subordinate_present_third: "aus~t", "aus~en";
        subordinate_preterite_first: "aus~te", "aus~ten";
        subordinate_preterite_second: "aus~test", "aus~tet";
        subordinate_preterite_third: "aus~te", "aus~ten";
        subordinate_subjunctive_i_first: "aus~e", "aus~en";
        subordinate_subjunctive_i_second: "aus~est", "aus~et";
        subordinate_subjunctive_i_third: "aus~e", referred;
        subordinate_subjunctive_i_i_first: "aus~te", "aus~ten";
        subordinate_subjunctive_i_i_second: "aus~test", "aus~tet";
        subordinate_subjunctive_i_i_third: "aus~te", "aus~ten";
        imperative: "~ aus|~e aus", "~t aus";
        infinitive: "aus~en";
        zu_infinitive: "auszu~en";
        participle: "aus~end", "ausge~t";
    ),
    // Auffahren: 27 words (mitfahren, radfahren, fortfahren, vorbeifahren, herunterfahren, entlangfahren).
    conjugation!("auffahren", 27, "~fahren";
        present_first: "fahre ~", "fahren ~";
        present_second: "fährst ~", "fahrt ~";
        present_third: "fährt ~", "fahren ~";
        preterite_first: "fuhr ~", "fuhren ~";
        preterite_second: "fuhrst ~", "fuhrt ~";
        preterite_third: "fuhr ~", "fuhren ~";
        subjunctive_i_first: "fahre ~", "fahren ~";
        subjunctive_i_second: "fahrest ~", "fahret ~";
        subjunctive_i_third: "fahre ~", referred;
        subjunctive_i_i_first: "führe ~", "führen ~";
        subjunctive_i_i_second: "führest ~", "führet ~";
        subjunctive_i_i_third: "führe ~", "führen ~";
        subordinate_present_first: "~fahre", "~fahren";
        subordinate_present_second: "~fährst", "~fahrt";
        subordinate_present_third: "~fährt", "~fahren";
        subordinate_preterite_first: "~fuhr", "~fuhren";
        subordinate_preterite_second: "~fuhrst", "~fuhrt";
        subordinate_preterite_third: "~fuhr", "~fuhren";
        subordinate_subjunctive_i_first: "~fahre", "~fahren";
        subordinate_subjunctive_i_second: "~fahrest", "~fahret";
        subordinate_subjunctive_i_third: "~fahre", referred;
        subordinate_subjunctive_i_i_first: "~führe", "~führen";
        subordinate_subjunctive_i_i_second: "~führest", "~führet";
        subordinate_subjunctive_i_i_third: "~führe", "~führen";
        imperative: "fahr ~", "fahrt ~";
        infinitive: "~fahren";
        zu_infinitive: "~zufahren";
        participle: "~fahrend", "~gefahren";
    ),
    // Verhalten: 26 words (verhalten, nachgeraten, verraten, erhalten, unterhalten, beraten).
    conjugation!("verhalten", 26, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~et";
        present_third: "~ä~", "~a~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~est|~ie~st", "~ie~et";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
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
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~a~|~a~e", "~a~et";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "~a~en";
    ),
    // Einsetzen: 26 words (einstürzen, eindosen, einspritzen, einschätzen, einsetzen, einflößen).
    conjugation!("einsetzen", 26, "ein~en";
        present_first: "~e ein", "~en ein";
        present_second: "~t ein", "~t ein";
        present_third: "~t ein", "~en ein";
        preterite_first: "~te ein", "~ten ein";
        preterite_second: "~test ein", "~tet ein";
        preterite_third: "~te ein", "~ten ein";
        subjunctive_i_first: "~e ein", "~en ein";
        subjunctive_i_second: "~est ein", "~et ein";
        subjunctive_i_third: "~e ein", referred;
        subjunctive_i_i_first: "~te ein", "~ten ein";
        subjunctive_i_i_second: "~test ein", "~tet ein";
        subjunctive_i_i_third: "~te ein", "~ten ein";
        subordinate_present_first: "ein~e", "ein~en";
        subordinate_present_second: "ein~t", "ein~t";
        subordinate_present_third: "ein~t", "ein~en";
        subordinate_preterite_first: "ein~te", "ein~ten";
        subordinate_preterite_second: "ein~test", "ein~tet";
        subordinate_preterite_third: "ein~te", "ein~ten";
        subordinate_subjunctive_i_first: "ein~e", "ein~en";
        subordinate_subjunctive_i_second: "ein~est", "ein~et";
        subordinate_subjunctive_i_third: "ein~e", referred;
        subordinate_subjunctive_i_i_first: "ein~te", "ein~ten";
        subordinate_subjunctive_i_i_second: "ein~test", "ein~tet";
        subordinate_subjunctive_i_i_third: "ein~te", "ein~ten";
        imperative: "~ ein|~e ein", "~t ein";
        infinitive: "ein~en";
        zu_infinitive: "einzu~en";
        participle: "ein~end", "einge~t";
    ),
    // Ansehen: 25 words (fernsehen, aussehen, voraussehen, vorhersehen, hellsehen, wiedersehen).
    conjugation!("ansehen", 25, "~sehen";
        present_first: "sehe ~", "sehen ~";
        present_second: "siehst ~", "seht ~";
        present_third: "sieht ~", "sehen ~";
        preterite_first: "sah ~", "sahen ~";
        preterite_second: "sahst ~", "saht ~";
        preterite_third: "sah ~", "sahen ~";
        subjunctive_i_first: "sehe ~", "sehen ~";
        subjunctive_i_second: "sehest ~", "sehet ~";
        subjunctive_i_third: "sehe ~", referred;
        subjunctive_i_i_first: "sähe ~", "sähen ~";
        subjunctive_i_i_second: "sähest ~", "sähet ~";
        subjunctive_i_i_third: "sähe ~", "sähen ~";
        subordinate_present_first: "~sehe", "~sehen";
        subordinate_present_second: "~siehst", "~seht";
        subordinate_present_third: "~sieht", "~sehen";
        subordinate_preterite_first: "~sah", "~sahen";
        subordinate_preterite_second: "~sahst", "~saht";
        subordinate_preterite_third: "~sah", "~sahen";
        subordinate_subjunctive_i_first: "~sehe", "~sehen";
        subordinate_subjunctive_i_second: "~sehest", "~sehet";
        subordinate_subjunctive_i_third: "~sehe", referred;
        subordinate_subjunctive_i_i_first: "~sähe", "~sähen";
        subordinate_subjunctive_i_i_second: "~sähest", "~sähet";
        subordinate_subjunctive_i_i_third: "~sähe", "~sähen";
        imperative: "sieh ~", "seht ~";
        infinitive: "~sehen";
        zu_infinitive: "~zusehen";
        participle: "~sehend", "~gesehen";
    ),
    // Aufsitzen: 24 words (auflösen, aufpassen, aufschluchzen, aufschließen, aufheizen, aufplatzen).
    conjugation!("aufsitzen", 24, "auf~en";
        present_first: "~e auf", "~en auf";
        present_second: "~t auf", "~t auf";
        present_third: "~t auf", "~en auf";
        preterite_first: "~te auf", "~ten auf";
        preterite_second: "~test auf", "~tet auf";
        preterite_third: "~te auf", "~ten auf";
        subjunctive_i_first: "~e auf", "~en auf";
        subjunctive_i_second: "~est auf", "~et auf";
        subjunctive_i_third: "~e auf", referred;
        subjunctive_i_i_first: "~te auf", "~ten auf";
        subjunctive_i_i_second: "~test auf", "~tet auf";
        subjunctive_i_i_third: "~te auf", "~ten auf";
        subordinate_present_first: "auf~e", "auf~en";
        subordinate_present_second: "auf~t", "auf~t";
        subordinate_present_third: "auf~t", "auf~en";
        subordinate_preterite_first: "auf~te", "auf~ten";
        subordinate_preterite_second: "auf~test", "auf~tet";
        subordinate_preterite_third: "auf~te", "auf~ten";
        subordinate_subjunctive_i_first: "auf~e", "auf~en";
        subordinate_subjunctive_i_second: "auf~est", "auf~et";
        subordinate_subjunctive_i_third: "auf~e", referred;
        subordinate_subjunctive_i_i_first: "auf~te", "auf~ten";
        subordinate_subjunctive_i_i_second: "auf~test", "auf~tet";
        subordinate_subjunctive_i_i_third: "auf~te", "auf~ten";
        imperative: "~ auf|~e auf", "~t auf";
        infinitive: "auf~en";
        zu_infinitive: "aufzu~en";
        participle: "auf~end", "aufge~t";
    ),
    // Gabeln: 24 words (gurgeln, gackern, gipfeln, googeln, gabeln, gliedern).
    conjugation!("gabeln", 24, "~~e~n";
        present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        present_second: "~~e~st", "~~e~t";
        present_third: "~~e~t", "~~e~n";
        preterite_first: "~~e~te", "~~e~ten";
        preterite_second: "~~e~test", "~~e~tet";
        preterite_third: "~~e~te", "~~e~ten";
        subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subjunctive_i_third: "~~e~e|~~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e~e|~~~e", referred;
        subordinate_subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subordinate_subjunctive_i_i_third: "~~e~te", "~~e~ten";
        imperative: "~~~e|~~e~|~~e~e", "~~e~t";
        infinitive: "~~e~n";
        zu_infinitive: "~~~zuziehen";
        participle: "~~e~nd", "~eg~e~t";
    ),
    // Aufarbeiten: 23 words (aufrüsten, aufzeichnen, aufrunden, aufbürden, aufatmen, aufwenden).
    conjugation!("aufarbeiten", 23, "auf~en";
        present_first: "~e auf", "~en auf";
        present_second: "~est auf", "~et auf";
        present_third: "~et auf", "~en auf";
        preterite_first: "~ete auf", "~eten auf";
        preterite_second: "~etest auf", "~etet auf";
        preterite_third: "~ete auf", "~eten auf";
        subjunctive_i_first: "~e auf", "~en auf";
        subjunctive_i_second: "~est auf", "~et auf";
        subjunctive_i_third: "~e auf", referred;
        subjunctive_i_i_first: "~ete auf", "~eten auf";
        subjunctive_i_i_second: "~etest auf", "~etet auf";
        subjunctive_i_i_third: "~ete auf", "~eten auf";
        subordinate_present_first: "auf~e", "auf~en";
        subordinate_present_second: "auf~est", "auf~et";
        subordinate_present_third: "auf~et", "auf~en";
        subordinate_preterite_first: "auf~ete", "auf~eten";
        subordinate_preterite_second: "auf~etest", "auf~etet";
        subordinate_preterite_third: "auf~ete", "auf~eten";
        subordinate_subjunctive_i_first: "auf~e", "auf~en";
        subordinate_subjunctive_i_second: "auf~est", "auf~et";
        subordinate_subjunctive_i_third: "auf~e", referred;
        subordinate_subjunctive_i_i_first: "auf~ete", "auf~eten";
        subordinate_subjunctive_i_i_second: "auf~etest", "auf~etet";
        subordinate_subjunctive_i_i_third: "auf~ete", "auf~eten";
        imperative: "~ auf|~e auf", "~et auf";
        infinitive: "auf~en";
        zu_infinitive: "aufzu~en";
        participle: "auf~end", "aufge~et";
    ),
    // Zurückfallen: 23 words (auffallen, abfallen, einfallen, anfallen, ausfallen, zusammenfallen).
    conjugation!("zurückfallen", 23, "~fallen";
        present_first: "falle ~", "fallen ~";
        present_second: "fällst ~", "fallt ~";
        present_third: "fällt ~", "fallen ~";
        preterite_first: "fiel ~", "fielen ~";
        preterite_second: "fielst ~", "fielt ~";
        preterite_third: "fiel ~", "fielen ~";
        subjunctive_i_first: "falle ~", "fallen ~";
        subjunctive_i_second: "fallest ~", "fallet ~";
        subjunctive_i_third: "falle ~", referred;
        subjunctive_i_i_first: "fiele ~", "fielen ~";
        subjunctive_i_i_second: "fielest ~", "fielet ~";
        subjunctive_i_i_third: "fiele ~", "fielen ~";
        subordinate_present_first: "~falle", "~fallen";
        subordinate_present_second: "~fällst", "~fallt";
        subordinate_present_third: "~fällt", "~fallen";
        subordinate_preterite_first: "~fiel", "~fielen";
        subordinate_preterite_second: "~fielst", "~fielt";
        subordinate_preterite_third: "~fiel", "~fielen";
        subordinate_subjunctive_i_first: "~falle", "~fallen";
        subordinate_subjunctive_i_second: "~fallest", "~fallet";
        subordinate_subjunctive_i_third: "~falle", referred;
        subordinate_subjunctive_i_i_first: "~fiele", "~fielen";
        subordinate_subjunctive_i_i_second: "~fielest", "~fielet";
        subordinate_subjunctive_i_i_third: "~fiele", "~fielen";
        imperative: "fall ~", "fallt ~";
        infinitive: "~fallen";
        zu_infinitive: "~zufallen";
        participle: "~fallend", "~gefallen";
    ),
    // Aufhaben: 23 words (vorhaben, liebhaben, dahaben, anhaben, umhaben, teilhaben).
    conjugation!("aufhaben", 23, "~haben";
        present_first: "habe ~", "haben ~";
        present_second: "hast ~", "habt ~";
        present_third: "hat ~", "haben ~";
        preterite_first: "hatte ~", "hatten ~";
        preterite_second: "hattest ~", "hattet ~";
        preterite_third: "hatte ~", "hatten ~";
        subjunctive_i_first: "habe ~", "haben ~";
        subjunctive_i_second: "habest ~", "habet ~";
        subjunctive_i_third: "habe ~", referred;
        subjunctive_i_i_first: "hätte ~", "hätten ~";
        subjunctive_i_i_second: "hättest ~", "hättet ~";
        subjunctive_i_i_third: "hätte ~", "hätten ~";
        subordinate_present_first: "~habe", "~haben";
        subordinate_present_second: "~hast", "~habt";
        subordinate_present_third: "~hat", "~haben";
        subordinate_preterite_first: "~hatte", "~hatten";
        subordinate_preterite_second: "~hattest", "~hattet";
        subordinate_preterite_third: "~hatte", "~hatten";
        subordinate_subjunctive_i_first: "~habe", "~haben";
        subordinate_subjunctive_i_second: "~habest", "~habet";
        subordinate_subjunctive_i_third: "~habe", referred;
        subordinate_subjunctive_i_i_first: "~hätte", "~hätten";
        subordinate_subjunctive_i_i_second: "~hättest", "~hättet";
        subordinate_subjunctive_i_i_third: "~hätte", "~hätten";
        imperative: "hab ~", "habt ~";
        infinitive: "~haben";
        zu_infinitive: "~zuhaben";
        participle: "~habend", "~gehabt";
    ),
    // Geniessen: 22 words (geniessen, verschliessen, erschiessen, vergiessen, danebenschiessen, beschiessen).
    conjugation!("geniessen", 22, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~t", "~ie~t";
        present_third: "~ie~t", "~ie~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~est|~o~t", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", referred;
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
        subordinate_subjunctive_i_third: "~ie~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~t", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ie~|~ie~e", "~ie~t";
        infinitive: "~ie~en";
        zu_infinitive: "abzu~e~n";
        participle: "~ie~end", "~o~en";
    ),
    // Entbinden: 22 words (stattfinden, verbinden, verschwinden, überwinden, erfinden, empfinden).
    conjugation!("entbinden", 22, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~est", "~i~et";
        present_third: "~i~et", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~est", "~a~et";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~i~e", "~i~en";
        subordinate_present_second: "~i~est", "~i~et";
        subordinate_present_third: "~i~et", "~i~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~est", "~a~et";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_second: "~i~est", "~i~et";
        subordinate_subjunctive_i_third: "~i~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~", "~i~et";
        infinitive: "~i~en";
        zu_infinitive: "abzu~e~n";
        participle: "~i~end", "~u~en";
    ),
    // Eintreten: 22 words (zurücktreten, austreten, auftreten, beitreten, eintreten, antreten).
    conjugation!("eintreten", 22, "~treten";
        present_first: "trete ~", "treten ~";
        present_second: "trittst ~", "tretet ~";
        present_third: "tritt ~", "treten ~";
        preterite_first: "trat ~", "traten ~";
        preterite_second: "tratest ~", "tratet ~";
        preterite_third: "trat ~", "traten ~";
        subjunctive_i_first: "trete ~", "treten ~";
        subjunctive_i_second: "tretest ~", "tretet ~";
        subjunctive_i_third: "trete ~", referred;
        subjunctive_i_i_first: "träte ~", "träten ~";
        subjunctive_i_i_second: "trätest ~", "trätet ~";
        subjunctive_i_i_third: "träte ~", "träten ~";
        subordinate_present_first: "~trete", "~treten";
        subordinate_present_second: "~trittst", "~tretet";
        subordinate_present_third: "~tritt", "~treten";
        subordinate_preterite_first: "~trat", "~traten";
        subordinate_preterite_second: "~tratest", "~tratet";
        subordinate_preterite_third: "~trat", "~traten";
        subordinate_subjunctive_i_first: "~trete", "~treten";
        subordinate_subjunctive_i_second: "~tretest", "~tretet";
        subordinate_subjunctive_i_third: "~trete", referred;
        subordinate_subjunctive_i_i_first: "~träte", "~träten";
        subordinate_subjunctive_i_i_second: "~trätest", "~trätet";
        subordinate_subjunctive_i_i_third: "~träte", "~träten";
        imperative: "tritt ~", "tretet ~";
        infinitive: "~treten";
        zu_infinitive: "~zutreten";
        participle: "~tretend", "~getreten";
    ),
    // Herumhängen: 21 words (herumfliegen, herumtreiben, herumschleppen, herumfragen, herumführen, herumzupfen).
    conjugation!("herumhängen", 21, "herum~en";
        present_first: "~e herum", "~en herum";
        present_second: "~st herum", "~t herum";
        present_third: "~t herum", "~en herum";
        preterite_first: "~te herum", "~ten herum";
        preterite_second: "~test herum", "~tet herum";
        preterite_third: "~te herum", "~ten herum";
        subjunctive_i_first: "~e herum", "~en herum";
        subjunctive_i_second: "~est herum", "~et herum";
        subjunctive_i_third: "~e herum", referred;
        subjunctive_i_i_first: "~te herum", "~ten herum";
        subjunctive_i_i_second: "~test herum", "~tet herum";
        subjunctive_i_i_third: "~te herum", "~ten herum";
        subordinate_present_first: "herum~e", "herum~en";
        subordinate_present_second: "herum~st", "herum~t";
        subordinate_present_third: "herum~t", "herum~en";
        subordinate_preterite_first: "herum~te", "herum~ten";
        subordinate_preterite_second: "herum~test", "herum~tet";
        subordinate_preterite_third: "herum~te", "herum~ten";
        subordinate_subjunctive_i_first: "herum~e", "herum~en";
        subordinate_subjunctive_i_second: "herum~est", "herum~et";
        subordinate_subjunctive_i_third: "herum~e", referred;
        subordinate_subjunctive_i_i_first: "herum~te", "herum~ten";
        subordinate_subjunctive_i_i_second: "herum~test", "herum~tet";
        subordinate_subjunctive_i_i_third: "herum~te", "herum~ten";
        imperative: "~ herum", "~t herum";
        infinitive: "herum~en";
        zu_infinitive: "herumzu~en";
        participle: "herum~end", "herumge~t";
    ),
    // Zurücklegen: 20 words (zurücklegen, lahmlegen, stilllegen, darlegen, loslegen, niederlegen).
    conjugation!("zurücklegen", 20, "~legen";
        present_first: "lege ~", "legen ~";
        present_second: "legst ~", "legt ~";
        present_third: "legt ~", "legen ~";
        preterite_first: "legte ~", "legten ~";
        preterite_second: "legtest ~", "legtet ~";
        preterite_third: "legte ~", "legten ~";
        subjunctive_i_first: "lege ~", "legen ~";
        subjunctive_i_second: "legest ~", "leget ~";
        subjunctive_i_third: "lege ~", referred;
        subjunctive_i_i_first: "legte ~", "legten ~";
        subjunctive_i_i_second: "legtest ~", "legtet ~";
        subjunctive_i_i_third: "legte ~", "legten ~";
        subordinate_present_first: "~lege", "~legen";
        subordinate_present_second: "~legst", "~legt";
        subordinate_present_third: "~legt", "~legen";
        subordinate_preterite_first: "~legte", "~legten";
        subordinate_preterite_second: "~legtest", "~legtet";
        subordinate_preterite_third: "~legte", "~legten";
        subordinate_subjunctive_i_first: "~lege", "~legen";
        subordinate_subjunctive_i_second: "~legest", "~leget";
        subordinate_subjunctive_i_third: "~lege", referred;
        subordinate_subjunctive_i_i_first: "~legte", "~legten";
        subordinate_subjunctive_i_i_second: "~legtest", "~legtet";
        subordinate_subjunctive_i_i_third: "~legte", "~legten";
        imperative: "leg ~", "legt ~";
        infinitive: "~legen";
        zu_infinitive: "~zulegen";
        participle: "~legend", "~gelegt";
    ),
    // Auseinandersetzen: 19 words (zurücksetzen, festsetzen, fortsetzen, auseinandersetzen, herabsetzen, voraussetzen).
    conjugation!("auseinandersetzen", 19, "~setzen";
        present_first: "setze ~", "setzen ~";
        present_second: "setzt ~", "setzt ~";
        present_third: "setzt ~", "setzen ~";
        preterite_first: "setzte ~", "setzten ~";
        preterite_second: "setztest ~", "setztet ~";
        preterite_third: "setzte ~", "setzten ~";
        subjunctive_i_first: "setze ~", "setzen ~";
        subjunctive_i_second: "setzest ~", "setzet ~";
        subjunctive_i_third: "setze ~", referred;
        subjunctive_i_i_first: "setzte ~", "setzten ~";
        subjunctive_i_i_second: "setztest ~", "setztet ~";
        subjunctive_i_i_third: "setzte ~", "setzten ~";
        subordinate_present_first: "~setze", "~setzen";
        subordinate_present_second: "~setzt", "~setzt";
        subordinate_present_third: "~setzt", "~setzen";
        subordinate_preterite_first: "~setzte", "~setzten";
        subordinate_preterite_second: "~setztest", "~setztet";
        subordinate_preterite_third: "~setzte", "~setzten";
        subordinate_subjunctive_i_first: "~setze", "~setzen";
        subordinate_subjunctive_i_second: "~setzest", "~setzet";
        subordinate_subjunctive_i_third: "~setze", referred;
        subordinate_subjunctive_i_i_first: "~setzte", "~setzten";
        subordinate_subjunctive_i_i_second: "~setztest", "~setztet";
        subordinate_subjunctive_i_i_third: "~setzte", "~setzten";
        imperative: "setz ~", "setzt ~";
        infinitive: "~setzen";
        zu_infinitive: "~zusetzen";
        participle: "~setzend", "~gesetzt";
    ),
    // Anliegen: 19 words (vorliegen, darüberliegen, wachliegen, daliegen, darniederliegen, naheliegen).
    conjugation!("anliegen", 19, "~liegen";
        present_first: "liege ~", "liegen ~";
        present_second: "liegst ~", "liegt ~";
        present_third: "liegt ~", "liegen ~";
        preterite_first: "lag ~", "lagen ~";
        preterite_second: "lagst ~", "lagt ~";
        preterite_third: "lag ~", "lagen ~";
        subjunctive_i_first: "liege ~", "liegen ~";
        subjunctive_i_second: "liegest ~", "lieget ~";
        subjunctive_i_third: "liege ~", referred;
        subjunctive_i_i_first: "läge ~", "lägen ~";
        subjunctive_i_i_second: "lägest ~", "läget ~";
        subjunctive_i_i_third: "läge ~", "lägen ~";
        subordinate_present_first: "~liege", "~liegen";
        subordinate_present_second: "~liegst", "~liegt";
        subordinate_present_third: "~liegt", "~liegen";
        subordinate_preterite_first: "~lag", "~lagen";
        subordinate_preterite_second: "~lagst", "~lagt";
        subordinate_preterite_third: "~lag", "~lagen";
        subordinate_subjunctive_i_first: "~liege", "~liegen";
        subordinate_subjunctive_i_second: "~liegest", "~lieget";
        subordinate_subjunctive_i_third: "~liege", referred;
        subordinate_subjunctive_i_i_first: "~läge", "~lägen";
        subordinate_subjunctive_i_i_second: "~lägest", "~läget";
        subordinate_subjunctive_i_i_third: "~läge", "~lägen";
        imperative: "lieg ~", "liegt ~";
        infinitive: "~liegen";
        zu_infinitive: "~zuliegen";
        participle: "~liegend", "~gelegen";
    ),
    // Fortführen: 19 words (fortpflanzen, fortführen, fortschreiben, fortlocken, fortbestehen, fortleben).
    conjugation!("fortführen", 19, "fort~en";
        present_first: "~e fort", "~en fort";
        present_second: "~st fort", "~t fort";
        present_third: "~t fort", "~en fort";
        preterite_first: "~te fort", "~ten fort";
        preterite_second: "~test fort", "~tet fort";
        preterite_third: "~te fort", "~ten fort";
        subjunctive_i_first: "~e fort", "~en fort";
        subjunctive_i_second: "~est fort", "~et fort";
        subjunctive_i_third: "~e fort", referred;
        subjunctive_i_i_first: "~te fort", "~ten fort";
        subjunctive_i_i_second: "~test fort", "~tet fort";
        subjunctive_i_i_third: "~te fort", "~ten fort";
        subordinate_present_first: "fort~e", "fort~en";
        subordinate_present_second: "fort~st", "fort~t";
        subordinate_present_third: "fort~t", "fort~en";
        subordinate_preterite_first: "fort~te", "fort~ten";
        subordinate_preterite_second: "fort~test", "fort~tet";
        subordinate_preterite_third: "fort~te", "fort~ten";
        subordinate_subjunctive_i_first: "fort~e", "fort~en";
        subordinate_subjunctive_i_second: "fort~est", "fort~et";
        subordinate_subjunctive_i_third: "fort~e", referred;
        subordinate_subjunctive_i_i_first: "fort~te", "fort~ten";
        subordinate_subjunctive_i_i_second: "fort~test", "fort~tet";
        subordinate_subjunctive_i_i_third: "fort~te", "fort~ten";
        imperative: "~ fort|~e fort", "~t fort";
        infinitive: "fort~en";
        zu_infinitive: "fortzu~en";
        participle: "fort~end", "fortge~t";
    ),
    // Öffnen: 18 words (regnen, öffnen, atmen, rechnen, leugnen, zeichnen).
    conjugation!("öffnen", 18, "~n";
        present_first: "~", "~n";
        present_second: "~st", "~t";
        present_third: "~t", "~n";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~", "~n";
        subjunctive_i_second: "~st", "~t";
        subjunctive_i_third: "~", referred;
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
        subordinate_subjunctive_i_third: "~", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~", "~t";
        infinitive: "~n";
        zu_infinitive: "sich zu ~en";
        participle: "~nd", "ge~t";
    ),
    // Verschreien: 18 words (verschleissen, bespeien, zerreissen, verreissen, zusammenreissen, mitreissen).
    conjugation!("verschreien", 18, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~t", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~", "~~en";
        preterite_second: "~~est", "~~t";
        preterite_third: "~~", "~~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~~e", "~~en";
        subjunctive_i_i_second: "~~est", "~~et";
        subjunctive_i_i_third: "~~e", "~~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~t", "~e~t";
        subordinate_present_third: "~e~t", "~e~en";
        subordinate_preterite_first: "~~", "~~en";
        subordinate_preterite_second: "~~est", "~~t";
        subordinate_preterite_third: "~~", "~~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_i_second: "~~est", "~~et";
        subordinate_subjunctive_i_i_third: "~~e", "~~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~~en";
    ),
    // Umwenden: 18 words (umschichten, umleiten, umschalten, umarbeiten, umschulden, umschütten).
    conjugation!("umwenden", 18, "um~en";
        present_first: "~e um", "~en um";
        present_second: "~est um", "~et um";
        present_third: "~et um", "~en um";
        preterite_first: "~ete um", "~eten um";
        preterite_second: "~etest um", "~etet um";
        preterite_third: "~ete um", "~eten um";
        subjunctive_i_first: "~e um", "~en um";
        subjunctive_i_second: "~est um", "~et um";
        subjunctive_i_third: "~e um", referred;
        subjunctive_i_i_first: "~ete um", "~eten um";
        subjunctive_i_i_second: "~etest um", "~etet um";
        subjunctive_i_i_third: "~ete um", "~eten um";
        subordinate_present_first: "um~e", "um~en";
        subordinate_present_second: "um~est", "um~et";
        subordinate_present_third: "um~et", "um~en";
        subordinate_preterite_first: "um~ete", "um~eten";
        subordinate_preterite_second: "um~etest", "um~etet";
        subordinate_preterite_third: "um~ete", "um~eten";
        subordinate_subjunctive_i_first: "um~e", "um~en";
        subordinate_subjunctive_i_second: "um~est", "um~et";
        subordinate_subjunctive_i_third: "um~e", referred;
        subordinate_subjunctive_i_i_first: "um~ete", "um~eten";
        subordinate_subjunctive_i_i_second: "um~etest", "um~etet";
        subordinate_subjunctive_i_i_third: "um~ete", "um~eten";
        imperative: "~ um|~e um", "~et um";
        infinitive: "um~en";
        zu_infinitive: "umzu~en";
        participle: "um~end", "umge~et";
    ),
    // Zuschütten: 18 words (zubereiten, zumuten, zuordnen, zurechnen, zuwenden, zusenden).
    conjugation!("zuschütten", 18, "zu~en";
        present_first: "~e zu", "~en zu";
        present_second: "~est zu", "~et zu";
        present_third: "~et zu", "~en zu";
        preterite_first: "~ete zu", "~eten zu";
        preterite_second: "~etest zu", "~etet zu";
        preterite_third: "~ete zu", "~eten zu";
        subjunctive_i_first: "~e zu", "~en zu";
        subjunctive_i_second: "~est zu", "~et zu";
        subjunctive_i_third: "~e zu", referred;
        subjunctive_i_i_first: "~ete zu", "~eten zu";
        subjunctive_i_i_second: "~etest zu", "~etet zu";
        subjunctive_i_i_third: "~ete zu", "~eten zu";
        subordinate_present_first: "zu~e", "zu~en";
        subordinate_present_second: "zu~est", "zu~et";
        subordinate_present_third: "zu~et", "zu~en";
        subordinate_preterite_first: "zu~ete", "zu~eten";
        subordinate_preterite_second: "zu~etest", "zu~etet";
        subordinate_preterite_third: "zu~ete", "zu~eten";
        subordinate_subjunctive_i_first: "zu~e", "zu~en";
        subordinate_subjunctive_i_second: "zu~est", "zu~et";
        subordinate_subjunctive_i_third: "zu~e", referred;
        subordinate_subjunctive_i_i_first: "zu~ete", "zu~eten";
        subordinate_subjunctive_i_i_second: "zu~etest", "zu~etet";
        subordinate_subjunctive_i_i_third: "zu~ete", "zu~eten";
        imperative: "~ zu|~e zu", "~et zu";
        infinitive: "zu~en";
        zu_infinitive: "zuzu~en";
        participle: "zu~end", "zuge~et";
    ),
    // Einwerfen: 18 words (vorwerfen, auswerfen, wegwerfen, hinauswerfen, umwerfen, zuwerfen).
    conjugation!("einwerfen", 18, "~werfen";
        present_first: "werfe ~", "werfen ~";
        present_second: "wirfst ~", "werft ~";
        present_third: "wirft ~", "werfen ~";
        preterite_first: "warf ~", "warfen ~";
        preterite_second: "warfst ~", "warft ~";
        preterite_third: "warf ~", "warfen ~";
        subjunctive_i_first: "werfe ~", "werfen ~";
        subjunctive_i_second: "werfest ~", "werfet ~";
        subjunctive_i_third: "werfe ~", referred;
        subjunctive_i_i_first: "würfe ~", "würfen ~";
        subjunctive_i_i_second: "würfest ~", "würfet ~";
        subjunctive_i_i_third: "würfe ~", "würfen ~";
        subordinate_present_first: "~werfe", "~werfen";
        subordinate_present_second: "~wirfst", "~werft";
        subordinate_present_third: "~wirft", "~werfen";
        subordinate_preterite_first: "~warf", "~warfen";
        subordinate_preterite_second: "~warfst", "~warft";
        subordinate_preterite_third: "~warf", "~warfen";
        subordinate_subjunctive_i_first: "~werfe", "~werfen";
        subordinate_subjunctive_i_second: "~werfest", "~werfet";
        subordinate_subjunctive_i_third: "~werfe", referred;
        subordinate_subjunctive_i_i_first: "~würfe", "~würfen";
        subordinate_subjunctive_i_i_second: "~würfest", "~würfet";
        subordinate_subjunctive_i_i_third: "~würfe", "~würfen";
        imperative: "wirf ~", "werft ~";
        infinitive: "~werfen";
        zu_infinitive: "~zuwerfen";
        participle: "~werfend", "~geworfen";
    ),
    // Verlieren: 17 words (entfliehen, verlieren, verschieben, gefrieren, überfliegen, hinausschieben).
    conjugation!("verlieren", 17, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~st", "~ie~t";
        present_third: "~ie~t", "~ie~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", referred;
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
        subordinate_subjunctive_i_third: "~ie~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ie~|~ie~e", "~ie~t";
        infinitive: "~ie~en";
        zu_infinitive: "abzu~e~n";
        participle: "~ie~end", "~o~en";
    ),
    // Verbeißen: 17 words (bescheißen, verschleißen, rausschmeißen, entreißen, verbeißen, zerreißen).
    conjugation!("verbeißen", 17, "~e~ßen";
        present_first: "~e~ße", "~e~ßen";
        present_second: "~e~ßt", "~e~ßt";
        present_third: "~e~ßt", "~e~ßen";
        preterite_first: "~~ss", "~~ssen";
        preterite_second: "~~ssest|~~sst", "~~sst";
        preterite_third: "~~ss", "~~ssen";
        subjunctive_i_first: "~e~ße", "~e~ßen";
        subjunctive_i_second: "~e~ßest", "~e~ßet";
        subjunctive_i_third: "~e~ße", referred;
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
        subordinate_subjunctive_i_third: "~e~ße", referred;
        subordinate_subjunctive_i_i_first: "~~sse", "~~ssen";
        subordinate_subjunctive_i_i_second: "~~ssest|~~sst", "~~sset|~~sst";
        subordinate_subjunctive_i_i_third: "~~sse", "~~ssen";
        imperative: "~e~ß|~e~ße", "~e~ßt";
        infinitive: "~e~ßen";
        zu_infinitive: "abzu~e~n";
        participle: "~e~ßend", "~~ssen";
    ),
    // Fressen: 17 words (fressen, messen, vergessen, abmessen, vermessen, auffressen).
    conjugation!("fressen", 17, "~essen";
        present_first: "~esse", "~essen";
        present_second: "~isst", "~esst";
        present_third: "~isst", "~essen";
        preterite_first: "~aß", "~aßen";
        preterite_second: "~aßest", "~aßt";
        preterite_third: "~aß", "~aßen";
        subjunctive_i_first: "~esse", "~essen";
        subjunctive_i_second: "~essest", "~esset";
        subjunctive_i_third: "~esse", referred;
        subjunctive_i_i_first: "~äße", "~äßen";
        subjunctive_i_i_second: "~äßest", "~äßet";
        subjunctive_i_i_third: "~äße", "~äßen";
        subordinate_present_first: "~esse", "~essen";
        subordinate_present_second: "~isst", "~esst";
        subordinate_present_third: "~isst", "~essen";
        subordinate_preterite_first: "~aß", "~aßen";
        subordinate_preterite_second: "~aßest", "~aßt";
        subordinate_preterite_third: "~aß", "~aßen";
        subordinate_subjunctive_i_first: "~esse", "~essen";
        subordinate_subjunctive_i_second: "~essest", "~esset";
        subordinate_subjunctive_i_third: "~esse", referred;
        subordinate_subjunctive_i_i_first: "~äße", "~äßen";
        subordinate_subjunctive_i_i_second: "~äßest", "~äßet";
        subordinate_subjunctive_i_i_third: "~äße", "~äßen";
        imperative: "~iss", "~esst";
        infinitive: "~essen";
        zu_infinitive: "~zuessen";
        participle: "~essend", "~essen";
    ),
    // Festmachen: 17 words (festlegen, feststellen, festschreiben, festquatschen, festfragen, festmachen).
    conjugation!("festmachen", 17, "fest~en";
        present_first: "~e fest", "~en fest";
        present_second: "~st fest", "~t fest";
        present_third: "~t fest", "~en fest";
        preterite_first: "~te fest", "~ten fest";
        preterite_second: "~test fest", "~tet fest";
        preterite_third: "~te fest", "~ten fest";
        subjunctive_i_first: "~e fest", "~en fest";
        subjunctive_i_second: "~est fest", "~et fest";
        subjunctive_i_third: "~e fest", referred;
        subjunctive_i_i_first: "~te fest", "~ten fest";
        subjunctive_i_i_second: "~test fest", "~tet fest";
        subjunctive_i_i_third: "~te fest", "~ten fest";
        subordinate_present_first: "fest~e", "fest~en";
        subordinate_present_second: "fest~st", "fest~t";
        subordinate_present_third: "fest~t", "fest~en";
        subordinate_preterite_first: "fest~te", "fest~ten";
        subordinate_preterite_second: "fest~test", "fest~tet";
        subordinate_preterite_third: "fest~te", "fest~ten";
        subordinate_subjunctive_i_first: "fest~e", "fest~en";
        subordinate_subjunctive_i_second: "fest~est", "fest~et";
        subordinate_subjunctive_i_third: "fest~e", referred;
        subordinate_subjunctive_i_i_first: "fest~te", "fest~ten";
        subordinate_subjunctive_i_i_second: "fest~test", "fest~tet";
        subordinate_subjunctive_i_i_third: "fest~te", "fest~ten";
        imperative: "~ fest", "~t fest";
        infinitive: "fest~en";
        zu_infinitive: "festzu~en";
        participle: "fest~end", "festge~t";
    ),
    // Darstellen: 17 words (darstellen, zufriedenstellen, zusammenstellen, wiederherstellen, sicherstellen, bereitstellen).
    conjugation!("darstellen", 17, "~stellen";
        present_first: "stelle ~", "stellen ~";
        present_second: "stellst ~", "stellt ~";
        present_third: "stellt ~", "stellen ~";
        preterite_first: "stellte ~", "stellten ~";
        preterite_second: "stelltest ~", "stelltet ~";
        preterite_third: "stellte ~", "stellten ~";
        subjunctive_i_first: "stelle ~", "stellen ~";
        subjunctive_i_second: "stellest ~", "stellet ~";
        subjunctive_i_third: "stelle ~", referred;
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
        subordinate_subjunctive_i_third: "~stelle", referred;
        subordinate_subjunctive_i_i_first: "~stellte", "~stellten";
        subordinate_subjunctive_i_i_second: "~stelltest", "~stelltet";
        subordinate_subjunctive_i_i_third: "~stellte", "~stellten";
        imperative: "stell ~", "stellt ~";
        infinitive: "~stellen";
        zu_infinitive: "~zustellen";
        participle: "~stellend", "~gestellt";
    ),
    // Hochfliegen: 17 words (hochfliegen, hochbeamen, hochspielen, hochkochen, hochstufen, hochheben).
    conjugation!("hochfliegen", 17, "hoch~en";
        present_first: "~e hoch", "~en hoch";
        present_second: "~st hoch", "~t hoch";
        present_third: "~t hoch", "~en hoch";
        preterite_first: "~te hoch", "~ten hoch";
        preterite_second: "~test hoch", "~tet hoch";
        preterite_third: "~te hoch", "~ten hoch";
        subjunctive_i_first: "~e hoch", "~en hoch";
        subjunctive_i_second: "~est hoch", "~et hoch";
        subjunctive_i_third: "~e hoch", referred;
        subjunctive_i_i_first: "~te hoch", "~ten hoch";
        subjunctive_i_i_second: "~test hoch", "~tet hoch";
        subjunctive_i_i_third: "~te hoch", "~ten hoch";
        subordinate_present_first: "hoch~e", "hoch~en";
        subordinate_present_second: "hoch~st", "hoch~t";
        subordinate_present_third: "hoch~t", "hoch~en";
        subordinate_preterite_first: "hoch~te", "hoch~ten";
        subordinate_preterite_second: "hoch~test", "hoch~tet";
        subordinate_preterite_third: "hoch~te", "hoch~ten";
        subordinate_subjunctive_i_first: "hoch~e", "hoch~en";
        subordinate_subjunctive_i_second: "hoch~est", "hoch~et";
        subordinate_subjunctive_i_third: "hoch~e", referred;
        subordinate_subjunctive_i_i_first: "hoch~te", "hoch~ten";
        subordinate_subjunctive_i_i_second: "hoch~test", "hoch~tet";
        subordinate_subjunctive_i_i_third: "hoch~te", "hoch~ten";
        imperative: "~ hoch|~e hoch", "~t hoch";
        infinitive: "hoch~en";
        zu_infinitive: "hochzu~en";
        participle: "hoch~end", "hochge~t";
    ),
    // Gewinnen: 16 words (beginnen, gewinnen, wiedergewinnen, beklimmen, erklimmen, verschwimmen).
    conjugation!("gewinnen", 16, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~st", "~i~t";
        present_third: "~i~t", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~i~e", "~i~en";
        subordinate_present_second: "~i~st", "~i~t";
        subordinate_present_third: "~i~t", "~i~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_second: "~i~est", "~i~et";
        subordinate_subjunctive_i_third: "~i~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~|~i~e", "~i~t";
        infinitive: "~i~en";
        zu_infinitive: "abzu~e~n";
        participle: "~i~end", "~o~en";
    ),
    // Bringen: 16 words (bringen, verbringen, darbringen, herausbringen, erbringen, beybringen).
    conjugation!("bringen", 16, "~ingen";
        present_first: "~inge", "~ingen";
        present_second: "~ingst", "~ingt";
        present_third: "~ingt", "~ingen";
        preterite_first: "~achte", "~achten";
        preterite_second: "~achtest", "~achtet";
        preterite_third: "~achte", "~achten";
        subjunctive_i_first: "~inge", "~ingen";
        subjunctive_i_second: "~ingest", "~inget";
        subjunctive_i_third: "~inge", referred;
        subjunctive_i_i_first: "~ächte", "~ächten";
        subjunctive_i_i_second: "~ächtest", "~ächtet";
        subjunctive_i_i_third: "~ächte", "~ächten";
        subordinate_present_first: "~inge", "~ingen";
        subordinate_present_second: "~ingst", "~ingt";
        subordinate_present_third: "~ingt", "~ingen";
        subordinate_preterite_first: "~achte", "~achten";
        subordinate_preterite_second: "~achtest", "~achtet";
        subordinate_preterite_third: "~achte", "~achten";
        subordinate_subjunctive_i_first: "~inge", "~ingen";
        subordinate_subjunctive_i_second: "~ingest", "~inget";
        subordinate_subjunctive_i_third: "~inge", referred;
        subordinate_subjunctive_i_i_first: "~ächte", "~ächten";
        subordinate_subjunctive_i_i_second: "~ächtest", "~ächtet";
        subordinate_subjunctive_i_i_third: "~ächte", "~ächten";
        imperative: "~ing", "~ingt";
        infinitive: "~ingen";
        zu_infinitive: "abzu~en";
        participle: "~ingend", "~acht";
    ),
    // Schmelzen: 16 words (schmelzen, dreschen, melken, schwellen, quellen, verschmelzen).
    conjugation!("schmelzen", 16, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est", "~ö~et";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~st", "~o~t";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est", "~ö~et";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~i~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~o~en";
    ),
    // Hinhauen: 16 words (hinzufügen, hinlegen, hinkriegen, hinstrecken, hinknieen, hinstellen).
    conjugation!("hinhauen", 16, "hin~en";
        present_first: "~e hin", "~en hin";
        present_second: "~st hin", "~t hin";
        present_third: "~t hin", "~en hin";
        preterite_first: "~te hin", "~ten hin";
        preterite_second: "~test hin", "~tet hin";
        preterite_third: "~te hin", "~ten hin";
        subjunctive_i_first: "~e hin", "~en hin";
        subjunctive_i_second: "~est hin", "~et hin";
        subjunctive_i_third: "~e hin", referred;
        subjunctive_i_i_first: "~te hin", "~ten hin";
        subjunctive_i_i_second: "~test hin", "~tet hin";
        subjunctive_i_i_third: "~te hin", "~ten hin";
        subordinate_present_first: "hin~e", "hin~en";
        subordinate_present_second: "hin~st", "hin~t";
        subordinate_present_third: "hin~t", "hin~en";
        subordinate_preterite_first: "hin~te", "hin~ten";
        subordinate_preterite_second: "hin~test", "hin~tet";
        subordinate_preterite_third: "hin~te", "hin~ten";
        subordinate_subjunctive_i_first: "hin~e", "hin~en";
        subordinate_subjunctive_i_second: "hin~est", "hin~et";
        subordinate_subjunctive_i_third: "hin~e", referred;
        subordinate_subjunctive_i_i_first: "hin~te", "hin~ten";
        subordinate_subjunctive_i_i_second: "hin~test", "hin~tet";
        subordinate_subjunctive_i_i_third: "hin~te", "hin~ten";
        imperative: "~ hin|~e hin", "~t hin";
        infinitive: "hin~en";
        zu_infinitive: "hinzu~en";
        participle: "hin~end", "hinge~t";
    ),
    // Durchnudeln: 16 words (durchrütteln, durchblättern, durchschimmern, durchschütteln, durchsickern, durchkoppeln).
    conjugation!("durchnudeln", 16, "durch~e~n";
        present_first: "~~e durch|~e~e durch|~e~ durch", "~e~n durch";
        present_second: "~e~st durch", "~e~t durch";
        present_third: "~e~t durch", "~e~n durch";
        preterite_first: "~e~te durch", "~e~ten durch";
        preterite_second: "~e~test durch", "~e~tet durch";
        preterite_third: "~e~te durch", "~e~ten durch";
        subjunctive_i_first: "~e~e durch|~~e durch", "~e~n durch";
        subjunctive_i_second: "~e~est durch|~~est durch", "~e~et durch|~~et durch";
        subjunctive_i_third: "~e~e durch|~~e durch", referred;
        subjunctive_i_i_first: "~e~te durch", "~e~ten durch";
        subjunctive_i_i_second: "~e~test durch", "~e~tet durch";
        subjunctive_i_i_third: "~e~te durch", "~e~ten durch";
        subordinate_present_first: "durch~~e|durch~e~e|durch~e~", "durch~e~n";
        subordinate_present_second: "durch~e~st", "durch~e~t";
        subordinate_present_third: "durch~e~t", "durch~e~n";
        subordinate_preterite_first: "durch~e~te", "durch~e~ten";
        subordinate_preterite_second: "durch~e~test", "durch~e~tet";
        subordinate_preterite_third: "durch~e~te", "durch~e~ten";
        subordinate_subjunctive_i_first: "durch~e~e|durch~~e", "durch~e~n";
        subordinate_subjunctive_i_second: "durch~e~est|durch~~est", "durch~e~et|durch~~et";
        subordinate_subjunctive_i_third: "durch~e~e|durch~~e", referred;
        subordinate_subjunctive_i_i_first: "durch~e~te", "durch~e~ten";
        subordinate_subjunctive_i_i_second: "durch~e~test", "durch~e~tet";
        subordinate_subjunctive_i_i_third: "durch~e~te", "durch~e~ten";
        imperative: "~~e durch|~e~ durch|~e~e durch", "~e~t durch";
        infinitive: "durch~e~n";
        zu_infinitive: "durchzu~e~n";
        participle: "durch~e~nd", "durchge~e~t";
    ),
    // Aufreißen: 16 words (ausreißen, abreißen, hinreißen, aufreißen, losreißen, auseinanderreißen).
    conjugation!("aufreißen", 16, "~reißen";
        present_first: "reiße ~", "reißen ~";
        present_second: "reißt ~", "reißt ~";
        present_third: "reißt ~", "reißen ~";
        preterite_first: "riss ~", "rissen ~";
        preterite_second: "rissest ~", "risst ~";
        preterite_third: "riss ~", "rissen ~";
        subjunctive_i_first: "reiße ~", "reißen ~";
        subjunctive_i_second: "reißest ~", "reißet ~";
        subjunctive_i_third: "reiße ~", referred;
        subjunctive_i_i_first: "risse ~", "rissen ~";
        subjunctive_i_i_second: "rissest ~", "risset ~";
        subjunctive_i_i_third: "risse ~", "rissen ~";
        subordinate_present_first: "~reiße", "~reißen";
        subordinate_present_second: "~reißt", "~reißt";
        subordinate_present_third: "~reißt", "~reißen";
        subordinate_preterite_first: "~riss", "~rissen";
        subordinate_preterite_second: "~rissest", "~risst";
        subordinate_preterite_third: "~riss", "~rissen";
        subordinate_subjunctive_i_first: "~reiße", "~reißen";
        subordinate_subjunctive_i_second: "~reißest", "~reißet";
        subordinate_subjunctive_i_third: "~reiße", referred;
        subordinate_subjunctive_i_i_first: "~risse", "~rissen";
        subordinate_subjunctive_i_i_second: "~rissest", "~risset";
        subordinate_subjunctive_i_i_third: "~risse", "~rissen";
        imperative: "reiß ~", "reißt ~";
        infinitive: "~reißen";
        zu_infinitive: "~zureißen";
        participle: "~reißend", "~gerissen";
    ),
    // Herumnudeln: 15 words (herumquasseln, herumballern, herumwuseln, herumdoktern, herumwirbeln, herumkrabbeln).
    conjugation!("herumnudeln", 15, "herum~e~n";
        present_first: "~~e herum|~e~e herum|~e~ herum", "~e~n herum";
        present_second: "~e~st herum", "~e~t herum";
        present_third: "~e~t herum", "~e~n herum";
        preterite_first: "~e~te herum", "~e~ten herum";
        preterite_second: "~e~test herum", "~e~tet herum";
        preterite_third: "~e~te herum", "~e~ten herum";
        subjunctive_i_first: "~e~e herum|~~e herum", "~e~n herum";
        subjunctive_i_second: "~e~est herum|~~est herum", "~e~et herum|~~et herum";
        subjunctive_i_third: "~e~e herum|~~e herum", referred;
        subjunctive_i_i_first: "~e~te herum", "~e~ten herum";
        subjunctive_i_i_second: "~e~test herum", "~e~tet herum";
        subjunctive_i_i_third: "~e~te herum", "~e~ten herum";
        subordinate_present_first: "herum~~e|herum~e~e|herum~e~", "herum~e~n";
        subordinate_present_second: "herum~e~st", "herum~e~t";
        subordinate_present_third: "herum~e~t", "herum~e~n";
        subordinate_preterite_first: "herum~e~te", "herum~e~ten";
        subordinate_preterite_second: "herum~e~test", "herum~e~tet";
        subordinate_preterite_third: "herum~e~te", "herum~e~ten";
        subordinate_subjunctive_i_first: "herum~e~e|herum~~e", "herum~e~n";
        subordinate_subjunctive_i_second: "herum~e~est|herum~~est", "herum~e~et|herum~~et";
        subordinate_subjunctive_i_third: "herum~e~e|herum~~e", referred;
        subordinate_subjunctive_i_i_first: "herum~e~te", "herum~e~ten";
        subordinate_subjunctive_i_i_second: "herum~e~test", "herum~e~tet";
        subordinate_subjunctive_i_i_third: "herum~e~te", "herum~e~ten";
        imperative: "~~e herum|~e~ herum|~e~e herum", "~e~t herum";
        infinitive: "herum~e~n";
        zu_infinitive: "herumzu~e~n";
        participle: "herum~e~nd", "herumge~e~t";
    ),
    // Auslesen: 15 words (lesen, genesen, vorlesen, auslesen, einlesen, erlesen).
    conjugation!("auslesen", 15, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~ie~t", "~e~t";
        present_third: "~ie~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~est", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~ie~t", "~e~t";
        subordinate_present_third: "~ie~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~est", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~ie~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~e~en";
    ),
    // Niederhauen: 15 words (niederknieen, niederstarren, niederbrennen, niederschauen, niedermähen, niederschreiben).
    conjugation!("niederhauen", 15, "nieder~en";
        present_first: "~e nieder", "~en nieder";
        present_second: "~st nieder", "~t nieder";
        present_third: "~t nieder", "~en nieder";
        preterite_first: "~te nieder", "~ten nieder";
        preterite_second: "~test nieder", "~tet nieder";
        preterite_third: "~te nieder", "~ten nieder";
        subjunctive_i_first: "~e nieder", "~en nieder";
        subjunctive_i_second: "~est nieder", "~et nieder";
        subjunctive_i_third: "~e nieder", referred;
        subjunctive_i_i_first: "~te nieder", "~ten nieder";
        subjunctive_i_i_second: "~test nieder", "~tet nieder";
        subjunctive_i_i_third: "~te nieder", "~ten nieder";
        subordinate_present_first: "nieder~e", "nieder~en";
        subordinate_present_second: "nieder~st", "nieder~t";
        subordinate_present_third: "nieder~t", "nieder~en";
        subordinate_preterite_first: "nieder~te", "nieder~ten";
        subordinate_preterite_second: "nieder~test", "nieder~tet";
        subordinate_preterite_third: "nieder~te", "nieder~ten";
        subordinate_subjunctive_i_first: "nieder~e", "nieder~en";
        subordinate_subjunctive_i_second: "nieder~est", "nieder~et";
        subordinate_subjunctive_i_third: "nieder~e", referred;
        subordinate_subjunctive_i_i_first: "nieder~te", "nieder~ten";
        subordinate_subjunctive_i_i_second: "nieder~test", "nieder~tet";
        subordinate_subjunctive_i_i_third: "nieder~te", "nieder~ten";
        imperative: "~ nieder", "~t nieder";
        infinitive: "nieder~en";
        zu_infinitive: "niederzu~en";
        participle: "nieder~end", "niederge~t";
    ),
    // Benennen: 14 words (erkennen, verkennen, benennen, ernennen, verbrennen, versenden).
    conjugation!("benennen", 14, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~a~te", "~a~ten";
        preterite_second: "~a~test", "~a~tet";
        preterite_third: "~a~te", "~a~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~a~t";
    ),
    // Mithören: 14 words (mitwirken, mitfühlen, mitteilen, mittheilen, mitbringen, mitspielen).
    conjugation!("mithören", 14, "mit~en";
        present_first: "~e mit", "~en mit";
        present_second: "~st mit", "~t mit";
        present_third: "~t mit", "~en mit";
        preterite_first: "~te mit", "~ten mit";
        preterite_second: "~test mit", "~tet mit";
        preterite_third: "~te mit", "~ten mit";
        subjunctive_i_first: "~e mit", "~en mit";
        subjunctive_i_second: "~est mit", "~et mit";
        subjunctive_i_third: "~e mit", referred;
        subjunctive_i_i_first: "~te mit", "~ten mit";
        subjunctive_i_i_second: "~test mit", "~tet mit";
        subjunctive_i_i_third: "~te mit", "~ten mit";
        subordinate_present_first: "mit~e", "mit~en";
        subordinate_present_second: "mit~st", "mit~t";
        subordinate_present_third: "mit~t", "mit~en";
        subordinate_preterite_first: "mit~te", "mit~ten";
        subordinate_preterite_second: "mit~test", "mit~tet";
        subordinate_preterite_third: "mit~te", "mit~ten";
        subordinate_subjunctive_i_first: "mit~e", "mit~en";
        subordinate_subjunctive_i_second: "mit~est", "mit~et";
        subordinate_subjunctive_i_third: "mit~e", referred;
        subordinate_subjunctive_i_i_first: "mit~te", "mit~ten";
        subordinate_subjunctive_i_i_second: "mit~test", "mit~tet";
        subordinate_subjunctive_i_i_third: "mit~te", "mit~ten";
        imperative: "~ mit", "~t mit";
        infinitive: "mit~en";
        zu_infinitive: "mitzu~en";
        participle: "mit~end", "mitge~t";
    ),
    // Schwingen: 13 words (trinken, springen, singen, wringen, dringen, klingen).
    conjugation!("schwingen", 13, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~st", "~i~t";
        present_third: "~i~t", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", referred;
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
        subordinate_subjunctive_i_third: "~i~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~|~i~e", "~i~t";
        infinitive: "~i~en";
        zu_infinitive: "abzu~e~n";
        participle: "~i~end", "ge~u~en";
    ),
    // Heißen: 13 words (heißen, weisen, erweisen, verweisen, beweisen, überweisen).
    conjugation!("heißen", 13, "~e~~en";
        present_first: "~e~~e", "~e~~en";
        present_second: "~e~~t", "~e~~t";
        present_third: "~e~~t", "~e~~en";
        preterite_first: "~~e~", "~~e~en";
        preterite_second: "~~e~est|~~e~t", "~~e~t";
        preterite_third: "~~e~", "~~e~en";
        subjunctive_i_first: "~e~~e", "~e~~en";
        subjunctive_i_second: "~e~~est", "~e~~et";
        subjunctive_i_third: "~e~~e", referred;
        subjunctive_i_i_first: "~~e~e", "~~e~en";
        subjunctive_i_i_second: "~~e~est|~~e~t", "~~e~et|~~e~t";
        subjunctive_i_i_third: "~~e~e", "~~e~en";
        subordinate_present_first: "~e~~e", "~e~~en";
        subordinate_present_second: "~e~~t", "~e~~t";
        subordinate_present_third: "~e~~t", "~e~~en";
        subordinate_preterite_first: "~~e~", "~~e~en";
        subordinate_preterite_second: "~~e~est|~~e~t", "~~e~t";
        subordinate_preterite_third: "~~e~", "~~e~en";
        subordinate_subjunctive_i_first: "~e~~e", "~e~~en";
        subordinate_subjunctive_i_second: "~e~~est", "~e~~et";
        subordinate_subjunctive_i_third: "~e~~e", referred;
        subordinate_subjunctive_i_i_first: "~~e~e", "~~e~en";
        subordinate_subjunctive_i_i_second: "~~e~est|~~e~t", "~~e~et|~~e~t";
        subordinate_subjunctive_i_i_third: "~~e~e", "~~e~en";
        imperative: "~e~~|~e~~e", "~e~~t";
        infinitive: "~e~~en";
        zu_infinitive: "~~~zuziehen";
        participle: "~e~~end", "~~e~en";
    ),
    // Abstoßen: 13 words (zustoßen, anstoßen, abstoßen, vorstoßen, zusammenstoßen, umstoßen).
    conjugation!("abstoßen", 13, "~stoßen";
        present_first: "stoße ~", "stoßen ~";
        present_second: "stößt ~", "stoßt ~";
        present_third: "stößt ~", "stoßen ~";
        preterite_first: "stieß ~", "stießen ~";
        preterite_second: "stießest ~", "stießt ~";
        preterite_third: "stieß ~", "stießen ~";
        subjunctive_i_first: "stoße ~", "stoßen ~";
        subjunctive_i_second: "stoßest ~", "stoßet ~";
        subjunctive_i_third: "stoße ~", referred;
        subjunctive_i_i_first: "stieße ~", "stießen ~";
        subjunctive_i_i_second: "stießest ~", "stießet ~";
        subjunctive_i_i_third: "stieße ~", "stießen ~";
        subordinate_present_first: "~stoße", "~stoßen";
        subordinate_present_second: "~stößt", "~stoßt";
        subordinate_present_third: "~stößt", "~stoßen";
        subordinate_preterite_first: "~stieß", "~stießen";
        subordinate_preterite_second: "~stießest", "~stießt";
        subordinate_preterite_third: "~stieß", "~stießen";
        subordinate_subjunctive_i_first: "~stoße", "~stoßen";
        subordinate_subjunctive_i_second: "~stoßest", "~stoßet";
        subordinate_subjunctive_i_third: "~stoße", referred;
        subordinate_subjunctive_i_i_first: "~stieße", "~stießen";
        subordinate_subjunctive_i_i_second: "~stießest", "~stießet";
        subordinate_subjunctive_i_i_third: "~stieße", "~stießen";
        imperative: "stoß ~", "stoßt ~";
        infinitive: "~stoßen";
        zu_infinitive: "~zustoßen";
        participle: "~stoßend", "~gestoßen";
    ),
    // Mitreden: 13 words (schlechtreden, mitreden, daherreden, klugreden, vorbeireden, herausreden).
    conjugation!("mitreden", 13, "~reden";
        present_first: "rede ~", "reden ~";
        present_second: "redest ~", "redet ~";
        present_third: "redet ~", "reden ~";
        preterite_first: "redete ~", "redeten ~";
        preterite_second: "redetest ~", "redetet ~";
        preterite_third: "redete ~", "redeten ~";
        subjunctive_i_first: "rede ~", "reden ~";
        subjunctive_i_second: "redest ~", "redet ~";
        subjunctive_i_third: "rede ~", referred;
        subjunctive_i_i_first: "redete ~", "redeten ~";
        subjunctive_i_i_second: "redetest ~", "redetet ~";
        subjunctive_i_i_third: "redete ~", "redeten ~";
        subordinate_present_first: "~rede", "~reden";
        subordinate_present_second: "~redest", "~redet";
        subordinate_present_third: "~redet", "~reden";
        subordinate_preterite_first: "~redete", "~redeten";
        subordinate_preterite_second: "~redetest", "~redetet";
        subordinate_preterite_third: "~redete", "~redeten";
        subordinate_subjunctive_i_first: "~rede", "~reden";
        subordinate_subjunctive_i_second: "~redest", "~redet";
        subordinate_subjunctive_i_third: "~rede", referred;
        subordinate_subjunctive_i_i_first: "~redete", "~redeten";
        subordinate_subjunctive_i_i_second: "~redetest", "~redetet";
        subordinate_subjunctive_i_i_third: "~redete", "~redeten";
        imperative: "red ~", "redet ~";
        infinitive: "~reden";
        zu_infinitive: "~zureden";
        participle: "~redend", "~geredet";
    ),
    // Eilen: 12 words (eggen, einen, ebben, eilen, engen, ehelichen).
    conjugation!("eilen", 12, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "g~e~t";
    ),
    // Erheben: 12 words (erheben, scheren, heben, bewegen, beheben, hervorheben).
    conjugation!("erheben", 12, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~o~en";
    ),
    // Stehen: 12 words (verstehen, stehen, gestehen, bestehen, widerstehen, missverstehen).
    conjugation!("stehen", 12, "~ehen";
        present_first: "~ehe", "~ehen";
        present_second: "~ehst", "~eht";
        present_third: "~eht", "~ehen";
        preterite_first: "~and", "~anden";
        preterite_second: "~andest|~andst", "~andet";
        preterite_third: "~and", "~anden";
        subjunctive_i_first: "~ehe", "~ehen";
        subjunctive_i_second: "~ehest", "~ehet";
        subjunctive_i_third: "~ehe", referred;
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
        subordinate_subjunctive_i_third: "~ehe", referred;
        subordinate_subjunctive_i_i_first: "~ände|~ünde", "~änden|~ünden";
        subordinate_subjunctive_i_i_second: "~ändest|~ündest", "~ändet|~ündet";
        subordinate_subjunctive_i_i_third: "~ände|~ünde", "~änden|~ünden";
        imperative: "~eh|~ehe", "~eht";
        infinitive: "~ehen";
        zu_infinitive: "abzu~en";
        participle: "~ehend", "~anden";
    ),
    // Glänzen: 12 words (grüssen, grüßen, glosen, glänzen, grunzen, grenzen).
    conjugation!("glänzen", 12, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~t", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "~eg~t";
    ),
    // Aufrufen: 12 words (zurückrufen, anrufen, aufrufen, hervorrufen, abrufen, ausrufen).
    conjugation!("aufrufen", 12, "~rufen";
        present_first: "rufe ~", "rufen ~";
        present_second: "rufst ~", "ruft ~";
        present_third: "ruft ~", "rufen ~";
        preterite_first: "rief ~", "riefen ~";
        preterite_second: "riefst ~", "rieft ~";
        preterite_third: "rief ~", "riefen ~";
        subjunctive_i_first: "rufe ~", "rufen ~";
        subjunctive_i_second: "rufest ~", "rufet ~";
        subjunctive_i_third: "rufe ~", referred;
        subjunctive_i_i_first: "riefe ~", "riefen ~";
        subjunctive_i_i_second: "riefest ~", "riefet ~";
        subjunctive_i_i_third: "riefe ~", "riefen ~";
        subordinate_present_first: "~rufe", "~rufen";
        subordinate_present_second: "~rufst", "~ruft";
        subordinate_present_third: "~ruft", "~rufen";
        subordinate_preterite_first: "~rief", "~riefen";
        subordinate_preterite_second: "~riefst", "~rieft";
        subordinate_preterite_third: "~rief", "~riefen";
        subordinate_subjunctive_i_first: "~rufe", "~rufen";
        subordinate_subjunctive_i_second: "~rufest", "~rufet";
        subordinate_subjunctive_i_third: "~rufe", referred;
        subordinate_subjunctive_i_i_first: "~riefe", "~riefen";
        subordinate_subjunctive_i_i_second: "~riefest", "~riefet";
        subordinate_subjunctive_i_i_third: "~riefe", "~riefen";
        imperative: "ruf ~", "ruft ~";
        infinitive: "~rufen";
        zu_infinitive: "~zurufen";
        participle: "~rufend", "~gerufen";
    ),
    // Bestreiten: 12 words (bestreiten, überschreiten, durchschreiten, fortschreiten, erstreiten, beschreiten).
    conjugation!("bestreiten", 12, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~est", "~e~et";
        present_third: "~e~et", "~e~en";
        preterite_first: "~~t", "~~ten";
        preterite_second: "~~test|~~tst", "~~tet";
        preterite_third: "~~t", "~~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~e~|~e~e", "~e~et";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~~ten";
    ),
    // Anvertrauen: 12 words (anvertrauen, anprobieren, anvisieren, angehören, anlegieren, angelangen).
    conjugation!("anvertrauen", 12, "an~en";
        present_first: "~e an", "~en an";
        present_second: "~st an", "~t an";
        present_third: "~t an", "~en an";
        preterite_first: "~te an", "~ten an";
        preterite_second: "~test an", "~tet an";
        preterite_third: "~te an", "~ten an";
        subjunctive_i_first: "~e an", "~en an";
        subjunctive_i_second: "~est an", "~et an";
        subjunctive_i_third: "~e an", referred;
        subjunctive_i_i_first: "~te an", "~ten an";
        subjunctive_i_i_second: "~test an", "~tet an";
        subjunctive_i_i_third: "~te an", "~ten an";
        subordinate_present_first: "an~e", "an~en";
        subordinate_present_second: "an~st", "an~t";
        subordinate_present_third: "an~t", "an~en";
        subordinate_preterite_first: "an~te", "an~ten";
        subordinate_preterite_second: "an~test", "an~tet";
        subordinate_preterite_third: "an~te", "an~ten";
        subordinate_subjunctive_i_first: "an~e", "an~en";
        subordinate_subjunctive_i_second: "an~est", "an~et";
        subordinate_subjunctive_i_third: "an~e", referred;
        subordinate_subjunctive_i_i_first: "an~te", "an~ten";
        subordinate_subjunctive_i_i_second: "an~test", "an~tet";
        subordinate_subjunctive_i_i_third: "an~te", "an~ten";
        imperative: "~ an|~e an", "~t an";
        infinitive: "an~en";
        zu_infinitive: "anzu~en";
        participle: "an~end", "an~t";
    ),
    // Herausstellen: 12 words (herausschauen, herausstellen, herausschreien, herausschaffen, herausschreiben, herausstreichen).
    conjugation!("herausstellen", 12, "herau~s~en";
        present_first: "~~e heraus", "~~en heraus";
        present_second: "~~st heraus", "~~t heraus";
        present_third: "~~t heraus", "~~en heraus";
        preterite_first: "~~te heraus", "~~ten heraus";
        preterite_second: "~~test heraus", "~~tet heraus";
        preterite_third: "~~te heraus", "~~ten heraus";
        subjunctive_i_first: "~~e heraus", "~~en heraus";
        subjunctive_i_second: "~~est heraus", "~~et heraus";
        subjunctive_i_third: "~~e heraus", referred;
        subjunctive_i_i_first: "~~te heraus", "~~ten heraus";
        subjunctive_i_i_second: "~~test heraus", "~~tet heraus";
        subjunctive_i_i_third: "~~te heraus", "~~ten heraus";
        subordinate_present_first: "herau~s~e", "herau~s~en";
        subordinate_present_second: "herau~s~st", "herau~s~t";
        subordinate_present_third: "herau~s~t", "herau~s~en";
        subordinate_preterite_first: "herau~s~te", "herau~s~ten";
        subordinate_preterite_second: "herau~s~test", "herau~s~tet";
        subordinate_preterite_third: "herau~s~te", "herau~s~ten";
        subordinate_subjunctive_i_first: "herau~s~e", "herau~s~en";
        subordinate_subjunctive_i_second: "herau~s~est", "herau~s~et";
        subordinate_subjunctive_i_third: "herau~s~e", referred;
        subordinate_subjunctive_i_i_first: "herau~s~te", "herau~s~ten";
        subordinate_subjunctive_i_i_second: "herau~s~test", "herau~s~tet";
        subordinate_subjunctive_i_i_third: "herau~s~te", "herau~s~ten";
        imperative: "~~ heraus", "~~t heraus";
        infinitive: "herau~s~en";
        zu_infinitive: "herau~zus~en";
        participle: "herau~s~end", "herau~ges~t";
    ),
    // Beibringen: 12 words (beispringen, beibringen, beilegen, beiwohnen, beistimmen, beibleiben).
    conjugation!("beibringen", 12, "bei~en";
        present_first: "~e bei", "~en bei";
        present_second: "~st bei", "~t bei";
        present_third: "~t bei", "~en bei";
        preterite_first: "~te bei", "~ten bei";
        preterite_second: "~test bei", "~tet bei";
        preterite_third: "~te bei", "~ten bei";
        subjunctive_i_first: "~e bei", "~en bei";
        subjunctive_i_second: "~est bei", "~et bei";
        subjunctive_i_third: "~e bei", referred;
        subjunctive_i_i_first: "~te bei", "~ten bei";
        subjunctive_i_i_second: "~test bei", "~tet bei";
        subjunctive_i_i_third: "~te bei", "~ten bei";
        subordinate_present_first: "bei~e", "bei~en";
        subordinate_present_second: "bei~st", "bei~t";
        subordinate_present_third: "bei~t", "bei~en";
        subordinate_preterite_first: "bei~te", "bei~ten";
        subordinate_preterite_second: "bei~test", "bei~tet";
        subordinate_preterite_third: "bei~te", "bei~ten";
        subordinate_subjunctive_i_first: "bei~e", "bei~en";
        subordinate_subjunctive_i_second: "bei~est", "bei~et";
        subordinate_subjunctive_i_third: "bei~e", referred;
        subordinate_subjunctive_i_i_first: "bei~te", "bei~ten";
        subordinate_subjunctive_i_i_second: "bei~test", "bei~tet";
        subordinate_subjunctive_i_i_third: "bei~te", "bei~ten";
        imperative: "~ bei|~e bei", "~t bei";
        infinitive: "bei~en";
        zu_infinitive: "beizu~en";
        participle: "bei~end", "beige~t";
    ),
    // Zusammenscharren: 12 words (zusammenstürzen, zusammenscharren, zusammenstreben, zusammenschrumpfen, zusammenschweißen, zusammensparen).
    conjugation!("zusammenscharren", 12, "zu~ammens~en";
        present_first: "~~e zusammen", "~~en zusammen";
        present_second: "~~st zusammen", "~~t zusammen";
        present_third: "~~t zusammen", "~~en zusammen";
        preterite_first: "~~te zusammen", "~~ten zusammen";
        preterite_second: "~~test zusammen", "~~tet zusammen";
        preterite_third: "~~te zusammen", "~~ten zusammen";
        subjunctive_i_first: "~~e zusammen", "~~en zusammen";
        subjunctive_i_second: "~~est zusammen", "~~et zusammen";
        subjunctive_i_third: "~~e zusammen", referred;
        subjunctive_i_i_first: "~~te zusammen", "~~ten zusammen";
        subjunctive_i_i_second: "~~test zusammen", "~~tet zusammen";
        subjunctive_i_i_third: "~~te zusammen", "~~ten zusammen";
        subordinate_present_first: "zu~ammens~e", "zu~ammens~en";
        subordinate_present_second: "zu~ammens~st", "zu~ammens~t";
        subordinate_present_third: "zu~ammens~t", "zu~ammens~en";
        subordinate_preterite_first: "zu~ammens~te", "zu~ammens~ten";
        subordinate_preterite_second: "zu~ammens~test", "zu~ammens~tet";
        subordinate_preterite_third: "zu~ammens~te", "zu~ammens~ten";
        subordinate_subjunctive_i_first: "zu~ammens~e", "zu~ammens~en";
        subordinate_subjunctive_i_second: "zu~ammens~est", "zu~ammens~et";
        subordinate_subjunctive_i_third: "zu~ammens~e", referred;
        subordinate_subjunctive_i_i_first: "zu~ammens~te", "zu~ammens~ten";
        subordinate_subjunctive_i_i_second: "zu~ammens~test", "zu~ammens~tet";
        subordinate_subjunctive_i_i_third: "zu~ammens~te", "zu~ammens~ten";
        imperative: "~~ zusammen", "~~t zusammen";
        infinitive: "zu~ammens~en";
        zu_infinitive: "zu~ammenzus~en";
        participle: "zu~ammens~end", "zu~ammenges~t";
    ),
    // Zusetzen: 12 words (zufließen, zusetzen, zuspitzen, zuschießen, zubeißen, zubeissen).
    conjugation!("zusetzen", 12, "zu~en";
        present_first: "~e zu", "~en zu";
        present_second: "~t zu", "~t zu";
        present_third: "~t zu", "~en zu";
        preterite_first: "~te zu", "~ten zu";
        preterite_second: "~test zu", "~tet zu";
        preterite_third: "~te zu", "~ten zu";
        subjunctive_i_first: "~e zu", "~en zu";
        subjunctive_i_second: "~est zu", "~et zu";
        subjunctive_i_third: "~e zu", referred;
        subjunctive_i_i_first: "~te zu", "~ten zu";
        subjunctive_i_i_second: "~test zu", "~tet zu";
        subjunctive_i_i_third: "~te zu", "~ten zu";
        subordinate_present_first: "zu~e", "zu~en";
        subordinate_present_second: "zu~t", "zu~t";
        subordinate_present_third: "zu~t", "zu~en";
        subordinate_preterite_first: "zu~te", "zu~ten";
        subordinate_preterite_second: "zu~test", "zu~tet";
        subordinate_preterite_third: "zu~te", "zu~ten";
        subordinate_subjunctive_i_first: "zu~e", "zu~en";
        subordinate_subjunctive_i_second: "zu~est", "zu~et";
        subordinate_subjunctive_i_third: "zu~e", referred;
        subordinate_subjunctive_i_i_first: "zu~te", "zu~ten";
        subordinate_subjunctive_i_i_second: "zu~test", "zu~tet";
        subordinate_subjunctive_i_i_third: "zu~te", "zu~ten";
        imperative: "~ zu", "~t zu";
        infinitive: "zu~en";
        zu_infinitive: "zuzu~en";
        participle: "zu~end", "zuge~t";
    ),
    // Abstossen: 12 words (abstossen, aufstossen, ausstossen, anstossen, umstossen, aufeinanderstossen).
    conjugation!("abstossen", 12, "~stossen";
        present_first: "stosse ~", "stossen ~";
        present_second: "stösst ~", "stosst ~";
        present_third: "stösst ~", "stossen ~";
        preterite_first: "stiess ~", "stiessen ~";
        preterite_second: "stiessest ~", "stiesst ~";
        preterite_third: "stiess ~", "stiessen ~";
        subjunctive_i_first: "stosse ~", "stossen ~";
        subjunctive_i_second: "stossest ~", "stosset ~";
        subjunctive_i_third: "stosse ~", referred;
        subjunctive_i_i_first: "stiesse ~", "stiessen ~";
        subjunctive_i_i_second: "stiessest ~", "stiesset ~";
        subjunctive_i_i_third: "stiesse ~", "stiessen ~";
        subordinate_present_first: "~stosse", "~stossen";
        subordinate_present_second: "~stösst", "~stosst";
        subordinate_present_third: "~stösst", "~stossen";
        subordinate_preterite_first: "~stiess", "~stiessen";
        subordinate_preterite_second: "~stiessest", "~stiesst";
        subordinate_preterite_third: "~stiess", "~stiessen";
        subordinate_subjunctive_i_first: "~stosse", "~stossen";
        subordinate_subjunctive_i_second: "~stossest", "~stosset";
        subordinate_subjunctive_i_third: "~stosse", referred;
        subordinate_subjunctive_i_i_first: "~stiesse", "~stiessen";
        subordinate_subjunctive_i_i_second: "~stiessest", "~stiesset";
        subordinate_subjunctive_i_i_third: "~stiesse", "~stiessen";
        imperative: "stoss ~", "stosst ~";
        infinitive: "~stossen";
        zu_infinitive: "~zustossen";
        participle: "~stossend", "~gestossen";
    ),
    // Erwachsen: 11 words (erwachsen, wachsen, überwachsen, aufwachsen, durchwachsen, heranwachsen).
    conjugation!("erwachsen", 11, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~t", "~a~t";
        present_third: "~ä~t", "~a~en";
        preterite_first: "~u~", "~u~en";
        preterite_second: "~u~est", "~u~t";
        preterite_third: "~u~", "~u~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
        subjunctive_i_i_first: "~ü~e", "~ü~en";
        subjunctive_i_i_second: "~ü~est", "~ü~et";
        subjunctive_i_i_third: "~ü~e", "~ü~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~t", "~a~t";
        subordinate_present_third: "~ä~t", "~a~en";
        subordinate_preterite_first: "~u~", "~u~en";
        subordinate_preterite_second: "~u~est", "~u~t";
        subordinate_preterite_third: "~u~", "~u~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est", "~ü~et";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~a~", "~a~t";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "~a~en";
    ),
    // Schreiben: 11 words (schreiben, scheinen, bleiben, treiben, steigen, leihen).
    conjugation!("schreiben", 11, "~e~~en";
        present_first: "~e~~e", "~e~~en";
        present_second: "~e~~st", "~e~~t";
        present_third: "~e~~t", "~e~~en";
        preterite_first: "~~e~", "~~e~en";
        preterite_second: "~~e~st", "~~e~t";
        preterite_third: "~~e~", "~~e~en";
        subjunctive_i_first: "~e~~e", "~e~~en";
        subjunctive_i_second: "~e~~est", "~e~~et";
        subjunctive_i_third: "~e~~e", referred;
        subjunctive_i_i_first: "~~e~e", "~~e~en";
        subjunctive_i_i_second: "~~e~est|~~e~st", "~~e~et|~~e~t";
        subjunctive_i_i_third: "~~e~e", "~~e~en";
        subordinate_present_first: "~e~~e", "~e~~en";
        subordinate_present_second: "~e~~st", "~e~~t";
        subordinate_present_third: "~e~~t", "~e~~en";
        subordinate_preterite_first: "~~e~", "~~e~en";
        subordinate_preterite_second: "~~e~st", "~~e~t";
        subordinate_preterite_third: "~~e~", "~~e~en";
        subordinate_subjunctive_i_first: "~e~~e", "~e~~en";
        subordinate_subjunctive_i_second: "~e~~est", "~e~~et";
        subordinate_subjunctive_i_third: "~e~~e", referred;
        subordinate_subjunctive_i_i_first: "~~e~e", "~~e~en";
        subordinate_subjunctive_i_i_second: "~~e~est|~~e~st", "~~e~et|~~e~t";
        subordinate_subjunctive_i_i_third: "~~e~e", "~~e~en";
        imperative: "~e~~|~e~~e", "~e~~t";
        infinitive: "~e~~en";
        zu_infinitive: "~~~zuziehen";
        participle: "~e~~end", "ge~~e~en";
    ),
    // Schneiden: 11 words (leiden, schneiden, beschneiden, verschneiden, erleiden, überschneiden).
    conjugation!("schneiden", 11, "~e~den";
        present_first: "~e~de", "~e~den";
        present_second: "~e~dest", "~e~det";
        present_third: "~e~det", "~e~den";
        preterite_first: "~~tt", "~~tten";
        preterite_second: "~~ttest|~~ttst", "~~ttet";
        preterite_third: "~~tt", "~~tten";
        subjunctive_i_first: "~e~de", "~e~den";
        subjunctive_i_second: "~e~dest", "~e~det";
        subjunctive_i_third: "~e~de", referred;
        subjunctive_i_i_first: "~~tte", "~~tten";
        subjunctive_i_i_second: "~~ttest", "~~ttet";
        subjunctive_i_i_third: "~~tte", "~~tten";
        subordinate_present_first: "~e~de", "~e~den";
        subordinate_present_second: "~e~dest", "~e~det";
        subordinate_present_third: "~e~det", "~e~den";
        subordinate_preterite_first: "~~tt", "~~tten";
        subordinate_preterite_second: "~~ttest|~~ttst", "~~ttet";
        subordinate_preterite_third: "~~tt", "~~tten";
        subordinate_subjunctive_i_first: "~e~de", "~e~den";
        subordinate_subjunctive_i_second: "~e~dest", "~e~det";
        subordinate_subjunctive_i_third: "~e~de", referred;
        subordinate_subjunctive_i_i_first: "~~tte", "~~tten";
        subordinate_subjunctive_i_i_second: "~~ttest", "~~ttet";
        subordinate_subjunctive_i_i_third: "~~tte", "~~tten";
        imperative: "~e~d|~e~de", "~e~det";
        infinitive: "~e~den";
        zu_infinitive: "abzu~e~n";
        participle: "~e~dend", "~~tten";
    ),
    // Zujubeln: 11 words (zuwandern, zusichern, zuliefern, zuflüstern, zujubeln, zuzwinkern).
    conjugation!("zujubeln", 11, "zu~e~n";
        present_first: "~~e zu|~e~e zu|~e~ zu", "~e~n zu";
        present_second: "~e~st zu", "~e~t zu";
        present_third: "~e~t zu", "~e~n zu";
        preterite_first: "~e~te zu", "~e~ten zu";
        preterite_second: "~e~test zu", "~e~tet zu";
        preterite_third: "~e~te zu", "~e~ten zu";
        subjunctive_i_first: "~e~e zu|~~e zu", "~e~n zu";
        subjunctive_i_second: "~e~est zu|~~est zu", "~e~et zu|~~et zu";
        subjunctive_i_third: "~e~e zu|~~e zu", referred;
        subjunctive_i_i_first: "~e~te zu", "~e~ten zu";
        subjunctive_i_i_second: "~e~test zu", "~e~tet zu";
        subjunctive_i_i_third: "~e~te zu", "~e~ten zu";
        subordinate_present_first: "zu~~e|zu~e~e|zu~e~", "zu~e~n";
        subordinate_present_second: "zu~e~st", "zu~e~t";
        subordinate_present_third: "zu~e~t", "zu~e~n";
        subordinate_preterite_first: "zu~e~te", "zu~e~ten";
        subordinate_preterite_second: "zu~e~test", "zu~e~tet";
        subordinate_preterite_third: "zu~e~te", "zu~e~ten";
        subordinate_subjunctive_i_first: "zu~e~e|zu~~e", "zu~e~n";
        subordinate_subjunctive_i_second: "zu~e~est|zu~~est", "zu~e~et|zu~~et";
        subordinate_subjunctive_i_third: "zu~e~e|zu~~e", referred;
        subordinate_subjunctive_i_i_first: "zu~e~te", "zu~e~ten";
        subordinate_subjunctive_i_i_second: "zu~e~test", "zu~e~tet";
        subordinate_subjunctive_i_i_third: "zu~e~te", "zu~e~ten";
        imperative: "~~e zu|~e~ zu|~e~e zu", "~e~t zu";
        infinitive: "zu~e~n";
        zu_infinitive: "zuzu~e~n";
        participle: "zu~e~nd", "zuge~e~t";
    ),
    // Hermachen: 11 words (herstellen, herrühren, herhören, herbringen, herholen, hersagen).
    conjugation!("hermachen", 11, "her~en";
        present_first: "~e her", "~en her";
        present_second: "~st her", "~t her";
        present_third: "~t her", "~en her";
        preterite_first: "~te her", "~ten her";
        preterite_second: "~test her", "~tet her";
        preterite_third: "~te her", "~ten her";
        subjunctive_i_first: "~e her", "~en her";
        subjunctive_i_second: "~est her", "~et her";
        subjunctive_i_third: "~e her", referred;
        subjunctive_i_i_first: "~te her", "~ten her";
        subjunctive_i_i_second: "~test her", "~tet her";
        subjunctive_i_i_third: "~te her", "~ten her";
        subordinate_present_first: "her~e", "her~en";
        subordinate_present_second: "her~st", "her~t";
        subordinate_present_third: "her~t", "her~en";
        subordinate_preterite_first: "her~te", "her~ten";
        subordinate_preterite_second: "her~test", "her~tet";
        subordinate_preterite_third: "her~te", "her~ten";
        subordinate_subjunctive_i_first: "her~e", "her~en";
        subordinate_subjunctive_i_second: "her~est", "her~et";
        subordinate_subjunctive_i_third: "her~e", referred;
        subordinate_subjunctive_i_i_first: "her~te", "her~ten";
        subordinate_subjunctive_i_i_second: "her~test", "her~tet";
        subordinate_subjunctive_i_i_third: "her~te", "her~ten";
        imperative: "~ her", "~t her";
        infinitive: "her~en";
        zu_infinitive: "herzu~en";
        participle: "her~end", "herge~t";
    ),
    // Runterkippen: 11 words (runterkippen, runterdrücken, runterwürgen, runterholen, runtermachen, runterschlucken).
    conjugation!("runterkippen", 11, "runter~en";
        present_first: "~e runter", "~en runter";
        present_second: "~st runter", "~t runter";
        present_third: "~t runter", "~en runter";
        preterite_first: "~te runter", "~ten runter";
        preterite_second: "~test runter", "~tet runter";
        preterite_third: "~te runter", "~ten runter";
        subjunctive_i_first: "~e runter", "~en runter";
        subjunctive_i_second: "~est runter", "~et runter";
        subjunctive_i_third: "~e runter", referred;
        subjunctive_i_i_first: "~te runter", "~ten runter";
        subjunctive_i_i_second: "~test runter", "~tet runter";
        subjunctive_i_i_third: "~te runter", "~ten runter";
        subordinate_present_first: "runter~e", "runter~en";
        subordinate_present_second: "runter~st", "runter~t";
        subordinate_present_third: "runter~t", "runter~en";
        subordinate_preterite_first: "runter~te", "runter~ten";
        subordinate_preterite_second: "runter~test", "runter~tet";
        subordinate_preterite_third: "runter~te", "runter~ten";
        subordinate_subjunctive_i_first: "runter~e", "runter~en";
        subordinate_subjunctive_i_second: "runter~est", "runter~et";
        subordinate_subjunctive_i_third: "runter~e", referred;
        subordinate_subjunctive_i_i_first: "runter~te", "runter~ten";
        subordinate_subjunctive_i_i_second: "runter~test", "runter~tet";
        subordinate_subjunctive_i_i_third: "runter~te", "runter~ten";
        imperative: "~ runter|~e runter", "~t runter";
        infinitive: "runter~en";
        zu_infinitive: "runterzu~en";
        participle: "runter~end", "runterge~t";
    ),
    // Laden: 11 words (laden, herunterladen, beladen, entladen, hochladen, verladen).
    conjugation!("laden", 11, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~et";
        present_third: "~ä~t", "~a~en";
        preterite_first: "~u~", "~u~en";
        preterite_second: "~u~est", "~u~et";
        preterite_third: "~u~", "~u~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
        subjunctive_i_i_first: "~ü~e", "~ü~en";
        subjunctive_i_i_second: "~ü~est", "~ü~et";
        subjunctive_i_i_third: "~ü~e", "~ü~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~st", "~a~et";
        subordinate_present_third: "~ä~t", "~a~en";
        subordinate_preterite_first: "~u~", "~u~en";
        subordinate_preterite_second: "~u~est", "~u~et";
        subordinate_preterite_third: "~u~", "~u~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est", "~ü~et";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~a~", "~a~et";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "~a~en";
    ),
    // Ausradieren: 11 words (austarieren, ausmanövrieren, ausprobieren, ausverkaufen, ausradieren, ausdiskutieren).
    conjugation!("ausradieren", 11, "aus~en";
        present_first: "~e aus", "~en aus";
        present_second: "~st aus", "~t aus";
        present_third: "~t aus", "~en aus";
        preterite_first: "~te aus", "~ten aus";
        preterite_second: "~test aus", "~tet aus";
        preterite_third: "~te aus", "~ten aus";
        subjunctive_i_first: "~e aus", "~en aus";
        subjunctive_i_second: "~est aus", "~et aus";
        subjunctive_i_third: "~e aus", referred;
        subjunctive_i_i_first: "~te aus", "~ten aus";
        subjunctive_i_i_second: "~test aus", "~tet aus";
        subjunctive_i_i_third: "~te aus", "~ten aus";
        subordinate_present_first: "aus~e", "aus~en";
        subordinate_present_second: "aus~st", "aus~t";
        subordinate_present_third: "aus~t", "aus~en";
        subordinate_preterite_first: "aus~te", "aus~ten";
        subordinate_preterite_second: "aus~test", "aus~tet";
        subordinate_preterite_third: "aus~te", "aus~ten";
        subordinate_subjunctive_i_first: "aus~e", "aus~en";
        subordinate_subjunctive_i_second: "aus~est", "aus~et";
        subordinate_subjunctive_i_third: "aus~e", referred;
        subordinate_subjunctive_i_i_first: "aus~te", "aus~ten";
        subordinate_subjunctive_i_i_second: "aus~test", "aus~tet";
        subordinate_subjunctive_i_i_third: "aus~te", "aus~ten";
        imperative: "~ aus|~e aus", "~t aus";
        infinitive: "aus~en";
        zu_infinitive: "auszu~en";
        participle: "aus~end", "aus~t";
    ),
    // Raushauen: 11 words (rauskriegen, rausfliegen, rausbringen, rausholen, raushauen, rausbekommen).
    conjugation!("raushauen", 11, "raus~en";
        present_first: "~e raus", "~en raus";
        present_second: "~st raus", "~t raus";
        present_third: "~t raus", "~en raus";
        preterite_first: "~te raus", "~ten raus";
        preterite_second: "~test raus", "~tet raus";
        preterite_third: "~te raus", "~ten raus";
        subjunctive_i_first: "~e raus", "~en raus";
        subjunctive_i_second: "~est raus", "~et raus";
        subjunctive_i_third: "~e raus", referred;
        subjunctive_i_i_first: "~te raus", "~ten raus";
        subjunctive_i_i_second: "~test raus", "~tet raus";
        subjunctive_i_i_third: "~te raus", "~ten raus";
        subordinate_present_first: "raus~e", "raus~en";
        subordinate_present_second: "raus~st", "raus~t";
        subordinate_present_third: "raus~t", "raus~en";
        subordinate_preterite_first: "raus~te", "raus~ten";
        subordinate_preterite_second: "raus~test", "raus~tet";
        subordinate_preterite_third: "raus~te", "raus~ten";
        subordinate_subjunctive_i_first: "raus~e", "raus~en";
        subordinate_subjunctive_i_second: "raus~est", "raus~et";
        subordinate_subjunctive_i_third: "raus~e", referred;
        subordinate_subjunctive_i_i_first: "raus~te", "raus~ten";
        subordinate_subjunctive_i_i_second: "raus~test", "raus~tet";
        subordinate_subjunctive_i_i_third: "raus~te", "raus~ten";
        imperative: "~ raus|~e raus", "~t raus";
        infinitive: "raus~en";
        zu_infinitive: "rauszu~en";
        participle: "raus~end", "rausge~t";
    ),
    // Abbinden: 11 words (abbinden, abfinden, abspringen, absinken, abklingen, abgewinnen).
    conjugation!("abbinden", 11, "ab~i~en";
        present_first: "~i~e ab", "~i~en ab";
        present_second: "~i~st ab", "~i~t ab";
        present_third: "~i~t ab", "~i~en ab";
        preterite_first: "~a~ ab", "~a~en ab";
        preterite_second: "~a~st ab", "~a~t ab";
        preterite_third: "~a~ ab", "~a~en ab";
        subjunctive_i_first: "~i~e ab", "~i~en ab";
        subjunctive_i_second: "~i~est ab", "~i~et ab";
        subjunctive_i_third: "~i~e ab", referred;
        subjunctive_i_i_first: "~ä~e ab", "~ä~en ab";
        subjunctive_i_i_second: "~ä~est ab|~ä~st ab", "~ä~et ab|~ä~t ab";
        subjunctive_i_i_third: "~ä~e ab", "~ä~en ab";
        subordinate_present_first: "ab~i~e", "ab~i~en";
        subordinate_present_second: "ab~i~st", "ab~i~t";
        subordinate_present_third: "ab~i~t", "ab~i~en";
        subordinate_preterite_first: "ab~a~", "ab~a~en";
        subordinate_preterite_second: "ab~a~st", "ab~a~t";
        subordinate_preterite_third: "ab~a~", "ab~a~en";
        subordinate_subjunctive_i_first: "ab~i~e", "ab~i~en";
        subordinate_subjunctive_i_second: "ab~i~est", "ab~i~et";
        subordinate_subjunctive_i_third: "ab~i~e", referred;
        subordinate_subjunctive_i_i_first: "ab~ä~e", "ab~ä~en";
        subordinate_subjunctive_i_i_second: "ab~ä~est|ab~ä~st", "ab~ä~et|ab~ä~t";
        subordinate_subjunctive_i_i_third: "ab~ä~e", "ab~ä~en";
        imperative: "~i~ ab|~i~e ab", "~i~t ab";
        infinitive: "ab~i~en";
        zu_infinitive: "abzu~i~en";
        participle: "ab~i~end", "abge~u~en";
    ),
    // Unterbringen: 11 words (unterbringen, unterfinanzieren, unterschieben, untertauchen, unterkriegen, unterschlüpfen).
    conjugation!("unterbringen", 11, "unter~en";
        present_first: "~e unter", "~en unter";
        present_second: "~st unter", "~t unter";
        present_third: "~t unter", "~en unter";
        preterite_first: "~te unter", "~ten unter";
        preterite_second: "~test unter", "~tet unter";
        preterite_third: "~te unter", "~ten unter";
        subjunctive_i_first: "~e unter", "~en unter";
        subjunctive_i_second: "~est unter", "~et unter";
        subjunctive_i_third: "~e unter", referred;
        subjunctive_i_i_first: "~te unter", "~ten unter";
        subjunctive_i_i_second: "~test unter", "~tet unter";
        subjunctive_i_i_third: "~te unter", "~ten unter";
        subordinate_present_first: "unter~e", "unter~en";
        subordinate_present_second: "unter~st", "unter~t";
        subordinate_present_third: "unter~t", "unter~en";
        subordinate_preterite_first: "unter~te", "unter~ten";
        subordinate_preterite_second: "unter~test", "unter~tet";
        subordinate_preterite_third: "unter~te", "unter~ten";
        subordinate_subjunctive_i_first: "unter~e", "unter~en";
        subordinate_subjunctive_i_second: "unter~est", "unter~et";
        subordinate_subjunctive_i_third: "unter~e", referred;
        subordinate_subjunctive_i_i_first: "unter~te", "unter~ten";
        subordinate_subjunctive_i_i_second: "unter~test", "unter~tet";
        subordinate_subjunctive_i_i_third: "unter~te", "unter~ten";
        imperative: "~ unter|~e unter", "~t unter";
        infinitive: "unter~en";
        zu_infinitive: "unterzu~en";
        participle: "unter~end", "unterge~t";
    ),
    // Rumbringen: 11 words (rumlecken, rumsitzen, rumexperimentieren, rumzicken, rumspringen, rumhängen).
    conjugation!("rumbringen", 11, "rum~en";
        present_first: "~e rum", "~en rum";
        present_second: "~st rum", "~t rum";
        present_third: "~t rum", "~en rum";
        preterite_first: "~te rum", "~ten rum";
        preterite_second: "~test rum", "~tet rum";
        preterite_third: "~te rum", "~ten rum";
        subjunctive_i_first: "~e rum", "~en rum";
        subjunctive_i_second: "~est rum", "~et rum";
        subjunctive_i_third: "~e rum", referred;
        subjunctive_i_i_first: "~te rum", "~ten rum";
        subjunctive_i_i_second: "~test rum", "~tet rum";
        subjunctive_i_i_third: "~te rum", "~ten rum";
        subordinate_present_first: "rum~e", "rum~en";
        subordinate_present_second: "rum~st", "rum~t";
        subordinate_present_third: "rum~t", "rum~en";
        subordinate_preterite_first: "rum~te", "rum~ten";
        subordinate_preterite_second: "rum~test", "rum~tet";
        subordinate_preterite_third: "rum~te", "rum~ten";
        subordinate_subjunctive_i_first: "rum~e", "rum~en";
        subordinate_subjunctive_i_second: "rum~est", "rum~et";
        subordinate_subjunctive_i_third: "rum~e", referred;
        subordinate_subjunctive_i_i_first: "rum~te", "rum~ten";
        subordinate_subjunctive_i_i_second: "rum~test", "rum~tet";
        subordinate_subjunctive_i_i_third: "rum~te", "rum~ten";
        imperative: "~ rum|~e rum", "~t rum";
        infinitive: "rum~en";
        zu_infinitive: "rumzu~en";
        participle: "rum~end", "rumge~t";
    ),
    // Abfahren: 10 words (abfahren, abfangen, abtragen, abwaschen, abladen, absaufen).
    conjugation!("abfahren", 10, "ab~a~en";
        present_first: "~a~e ab", "~a~en ab";
        present_second: "~ä~st ab", "~a~t ab";
        present_third: "~ä~t ab", "~a~en ab";
        preterite_first: "~u~ ab", "~u~en ab";
        preterite_second: "~u~st ab", "~u~t ab";
        preterite_third: "~u~ ab", "~u~en ab";
        subjunctive_i_first: "~a~e ab", "~a~en ab";
        subjunctive_i_second: "~a~est ab", "~a~et ab";
        subjunctive_i_third: "~a~e ab", referred;
        subjunctive_i_i_first: "~ü~e ab", "~ü~en ab";
        subjunctive_i_i_second: "~ü~est ab", "~ü~et ab";
        subjunctive_i_i_third: "~ü~e ab", "~ü~en ab";
        subordinate_present_first: "ab~a~e", "ab~a~en";
        subordinate_present_second: "ab~ä~st", "ab~a~t";
        subordinate_present_third: "ab~ä~t", "ab~a~en";
        subordinate_preterite_first: "ab~u~", "ab~u~en";
        subordinate_preterite_second: "ab~u~st", "ab~u~t";
        subordinate_preterite_third: "ab~u~", "ab~u~en";
        subordinate_subjunctive_i_first: "ab~a~e", "ab~a~en";
        subordinate_subjunctive_i_second: "ab~a~est", "ab~a~et";
        subordinate_subjunctive_i_third: "ab~a~e", referred;
        subordinate_subjunctive_i_i_first: "ab~ü~e", "ab~ü~en";
        subordinate_subjunctive_i_i_second: "ab~ü~est", "ab~ü~et";
        subordinate_subjunctive_i_i_third: "ab~ü~e", "ab~ü~en";
        imperative: "~a~ ab|~a~e ab", "~a~t ab";
        infinitive: "ab~a~en";
        zu_infinitive: "abzu~a~en";
        participle: "ab~a~end", "abge~a~en";
    ),
    // Giften: 10 words (gilden, gründen, giften, glätten, ghosten, gnaden).
    conjugation!("giften", 10, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~est", "~~et";
        present_third: "~~et", "~~en";
        preterite_first: "~~ete", "~~eten";
        preterite_second: "~~etest", "~~etet";
        preterite_third: "~~ete", "~~eten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~ete", "~~eten";
        subordinate_subjunctive_i_i_second: "~~etest", "~~etet";
        subordinate_subjunctive_i_i_third: "~~ete", "~~eten";
        imperative: "~~|~~e", "~~et";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "~eg~et";
    ),
    // Abzapfen: 10 words (abzocken, abzählen, abzielen, abzäumen, abzäunen, abzapfen).
    conjugation!("abzapfen", 10, "ab~~en";
        present_first: "~~e ab", "~~en ab";
        present_second: "~~st ab", "~~t ab";
        present_third: "~~t ab", "~~en ab";
        preterite_first: "~~te ab", "~~ten ab";
        preterite_second: "~~test ab", "~~tet ab";
        preterite_third: "~~te ab", "~~ten ab";
        subjunctive_i_first: "~~e ab", "~~en ab";
        subjunctive_i_second: "~~est ab", "~~et ab";
        subjunctive_i_third: "~~e ab", referred;
        subjunctive_i_i_first: "~~te ab", "~~ten ab";
        subjunctive_i_i_second: "~~test ab", "~~tet ab";
        subjunctive_i_i_third: "~~te ab", "~~ten ab";
        subordinate_present_first: "ab~~e", "ab~~en";
        subordinate_present_second: "ab~~st", "ab~~t";
        subordinate_present_third: "ab~~t", "ab~~en";
        subordinate_preterite_first: "ab~~te", "ab~~ten";
        subordinate_preterite_second: "ab~~test", "ab~~tet";
        subordinate_preterite_third: "ab~~te", "ab~~ten";
        subordinate_subjunctive_i_first: "ab~~e", "ab~~en";
        subordinate_subjunctive_i_second: "ab~~est", "ab~~et";
        subordinate_subjunctive_i_third: "ab~~e", referred;
        subordinate_subjunctive_i_i_first: "ab~~te", "ab~~ten";
        subordinate_subjunctive_i_i_second: "ab~~test", "ab~~tet";
        subordinate_subjunctive_i_i_third: "ab~~te", "ab~~ten";
        imperative: "~~ ab|~~e ab", "~~t ab";
        infinitive: "ab~~en";
        zu_infinitive: "ab~uz~en";
        participle: "ab~~end", "abge~~t";
    ),
    // Reinhauen: 10 words (reinlegen, reinstecken, reinplatzen, reinmachen, reinhängen, reinhauen).
    conjugation!("reinhauen", 10, "rein~en";
        present_first: "~e rein", "~en rein";
        present_second: "~st rein", "~t rein";
        present_third: "~t rein", "~en rein";
        preterite_first: "~te rein", "~ten rein";
        preterite_second: "~test rein", "~tet rein";
        preterite_third: "~te rein", "~ten rein";
        subjunctive_i_first: "~e rein", "~en rein";
        subjunctive_i_second: "~est rein", "~et rein";
        subjunctive_i_third: "~e rein", referred;
        subjunctive_i_i_first: "~te rein", "~ten rein";
        subjunctive_i_i_second: "~test rein", "~tet rein";
        subjunctive_i_i_third: "~te rein", "~ten rein";
        subordinate_present_first: "rein~e", "rein~en";
        subordinate_present_second: "rein~st", "rein~t";
        subordinate_present_third: "rein~t", "rein~en";
        subordinate_preterite_first: "rein~te", "rein~ten";
        subordinate_preterite_second: "rein~test", "rein~tet";
        subordinate_preterite_third: "rein~te", "rein~ten";
        subordinate_subjunctive_i_first: "rein~e", "rein~en";
        subordinate_subjunctive_i_second: "rein~est", "rein~et";
        subordinate_subjunctive_i_third: "rein~e", referred;
        subordinate_subjunctive_i_i_first: "rein~te", "rein~ten";
        subordinate_subjunctive_i_i_second: "rein~test", "rein~tet";
        subordinate_subjunctive_i_i_third: "rein~te", "rein~ten";
        imperative: "~ rein|~e rein", "~t rein";
        infinitive: "rein~en";
        zu_infinitive: "reinzu~en";
        participle: "rein~end", "reinge~t";
    ),
    // Heimführen: 10 words (heimkehren, heimzahlen, heimsuchen, heimholen, heimbringen, heimführen).
    conjugation!("heimführen", 10, "heim~en";
        present_first: "~e heim", "~en heim";
        present_second: "~st heim", "~t heim";
        present_third: "~t heim", "~en heim";
        preterite_first: "~te heim", "~ten heim";
        preterite_second: "~test heim", "~tet heim";
        preterite_third: "~te heim", "~ten heim";
        subjunctive_i_first: "~e heim", "~en heim";
        subjunctive_i_second: "~est heim", "~et heim";
        subjunctive_i_third: "~e heim", referred;
        subjunctive_i_i_first: "~te heim", "~ten heim";
        subjunctive_i_i_second: "~test heim", "~tet heim";
        subjunctive_i_i_third: "~te heim", "~ten heim";
        subordinate_present_first: "heim~e", "heim~en";
        subordinate_present_second: "heim~st", "heim~t";
        subordinate_present_third: "heim~t", "heim~en";
        subordinate_preterite_first: "heim~te", "heim~ten";
        subordinate_preterite_second: "heim~test", "heim~tet";
        subordinate_preterite_third: "heim~te", "heim~ten";
        subordinate_subjunctive_i_first: "heim~e", "heim~en";
        subordinate_subjunctive_i_second: "heim~est", "heim~et";
        subordinate_subjunctive_i_third: "heim~e", referred;
        subordinate_subjunctive_i_i_first: "heim~te", "heim~ten";
        subordinate_subjunctive_i_i_second: "heim~test", "heim~tet";
        subordinate_subjunctive_i_i_third: "heim~te", "heim~ten";
        imperative: "~ heim|~e heim", "~t heim";
        infinitive: "heim~en";
        zu_infinitive: "heimzu~en";
        participle: "heim~end", "heimge~t";
    ),
    // Herreisen: 10 words (weiterreisen, heimreisen, fortreisen, davonreisen, herumreisen, mitreisen).
    conjugation!("herreisen", 10, "~reisen";
        present_first: "reise ~", "reisen ~";
        present_second: "reist ~", "reist ~";
        present_third: "reist ~", "reisen ~";
        preterite_first: "reiste ~", "reisten ~";
        preterite_second: "reistest ~", "reistet ~";
        preterite_third: "reiste ~", "reisten ~";
        subjunctive_i_first: "reise ~", "reisen ~";
        subjunctive_i_second: "reisest ~", "reiset ~";
        subjunctive_i_third: "reise ~", referred;
        subjunctive_i_i_first: "reiste ~", "reisten ~";
        subjunctive_i_i_second: "reistest ~", "reistet ~";
        subjunctive_i_i_third: "reiste ~", "reisten ~";
        subordinate_present_first: "~reise", "~reisen";
        subordinate_present_second: "~reist", "~reist";
        subordinate_present_third: "~reist", "~reisen";
        subordinate_preterite_first: "~reiste", "~reisten";
        subordinate_preterite_second: "~reistest", "~reistet";
        subordinate_preterite_third: "~reiste", "~reisten";
        subordinate_subjunctive_i_first: "~reise", "~reisen";
        subordinate_subjunctive_i_second: "~reisest", "~reiset";
        subordinate_subjunctive_i_third: "~reise", referred;
        subordinate_subjunctive_i_i_first: "~reiste", "~reisten";
        subordinate_subjunctive_i_i_second: "~reistest", "~reistet";
        subordinate_subjunctive_i_i_third: "~reiste", "~reisten";
        imperative: "reis ~", "reist ~";
        infinitive: "~reisen";
        zu_infinitive: "~zureisen";
        participle: "~reisend", "~gereist";
    ),
    // Verfallen: 9 words (befallen, fallen, gefallen, überfallen, zerfallen, missfallen).
    conjugation!("verfallen", 9, "~a~len";
        present_first: "~a~le", "~a~len";
        present_second: "~ä~lst", "~a~lt";
        present_third: "~ä~lt", "~a~len";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~st", "~ie~t";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~a~le", "~a~len";
        subjunctive_i_second: "~a~lest", "~a~let";
        subjunctive_i_third: "~a~le", referred;
        subjunctive_i_i_first: "~ie~e", "~ie~en";
        subjunctive_i_i_second: "~ie~est|~ie~st", "~ie~et|~ie~t";
        subjunctive_i_i_third: "~ie~e", "~ie~en";
        subordinate_present_first: "~a~le", "~a~len";
        subordinate_present_second: "~ä~lst", "~a~lt";
        subordinate_present_third: "~ä~lt", "~a~len";
        subordinate_preterite_first: "~ie~", "~ie~en";
        subordinate_preterite_second: "~ie~st", "~ie~t";
        subordinate_preterite_third: "~ie~", "~ie~en";
        subordinate_subjunctive_i_first: "~a~le", "~a~len";
        subordinate_subjunctive_i_second: "~a~lest", "~a~let";
        subordinate_subjunctive_i_third: "~a~le", referred;
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est|~ie~st", "~ie~et|~ie~t";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~a~l|~a~le", "~a~lt";
        infinitive: "~a~len";
        zu_infinitive: "abzu~e~n";
        participle: "~a~lend", "~a~len";
    ),
    // Fliegen: 9 words (biegen, wiegen, fliehen, fliegen, riechen, frieren).
    conjugation!("fliegen", 9, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~st", "~ie~t";
        present_third: "~ie~t", "~ie~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", referred;
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
        subordinate_subjunctive_i_third: "~ie~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ie~|~ie~e", "~ie~t";
        infinitive: "~ie~en";
        zu_infinitive: "abzu~e~n";
        participle: "~ie~end", "ge~o~en";
    ),
    // Tun: 9 words (tun, fensterln, thun, aufthun, anthun, äußerln).
    conjugation!("tun", 9, "~n";
        present_first: "~e", "~n";
        present_second: "~st", "~t";
        present_third: "~t", "~n";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~", "~t";
        infinitive: "~n";
        zu_infinitive: "sich zu ~en";
        participle: "~nd", "ge~t";
    ),
    // Lassen: 9 words (lassen, verlassen, belassen, entlassen, erlassen, unterlassen).
    conjugation!("lassen", 9, "~assen";
        present_first: "~asse", "~assen";
        present_second: "~ässt", "~asst";
        present_third: "~ässt", "~assen";
        preterite_first: "~ieß", "~ießen";
        preterite_second: "~ießest|~ießt", "~ießt";
        preterite_third: "~ieß", "~ießen";
        subjunctive_i_first: "~asse", "~assen";
        subjunctive_i_second: "~assest", "~asset";
        subjunctive_i_third: "~asse", referred;
        subjunctive_i_i_first: "~ieße", "~ießen";
        subjunctive_i_i_second: "~ießest|~ießt", "~ießet|~ießt";
        subjunctive_i_i_third: "~ieße", "~ießen";
        subordinate_present_first: "~asse", "~assen";
        subordinate_present_second: "~ässt", "~asst";
        subordinate_present_third: "~ässt", "~assen";
        subordinate_preterite_first: "~ieß", "~ießen";
        subordinate_preterite_second: "~ießest|~ießt", "~ießt";
        subordinate_preterite_third: "~ieß", "~ießen";
        subordinate_subjunctive_i_first: "~asse", "~assen";
        subordinate_subjunctive_i_second: "~assest", "~asset";
        subordinate_subjunctive_i_third: "~asse", referred;
        subordinate_subjunctive_i_i_first: "~ieße", "~ießen";
        subordinate_subjunctive_i_i_second: "~ießest|~ießt", "~ießet|~ießt";
        subordinate_subjunctive_i_i_third: "~ieße", "~ießen";
        imperative: "~ass|~asse", "~asst";
        infinitive: "~assen";
        zu_infinitive: "abzu~en";
        participle: "~assend", "~assen";
    ),
    // Aufpolieren: 9 words (auferlegen, aufbewahren, aufmarschieren, aufdatieren, aufbegehren, auferwecken).
    conjugation!("aufpolieren", 9, "auf~en";
        present_first: "~e auf", "~en auf";
        present_second: "~st auf", "~t auf";
        present_third: "~t auf", "~en auf";
        preterite_first: "~te auf", "~ten auf";
        preterite_second: "~test auf", "~tet auf";
        preterite_third: "~te auf", "~ten auf";
        subjunctive_i_first: "~e auf", "~en auf";
        subjunctive_i_second: "~est auf", "~et auf";
        subjunctive_i_third: "~e auf", referred;
        subjunctive_i_i_first: "~te auf", "~ten auf";
        subjunctive_i_i_second: "~test auf", "~tet auf";
        subjunctive_i_i_third: "~te auf", "~ten auf";
        subordinate_present_first: "auf~e", "auf~en";
        subordinate_present_second: "auf~st", "auf~t";
        subordinate_present_third: "auf~t", "auf~en";
        subordinate_preterite_first: "auf~te", "auf~ten";
        subordinate_preterite_second: "auf~test", "auf~tet";
        subordinate_preterite_third: "auf~te", "auf~ten";
        subordinate_subjunctive_i_first: "auf~e", "auf~en";
        subordinate_subjunctive_i_second: "auf~est", "auf~et";
        subordinate_subjunctive_i_third: "auf~e", referred;
        subordinate_subjunctive_i_i_first: "auf~te", "auf~ten";
        subordinate_subjunctive_i_i_second: "auf~test", "auf~tet";
        subordinate_subjunctive_i_i_third: "auf~te", "auf~ten";
        imperative: "~ auf|~e auf", "~t auf";
        infinitive: "auf~en";
        zu_infinitive: "aufzu~en";
        participle: "auf~end", "auf~t";
    ),
    // Gehen: 9 words (gehen, umgehen, begehen, vergehen, übergehen, entgehen).
    conjugation!("gehen", 9, "~ehen";
        present_first: "~ehe", "~ehen";
        present_second: "~ehst", "~eht";
        present_third: "~eht", "~ehen";
        preterite_first: "~ing", "~ingen";
        preterite_second: "~ingst", "~ingt";
        preterite_third: "~ing", "~ingen";
        subjunctive_i_first: "~ehe", "~ehen";
        subjunctive_i_second: "~ehest", "~ehet";
        subjunctive_i_third: "~ehe", referred;
        subjunctive_i_i_first: "~inge", "~ingen";
        subjunctive_i_i_second: "~ingest|~ingst", "~inget|~ingt";
        subjunctive_i_i_third: "~inge", "~ingen";
        subordinate_present_first: "~ehe", "~ehen";
        subordinate_present_second: "~ehst", "~eht";
        subordinate_present_third: "~eht", "~ehen";
        subordinate_preterite_first: "~ing", "~ingen";
        subordinate_preterite_second: "~ingst", "~ingt";
        subordinate_preterite_third: "~ing", "~ingen";
        subordinate_subjunctive_i_first: "~ehe", "~ehen";
        subordinate_subjunctive_i_second: "~ehest", "~ehet";
        subordinate_subjunctive_i_third: "~ehe", referred;
        subordinate_subjunctive_i_i_first: "~inge", "~ingen";
        subordinate_subjunctive_i_i_second: "~ingest|~ingst", "~inget|~ingt";
        subordinate_subjunctive_i_i_third: "~inge", "~ingen";
        imperative: "~eh|~ehe", "~eht";
        infinitive: "~ehen";
        zu_infinitive: "abzu~en";
        participle: "~ehend", "~angen";
    ),
    // Gerben: 9 words (gerben, wechselwirken, gellen, gehren, gecken, lobsagen).
    conjugation!("gerben", 9, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "~ge~t";
    ),
    // Sehen: 9 words (sehen, stehlen, geschehen, versehen, übersehen, davonstehlen).
    conjugation!("sehen", 9, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~ie~st", "~e~t";
        present_third: "~ie~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~ie~st", "~e~t";
        subordinate_present_third: "~ie~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~ie~|~ie~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~e~en";
    ),
    // Blasen: 9 words (blasen, schlafen, entschlafen, beschlafen, verschlafen, durchschlafen).
    conjugation!("blasen", 9, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~t";
        present_third: "~ä~t", "~a~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~st", "~ie~t";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
        subjunctive_i_i_first: "~ie~e", "~ie~en";
        subjunctive_i_i_second: "~ie~est", "~ie~et|~ie~t";
        subjunctive_i_i_third: "~ie~e", "~ie~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~st", "~a~t";
        subordinate_present_third: "~ä~t", "~a~en";
        subordinate_preterite_first: "~ie~", "~ie~en";
        subordinate_preterite_second: "~ie~st", "~ie~t";
        subordinate_preterite_third: "~ie~", "~ie~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est", "~ie~et|~ie~t";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~a~|~a~e", "~a~t";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "~a~en";
    ),
    // Abwerfen: 9 words (abbrechen, abwerben, absterben, abwerfen, ablesen, abstechen).
    conjugation!("abwerfen", 9, "ab~e~en";
        present_first: "~e~e ab", "~e~en ab";
        present_second: "~i~st ab", "~e~t ab";
        present_third: "~i~t ab", "~e~en ab";
        preterite_first: "~a~ ab", "~a~en ab";
        preterite_second: "~a~st ab", "~a~t ab";
        preterite_third: "~a~ ab", "~a~en ab";
        subjunctive_i_first: "~e~e ab", "~e~en ab";
        subjunctive_i_second: "~e~est ab", "~e~et ab";
        subjunctive_i_third: "~e~e ab", referred;
        subjunctive_i_i_first: "~ü~e ab", "~ü~en ab";
        subjunctive_i_i_second: "~ü~est ab", "~ü~et ab";
        subjunctive_i_i_third: "~ü~e ab", "~ü~en ab";
        subordinate_present_first: "ab~e~e", "ab~e~en";
        subordinate_present_second: "ab~i~st", "ab~e~t";
        subordinate_present_third: "ab~i~t", "ab~e~en";
        subordinate_preterite_first: "ab~a~", "ab~a~en";
        subordinate_preterite_second: "ab~a~st", "ab~a~t";
        subordinate_preterite_third: "ab~a~", "ab~a~en";
        subordinate_subjunctive_i_first: "ab~e~e", "ab~e~en";
        subordinate_subjunctive_i_second: "ab~e~est", "ab~e~et";
        subordinate_subjunctive_i_third: "ab~e~e", referred;
        subordinate_subjunctive_i_i_first: "ab~ü~e", "ab~ü~en";
        subordinate_subjunctive_i_i_second: "ab~ü~est", "ab~ü~et";
        subordinate_subjunctive_i_i_third: "ab~ü~e", "ab~ü~en";
        imperative: "~i~ ab", "~e~t ab";
        infinitive: "ab~e~en";
        zu_infinitive: "abzu~e~en";
        participle: "ab~e~end", "abge~o~en";
    ),
    // Treten: 9 words (treten, bersten, gelten, schelten, vertreten, vergelten).
    conjugation!("treten", 9, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~tst", "~e~et";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~est|~a~st", "~a~et";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~tst", "~e~et";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~est|~a~st", "~a~et";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~t", "~e~et";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~e~en";
    ),
    // Begreifen: 9 words (begreifen, ergreifen, verpfeifen, verkneifen, vergreifen, herausgreifen).
    conjugation!("begreifen", 9, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~f", "~~fen";
        preterite_second: "~~fst", "~~ft";
        preterite_third: "~~f", "~~fen";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~fe", "~~fen";
        subordinate_subjunctive_i_i_second: "~~fest|~~fst", "~~fet|~~ft";
        subordinate_subjunctive_i_i_third: "~~fe", "~~fen";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~~fen";
    ),
    // Hineinbringen: 9 words (hineinblicken, hineinmischen, hineinzoomen, hineinspielen, hineinbringen, hineinmusen).
    conjugation!("hineinbringen", 9, "hinein~en";
        present_first: "~e hinein", "~en hinein";
        present_second: "~st hinein", "~t hinein";
        present_third: "~t hinein", "~en hinein";
        preterite_first: "~te hinein", "~ten hinein";
        preterite_second: "~test hinein", "~tet hinein";
        preterite_third: "~te hinein", "~ten hinein";
        subjunctive_i_first: "~e hinein", "~en hinein";
        subjunctive_i_second: "~est hinein", "~et hinein";
        subjunctive_i_third: "~e hinein", referred;
        subjunctive_i_i_first: "~te hinein", "~ten hinein";
        subjunctive_i_i_second: "~test hinein", "~tet hinein";
        subjunctive_i_i_third: "~te hinein", "~ten hinein";
        subordinate_present_first: "hinein~e", "hinein~en";
        subordinate_present_second: "hinein~st", "hinein~t";
        subordinate_present_third: "hinein~t", "hinein~en";
        subordinate_preterite_first: "hinein~te", "hinein~ten";
        subordinate_preterite_second: "hinein~test", "hinein~tet";
        subordinate_preterite_third: "hinein~te", "hinein~ten";
        subordinate_subjunctive_i_first: "hinein~e", "hinein~en";
        subordinate_subjunctive_i_second: "hinein~est", "hinein~et";
        subordinate_subjunctive_i_third: "hinein~e", referred;
        subordinate_subjunctive_i_i_first: "hinein~te", "hinein~ten";
        subordinate_subjunctive_i_i_second: "hinein~test", "hinein~tet";
        subordinate_subjunctive_i_i_third: "hinein~te", "hinein~ten";
        imperative: "~ hinein|~e hinein", "~t hinein";
        infinitive: "hinein~en";
        zu_infinitive: "hineinzu~en";
        participle: "hinein~end", "hineinge~t";
    ),
    // Übergreifen: 9 words (überschnappen, überspielen, übergreifen, überbeugen, überbleiben, überdecken).
    conjugation!("übergreifen", 9, "über~en";
        present_first: "~e über", "~en über";
        present_second: "~st über", "~t über";
        present_third: "~t über", "~en über";
        preterite_first: "~te über", "~ten über";
        preterite_second: "~test über", "~tet über";
        preterite_third: "~te über", "~ten über";
        subjunctive_i_first: "~e über", "~en über";
        subjunctive_i_second: "~est über", "~et über";
        subjunctive_i_third: "~e über", referred;
        subjunctive_i_i_first: "~te über", "~ten über";
        subjunctive_i_i_second: "~test über", "~tet über";
        subjunctive_i_i_third: "~te über", "~ten über";
        subordinate_present_first: "über~e", "über~en";
        subordinate_present_second: "über~st", "über~t";
        subordinate_present_third: "über~t", "über~en";
        subordinate_preterite_first: "über~te", "über~ten";
        subordinate_preterite_second: "über~test", "über~tet";
        subordinate_preterite_third: "über~te", "über~ten";
        subordinate_subjunctive_i_first: "über~e", "über~en";
        subordinate_subjunctive_i_second: "über~est", "über~et";
        subordinate_subjunctive_i_third: "über~e", referred;
        subordinate_subjunctive_i_i_first: "über~te", "über~ten";
        subordinate_subjunctive_i_i_second: "über~test", "über~tet";
        subordinate_subjunctive_i_i_third: "über~te", "über~ten";
        imperative: "~ über", "~t über";
        infinitive: "über~en";
        zu_infinitive: "überzu~en";
        participle: "über~end", "überge~t";
    ),
    // Nacharbeiten: 9 words (nachrechnen, nachbilden, nachrüsten, nachdichten, nachbeten, nacharten).
    conjugation!("nacharbeiten", 9, "nach~en";
        present_first: "~e nach", "~en nach";
        present_second: "~est nach", "~et nach";
        present_third: "~et nach", "~en nach";
        preterite_first: "~ete nach", "~eten nach";
        preterite_second: "~etest nach", "~etet nach";
        preterite_third: "~ete nach", "~eten nach";
        subjunctive_i_first: "~e nach", "~en nach";
        subjunctive_i_second: "~est nach", "~et nach";
        subjunctive_i_third: "~e nach", referred;
        subjunctive_i_i_first: "~ete nach", "~eten nach";
        subjunctive_i_i_second: "~etest nach", "~etet nach";
        subjunctive_i_i_third: "~ete nach", "~eten nach";
        subordinate_present_first: "nach~e", "nach~en";
        subordinate_present_second: "nach~est", "nach~et";
        subordinate_present_third: "nach~et", "nach~en";
        subordinate_preterite_first: "nach~ete", "nach~eten";
        subordinate_preterite_second: "nach~etest", "nach~etet";
        subordinate_preterite_third: "nach~ete", "nach~eten";
        subordinate_subjunctive_i_first: "nach~e", "nach~en";
        subordinate_subjunctive_i_second: "nach~est", "nach~et";
        subordinate_subjunctive_i_third: "nach~e", referred;
        subordinate_subjunctive_i_i_first: "nach~ete", "nach~eten";
        subordinate_subjunctive_i_i_second: "nach~etest", "nach~etet";
        subordinate_subjunctive_i_i_third: "nach~ete", "nach~eten";
        imperative: "~ nach|~e nach", "~et nach";
        infinitive: "nach~en";
        zu_infinitive: "nachzu~en";
        participle: "nach~end", "nachge~et";
    ),
    // Abessen: 9 words (mitessen, aufessen, durchessen, weiteressen, leeressen, anessen).
    conjugation!("abessen", 9, "~essen";
        present_first: "esse ~", "essen ~";
        present_second: "isst ~", "esst ~";
        present_third: "isst ~", "essen ~";
        preterite_first: "aß ~", "aßen ~";
        preterite_second: "aßest ~", "aßt ~";
        preterite_third: "aß ~", "aßen ~";
        subjunctive_i_first: "esse ~", "essen ~";
        subjunctive_i_second: "essest ~", "esset ~";
        subjunctive_i_third: "esse ~", referred;
        subjunctive_i_i_first: "äße ~", "äßen ~";
        subjunctive_i_i_second: "äßest ~", "äßet ~";
        subjunctive_i_i_third: "äße ~", "äßen ~";
        subordinate_present_first: "~esse", "~essen";
        subordinate_present_second: "~isst", "~esst";
        subordinate_present_third: "~isst", "~essen";
        subordinate_preterite_first: "~aß", "~aßen";
        subordinate_preterite_second: "~aßest", "~aßt";
        subordinate_preterite_third: "~aß", "~aßen";
        subordinate_subjunctive_i_first: "~esse", "~essen";
        subordinate_subjunctive_i_second: "~essest", "~esset";
        subordinate_subjunctive_i_third: "~esse", referred;
        subordinate_subjunctive_i_i_first: "~äße", "~äßen";
        subordinate_subjunctive_i_i_second: "~äßest", "~äßet";
        subordinate_subjunctive_i_i_third: "~äße", "~äßen";
        imperative: "iss ~", "esst ~";
        infinitive: "~essen";
        zu_infinitive: "~zuessen";
        participle: "~essend", "~gegessen";
    ),
    // Anstreichen: 9 words (angleichen, anschleichen, anspeien, anstreichen, anschreien, anschmeissen).
    conjugation!("anstreichen", 9, "an~e~en";
        present_first: "~e~e an", "~e~en an";
        present_second: "~e~t an", "~e~t an";
        present_third: "~e~t an", "~e~en an";
        preterite_first: "~~ an", "~~en an";
        preterite_second: "~~est an", "~~t an";
        preterite_third: "~~ an", "~~en an";
        subjunctive_i_first: "~e~e an", "~e~en an";
        subjunctive_i_second: "~e~est an", "~e~et an";
        subjunctive_i_third: "~e~e an", referred;
        subjunctive_i_i_first: "~~e an", "~~en an";
        subjunctive_i_i_second: "~~est an", "~~et an|~~t an";
        subjunctive_i_i_third: "~~e an", "~~en an";
        subordinate_present_first: "an~e~e", "an~e~en";
        subordinate_present_second: "an~e~t", "an~e~t";
        subordinate_present_third: "an~e~t", "an~e~en";
        subordinate_preterite_first: "an~~", "an~~en";
        subordinate_preterite_second: "an~~est", "an~~t";
        subordinate_preterite_third: "an~~", "an~~en";
        subordinate_subjunctive_i_first: "an~e~e", "an~e~en";
        subordinate_subjunctive_i_second: "an~e~est", "an~e~et";
        subordinate_subjunctive_i_third: "an~e~e", referred;
        subordinate_subjunctive_i_i_first: "an~~e", "an~~en";
        subordinate_subjunctive_i_i_second: "an~~est", "an~~et|an~~t";
        subordinate_subjunctive_i_i_third: "an~~e", "an~~en";
        imperative: "~e~ an|~e~e an", "~e~t an";
        infinitive: "an~e~en";
        zu_infinitive: "anzu~e~en";
        participle: "an~e~end", "ange~~en";
    ),
    // Gebieten: 8 words (bieten, verbieten, feilbieten, darbieten, gebieten, entbieten).
    conjugation!("gebieten", 8, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~est", "~ie~et";
        present_third: "~ie~et", "~ie~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~est|~o~st", "~o~et";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", referred;
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est", "~ö~et";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~ie~e", "~ie~en";
        subordinate_present_second: "~ie~est", "~ie~et";
        subordinate_present_third: "~ie~et", "~ie~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~est|~o~st", "~o~et";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_third: "~ie~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est", "~ö~et";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ie~|~ie~e", "~ie~et";
        infinitive: "~ie~en";
        zu_infinitive: "abzu~e~n";
        participle: "~ie~end", "~o~en";
    ),
    // Durchsetzen: 8 words (durchsetzen, durchkreuzen, durchputzen, durchsitzen, durchbumsen, durchpausen).
    conjugation!("durchsetzen", 8, "durch~en";
        present_first: "~e durch", "~en durch";
        present_second: "~t durch", "~t durch";
        present_third: "~t durch", "~en durch";
        preterite_first: "~te durch", "~ten durch";
        preterite_second: "~test durch", "~tet durch";
        preterite_third: "~te durch", "~ten durch";
        subjunctive_i_first: "~e durch", "~en durch";
        subjunctive_i_second: "~est durch", "~et durch";
        subjunctive_i_third: "~e durch", referred;
        subjunctive_i_i_first: "~te durch", "~ten durch";
        subjunctive_i_i_second: "~test durch", "~tet durch";
        subjunctive_i_i_third: "~te durch", "~ten durch";
        subordinate_present_first: "durch~e", "durch~en";
        subordinate_present_second: "durch~t", "durch~t";
        subordinate_present_third: "durch~t", "durch~en";
        subordinate_preterite_first: "durch~te", "durch~ten";
        subordinate_preterite_second: "durch~test", "durch~tet";
        subordinate_preterite_third: "durch~te", "durch~ten";
        subordinate_subjunctive_i_first: "durch~e", "durch~en";
        subordinate_subjunctive_i_second: "durch~est", "durch~et";
        subordinate_subjunctive_i_third: "durch~e", referred;
        subordinate_subjunctive_i_i_first: "durch~te", "durch~ten";
        subordinate_subjunctive_i_i_second: "durch~test", "durch~tet";
        subordinate_subjunctive_i_i_third: "durch~te", "durch~ten";
        imperative: "~ durch|~e durch", "~t durch";
        infinitive: "durch~en";
        zu_infinitive: "durchzu~en";
        participle: "durch~end", "durchge~t";
    ),
    // Begleichen: 8 words (vergleichen, unterstreichen, verstreichen, entweichen, verbleichen, begleichen).
    conjugation!("begleichen", 8, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~", "~~en";
        preterite_second: "~~st", "~~t";
        preterite_third: "~~", "~~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_i_second: "~~est|~~st", "~~et|~~t";
        subordinate_subjunctive_i_i_third: "~~e", "~~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~~en";
    ),
    // Denken: 8 words (denken, bedenken, gedenken, verdenken, überdenken, erdenken).
    conjugation!("denken", 8, "~enken";
        present_first: "~enke", "~enken";
        present_second: "~enkst", "~enkt";
        present_third: "~enkt", "~enken";
        preterite_first: "~achte", "~achten";
        preterite_second: "~achtest", "~achtet";
        preterite_third: "~achte", "~achten";
        subjunctive_i_first: "~enke", "~enken";
        subjunctive_i_second: "~enkest", "~enket";
        subjunctive_i_third: "~enke", referred;
        subjunctive_i_i_first: "~ächte", "~ächten";
        subjunctive_i_i_second: "~ächtest", "~ächtet";
        subjunctive_i_i_third: "~ächte", "~ächten";
        subordinate_present_first: "~enke", "~enken";
        subordinate_present_second: "~enkst", "~enkt";
        subordinate_present_third: "~enkt", "~enken";
        subordinate_preterite_first: "~achte", "~achten";
        subordinate_preterite_second: "~achtest", "~achtet";
        subordinate_preterite_third: "~achte", "~achten";
        subordinate_subjunctive_i_first: "~enke", "~enken";
        subordinate_subjunctive_i_second: "~enkest", "~enket";
        subordinate_subjunctive_i_third: "~enke", referred;
        subordinate_subjunctive_i_i_first: "~ächte", "~ächten";
        subordinate_subjunctive_i_i_second: "~ächtest", "~ächtet";
        subordinate_subjunctive_i_i_third: "~ächte", "~ächten";
        imperative: "~enk|~enke", "~enkt";
        infinitive: "~enken";
        zu_infinitive: "abzu~en";
        participle: "~enkend", "~acht";
    ),
    // Fangen: 8 words (fangen, empfangen, verfangen, auffangen, befangen, umfangen).
    conjugation!("fangen", 8, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~t";
        present_third: "~ä~t", "~a~en";
        preterite_first: "~i~", "~i~en";
        preterite_second: "~i~st", "~i~t";
        preterite_third: "~i~", "~i~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
        subjunctive_i_i_first: "~i~e", "~i~en";
        subjunctive_i_i_second: "~i~est|~i~st", "~i~et|~i~t";
        subjunctive_i_i_third: "~i~e", "~i~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~st", "~a~t";
        subordinate_present_third: "~ä~t", "~a~en";
        subordinate_preterite_first: "~i~", "~i~en";
        subordinate_preterite_second: "~i~st", "~i~t";
        subordinate_preterite_third: "~i~", "~i~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_i_second: "~i~est|~i~st", "~i~et|~i~t";
        subordinate_subjunctive_i_i_third: "~i~e", "~i~en";
        imperative: "~a~|~a~e", "~a~t";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "~a~en";
    ),
    // Einverleiben: 8 words (einbetonieren, einbalsamieren, einprogrammieren, einmarschieren, einverleiben, einbestellen).
    conjugation!("einverleiben", 8, "ein~en";
        present_first: "~e ein", "~en ein";
        present_second: "~st ein", "~t ein";
        present_third: "~t ein", "~en ein";
        preterite_first: "~te ein", "~ten ein";
        preterite_second: "~test ein", "~tet ein";
        preterite_third: "~te ein", "~ten ein";
        subjunctive_i_first: "~e ein", "~en ein";
        subjunctive_i_second: "~est ein", "~et ein";
        subjunctive_i_third: "~e ein", referred;
        subjunctive_i_i_first: "~te ein", "~ten ein";
        subjunctive_i_i_second: "~test ein", "~tet ein";
        subjunctive_i_i_third: "~te ein", "~ten ein";
        subordinate_present_first: "ein~e", "ein~en";
        subordinate_present_second: "ein~st", "ein~t";
        subordinate_present_third: "ein~t", "ein~en";
        subordinate_preterite_first: "ein~te", "ein~ten";
        subordinate_preterite_second: "ein~test", "ein~tet";
        subordinate_preterite_third: "ein~te", "ein~ten";
        subordinate_subjunctive_i_first: "ein~e", "ein~en";
        subordinate_subjunctive_i_second: "ein~est", "ein~et";
        subordinate_subjunctive_i_third: "ein~e", referred;
        subordinate_subjunctive_i_i_first: "ein~te", "ein~ten";
        subordinate_subjunctive_i_i_second: "ein~test", "ein~tet";
        subordinate_subjunctive_i_i_third: "ein~te", "ein~ten";
        imperative: "~ ein|~e ein", "~t ein";
        infinitive: "ein~en";
        zu_infinitive: "einzu~en";
        participle: "ein~end", "ein~t";
    ),
    // Beziehen: 8 words (ziehen, hinterziehen, unterziehen, beziehen, erziehen, entziehen).
    conjugation!("beziehen", 8, "~iehen";
        present_first: "~iehe", "~iehen";
        present_second: "~iehst", "~ieht";
        present_third: "~ieht", "~iehen";
        preterite_first: "~og", "~ogen";
        preterite_second: "~ogst", "~ogt";
        preterite_third: "~og", "~ogen";
        subjunctive_i_first: "~iehe", "~iehen";
        subjunctive_i_second: "~iehest", "~iehet";
        subjunctive_i_third: "~iehe", referred;
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
        subordinate_subjunctive_i_third: "~iehe", referred;
        subordinate_subjunctive_i_i_first: "~öge", "~ögen";
        subordinate_subjunctive_i_i_second: "~ögest|~ögst", "~öget|~ögt";
        subordinate_subjunctive_i_i_third: "~öge", "~ögen";
        imperative: "~ieh|~iehe", "~ieht";
        infinitive: "~iehen";
        zu_infinitive: "abzu~en";
        participle: "~iehend", "~ogen";
    ),
    // Einschlagen: 8 words (einschlafen, einladen, eintragen, einfahren, einfangen, einschlagen).
    conjugation!("einschlagen", 8, "ein~a~en";
        present_first: "~a~e ein", "~a~en ein";
        present_second: "~ä~st ein", "~a~t ein";
        present_third: "~ä~t ein", "~a~en ein";
        preterite_first: "~u~ ein", "~u~en ein";
        preterite_second: "~u~st ein", "~u~t ein";
        preterite_third: "~u~ ein", "~u~en ein";
        subjunctive_i_first: "~a~e ein", "~a~en ein";
        subjunctive_i_second: "~a~est ein", "~a~et ein";
        subjunctive_i_third: "~a~e ein", referred;
        subjunctive_i_i_first: "~ü~e ein", "~ü~en ein";
        subjunctive_i_i_second: "~ü~est ein", "~ü~et ein";
        subjunctive_i_i_third: "~ü~e ein", "~ü~en ein";
        subordinate_present_first: "ein~a~e", "ein~a~en";
        subordinate_present_second: "ein~ä~st", "ein~a~t";
        subordinate_present_third: "ein~ä~t", "ein~a~en";
        subordinate_preterite_first: "ein~u~", "ein~u~en";
        subordinate_preterite_second: "ein~u~st", "ein~u~t";
        subordinate_preterite_third: "ein~u~", "ein~u~en";
        subordinate_subjunctive_i_first: "ein~a~e", "ein~a~en";
        subordinate_subjunctive_i_second: "ein~a~est", "ein~a~et";
        subordinate_subjunctive_i_third: "ein~a~e", referred;
        subordinate_subjunctive_i_i_first: "ein~ü~e", "ein~ü~en";
        subordinate_subjunctive_i_i_second: "ein~ü~est", "ein~ü~et";
        subordinate_subjunctive_i_i_third: "ein~ü~e", "ein~ü~en";
        imperative: "~a~ ein|~a~e ein", "~a~t ein";
        infinitive: "ein~a~en";
        zu_infinitive: "einzu~a~en";
        participle: "ein~a~end", "einge~a~en";
    ),
    // Ausschlagen: 8 words (auswachsen, ausgraben, austragen, ausladen, ausschlagen, ausschlafen).
    conjugation!("ausschlagen", 8, "aus~a~en";
        present_first: "~a~e aus", "~a~en aus";
        present_second: "~ä~st aus", "~a~t aus";
        present_third: "~ä~t aus", "~a~en aus";
        preterite_first: "~u~ aus", "~u~en aus";
        preterite_second: "~u~st aus", "~u~t aus";
        preterite_third: "~u~ aus", "~u~en aus";
        subjunctive_i_first: "~a~e aus", "~a~en aus";
        subjunctive_i_second: "~a~est aus", "~a~et aus";
        subjunctive_i_third: "~a~e aus", referred;
        subjunctive_i_i_first: "~ü~e aus", "~ü~en aus";
        subjunctive_i_i_second: "~ü~est aus", "~ü~et aus";
        subjunctive_i_i_third: "~ü~e aus", "~ü~en aus";
        subordinate_present_first: "aus~a~e", "aus~a~en";
        subordinate_present_second: "aus~ä~st", "aus~a~t";
        subordinate_present_third: "aus~ä~t", "aus~a~en";
        subordinate_preterite_first: "aus~u~", "aus~u~en";
        subordinate_preterite_second: "aus~u~st", "aus~u~t";
        subordinate_preterite_third: "aus~u~", "aus~u~en";
        subordinate_subjunctive_i_first: "aus~a~e", "aus~a~en";
        subordinate_subjunctive_i_second: "aus~a~est", "aus~a~et";
        subordinate_subjunctive_i_third: "aus~a~e", referred;
        subordinate_subjunctive_i_i_first: "aus~ü~e", "aus~ü~en";
        subordinate_subjunctive_i_i_second: "aus~ü~est", "aus~ü~et";
        subordinate_subjunctive_i_i_third: "aus~ü~e", "aus~ü~en";
        imperative: "~a~ aus|~a~e aus", "~a~t aus";
        infinitive: "aus~a~en";
        zu_infinitive: "auszu~a~en";
        participle: "aus~a~end", "ausge~a~en";
    ),
    // Geifern: 8 words (endlagern, geifern, gendern, nachtwandeln, maßschneidern, massschneidern).
    conjugation!("geifern", 8, "~~e~n";
        present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        present_second: "~~e~st", "~~e~t";
        present_third: "~~e~t", "~~e~n";
        preterite_first: "~~e~te", "~~e~ten";
        preterite_second: "~~e~test", "~~e~tet";
        preterite_third: "~~e~te", "~~e~ten";
        subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subjunctive_i_third: "~~e~e|~~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e~e|~~~e", referred;
        subordinate_subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subordinate_subjunctive_i_i_third: "~~e~te", "~~e~ten";
        imperative: "~~~e|~~e~|~~e~e", "~~e~t";
        infinitive: "~~e~n";
        zu_infinitive: "~~~zuziehen";
        participle: "~~e~nd", "~ge~e~t";
    ),
    // Hinauswollen: 8 words (hinwollen, hinaufwollen, übelwollen, wohlwollen, hineinwollen, wegwollen).
    conjugation!("hinauswollen", 8, "~wollen";
        present_first: "will ~", "wollen ~";
        present_second: "willst ~", "wollt ~";
        present_third: "will ~", "wollen ~";
        preterite_first: "wollte ~", "wollten ~";
        preterite_second: "wolltest ~", "wolltet ~";
        preterite_third: "wollte ~", "wollten ~";
        subjunctive_i_first: "wolle ~", "wollen ~";
        subjunctive_i_second: "wollest ~", "wollet ~";
        subjunctive_i_third: "wolle ~", referred;
        subjunctive_i_i_first: "wollte ~", "wollten ~";
        subjunctive_i_i_second: "wolltest ~", "wolltet ~";
        subjunctive_i_i_third: "wollte ~", "wollten ~";
        subordinate_present_first: "~will", "~wollen";
        subordinate_present_second: "~willst", "~wollt";
        subordinate_present_third: "~will", "~wollen";
        subordinate_preterite_first: "~wollte", "~wollten";
        subordinate_preterite_second: "~wolltest", "~wolltet";
        subordinate_preterite_third: "~wollte", "~wollten";
        subordinate_subjunctive_i_first: "~wolle", "~wollen";
        subordinate_subjunctive_i_second: "~wollest", "~wollet";
        subordinate_subjunctive_i_third: "~wolle", referred;
        subordinate_subjunctive_i_i_first: "~wollte", "~wollten";
        subordinate_subjunctive_i_i_second: "~wolltest", "~wolltet";
        subordinate_subjunctive_i_i_third: "~wollte", "~wollten";
        imperative: "~", "~t";
        infinitive: "~wollen";
        zu_infinitive: "~zuwollen";
        participle: "~wollend", "~gewollt";
    ),
    // Weiterzahlen: 8 words (weiterexistieren, weiterstudieren, weiterverkaufen, weiterbestehen, weiterkämpfen, weiterzahlen).
    conjugation!("weiterzahlen", 8, "weiter~en";
        present_first: "~e weiter", "~en weiter";
        present_second: "~st weiter", "~t weiter";
        present_third: "~t weiter", "~en weiter";
        preterite_first: "~te weiter", "~ten weiter";
        preterite_second: "~test weiter", "~tet weiter";
        preterite_third: "~te weiter", "~ten weiter";
        subjunctive_i_first: "~e weiter", "~en weiter";
        subjunctive_i_second: "~est weiter", "~et weiter";
        subjunctive_i_third: "~e weiter", referred;
        subjunctive_i_i_first: "~te weiter", "~ten weiter";
        subjunctive_i_i_second: "~test weiter", "~tet weiter";
        subjunctive_i_i_third: "~te weiter", "~ten weiter";
        subordinate_present_first: "weiter~e", "weiter~en";
        subordinate_present_second: "weiter~st", "weiter~t";
        subordinate_present_third: "weiter~t", "weiter~en";
        subordinate_preterite_first: "weiter~te", "weiter~ten";
        subordinate_preterite_second: "weiter~test", "weiter~tet";
        subordinate_preterite_third: "weiter~te", "weiter~ten";
        subordinate_subjunctive_i_first: "weiter~e", "weiter~en";
        subordinate_subjunctive_i_second: "weiter~est", "weiter~et";
        subordinate_subjunctive_i_third: "weiter~e", referred;
        subordinate_subjunctive_i_i_first: "weiter~te", "weiter~ten";
        subordinate_subjunctive_i_i_second: "weiter~test", "weiter~tet";
        subordinate_subjunctive_i_i_third: "weiter~te", "weiter~ten";
        imperative: "~ weiter|~e weiter", "~t weiter";
        infinitive: "weiter~en";
        zu_infinitive: "weiterzu~en";
        participle: "weiter~end", "weiter~t";
    ),
    // Speien: 7 words (scheissen, schreien, speien, reissen, beissen, schmeissen).
    conjugation!("speien", 7, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~t", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~", "~~en";
        preterite_second: "~~est|~~t", "~~t";
        preterite_third: "~~", "~~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_i_second: "~~est|~~t", "~~et|~~t";
        subordinate_subjunctive_i_i_third: "~~e", "~~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~~en";
    ),
    // Absteigen: 7 words (abschreiben, abtreiben, abweisen, abreiben, absteigen, abscheiden).
    conjugation!("absteigen", 7, "ab~e~~en";
        present_first: "~e~~e ab", "~e~~en ab";
        present_second: "~e~~st ab", "~e~~t ab";
        present_third: "~e~~t ab", "~e~~en ab";
        preterite_first: "~~e~ ab", "~~e~en ab";
        preterite_second: "~~e~st ab", "~~e~t ab";
        preterite_third: "~~e~ ab", "~~e~en ab";
        subjunctive_i_first: "~e~~e ab", "~e~~en ab";
        subjunctive_i_second: "~e~~est ab", "~e~~et ab";
        subjunctive_i_third: "~e~~e ab", referred;
        subjunctive_i_i_first: "~~e~e ab", "~~e~en ab";
        subjunctive_i_i_second: "~~e~est ab|~~e~st ab", "~~e~et ab|~~e~t ab";
        subjunctive_i_i_third: "~~e~e ab", "~~e~en ab";
        subordinate_present_first: "ab~e~~e", "ab~e~~en";
        subordinate_present_second: "ab~e~~st", "ab~e~~t";
        subordinate_present_third: "ab~e~~t", "ab~e~~en";
        subordinate_preterite_first: "ab~~e~", "ab~~e~en";
        subordinate_preterite_second: "ab~~e~st", "ab~~e~t";
        subordinate_preterite_third: "ab~~e~", "ab~~e~en";
        subordinate_subjunctive_i_first: "ab~e~~e", "ab~e~~en";
        subordinate_subjunctive_i_second: "ab~e~~est", "ab~e~~et";
        subordinate_subjunctive_i_third: "ab~e~~e", referred;
        subordinate_subjunctive_i_i_first: "ab~~e~e", "ab~~e~en";
        subordinate_subjunctive_i_i_second: "ab~~e~est|ab~~e~st", "ab~~e~et|ab~~e~t";
        subordinate_subjunctive_i_i_third: "ab~~e~e", "ab~~e~en";
        imperative: "~e~~ ab|~e~~e ab", "~e~~t ab";
        infinitive: "ab~e~~en";
        zu_infinitive: "abzu~e~~en";
        participle: "ab~e~~end", "abge~~e~en";
    ),
    // Anschreiben: 7 words (anschreiben, anweisen, antreiben, ansteigen, anpreisen, anscheinen).
    conjugation!("anschreiben", 7, "an~e~~en";
        present_first: "~e~~e an", "~e~~en an";
        present_second: "~e~~st an", "~e~~t an";
        present_third: "~e~~t an", "~e~~en an";
        preterite_first: "~~e~ an", "~~e~en an";
        preterite_second: "~~e~st an", "~~e~t an";
        preterite_third: "~~e~ an", "~~e~en an";
        subjunctive_i_first: "~e~~e an", "~e~~en an";
        subjunctive_i_second: "~e~~est an", "~e~~et an";
        subjunctive_i_third: "~e~~e an", referred;
        subjunctive_i_i_first: "~~e~e an", "~~e~en an";
        subjunctive_i_i_second: "~~e~est an|~~e~st an", "~~e~et an|~~e~t an";
        subjunctive_i_i_third: "~~e~e an", "~~e~en an";
        subordinate_present_first: "an~e~~e", "an~e~~en";
        subordinate_present_second: "an~e~~st", "an~e~~t";
        subordinate_present_third: "an~e~~t", "an~e~~en";
        subordinate_preterite_first: "an~~e~", "an~~e~en";
        subordinate_preterite_second: "an~~e~st", "an~~e~t";
        subordinate_preterite_third: "an~~e~", "an~~e~en";
        subordinate_subjunctive_i_first: "an~e~~e", "an~e~~en";
        subordinate_subjunctive_i_second: "an~e~~est", "an~e~~et";
        subordinate_subjunctive_i_third: "an~e~~e", referred;
        subordinate_subjunctive_i_i_first: "an~~e~e", "an~~e~en";
        subordinate_subjunctive_i_i_second: "an~~e~est|an~~e~st", "an~~e~et|an~~e~t";
        subordinate_subjunctive_i_i_third: "an~~e~e", "an~~e~en";
        imperative: "~e~~ an|~e~~e an", "~e~~t an";
        infinitive: "an~e~~en";
        zu_infinitive: "anzu~e~~en";
        participle: "an~e~~end", "ange~~e~en";
    ),
    // Schmeißen: 7 words (scheißen, beißen, schleißen, reißen, schmeißen, spleißen).
    conjugation!("schmeißen", 7, "~e~ßen";
        present_first: "~e~ße", "~e~ßen";
        present_second: "~e~ßt", "~e~ßt";
        present_third: "~e~ßt", "~e~ßen";
        preterite_first: "~~ss", "~~ssen";
        preterite_second: "~~ssest|~~sst", "~~sst";
        preterite_third: "~~ss", "~~ssen";
        subjunctive_i_first: "~e~ße", "~e~ßen";
        subjunctive_i_second: "~e~ßest", "~e~ßet";
        subjunctive_i_third: "~e~ße", referred;
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
        subordinate_subjunctive_i_third: "~e~ße", referred;
        subordinate_subjunctive_i_i_first: "~~sse", "~~ssen";
        subordinate_subjunctive_i_i_second: "~~ssest|~~sst", "~~sset|~~sst";
        subordinate_subjunctive_i_i_third: "~~sse", "~~ssen";
        imperative: "~e~ß|~e~ße", "~e~ßt";
        infinitive: "~e~ßen";
        zu_infinitive: "abzu~e~n";
        participle: "~e~ßend", "ge~~ssen";
    ),
    // Rumeiern: 7 words (rumkümmeln, rumalbern, rumeiern, rumkrabbeln, rumgammeln, rumlungern).
    conjugation!("rumeiern", 7, "rum~e~n";
        present_first: "~~e rum|~e~e rum|~e~ rum", "~e~n rum";
        present_second: "~e~st rum", "~e~t rum";
        present_third: "~e~t rum", "~e~n rum";
        preterite_first: "~e~te rum", "~e~ten rum";
        preterite_second: "~e~test rum", "~e~tet rum";
        preterite_third: "~e~te rum", "~e~ten rum";
        subjunctive_i_first: "~e~e rum|~~e rum", "~e~n rum";
        subjunctive_i_second: "~e~est rum|~~est rum", "~e~et rum|~~et rum";
        subjunctive_i_third: "~e~e rum|~~e rum", referred;
        subjunctive_i_i_first: "~e~te rum", "~e~ten rum";
        subjunctive_i_i_second: "~e~test rum", "~e~tet rum";
        subjunctive_i_i_third: "~e~te rum", "~e~ten rum";
        subordinate_present_first: "rum~~e|rum~e~e|rum~e~", "rum~e~n";
        subordinate_present_second: "rum~e~st", "rum~e~t";
        subordinate_present_third: "rum~e~t", "rum~e~n";
        subordinate_preterite_first: "rum~e~te", "rum~e~ten";
        subordinate_preterite_second: "rum~e~test", "rum~e~tet";
        subordinate_preterite_third: "rum~e~te", "rum~e~ten";
        subordinate_subjunctive_i_first: "rum~e~e|rum~~e", "rum~e~n";
        subordinate_subjunctive_i_second: "rum~e~est|rum~~est", "rum~e~et|rum~~et";
        subordinate_subjunctive_i_third: "rum~e~e|rum~~e", referred;
        subordinate_subjunctive_i_i_first: "rum~e~te", "rum~e~ten";
        subordinate_subjunctive_i_i_second: "rum~e~test", "rum~e~tet";
        subordinate_subjunctive_i_i_third: "rum~e~te", "rum~e~ten";
        imperative: "~~e rum|~e~ rum|~e~e rum", "~e~t rum";
        infinitive: "rum~e~n";
        zu_infinitive: "rumzu~e~n";
        participle: "rum~e~nd", "rumge~e~t";
    ),
    // Kommen: 7 words (kommen, bekommen, entkommen, überkommen, hinzubekommen, verkommen).
    conjugation!("kommen", 7, "~o~men";
        present_first: "~o~me", "~o~men";
        present_second: "~o~mst", "~o~mt";
        present_third: "~o~mt", "~o~men";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~o~me", "~o~men";
        subjunctive_i_second: "~o~mest", "~o~met";
        subjunctive_i_third: "~o~me", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~o~me", "~o~men";
        subordinate_present_second: "~o~mst", "~o~mt";
        subordinate_present_third: "~o~mt", "~o~men";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~o~me", "~o~men";
        subordinate_subjunctive_i_second: "~o~mest", "~o~met";
        subordinate_subjunctive_i_third: "~o~me", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~o~m|~o~me", "~o~mt";
        infinitive: "~o~men";
        zu_infinitive: "abzu~e~n";
        participle: "~o~mend", "~o~men";
    ),
    // Scheiden: 7 words (meiden, vermeiden, scheiden, bescheiden, unterscheiden, entscheiden).
    conjugation!("scheiden", 7, "~~iden";
        present_first: "~~ide", "~~iden";
        present_second: "~~idest", "~~idet";
        present_third: "~~idet", "~~iden";
        preterite_first: "~i~d", "~i~den";
        preterite_second: "~i~dest|~i~dst", "~i~det";
        preterite_third: "~i~d", "~i~den";
        subjunctive_i_first: "~~ide", "~~iden";
        subjunctive_i_second: "~~idest", "~~idet";
        subjunctive_i_third: "~~ide", referred;
        subjunctive_i_i_first: "~i~de", "~i~den";
        subjunctive_i_i_second: "~i~dest", "~i~det";
        subjunctive_i_i_third: "~i~de", "~i~den";
        subordinate_present_first: "~~ide", "~~iden";
        subordinate_present_second: "~~idest", "~~idet";
        subordinate_present_third: "~~idet", "~~iden";
        subordinate_preterite_first: "~i~d", "~i~den";
        subordinate_preterite_second: "~i~dest|~i~dst", "~i~det";
        subordinate_preterite_third: "~i~d", "~i~den";
        subordinate_subjunctive_i_first: "~~ide", "~~iden";
        subordinate_subjunctive_i_second: "~~idest", "~~idet";
        subordinate_subjunctive_i_third: "~~ide", referred;
        subordinate_subjunctive_i_i_first: "~i~de", "~i~den";
        subordinate_subjunctive_i_i_second: "~i~dest", "~i~det";
        subordinate_subjunctive_i_i_third: "~i~de", "~i~den";
        imperative: "~~id|~~ide", "~~idet";
        infinitive: "~~iden";
        zu_infinitive: "abzu~e~n";
        participle: "~~idend", "~i~den";
    ),
    // Schwimmen: 7 words (klimmen, schwimmen, spinnen, glimmen, sinnen, rinnen).
    conjugation!("schwimmen", 7, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~st", "~i~t";
        present_third: "~i~t", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est", "~ä~et";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~i~e", "~i~en";
        subordinate_present_second: "~i~st", "~i~t";
        subordinate_present_third: "~i~t", "~i~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~i~e", "~i~en";
        subordinate_subjunctive_i_second: "~i~est", "~i~et";
        subordinate_subjunctive_i_third: "~i~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~|~i~e", "~i~t";
        infinitive: "~i~en";
        zu_infinitive: "abzu~e~n";
        participle: "~i~end", "ge~o~en";
    ),
    // Betrügen: 7 words (lügen, trügen, betrügen, belügen, zusammenlügen, erlügen).
    conjugation!("betrügen", 7, "~ü~en";
        present_first: "~ü~e", "~ü~en";
        present_second: "~ü~st", "~ü~t";
        present_third: "~ü~t", "~ü~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~st", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ü~e", "~ü~en";
        subjunctive_i_second: "~ü~est", "~ü~et";
        subjunctive_i_third: "~ü~e", referred;
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~ü~e", "~ü~en";
        subordinate_present_second: "~ü~st", "~ü~t";
        subordinate_present_third: "~ü~t", "~ü~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~st", "~o~t";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_second: "~ü~est", "~ü~et";
        subordinate_subjunctive_i_third: "~ü~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~st", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ü~|~ü~e", "~ü~t";
        infinitive: "~ü~en";
        zu_infinitive: "abzu~e~n";
        participle: "~ü~end", "~o~en";
    ),
    // Auffliegen: 7 words (aufschieben, aufwiegen, aufbieten, auffliegen, aufschliessen, aufschiessen).
    conjugation!("auffliegen", 7, "auf~ie~en";
        present_first: "~ie~e auf", "~ie~en auf";
        present_second: "~ie~st auf", "~ie~t auf";
        present_third: "~ie~t auf", "~ie~en auf";
        preterite_first: "~o~ auf", "~o~en auf";
        preterite_second: "~o~st auf", "~o~t auf";
        preterite_third: "~o~ auf", "~o~en auf";
        subjunctive_i_first: "~ie~e auf", "~ie~en auf";
        subjunctive_i_second: "~ie~est auf", "~ie~et auf";
        subjunctive_i_third: "~ie~e auf", referred;
        subjunctive_i_i_first: "~ö~e auf", "~ö~en auf";
        subjunctive_i_i_second: "~ö~est auf", "~ö~et auf|~ö~t auf";
        subjunctive_i_i_third: "~ö~e auf", "~ö~en auf";
        subordinate_present_first: "auf~ie~e", "auf~ie~en";
        subordinate_present_second: "auf~ie~st", "auf~ie~t";
        subordinate_present_third: "auf~ie~t", "auf~ie~en";
        subordinate_preterite_first: "auf~o~", "auf~o~en";
        subordinate_preterite_second: "auf~o~st", "auf~o~t";
        subordinate_preterite_third: "auf~o~", "auf~o~en";
        subordinate_subjunctive_i_first: "auf~ie~e", "auf~ie~en";
        subordinate_subjunctive_i_second: "auf~ie~est", "auf~ie~et";
        subordinate_subjunctive_i_third: "auf~ie~e", referred;
        subordinate_subjunctive_i_i_first: "auf~ö~e", "auf~ö~en";
        subordinate_subjunctive_i_i_second: "auf~ö~est", "auf~ö~et|auf~ö~t";
        subordinate_subjunctive_i_i_third: "auf~ö~e", "auf~ö~en";
        imperative: "~ie~ auf|~ie~e auf", "~ie~t auf";
        infinitive: "auf~ie~en";
        zu_infinitive: "aufzu~ie~en";
        participle: "auf~ie~end", "aufge~o~en";
    ),
    // Abreagieren: 7 words (abtransportieren, abdirigieren, abdestillieren, abgewöhnen, abverlangen, abreagieren).
    conjugation!("abreagieren", 7, "ab~en";
        present_first: "~e ab", "~en ab";
        present_second: "~st ab", "~t ab";
        present_third: "~t ab", "~en ab";
        preterite_first: "~te ab", "~ten ab";
        preterite_second: "~test ab", "~tet ab";
        preterite_third: "~te ab", "~ten ab";
        subjunctive_i_first: "~e ab", "~en ab";
        subjunctive_i_second: "~est ab", "~et ab";
        subjunctive_i_third: "~e ab", referred;
        subjunctive_i_i_first: "~te ab", "~ten ab";
        subjunctive_i_i_second: "~test ab", "~tet ab";
        subjunctive_i_i_third: "~te ab", "~ten ab";
        subordinate_present_first: "ab~e", "ab~en";
        subordinate_present_second: "ab~st", "ab~t";
        subordinate_present_third: "ab~t", "ab~en";
        subordinate_preterite_first: "ab~te", "ab~ten";
        subordinate_preterite_second: "ab~test", "ab~tet";
        subordinate_preterite_third: "ab~te", "ab~ten";
        subordinate_subjunctive_i_first: "ab~e", "ab~en";
        subordinate_subjunctive_i_second: "ab~est", "ab~et";
        subordinate_subjunctive_i_third: "ab~e", referred;
        subordinate_subjunctive_i_i_first: "ab~te", "ab~ten";
        subordinate_subjunctive_i_i_second: "ab~test", "ab~tet";
        subordinate_subjunctive_i_i_third: "ab~te", "ab~ten";
        imperative: "~ ab|~e ab", "~t ab";
        infinitive: "ab~en";
        zu_infinitive: "abzu~en";
        participle: "ab~end", "ab~t";
    ),
    // Auftreiben: 7 words (aufweisen, aufschreiben, aufsteigen, aufreiben, auftreiben, aufbleiben).
    conjugation!("auftreiben", 7, "auf~e~~en";
        present_first: "~e~~e auf", "~e~~en auf";
        present_second: "~e~~st auf", "~e~~t auf";
        present_third: "~e~~t auf", "~e~~en auf";
        preterite_first: "~~e~ auf", "~~e~en auf";
        preterite_second: "~~e~st auf", "~~e~t auf";
        preterite_third: "~~e~ auf", "~~e~en auf";
        subjunctive_i_first: "~e~~e auf", "~e~~en auf";
        subjunctive_i_second: "~e~~est auf", "~e~~et auf";
        subjunctive_i_third: "~e~~e auf", referred;
        subjunctive_i_i_first: "~~e~e auf", "~~e~en auf";
        subjunctive_i_i_second: "~~e~est auf|~~e~st auf", "~~e~et auf|~~e~t auf";
        subjunctive_i_i_third: "~~e~e auf", "~~e~en auf";
        subordinate_present_first: "auf~e~~e", "auf~e~~en";
        subordinate_present_second: "auf~e~~st", "auf~e~~t";
        subordinate_present_third: "auf~e~~t", "auf~e~~en";
        subordinate_preterite_first: "auf~~e~", "auf~~e~en";
        subordinate_preterite_second: "auf~~e~st", "auf~~e~t";
        subordinate_preterite_third: "auf~~e~", "auf~~e~en";
        subordinate_subjunctive_i_first: "auf~e~~e", "auf~e~~en";
        subordinate_subjunctive_i_second: "auf~e~~est", "auf~e~~et";
        subordinate_subjunctive_i_third: "auf~e~~e", referred;
        subordinate_subjunctive_i_i_first: "auf~~e~e", "auf~~e~en";
        subordinate_subjunctive_i_i_second: "auf~~e~est|auf~~e~st", "auf~~e~et|auf~~e~t";
        subordinate_subjunctive_i_i_third: "auf~~e~e", "auf~~e~en";
        imperative: "~e~~ auf|~e~~e auf", "~e~~t auf";
        infinitive: "auf~e~~en";
        zu_infinitive: "aufzu~e~~en";
        participle: "auf~e~~end", "aufge~~e~en";
    ),
    // Nehmen: 7 words (nehmen, benehmen, übernehmen, vernehmen, unternehmen, entnehmen).
    conjugation!("nehmen", 7, "~eh~en";
        present_first: "~eh~e", "~eh~en";
        present_second: "~i~mst", "~eh~t";
        present_third: "~i~mt", "~eh~en";
        preterite_first: "~ah~", "~ah~en";
        preterite_second: "~ah~st", "~ah~t";
        preterite_third: "~ah~", "~ah~en";
        subjunctive_i_first: "~eh~e", "~eh~en";
        subjunctive_i_second: "~eh~est", "~eh~et";
        subjunctive_i_third: "~eh~e", referred;
        subjunctive_i_i_first: "~äh~e", "~äh~en";
        subjunctive_i_i_second: "~äh~est|~äh~st", "~äh~et|~äh~t";
        subjunctive_i_i_third: "~äh~e", "~äh~en";
        subordinate_present_first: "~eh~e", "~eh~en";
        subordinate_present_second: "~i~mst", "~eh~t";
        subordinate_present_third: "~i~mt", "~eh~en";
        subordinate_preterite_first: "~ah~", "~ah~en";
        subordinate_preterite_second: "~ah~st", "~ah~t";
        subordinate_preterite_third: "~ah~", "~ah~en";
        subordinate_subjunctive_i_first: "~eh~e", "~eh~en";
        subordinate_subjunctive_i_second: "~eh~est", "~eh~et";
        subordinate_subjunctive_i_third: "~eh~e", referred;
        subordinate_subjunctive_i_i_first: "~äh~e", "~äh~en";
        subordinate_subjunctive_i_i_second: "~äh~est|~äh~st", "~äh~et|~äh~t";
        subordinate_subjunctive_i_i_third: "~äh~e", "~äh~en";
        imperative: "~i~m", "~eh~t";
        infinitive: "~eh~en";
        zu_infinitive: "abzu~e~n";
        participle: "~eh~end", "~o~men";
    ),
    // Austrinken: 7 words (austrinken, ausschwimmen, ausklingen, ausrinnen, auswringen, ausbedingen).
    conjugation!("austrinken", 7, "aus~i~en";
        present_first: "~i~e aus", "~i~en aus";
        present_second: "~i~st aus", "~i~t aus";
        present_third: "~i~t aus", "~i~en aus";
        preterite_first: "~a~ aus", "~a~en aus";
        preterite_second: "~a~st aus", "~a~t aus";
        preterite_third: "~a~ aus", "~a~en aus";
        subjunctive_i_first: "~i~e aus", "~i~en aus";
        subjunctive_i_second: "~i~est aus", "~i~et aus";
        subjunctive_i_third: "~i~e aus", referred;
        subjunctive_i_i_first: "~ä~e aus", "~ä~en aus";
        subjunctive_i_i_second: "~ä~est aus|~ä~st aus", "~ä~et aus|~ä~t aus";
        subjunctive_i_i_third: "~ä~e aus", "~ä~en aus";
        subordinate_present_first: "aus~i~e", "aus~i~en";
        subordinate_present_second: "aus~i~st", "aus~i~t";
        subordinate_present_third: "aus~i~t", "aus~i~en";
        subordinate_preterite_first: "aus~a~", "aus~a~en";
        subordinate_preterite_second: "aus~a~st", "aus~a~t";
        subordinate_preterite_third: "aus~a~", "aus~a~en";
        subordinate_subjunctive_i_first: "aus~i~e", "aus~i~en";
        subordinate_subjunctive_i_second: "aus~i~est", "aus~i~et";
        subordinate_subjunctive_i_third: "aus~i~e", referred;
        subordinate_subjunctive_i_i_first: "aus~ä~e", "aus~ä~en";
        subordinate_subjunctive_i_i_second: "aus~ä~est|aus~ä~st", "aus~ä~et|aus~ä~t";
        subordinate_subjunctive_i_i_third: "aus~ä~e", "aus~ä~en";
        imperative: "~i~ aus|~i~e aus", "~i~t aus";
        infinitive: "aus~i~en";
        zu_infinitive: "auszu~i~en";
        participle: "aus~i~end", "ausge~u~en";
    ),
    // Zurückführen: 7 words (irreführen, zusammenführen, zurückführen, herbeiführen, heranführen, weiterführen).
    conjugation!("zurückführen", 7, "~führen";
        present_first: "führe ~", "führen ~";
        present_second: "führst ~", "führt ~";
        present_third: "führt ~", "führen ~";
        preterite_first: "führte ~", "führten ~";
        preterite_second: "führtest ~", "führtet ~";
        preterite_third: "führte ~", "führten ~";
        subjunctive_i_first: "führe ~", "führen ~";
        subjunctive_i_second: "führest ~", "führet ~";
        subjunctive_i_third: "führe ~", referred;
        subjunctive_i_i_first: "führte ~", "führten ~";
        subjunctive_i_i_second: "führtest ~", "führtet ~";
        subjunctive_i_i_third: "führte ~", "führten ~";
        subordinate_present_first: "~führe", "~führen";
        subordinate_present_second: "~führst", "~führt";
        subordinate_present_third: "~führt", "~führen";
        subordinate_preterite_first: "~führte", "~führten";
        subordinate_preterite_second: "~führtest", "~führtet";
        subordinate_preterite_third: "~führte", "~führten";
        subordinate_subjunctive_i_first: "~führe", "~führen";
        subordinate_subjunctive_i_second: "~führest", "~führet";
        subordinate_subjunctive_i_third: "~führe", referred;
        subordinate_subjunctive_i_i_first: "~führte", "~führten";
        subordinate_subjunctive_i_i_second: "~führtest", "~führtet";
        subordinate_subjunctive_i_i_third: "~führte", "~führten";
        imperative: "führ ~", "führt ~";
        infinitive: "~führen";
        zu_infinitive: "~zuführen";
        participle: "~führend", "~geführt";
    ),
    // Vorverlegen: 7 words (vorverzinnen, vorbestellen, vorfinanzieren, vorverlegen, vorprogrammieren, vorformulieren).
    conjugation!("vorverlegen", 7, "vor~en";
        present_first: "~e vor", "~en vor";
        present_second: "~st vor", "~t vor";
        present_third: "~t vor", "~en vor";
        preterite_first: "~te vor", "~ten vor";
        preterite_second: "~test vor", "~tet vor";
        preterite_third: "~te vor", "~ten vor";
        subjunctive_i_first: "~e vor", "~en vor";
        subjunctive_i_second: "~est vor", "~et vor";
        subjunctive_i_third: "~e vor", referred;
        subjunctive_i_i_first: "~te vor", "~ten vor";
        subjunctive_i_i_second: "~test vor", "~tet vor";
        subjunctive_i_i_third: "~te vor", "~ten vor";
        subordinate_present_first: "vor~e", "vor~en";
        subordinate_present_second: "vor~st", "vor~t";
        subordinate_present_third: "vor~t", "vor~en";
        subordinate_preterite_first: "vor~te", "vor~ten";
        subordinate_preterite_second: "vor~test", "vor~tet";
        subordinate_preterite_third: "vor~te", "vor~ten";
        subordinate_subjunctive_i_first: "vor~e", "vor~en";
        subordinate_subjunctive_i_second: "vor~est", "vor~et";
        subordinate_subjunctive_i_third: "vor~e", referred;
        subordinate_subjunctive_i_i_first: "vor~te", "vor~ten";
        subordinate_subjunctive_i_i_second: "vor~test", "vor~tet";
        subordinate_subjunctive_i_i_third: "vor~te", "vor~ten";
        imperative: "~ vor|~e vor", "~t vor";
        infinitive: "vor~en";
        zu_infinitive: "vorzu~en";
        participle: "vor~end", "vor~t";
    ),
    // Verhauen: 7 words (erschaffen, zermahlen, behauen, verhauen, zerhauen, vermahlen).
    conjugation!("verhauen", 7, "~en";
        present_first: "~e", "~en";
        present_second: "~st", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
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
        subordinate_subjunctive_i_third: "~e", referred;
        subordinate_subjunctive_i_i_first: "~te", "~ten";
        subordinate_subjunctive_i_i_second: "~test", "~tet";
        subordinate_subjunctive_i_i_third: "~te", "~ten";
        imperative: "~|~e", "~t";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "~en";
    ),
    // Nachsetzen: 7 words (nachweisen, nachwürzen, nachsitzen, nachreisen, nachgießen, nachsetzen).
    conjugation!("nachsetzen", 7, "nach~en";
        present_first: "~e nach", "~en nach";
        present_second: "~t nach", "~t nach";
        present_third: "~t nach", "~en nach";
        preterite_first: "~te nach", "~ten nach";
        preterite_second: "~test nach", "~tet nach";
        preterite_third: "~te nach", "~ten nach";
        subjunctive_i_first: "~e nach", "~en nach";
        subjunctive_i_second: "~est nach", "~et nach";
        subjunctive_i_third: "~e nach", referred;
        subjunctive_i_i_first: "~te nach", "~ten nach";
        subjunctive_i_i_second: "~test nach", "~tet nach";
        subjunctive_i_i_third: "~te nach", "~ten nach";
        subordinate_present_first: "nach~e", "nach~en";
        subordinate_present_second: "nach~t", "nach~t";
        subordinate_present_third: "nach~t", "nach~en";
        subordinate_preterite_first: "nach~te", "nach~ten";
        subordinate_preterite_second: "nach~test", "nach~tet";
        subordinate_preterite_third: "nach~te", "nach~ten";
        subordinate_subjunctive_i_first: "nach~e", "nach~en";
        subordinate_subjunctive_i_second: "nach~est", "nach~et";
        subordinate_subjunctive_i_third: "nach~e", referred;
        subordinate_subjunctive_i_i_first: "nach~te", "nach~ten";
        subordinate_subjunctive_i_i_second: "nach~test", "nach~tet";
        subordinate_subjunctive_i_i_third: "nach~te", "nach~ten";
        imperative: "~ nach|~e nach", "~t nach";
        infinitive: "nach~en";
        zu_infinitive: "nachzu~en";
        participle: "nach~end", "nachge~t";
    ),
    // Durchleiten: 7 words (durchbluten, durchatmen, durcharbeiten, durchwaten, durchschneiden, durchleiten).
    conjugation!("durchleiten", 7, "durch~en";
        present_first: "~e durch", "~en durch";
        present_second: "~est durch", "~et durch";
        present_third: "~et durch", "~en durch";
        preterite_first: "~ete durch", "~eten durch";
        preterite_second: "~etest durch", "~etet durch";
        preterite_third: "~ete durch", "~eten durch";
        subjunctive_i_first: "~e durch", "~en durch";
        subjunctive_i_second: "~est durch", "~et durch";
        subjunctive_i_third: "~e durch", referred;
        subjunctive_i_i_first: "~ete durch", "~eten durch";
        subjunctive_i_i_second: "~etest durch", "~etet durch";
        subjunctive_i_i_third: "~ete durch", "~eten durch";
        subordinate_present_first: "durch~e", "durch~en";
        subordinate_present_second: "durch~est", "durch~et";
        subordinate_present_third: "durch~et", "durch~en";
        subordinate_preterite_first: "durch~ete", "durch~eten";
        subordinate_preterite_second: "durch~etest", "durch~etet";
        subordinate_preterite_third: "durch~ete", "durch~eten";
        subordinate_subjunctive_i_first: "durch~e", "durch~en";
        subordinate_subjunctive_i_second: "durch~est", "durch~et";
        subordinate_subjunctive_i_third: "durch~e", referred;
        subordinate_subjunctive_i_i_first: "durch~ete", "durch~eten";
        subordinate_subjunctive_i_i_second: "durch~etest", "durch~etet";
        subordinate_subjunctive_i_i_third: "durch~ete", "durch~eten";
        imperative: "~ durch|~e durch", "~et durch";
        infinitive: "durch~en";
        zu_infinitive: "durchzu~en";
        participle: "durch~end", "durchge~et";
    ),
    // Totsagen: 7 words (totquetschen, totmachen, totlachen, totsagen, tottrinken, totkochen).
    conjugation!("totsagen", 7, "tot~en";
        present_first: "~e tot", "~en tot";
        present_second: "~st tot", "~t tot";
        present_third: "~t tot", "~en tot";
        preterite_first: "~te tot", "~ten tot";
        preterite_second: "~test tot", "~tet tot";
        preterite_third: "~te tot", "~ten tot";
        subjunctive_i_first: "~e tot", "~en tot";
        subjunctive_i_second: "~est tot", "~et tot";
        subjunctive_i_third: "~e tot", referred;
        subjunctive_i_i_first: "~te tot", "~ten tot";
        subjunctive_i_i_second: "~test tot", "~tet tot";
        subjunctive_i_i_third: "~te tot", "~ten tot";
        subordinate_present_first: "tot~e", "tot~en";
        subordinate_present_second: "tot~st", "tot~t";
        subordinate_present_third: "tot~t", "tot~en";
        subordinate_preterite_first: "tot~te", "tot~ten";
        subordinate_preterite_second: "tot~test", "tot~tet";
        subordinate_preterite_third: "tot~te", "tot~ten";
        subordinate_subjunctive_i_first: "tot~e", "tot~en";
        subordinate_subjunctive_i_second: "tot~est", "tot~et";
        subordinate_subjunctive_i_third: "tot~e", referred;
        subordinate_subjunctive_i_i_first: "tot~te", "tot~ten";
        subordinate_subjunctive_i_i_second: "tot~test", "tot~tet";
        subordinate_subjunctive_i_i_third: "tot~te", "tot~ten";
        imperative: "~ tot|~e tot", "~t tot";
        infinitive: "tot~en";
        zu_infinitive: "totzu~en";
        participle: "tot~end", "totge~t";
    ),
    // Vollstopfen: 7 words (vollstopfen, vollfüllen, vollmachen, volltanken, vollpumpen, vollquatschen).
    conjugation!("vollstopfen", 7, "voll~en";
        present_first: "~e voll", "~en voll";
        present_second: "~st voll", "~t voll";
        present_third: "~t voll", "~en voll";
        preterite_first: "~te voll", "~ten voll";
        preterite_second: "~test voll", "~tet voll";
        preterite_third: "~te voll", "~ten voll";
        subjunctive_i_first: "~e voll", "~en voll";
        subjunctive_i_second: "~est voll", "~et voll";
        subjunctive_i_third: "~e voll", referred;
        subjunctive_i_i_first: "~te voll", "~ten voll";
        subjunctive_i_i_second: "~test voll", "~tet voll";
        subjunctive_i_i_third: "~te voll", "~ten voll";
        subordinate_present_first: "voll~e", "voll~en";
        subordinate_present_second: "voll~st", "voll~t";
        subordinate_present_third: "voll~t", "voll~en";
        subordinate_preterite_first: "voll~te", "voll~ten";
        subordinate_preterite_second: "voll~test", "voll~tet";
        subordinate_preterite_third: "voll~te", "voll~ten";
        subordinate_subjunctive_i_first: "voll~e", "voll~en";
        subordinate_subjunctive_i_second: "voll~est", "voll~et";
        subordinate_subjunctive_i_third: "voll~e", referred;
        subordinate_subjunctive_i_i_first: "voll~te", "voll~ten";
        subordinate_subjunctive_i_i_second: "voll~test", "voll~tet";
        subordinate_subjunctive_i_i_third: "voll~te", "voll~ten";
        imperative: "~ voll|~e voll", "~t voll";
        infinitive: "voll~en";
        zu_infinitive: "vollzu~en";
        participle: "voll~end", "vollge~t";
    ),
    // Fechten: 7 words (fechten, anfechten, verfechten, verflechten, ausfechten, einflechten).
    conjugation!("fechten", 7, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~et";
        present_third: "~i~", "~e~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~est", "~o~et";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ö~e", "~ö~en";
        subjunctive_i_i_second: "~ö~est", "~ö~et";
        subjunctive_i_i_third: "~ö~e", "~ö~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~et";
        subordinate_present_third: "~i~", "~e~en";
        subordinate_preterite_first: "~o~", "~o~en";
        subordinate_preterite_second: "~o~est", "~o~et";
        subordinate_preterite_third: "~o~", "~o~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est", "~ö~et";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~i~", "~e~et";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~o~en";
    ),
    // Stoßen: 7 words (stoßen, verstoßen, stossen, bestoßen, zerstoßen, zerstossen).
    conjugation!("stoßen", 7, "~o~en";
        present_first: "~o~e", "~o~en";
        present_second: "~ö~t", "~o~t";
        present_third: "~ö~t", "~o~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~est|~ie~t", "~ie~t";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~o~e", "~o~en";
        subjunctive_i_second: "~o~est", "~o~et";
        subjunctive_i_third: "~o~e", referred;
        subjunctive_i_i_first: "~ie~e", "~ie~en";
        subjunctive_i_i_second: "~ie~est|~ie~t", "~ie~et|~ie~t";
        subjunctive_i_i_third: "~ie~e", "~ie~en";
        subordinate_present_first: "~o~e", "~o~en";
        subordinate_present_second: "~ö~t", "~o~t";
        subordinate_present_third: "~ö~t", "~o~en";
        subordinate_preterite_first: "~ie~", "~ie~en";
        subordinate_preterite_second: "~ie~est|~ie~t", "~ie~t";
        subordinate_preterite_third: "~ie~", "~ie~en";
        subordinate_subjunctive_i_first: "~o~e", "~o~en";
        subordinate_subjunctive_i_second: "~o~est", "~o~et";
        subordinate_subjunctive_i_third: "~o~e", referred;
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est|~ie~t", "~ie~et|~ie~t";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~o~|~o~e", "~o~t";
        infinitive: "~o~en";
        zu_infinitive: "abzu~e~n";
        participle: "~o~end", "~o~en";
    ),
    // Laufen: 6 words (laufen, zerlaufen, verlaufen, belaufen, unterlaufen, entlaufen).
    conjugation!("laufen", 6, "~au~en";
        present_first: "~au~e", "~au~en";
        present_second: "~äu~st", "~au~t";
        present_third: "~äu~t", "~au~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~st", "~ie~t";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~au~e", "~au~en";
        subjunctive_i_second: "~au~est", "~au~et";
        subjunctive_i_third: "~au~e", referred;
        subjunctive_i_i_first: "~ie~e", "~ie~en";
        subjunctive_i_i_second: "~ie~est|~ie~st", "~ie~et|~ie~t";
        subjunctive_i_i_third: "~ie~e", "~ie~en";
        subordinate_present_first: "~au~e", "~au~en";
        subordinate_present_second: "~äu~st", "~au~t";
        subordinate_present_third: "~äu~t", "~au~en";
        subordinate_preterite_first: "~ie~", "~ie~en";
        subordinate_preterite_second: "~ie~st", "~ie~t";
        subordinate_preterite_third: "~ie~", "~ie~en";
        subordinate_subjunctive_i_first: "~au~e", "~au~en";
        subordinate_subjunctive_i_second: "~au~est", "~au~et";
        subordinate_subjunctive_i_third: "~au~e", referred;
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est|~ie~st", "~ie~et|~ie~t";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~au~|~au~e", "~au~t";
        infinitive: "~au~en";
        zu_infinitive: "abzu~e~n";
        participle: "~au~end", "~au~en";
    ),
    // Ergeben: 6 words (ergeben, vergeben, umgeben, begeben, übergeben, vermeßen).
    conjugation!("ergeben", 6, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~e~en";
    ),
    // Lossagen: 6 words (vorhersagen, voraussagen, weitersagen, durchsagen, lossagen, zuvorsagen).
    conjugation!("lossagen", 6, "~sagen";
        present_first: "sage ~", "sagen ~";
        present_second: "sagst ~", "sagt ~";
        present_third: "sagt ~", "sagen ~";
        preterite_first: "sagte ~", "sagten ~";
        preterite_second: "sagtest ~", "sagtet ~";
        preterite_third: "sagte ~", "sagten ~";
        subjunctive_i_first: "sage ~", "sagen ~";
        subjunctive_i_second: "sagest ~", "saget ~";
        subjunctive_i_third: "sage ~", referred;
        subjunctive_i_i_first: "sagte ~", "sagten ~";
        subjunctive_i_i_second: "sagtest ~", "sagtet ~";
        subjunctive_i_i_third: "sagte ~", "sagten ~";
        subordinate_present_first: "~sage", "~sagen";
        subordinate_present_second: "~sagst", "~sagt";
        subordinate_present_third: "~sagt", "~sagen";
        subordinate_preterite_first: "~sagte", "~sagten";
        subordinate_preterite_second: "~sagtest", "~sagtet";
        subordinate_preterite_third: "~sagte", "~sagten";
        subordinate_subjunctive_i_first: "~sage", "~sagen";
        subordinate_subjunctive_i_second: "~sagest", "~saget";
        subordinate_subjunctive_i_third: "~sage", referred;
        subordinate_subjunctive_i_i_first: "~sagte", "~sagten";
        subordinate_subjunctive_i_i_second: "~sagtest", "~sagtet";
        subordinate_subjunctive_i_i_third: "~sagte", "~sagten";
        imperative: "sag ~|sage ~", "sagt ~";
        infinitive: "~sagen";
        zu_infinitive: "~zusagen";
        participle: "~sagend", "~gesagt";
    ),
    // Umwandeln: 6 words (umsiedeln, umkrempeln, umwandeln, umändern, umsatteln, umblättern).
    conjugation!("umwandeln", 6, "um~e~n";
        present_first: "~~e um|~e~e um|~e~ um", "~e~n um";
        present_second: "~e~st um", "~e~t um";
        present_third: "~e~t um", "~e~n um";
        preterite_first: "~e~te um", "~e~ten um";
        preterite_second: "~e~test um", "~e~tet um";
        preterite_third: "~e~te um", "~e~ten um";
        subjunctive_i_first: "~e~e um|~~e um", "~e~n um";
        subjunctive_i_second: "~e~est um|~~est um", "~e~et um|~~et um";
        subjunctive_i_third: "~e~e um|~~e um", referred;
        subjunctive_i_i_first: "~e~te um", "~e~ten um";
        subjunctive_i_i_second: "~e~test um", "~e~tet um";
        subjunctive_i_i_third: "~e~te um", "~e~ten um";
        subordinate_present_first: "um~~e|um~e~e|um~e~", "um~e~n";
        subordinate_present_second: "um~e~st", "um~e~t";
        subordinate_present_third: "um~e~t", "um~e~n";
        subordinate_preterite_first: "um~e~te", "um~e~ten";
        subordinate_preterite_second: "um~e~test", "um~e~tet";
        subordinate_preterite_third: "um~e~te", "um~e~ten";
        subordinate_subjunctive_i_first: "um~e~e|um~~e", "um~e~n";
        subordinate_subjunctive_i_second: "um~e~est|um~~est", "um~e~et|um~~et";
        subordinate_subjunctive_i_third: "um~e~e|um~~e", referred;
        subordinate_subjunctive_i_i_first: "um~e~te", "um~e~ten";
        subordinate_subjunctive_i_i_second: "um~e~test", "um~e~tet";
        subordinate_subjunctive_i_i_third: "um~e~te", "um~e~ten";
        imperative: "~~e um|~e~ um|~e~e um", "~e~t um";
        infinitive: "um~e~n";
        zu_infinitive: "umzu~e~n";
        participle: "um~e~nd", "umge~e~t";
    ),
    // Aufspringen: 6 words (aufspringen, auffinden, aufbinden, aufzwingen, aufdringen, aufklingen).
    conjugation!("aufspringen", 6, "auf~i~en";
        present_first: "~i~e auf", "~i~en auf";
        present_second: "~i~st auf", "~i~t auf";
        present_third: "~i~t auf", "~i~en auf";
        preterite_first: "~a~ auf", "~a~en auf";
        preterite_second: "~a~st auf", "~a~t auf";
        preterite_third: "~a~ auf", "~a~en auf";
        subjunctive_i_first: "~i~e auf", "~i~en auf";
        subjunctive_i_second: "~i~est auf", "~i~et auf";
        subjunctive_i_third: "~i~e auf", referred;
        subjunctive_i_i_first: "~ä~e auf", "~ä~en auf";
        subjunctive_i_i_second: "~ä~est auf", "~ä~et auf";
        subjunctive_i_i_third: "~ä~e auf", "~ä~en auf";
        subordinate_present_first: "auf~i~e", "auf~i~en";
        subordinate_present_second: "auf~i~st", "auf~i~t";
        subordinate_present_third: "auf~i~t", "auf~i~en";
        subordinate_preterite_first: "auf~a~", "auf~a~en";
        subordinate_preterite_second: "auf~a~st", "auf~a~t";
        subordinate_preterite_third: "auf~a~", "auf~a~en";
        subordinate_subjunctive_i_first: "auf~i~e", "auf~i~en";
        subordinate_subjunctive_i_second: "auf~i~est", "auf~i~et";
        subordinate_subjunctive_i_third: "auf~i~e", referred;
        subordinate_subjunctive_i_i_first: "auf~ä~e", "auf~ä~en";
        subordinate_subjunctive_i_i_second: "auf~ä~est", "auf~ä~et";
        subordinate_subjunctive_i_i_third: "auf~ä~e", "auf~ä~en";
        imperative: "~i~ auf|~i~e auf", "~i~t auf";
        infinitive: "auf~i~en";
        zu_infinitive: "aufzu~i~en";
        participle: "auf~i~end", "aufge~u~en";
    ),
    // Umgewöhnen: 6 words (umstrukturieren, umverteilen, umkristallisieren, umformulieren, umgewöhnen, umdisponieren).
    conjugation!("umgewöhnen", 6, "um~en";
        present_first: "~e um", "~en um";
        present_second: "~st um", "~t um";
        present_third: "~t um", "~en um";
        preterite_first: "~te um", "~ten um";
        preterite_second: "~test um", "~tet um";
        preterite_third: "~te um", "~ten um";
        subjunctive_i_first: "~e um", "~en um";
        subjunctive_i_second: "~est um", "~et um";
        subjunctive_i_third: "~e um", referred;
        subjunctive_i_i_first: "~te um", "~ten um";
        subjunctive_i_i_second: "~test um", "~tet um";
        subjunctive_i_i_third: "~te um", "~ten um";
        subordinate_present_first: "um~e", "um~en";
        subordinate_present_second: "um~st", "um~t";
        subordinate_present_third: "um~t", "um~en";
        subordinate_preterite_first: "um~te", "um~ten";
        subordinate_preterite_second: "um~test", "um~tet";
        subordinate_preterite_third: "um~te", "um~ten";
        subordinate_subjunctive_i_first: "um~e", "um~en";
        subordinate_subjunctive_i_second: "um~est", "um~et";
        subordinate_subjunctive_i_third: "um~e", referred;
        subordinate_subjunctive_i_i_first: "um~te", "um~ten";
        subordinate_subjunctive_i_i_second: "um~test", "um~tet";
        subordinate_subjunctive_i_i_third: "um~te", "um~ten";
        imperative: "~ um|~e um", "~t um";
        infinitive: "um~en";
        zu_infinitive: "umzu~en";
        participle: "um~end", "um~t";
    ),
    // Aussiedeln: 6 words (ausschildern, ausschütteln, aussondern, ausspeichern, aussiedeln, aussteuern).
    conjugation!("aussiedeln", 6, "au~s~e~n";
        present_first: "~~~e aus|~~e~e aus|~~e~ aus", "~~e~n aus";
        present_second: "~~e~st aus", "~~e~t aus";
        present_third: "~~e~t aus", "~~e~n aus";
        preterite_first: "~~e~te aus", "~~e~ten aus";
        preterite_second: "~~e~test aus", "~~e~tet aus";
        preterite_third: "~~e~te aus", "~~e~ten aus";
        subjunctive_i_first: "~~e~e aus|~~~e aus", "~~e~n aus";
        subjunctive_i_second: "~~e~est aus|~~~est aus", "~~e~et aus|~~~et aus";
        subjunctive_i_third: "~~e~e aus|~~~e aus", referred;
        subjunctive_i_i_first: "~~e~te aus", "~~e~ten aus";
        subjunctive_i_i_second: "~~e~test aus", "~~e~tet aus";
        subjunctive_i_i_third: "~~e~te aus", "~~e~ten aus";
        subordinate_present_first: "au~s~~e|au~s~e~e|au~s~e~", "au~s~e~n";
        subordinate_present_second: "au~s~e~st", "au~s~e~t";
        subordinate_present_third: "au~s~e~t", "au~s~e~n";
        subordinate_preterite_first: "au~s~e~te", "au~s~e~ten";
        subordinate_preterite_second: "au~s~e~test", "au~s~e~tet";
        subordinate_preterite_third: "au~s~e~te", "au~s~e~ten";
        subordinate_subjunctive_i_first: "au~s~e~e|au~s~~e", "au~s~e~n";
        subordinate_subjunctive_i_second: "au~s~e~est|au~s~~est", "au~s~e~et|au~s~~et";
        subordinate_subjunctive_i_third: "au~s~e~e|au~s~~e", referred;
        subordinate_subjunctive_i_i_first: "au~s~e~te", "au~s~e~ten";
        subordinate_subjunctive_i_i_second: "au~s~e~test", "au~s~e~tet";
        subordinate_subjunctive_i_i_third: "au~s~e~te", "au~s~e~ten";
        imperative: "~~~e aus|~~e~ aus|~~e~e aus", "~~e~t aus";
        infinitive: "au~s~e~n";
        zu_infinitive: "au~zus~e~n";
        participle: "au~s~e~nd", "au~ges~e~t";
    ),
    // Abbauen: 6 words (abbauen, abblocken, auseinanderbauen, abblühen, abbrühen, abbrummen).
    conjugation!("abbauen", 6, "a~b~en";
        present_first: "~~e ab", "~~en ab";
        present_second: "~~st ab", "~~t ab";
        present_third: "~~t ab", "~~en ab";
        preterite_first: "~~te ab", "~~ten ab";
        preterite_second: "~~test ab", "~~tet ab";
        preterite_third: "~~te ab", "~~ten ab";
        subjunctive_i_first: "~~e ab", "~~en ab";
        subjunctive_i_second: "~~est ab", "~~et ab";
        subjunctive_i_third: "~~e ab", referred;
        subjunctive_i_i_first: "~~te ab", "~~ten ab";
        subjunctive_i_i_second: "~~test ab", "~~tet ab";
        subjunctive_i_i_third: "~~te ab", "~~ten ab";
        subordinate_present_first: "a~b~e", "a~b~en";
        subordinate_present_second: "a~b~st", "a~b~t";
        subordinate_present_third: "a~b~t", "a~b~en";
        subordinate_preterite_first: "a~b~te", "a~b~ten";
        subordinate_preterite_second: "a~b~test", "a~b~tet";
        subordinate_preterite_third: "a~b~te", "a~b~ten";
        subordinate_subjunctive_i_first: "a~b~e", "a~b~en";
        subordinate_subjunctive_i_second: "a~b~est", "a~b~et";
        subordinate_subjunctive_i_third: "a~b~e", referred;
        subordinate_subjunctive_i_i_first: "a~b~te", "a~b~ten";
        subordinate_subjunctive_i_i_second: "a~b~test", "a~b~tet";
        subordinate_subjunctive_i_i_third: "a~b~te", "a~b~ten";
        imperative: "~~ ab|~~e ab", "~~t ab";
        infinitive: "a~b~en";
        zu_infinitive: "a~zub~en";
        participle: "a~b~end", "a~geb~t";
    ),
    // Einbiegen: 6 words (einfrieren, einschliessen, einschieben, einschiessen, einfliessen, einbiegen).
    conjugation!("einbiegen", 6, "ein~ie~en";
        present_first: "~ie~e ein", "~ie~en ein";
        present_second: "~ie~t ein", "~ie~t ein";
        present_third: "~ie~t ein", "~ie~en ein";
        preterite_first: "~o~ ein", "~o~en ein";
        preterite_second: "~o~est ein", "~o~t ein";
        preterite_third: "~o~ ein", "~o~en ein";
        subjunctive_i_first: "~ie~e ein", "~ie~en ein";
        subjunctive_i_second: "~ie~est ein", "~ie~et ein";
        subjunctive_i_third: "~ie~e ein", referred;
        subjunctive_i_i_first: "~ö~e ein", "~ö~en ein";
        subjunctive_i_i_second: "~ö~est ein", "~ö~et ein";
        subjunctive_i_i_third: "~ö~e ein", "~ö~en ein";
        subordinate_present_first: "ein~ie~e", "ein~ie~en";
        subordinate_present_second: "ein~ie~t", "ein~ie~t";
        subordinate_present_third: "ein~ie~t", "ein~ie~en";
        subordinate_preterite_first: "ein~o~", "ein~o~en";
        subordinate_preterite_second: "ein~o~est", "ein~o~t";
        subordinate_preterite_third: "ein~o~", "ein~o~en";
        subordinate_subjunctive_i_first: "ein~ie~e", "ein~ie~en";
        subordinate_subjunctive_i_second: "ein~ie~est", "ein~ie~et";
        subordinate_subjunctive_i_third: "ein~ie~e", referred;
        subordinate_subjunctive_i_i_first: "ein~ö~e", "ein~ö~en";
        subordinate_subjunctive_i_i_second: "ein~ö~est", "ein~ö~et";
        subordinate_subjunctive_i_i_third: "ein~ö~e", "ein~ö~en";
        imperative: "~ie~ ein", "~ie~t ein";
        infinitive: "ein~ie~en";
        zu_infinitive: "einzu~ie~en";
        participle: "ein~ie~end", "einge~o~en";
    ),
    // Vorsetzen: 6 words (vorheizen, vorweisen, vorsetzen, vorschützen, vorschießen, vorschiessen).
    conjugation!("vorsetzen", 6, "vor~en";
        present_first: "~e vor", "~en vor";
        present_second: "~t vor", "~t vor";
        present_third: "~t vor", "~en vor";
        preterite_first: "~te vor", "~ten vor";
        preterite_second: "~test vor", "~tet vor";
        preterite_third: "~te vor", "~ten vor";
        subjunctive_i_first: "~e vor", "~en vor";
        subjunctive_i_second: "~est vor", "~et vor";
        subjunctive_i_third: "~e vor", referred;
        subjunctive_i_i_first: "~te vor", "~ten vor";
        subjunctive_i_i_second: "~test vor", "~tet vor";
        subjunctive_i_i_third: "~te vor", "~ten vor";
        subordinate_present_first: "vor~e", "vor~en";
        subordinate_present_second: "vor~t", "vor~t";
        subordinate_present_third: "vor~t", "vor~en";
        subordinate_preterite_first: "vor~te", "vor~ten";
        subordinate_preterite_second: "vor~test", "vor~tet";
        subordinate_preterite_third: "vor~te", "vor~ten";
        subordinate_subjunctive_i_first: "vor~e", "vor~en";
        subordinate_subjunctive_i_second: "vor~est", "vor~et";
        subordinate_subjunctive_i_third: "vor~e", referred;
        subordinate_subjunctive_i_i_first: "vor~te", "vor~ten";
        subordinate_subjunctive_i_i_second: "vor~test", "vor~tet";
        subordinate_subjunctive_i_i_third: "vor~te", "vor~ten";
        imperative: "~ vor", "~t vor";
        infinitive: "vor~en";
        zu_infinitive: "vorzu~en";
        participle: "vor~end", "vorge~t";
    ),
    // Anfahren: 6 words (anfangen, anwachsen, anschlagen, antragen, anfahren, ansaufen).
    conjugation!("anfahren", 6, "an~a~en";
        present_first: "~a~e an", "~a~en an";
        present_second: "~ä~st an", "~a~t an";
        present_third: "~ä~t an", "~a~en an";
        preterite_first: "~u~ an", "~u~en an";
        preterite_second: "~u~st an", "~u~t an";
        preterite_third: "~u~ an", "~u~en an";
        subjunctive_i_first: "~a~e an", "~a~en an";
        subjunctive_i_second: "~a~est an", "~a~et an";
        subjunctive_i_third: "~a~e an", referred;
        subjunctive_i_i_first: "~ü~e an", "~ü~en an";
        subjunctive_i_i_second: "~ü~est an", "~ü~et an|~ü~t an";
        subjunctive_i_i_third: "~ü~e an", "~ü~en an";
        subordinate_present_first: "an~a~e", "an~a~en";
        subordinate_present_second: "an~ä~st", "an~a~t";
        subordinate_present_third: "an~ä~t", "an~a~en";
        subordinate_preterite_first: "an~u~", "an~u~en";
        subordinate_preterite_second: "an~u~st", "an~u~t";
        subordinate_preterite_third: "an~u~", "an~u~en";
        subordinate_subjunctive_i_first: "an~a~e", "an~a~en";
        subordinate_subjunctive_i_second: "an~a~est", "an~a~et";
        subordinate_subjunctive_i_third: "an~a~e", referred;
        subordinate_subjunctive_i_i_first: "an~ü~e", "an~ü~en";
        subordinate_subjunctive_i_i_second: "an~ü~est", "an~ü~et|an~ü~t";
        subordinate_subjunctive_i_i_third: "an~ü~e", "an~ü~en";
        imperative: "~a~ an|~a~e an", "~a~t an";
        infinitive: "an~a~en";
        zu_infinitive: "anzu~a~en";
        participle: "an~a~end", "ange~a~en";
    ),
    // Herumgehen: 6 words (hervorgehen, herumgehen, hereingehen, herausgehen, heruntergehen, heraufgehen).
    conjugation!("herumgehen", 6, "~~~gehen";
        present_first: "ge~~ he~", "ge~~n he~";
        present_second: "ge~st h~~", "ge~t h~~";
        present_third: "ge~t h~~", "ge~~n he~";
        preterite_first: "ging ~~~", "gingen ~~~";
        preterite_second: "gingst ~~~", "gingt ~~~";
        preterite_third: "ging ~~~", "gingen ~~~";
        subjunctive_i_first: "ge~~ he~", "ge~~n he~";
        subjunctive_i_second: "ge~~st he~", "ge~~t he~";
        subjunctive_i_third: "ge~~ he~", referred;
        subjunctive_i_i_first: "ginge ~~~", "gingen ~~~";
        subjunctive_i_i_second: "gingest ~~~|gingst ~~~", "ginget ~~~|gingt ~~~";
        subjunctive_i_i_third: "ginge ~~~", "gingen ~~~";
        subordinate_present_first: "~~~gehe", "~~~gehen";
        subordinate_present_second: "~~~gehst", "~~~geht";
        subordinate_present_third: "~~~geht", "~~~gehen";
        subordinate_preterite_first: "~~~ging", "~~~gingen";
        subordinate_preterite_second: "~~~gingst", "~~~gingt";
        subordinate_preterite_third: "~~~ging", "~~~gingen";
        subordinate_subjunctive_i_first: "~~~gehe", "~~~gehen";
        subordinate_subjunctive_i_second: "~~~gehest", "~~~gehet";
        subordinate_subjunctive_i_third: "~~~gehe", referred;
        subordinate_subjunctive_i_i_first: "~~~ginge", "~~~gingen";
        subordinate_subjunctive_i_i_second: "~~~gingest|~~~gingst", "~~~ginget|~~~gingt";
        subordinate_subjunctive_i_i_third: "~~~ginge", "~~~gingen";
        imperative: "ge~ h~~|ge~~ he~", "ge~t h~~";
        infinitive: "~~~gehen";
        zu_infinitive: "~~~zugehen";
        participle: "~~~gehend", "~~~gegangen";
    ),
    // Nacheifern: 6 words (nacheifern, nachtrauern, nachplappern, nachbehandeln, nachwandeln, nachverhandeln).
    conjugation!("nacheifern", 6, "nach~e~n";
        present_first: "~~e nach|~e~e nach|~e~ nach", "~e~n nach";
        present_second: "~e~st nach", "~e~t nach";
        present_third: "~e~t nach", "~e~n nach";
        preterite_first: "~e~te nach", "~e~ten nach";
        preterite_second: "~e~test nach", "~e~tet nach";
        preterite_third: "~e~te nach", "~e~ten nach";
        subjunctive_i_first: "~e~e nach|~~e nach", "~e~n nach";
        subjunctive_i_second: "~e~est nach|~~est nach", "~e~et nach|~~et nach";
        subjunctive_i_third: "~e~e nach|~~e nach", referred;
        subjunctive_i_i_first: "~e~te nach", "~e~ten nach";
        subjunctive_i_i_second: "~e~test nach", "~e~tet nach";
        subjunctive_i_i_third: "~e~te nach", "~e~ten nach";
        subordinate_present_first: "nach~~e|nach~e~e|nach~e~", "nach~e~n";
        subordinate_present_second: "nach~e~st", "nach~e~t";
        subordinate_present_third: "nach~e~t", "nach~e~n";
        subordinate_preterite_first: "nach~e~te", "nach~e~ten";
        subordinate_preterite_second: "nach~e~test", "nach~e~tet";
        subordinate_preterite_third: "nach~e~te", "nach~e~ten";
        subordinate_subjunctive_i_first: "nach~e~e|nach~~e", "nach~e~n";
        subordinate_subjunctive_i_second: "nach~e~est|nach~~est", "nach~e~et|nach~~et";
        subordinate_subjunctive_i_third: "nach~e~e|nach~~e", referred;
        subordinate_subjunctive_i_i_first: "nach~e~te", "nach~e~ten";
        subordinate_subjunctive_i_i_second: "nach~e~test", "nach~e~tet";
        subordinate_subjunctive_i_i_third: "nach~e~te", "nach~e~ten";
        imperative: "~~e nach|~e~ nach|~e~e nach", "~e~t nach";
        infinitive: "nach~e~n";
        zu_infinitive: "nachzu~e~n";
        participle: "nach~e~nd", "nachge~e~t";
    ),
    // Saufen: 6 words (saufen, besaufen, versaufen, ersaufen, totsaufen, schönsaufen).
    conjugation!("saufen", 6, "~au~en";
        present_first: "~au~e", "~au~en";
        present_second: "~äu~st", "~au~t";
        present_third: "~äu~t", "~au~en";
        preterite_first: "~o~f", "~o~fen";
        preterite_second: "~o~fst", "~o~ft";
        preterite_third: "~o~f", "~o~fen";
        subjunctive_i_first: "~au~e", "~au~en";
        subjunctive_i_second: "~au~est", "~au~et";
        subjunctive_i_third: "~au~e", referred;
        subjunctive_i_i_first: "~ö~fe", "~ö~fen";
        subjunctive_i_i_second: "~ö~fest|~ö~fst", "~ö~fet|~ö~ft";
        subjunctive_i_i_third: "~ö~fe", "~ö~fen";
        subordinate_present_first: "~au~e", "~au~en";
        subordinate_present_second: "~äu~st", "~au~t";
        subordinate_present_third: "~äu~t", "~au~en";
        subordinate_preterite_first: "~o~f", "~o~fen";
        subordinate_preterite_second: "~o~fst", "~o~ft";
        subordinate_preterite_third: "~o~f", "~o~fen";
        subordinate_subjunctive_i_first: "~au~e", "~au~en";
        subordinate_subjunctive_i_second: "~au~est", "~au~et";
        subordinate_subjunctive_i_third: "~au~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~fe", "~ö~fen";
        subordinate_subjunctive_i_i_second: "~ö~fest|~ö~fst", "~ö~fet|~ö~ft";
        subordinate_subjunctive_i_i_third: "~ö~fe", "~ö~fen";
        imperative: "~au~|~au~e", "~au~t";
        infinitive: "~au~en";
        zu_infinitive: "abzu~e~n";
        participle: "~au~end", "~o~fen";
    ),
    // Vorankündigen: 6 words (vorantreiben, voranstellen, vorankündigen, voranpreschen, voranschieben, voranmachen).
    conjugation!("vorankündigen", 6, "voran~en";
        present_first: "~e voran", "~en voran";
        present_second: "~st voran", "~t voran";
        present_third: "~t voran", "~en voran";
        preterite_first: "~te voran", "~ten voran";
        preterite_second: "~test voran", "~tet voran";
        preterite_third: "~te voran", "~ten voran";
        subjunctive_i_first: "~e voran", "~en voran";
        subjunctive_i_second: "~est voran", "~et voran";
        subjunctive_i_third: "~e voran", referred;
        subjunctive_i_i_first: "~te voran", "~ten voran";
        subjunctive_i_i_second: "~test voran", "~tet voran";
        subjunctive_i_i_third: "~te voran", "~ten voran";
        subordinate_present_first: "voran~e", "voran~en";
        subordinate_present_second: "voran~st", "voran~t";
        subordinate_present_third: "voran~t", "voran~en";
        subordinate_preterite_first: "voran~te", "voran~ten";
        subordinate_preterite_second: "voran~test", "voran~tet";
        subordinate_preterite_third: "voran~te", "voran~ten";
        subordinate_subjunctive_i_first: "voran~e", "voran~en";
        subordinate_subjunctive_i_second: "voran~est", "voran~et";
        subordinate_subjunctive_i_third: "voran~e", referred;
        subordinate_subjunctive_i_i_first: "voran~te", "voran~ten";
        subordinate_subjunctive_i_i_second: "voran~test", "voran~tet";
        subordinate_subjunctive_i_i_third: "voran~te", "voran~ten";
        imperative: "~ voran", "~t voran";
        infinitive: "voran~en";
        zu_infinitive: "voranzu~en";
        participle: "voran~end", "vorange~t";
    ),
    // Sprechen: 5 words (bergen, sprechen, brechen, stechen, treffen).
    conjugation!("sprechen", 5, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~o~en";
    ),
    // Fahren: 5 words (fahren, waschen, schlagen, graben, tragen).
    conjugation!("fahren", 5, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~t";
        present_third: "~ä~t", "~a~en";
        preterite_first: "~u~", "~u~en";
        preterite_second: "~u~st", "~u~t";
        preterite_third: "~u~", "~u~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
        subjunctive_i_i_first: "~ü~e", "~ü~en";
        subjunctive_i_i_second: "~ü~est|~ü~st", "~ü~et|~ü~t";
        subjunctive_i_i_third: "~ü~e", "~ü~en";
        subordinate_present_first: "~a~e", "~a~en";
        subordinate_present_second: "~ä~st", "~a~t";
        subordinate_present_third: "~ä~t", "~a~en";
        subordinate_preterite_first: "~u~", "~u~en";
        subordinate_preterite_second: "~u~st", "~u~t";
        subordinate_preterite_third: "~u~", "~u~en";
        subordinate_subjunctive_i_first: "~a~e", "~a~en";
        subordinate_subjunctive_i_second: "~a~est", "~a~et";
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est|~ü~st", "~ü~et|~ü~t";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~a~|~a~e", "~a~t";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "ge~a~en";
    ),
    // Werden: 5 words (helfen, werden, werfen, sterben, werben).
    conjugation!("werden", 5, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~i~st", "~e~t";
        present_third: "~i~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ü~e", "~ü~en";
        subjunctive_i_i_second: "~ü~est|~ü~st", "~ü~et|~ü~t";
        subjunctive_i_i_third: "~ü~e", "~ü~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~i~st", "~e~t";
        subordinate_present_third: "~i~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ü~e", "~ü~en";
        subordinate_subjunctive_i_i_second: "~ü~est|~ü~st", "~ü~et|~ü~t";
        subordinate_subjunctive_i_i_third: "~ü~e", "~ü~en";
        imperative: "~i~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~o~en";
    ),
    // Binden: 5 words (finden, binden, winden, schwinden, schinden).
    conjugation!("binden", 5, "~i~en";
        present_first: "~i~e", "~i~en";
        present_second: "~i~est", "~i~et";
        present_third: "~i~et", "~i~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~est|~a~st", "~a~et";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~i~e", "~i~en";
        subjunctive_i_second: "~i~est", "~i~et";
        subjunctive_i_third: "~i~e", referred;
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
        subordinate_subjunctive_i_third: "~i~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est", "~ä~et";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~i~|~i~e", "~i~et";
        infinitive: "~i~en";
        zu_infinitive: "abzu~e~n";
        participle: "~i~end", "ge~u~en";
    ),
    // Abbröckeln: 5 words (abblättern, abbröckeln, abbügeln, abballern, abbaggern).
    conjugation!("abbröckeln", 5, "a~b~e~n";
        present_first: "~~~e ab|~~e~e ab|~~e~ ab", "~~e~n ab";
        present_second: "~~e~st ab", "~~e~t ab";
        present_third: "~~e~t ab", "~~e~n ab";
        preterite_first: "~~e~te ab", "~~e~ten ab";
        preterite_second: "~~e~test ab", "~~e~tet ab";
        preterite_third: "~~e~te ab", "~~e~ten ab";
        subjunctive_i_first: "~~e~e ab|~~~e ab", "~~e~n ab";
        subjunctive_i_second: "~~e~est ab|~~~est ab", "~~e~et ab|~~~et ab";
        subjunctive_i_third: "~~e~e ab|~~~e ab", referred;
        subjunctive_i_i_first: "~~e~te ab", "~~e~ten ab";
        subjunctive_i_i_second: "~~e~test ab", "~~e~tet ab";
        subjunctive_i_i_third: "~~e~te ab", "~~e~ten ab";
        subordinate_present_first: "a~b~~e|a~b~e~e|a~b~e~", "a~b~e~n";
        subordinate_present_second: "a~b~e~st", "a~b~e~t";
        subordinate_present_third: "a~b~e~t", "a~b~e~n";
        subordinate_preterite_first: "a~b~e~te", "a~b~e~ten";
        subordinate_preterite_second: "a~b~e~test", "a~b~e~tet";
        subordinate_preterite_third: "a~b~e~te", "a~b~e~ten";
        subordinate_subjunctive_i_first: "a~b~e~e|a~b~~e", "a~b~e~n";
        subordinate_subjunctive_i_second: "a~b~e~est|a~b~~est", "a~b~e~et|a~b~~et";
        subordinate_subjunctive_i_third: "a~b~e~e|a~b~~e", referred;
        subordinate_subjunctive_i_i_first: "a~b~e~te", "a~b~e~ten";
        subordinate_subjunctive_i_i_second: "a~b~e~test", "a~b~e~tet";
        subordinate_subjunctive_i_i_third: "a~b~e~te", "a~b~e~ten";
        imperative: "~~~e ab|~~e~ ab|~~e~e ab", "~~e~t ab";
        infinitive: "a~b~e~n";
        zu_infinitive: "a~zub~e~n";
        participle: "a~b~e~nd", "a~geb~e~t";
    ),
    // Pflegen: 5 words (bellen, pflegen, weben, bleichen, kneipen).
    conjugation!("pflegen", 5, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~e~te", "~e~ten";
        preterite_second: "~e~test", "~e~tet";
        preterite_third: "~e~te", "~e~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~e~te|~ö~e", "~e~ten|~ö~en";
        subordinate_subjunctive_i_i_second: "~e~test|~ö~est|~ö~st", "~e~tet|~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~e~te|~ö~e", "~e~ten|~ö~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~e~t|ge~o~en";
    ),
    // Schießen: 5 words (fließen, gießen, schließen, schießen, sprießen).
    conjugation!("schießen", 5, "~ießen";
        present_first: "~ieße", "~ießen";
        present_second: "~ießt", "~ießt";
        present_third: "~ießt", "~ießen";
        preterite_first: "~oss", "~ossen";
        preterite_second: "~ossest|~osst", "~osst";
        preterite_third: "~oss", "~ossen";
        subjunctive_i_first: "~ieße", "~ießen";
        subjunctive_i_second: "~ießest", "~ießet";
        subjunctive_i_third: "~ieße", referred;
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
        subordinate_subjunctive_i_third: "~ieße", referred;
        subordinate_subjunctive_i_i_first: "~össe", "~össen";
        subordinate_subjunctive_i_i_second: "~össest|~össt", "~össet|~össt";
        subordinate_subjunctive_i_i_third: "~össe", "~össen";
        imperative: "~ieß|~ieße", "~ießt";
        infinitive: "~ießen";
        zu_infinitive: "abzu~en";
        participle: "~ießend", "ge~ossen";
    ),
    // Ekeln: 5 words (eiern, eitern, ekeln, entern, eifern).
    conjugation!("ekeln", 5, "~~e~n";
        present_first: "~~~e|~~e~e|~~e~", "~~e~n";
        present_second: "~~e~st", "~~e~t";
        present_third: "~~e~t", "~~e~n";
        preterite_first: "~~e~te", "~~e~ten";
        preterite_second: "~~e~test", "~~e~tet";
        preterite_third: "~~e~te", "~~e~ten";
        subjunctive_i_first: "~~e~e|~~~e", "~~e~n";
        subjunctive_i_second: "~~e~est|~~~est", "~~e~et|~~~et";
        subjunctive_i_third: "~~e~e|~~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e~e|~~~e", referred;
        subordinate_subjunctive_i_i_first: "~~e~te", "~~e~ten";
        subordinate_subjunctive_i_i_second: "~~e~test", "~~e~tet";
        subordinate_subjunctive_i_i_third: "~~e~te", "~~e~ten";
        imperative: "~~~e|~~e~|~~e~e", "~~e~t";
        infinitive: "~~e~n";
        zu_infinitive: "~~~zuziehen";
        participle: "~~e~nd", "g~e~e~t";
    ),
    // Anhalten: 5 words (anhalten, anbraten, anbehalten, anraten, anblasen).
    conjugation!("anhalten", 5, "an~a~en";
        present_first: "~a~e an", "~a~en an";
        present_second: "~ä~st an", "~a~et an";
        present_third: "~ä~ an", "~a~en an";
        preterite_first: "~ie~ an", "~ie~en an";
        preterite_second: "~ie~est an|~ie~st an", "~ie~et an";
        preterite_third: "~ie~ an", "~ie~en an";
        subjunctive_i_first: "~a~e an", "~a~en an";
        subjunctive_i_second: "~a~est an", "~a~et an";
        subjunctive_i_third: "~a~e an", referred;
        subjunctive_i_i_first: "~ie~e an", "~ie~en an";
        subjunctive_i_i_second: "~ie~est an", "~ie~et an";
        subjunctive_i_i_third: "~ie~e an", "~ie~en an";
        subordinate_present_first: "an~a~e", "an~a~en";
        subordinate_present_second: "an~ä~st", "an~a~et";
        subordinate_present_third: "an~ä~", "an~a~en";
        subordinate_preterite_first: "an~ie~", "an~ie~en";
        subordinate_preterite_second: "an~ie~est|an~ie~st", "an~ie~et";
        subordinate_preterite_third: "an~ie~", "an~ie~en";
        subordinate_subjunctive_i_first: "an~a~e", "an~a~en";
        subordinate_subjunctive_i_second: "an~a~est", "an~a~et";
        subordinate_subjunctive_i_third: "an~a~e", referred;
        subordinate_subjunctive_i_i_first: "an~ie~e", "an~ie~en";
        subordinate_subjunctive_i_i_second: "an~ie~est", "an~ie~et";
        subordinate_subjunctive_i_i_third: "an~ie~e", "an~ie~en";
        imperative: "~a~ an|~a~e an", "~a~et an";
        infinitive: "an~a~en";
        zu_infinitive: "anzu~a~en";
        participle: "an~a~end", "ange~a~en";
    ),
    // Ausschlachten: 5 words (ausschalten, ausschütten, ausstatten, ausschmieden, ausschlachten).
    conjugation!("ausschlachten", 5, "au~s~en";
        present_first: "~~e aus", "~~en aus";
        present_second: "~~est aus", "~~et aus";
        present_third: "~~et aus", "~~en aus";
        preterite_first: "~~ete aus", "~~eten aus";
        preterite_second: "~~etest aus", "~~etet aus";
        preterite_third: "~~ete aus", "~~eten aus";
        subjunctive_i_first: "~~e aus", "~~en aus";
        subjunctive_i_second: "~~est aus", "~~et aus";
        subjunctive_i_third: "~~e aus", referred;
        subjunctive_i_i_first: "~~ete aus", "~~eten aus";
        subjunctive_i_i_second: "~~etest aus", "~~etet aus";
        subjunctive_i_i_third: "~~ete aus", "~~eten aus";
        subordinate_present_first: "au~s~e", "au~s~en";
        subordinate_present_second: "au~s~est", "au~s~et";
        subordinate_present_third: "au~s~et", "au~s~en";
        subordinate_preterite_first: "au~s~ete", "au~s~eten";
        subordinate_preterite_second: "au~s~etest", "au~s~etet";
        subordinate_preterite_third: "au~s~ete", "au~s~eten";
        subordinate_subjunctive_i_first: "au~s~e", "au~s~en";
        subordinate_subjunctive_i_second: "au~s~est", "au~s~et";
        subordinate_subjunctive_i_third: "au~s~e", referred;
        subordinate_subjunctive_i_i_first: "au~s~ete", "au~s~eten";
        subordinate_subjunctive_i_i_second: "au~s~etest", "au~s~etet";
        subordinate_subjunctive_i_i_third: "au~s~ete", "au~s~eten";
        imperative: "~~ aus|~~e aus", "~~et aus";
        infinitive: "au~s~en";
        zu_infinitive: "au~zus~en";
        participle: "au~s~end", "au~ges~et";
    ),
    // Notlanden: 5 words (notlanden, geten, bauchreden, stoßlüften, stosslüften).
    conjugation!("notlanden", 5, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~est", "~~et";
        present_third: "~~et", "~~en";
        preterite_first: "~~ete", "~~eten";
        preterite_second: "~~etest", "~~etet";
        preterite_third: "~~ete", "~~eten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~ete", "~~eten";
        subordinate_subjunctive_i_i_second: "~~etest", "~~etet";
        subordinate_subjunctive_i_i_third: "~~ete", "~~eten";
        imperative: "~~|~~e", "~~et";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "~ge~et";
    ),
    // Empfehlen: 5 words (befehlen, empfehlen, bestehlen, anbefehlen, verstehlen).
    conjugation!("empfehlen", 5, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~ie~st", "~e~t";
        present_third: "~ie~t", "~e~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subjunctive_i_i_second: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
        subjunctive_i_i_third: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~ie~st", "~e~t";
        subordinate_present_third: "~ie~t", "~e~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e|~ö~e", "~ä~en|~ö~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ä~e|~ö~e", "~ä~en|~ö~en";
        imperative: "~ie~", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~o~en";
    ),
    // Einsteigen: 5 words (einweisen, einschreiben, einsteigen, eintreiben, einreiben).
    conjugation!("einsteigen", 5, "ein~e~~en";
        present_first: "~e~~e ein", "~e~~en ein";
        present_second: "~e~~st ein", "~e~~t ein";
        present_third: "~e~~t ein", "~e~~en ein";
        preterite_first: "~~e~ ein", "~~e~en ein";
        preterite_second: "~~e~st ein", "~~e~t ein";
        preterite_third: "~~e~ ein", "~~e~en ein";
        subjunctive_i_first: "~e~~e ein", "~e~~en ein";
        subjunctive_i_second: "~e~~est ein", "~e~~et ein";
        subjunctive_i_third: "~e~~e ein", referred;
        subjunctive_i_i_first: "~~e~e ein", "~~e~en ein";
        subjunctive_i_i_second: "~~e~est ein|~~e~st ein", "~~e~et ein|~~e~t ein";
        subjunctive_i_i_third: "~~e~e ein", "~~e~en ein";
        subordinate_present_first: "ein~e~~e", "ein~e~~en";
        subordinate_present_second: "ein~e~~st", "ein~e~~t";
        subordinate_present_third: "ein~e~~t", "ein~e~~en";
        subordinate_preterite_first: "ein~~e~", "ein~~e~en";
        subordinate_preterite_second: "ein~~e~st", "ein~~e~t";
        subordinate_preterite_third: "ein~~e~", "ein~~e~en";
        subordinate_subjunctive_i_first: "ein~e~~e", "ein~e~~en";
        subordinate_subjunctive_i_second: "ein~e~~est", "ein~e~~et";
        subordinate_subjunctive_i_third: "ein~e~~e", referred;
        subordinate_subjunctive_i_i_first: "ein~~e~e", "ein~~e~en";
        subordinate_subjunctive_i_i_second: "ein~~e~est|ein~~e~st", "ein~~e~et|ein~~e~t";
        subordinate_subjunctive_i_i_third: "ein~~e~e", "ein~~e~en";
        imperative: "~e~~ ein|~e~~e ein", "~e~~t ein";
        infinitive: "ein~e~~en";
        zu_infinitive: "einzu~e~~en";
        participle: "ein~e~~end", "einge~~e~en";
    ),
    // Hindeuten: 5 words (hinrichten, hinknien, hinarbeiten, hinmorden, hindeuten).
    conjugation!("hindeuten", 5, "hin~en";
        present_first: "~e hin", "~en hin";
        present_second: "~est hin", "~et hin";
        present_third: "~et hin", "~en hin";
        preterite_first: "~ete hin", "~eten hin";
        preterite_second: "~etest hin", "~etet hin";
        preterite_third: "~ete hin", "~eten hin";
        subjunctive_i_first: "~e hin", "~en hin";
        subjunctive_i_second: "~est hin", "~et hin";
        subjunctive_i_third: "~e hin", referred;
        subjunctive_i_i_first: "~ete hin", "~eten hin";
        subjunctive_i_i_second: "~etest hin", "~etet hin";
        subjunctive_i_i_third: "~ete hin", "~eten hin";
        subordinate_present_first: "hin~e", "hin~en";
        subordinate_present_second: "hin~est", "hin~et";
        subordinate_present_third: "hin~et", "hin~en";
        subordinate_preterite_first: "hin~ete", "hin~eten";
        subordinate_preterite_second: "hin~etest", "hin~etet";
        subordinate_preterite_third: "hin~ete", "hin~eten";
        subordinate_subjunctive_i_first: "hin~e", "hin~en";
        subordinate_subjunctive_i_second: "hin~est", "hin~et";
        subordinate_subjunctive_i_third: "hin~e", referred;
        subordinate_subjunctive_i_i_first: "hin~ete", "hin~eten";
        subordinate_subjunctive_i_i_second: "hin~etest", "hin~etet";
        subordinate_subjunctive_i_i_third: "hin~ete", "hin~eten";
        imperative: "~ hin|~e hin", "~et hin";
        infinitive: "hin~en";
        zu_infinitive: "hinzu~en";
        participle: "hin~end", "hinge~et";
    ),
    // Aufzeigen: 5 words (aufzählen, aufzeigen, aufzehren, aufzahlen, aufzwicken).
    conjugation!("aufzeigen", 5, "auf~~en";
        present_first: "~~e auf", "~~en auf";
        present_second: "~~st auf", "~~t auf";
        present_third: "~~t auf", "~~en auf";
        preterite_first: "~~te auf", "~~ten auf";
        preterite_second: "~~test auf", "~~tet auf";
        preterite_third: "~~te auf", "~~ten auf";
        subjunctive_i_first: "~~e auf", "~~en auf";
        subjunctive_i_second: "~~est auf", "~~et auf";
        subjunctive_i_third: "~~e auf", referred;
        subjunctive_i_i_first: "~~te auf", "~~ten auf";
        subjunctive_i_i_second: "~~test auf", "~~tet auf";
        subjunctive_i_i_third: "~~te auf", "~~ten auf";
        subordinate_present_first: "auf~~e", "auf~~en";
        subordinate_present_second: "auf~~st", "auf~~t";
        subordinate_present_third: "auf~~t", "auf~~en";
        subordinate_preterite_first: "auf~~te", "auf~~ten";
        subordinate_preterite_second: "auf~~test", "auf~~tet";
        subordinate_preterite_third: "auf~~te", "auf~~ten";
        subordinate_subjunctive_i_first: "auf~~e", "auf~~en";
        subordinate_subjunctive_i_second: "auf~~est", "auf~~et";
        subordinate_subjunctive_i_third: "auf~~e", referred;
        subordinate_subjunctive_i_i_first: "auf~~te", "auf~~ten";
        subordinate_subjunctive_i_i_second: "auf~~test", "auf~~tet";
        subordinate_subjunctive_i_i_third: "auf~~te", "auf~~ten";
        imperative: "~~ auf|~~e auf", "~~t auf";
        infinitive: "auf~~en";
        zu_infinitive: "auf~uz~en";
        participle: "auf~~end", "aufge~~t";
    ),
    // Hinsetzen: 5 words (hinsetzen, hinweisen, hinreissen, hinreisen, hinstürzen).
    conjugation!("hinsetzen", 5, "hin~en";
        present_first: "~e hin", "~en hin";
        present_second: "~t hin", "~t hin";
        present_third: "~t hin", "~en hin";
        preterite_first: "~te hin", "~ten hin";
        preterite_second: "~test hin", "~tet hin";
        preterite_third: "~te hin", "~ten hin";
        subjunctive_i_first: "~e hin", "~en hin";
        subjunctive_i_second: "~est hin", "~et hin";
        subjunctive_i_third: "~e hin", referred;
        subjunctive_i_i_first: "~te hin", "~ten hin";
        subjunctive_i_i_second: "~test hin", "~tet hin";
        subjunctive_i_i_third: "~te hin", "~ten hin";
        subordinate_present_first: "hin~e", "hin~en";
        subordinate_present_second: "hin~t", "hin~t";
        subordinate_present_third: "hin~t", "hin~en";
        subordinate_preterite_first: "hin~te", "hin~ten";
        subordinate_preterite_second: "hin~test", "hin~tet";
        subordinate_preterite_third: "hin~te", "hin~ten";
        subordinate_subjunctive_i_first: "hin~e", "hin~en";
        subordinate_subjunctive_i_second: "hin~est", "hin~et";
        subordinate_subjunctive_i_third: "hin~e", referred;
        subordinate_subjunctive_i_i_first: "hin~te", "hin~ten";
        subordinate_subjunctive_i_i_second: "hin~test", "hin~tet";
        subordinate_subjunctive_i_i_third: "hin~te", "hin~ten";
        imperative: "~ hin|~e hin", "~t hin";
        infinitive: "hin~en";
        zu_infinitive: "hinzu~en";
        participle: "hin~end", "hinge~t";
    ),
    // Nachtragen: 5 words (nachschlagen, nachladen, nachfahren, nachtragen, nachwachsen).
    conjugation!("nachtragen", 5, "nach~a~en";
        present_first: "~a~e nach", "~a~en nach";
        present_second: "~ä~st nach", "~a~t nach";
        present_third: "~ä~t nach", "~a~en nach";
        preterite_first: "~u~ nach", "~u~en nach";
        preterite_second: "~u~st nach", "~u~t nach";
        preterite_third: "~u~ nach", "~u~en nach";
        subjunctive_i_first: "~a~e nach", "~a~en nach";
        subjunctive_i_second: "~a~est nach", "~a~et nach";
        subjunctive_i_third: "~a~e nach", referred;
        subjunctive_i_i_first: "~ü~e nach", "~ü~en nach";
        subjunctive_i_i_second: "~ü~est nach|~ü~st nach", "~ü~et nach|~ü~t nach";
        subjunctive_i_i_third: "~ü~e nach", "~ü~en nach";
        subordinate_present_first: "nach~a~e", "nach~a~en";
        subordinate_present_second: "nach~ä~st", "nach~a~t";
        subordinate_present_third: "nach~ä~t", "nach~a~en";
        subordinate_preterite_first: "nach~u~", "nach~u~en";
        subordinate_preterite_second: "nach~u~st", "nach~u~t";
        subordinate_preterite_third: "nach~u~", "nach~u~en";
        subordinate_subjunctive_i_first: "nach~a~e", "nach~a~en";
        subordinate_subjunctive_i_second: "nach~a~est", "nach~a~et";
        subordinate_subjunctive_i_third: "nach~a~e", referred;
        subordinate_subjunctive_i_i_first: "nach~ü~e", "nach~ü~en";
        subordinate_subjunctive_i_i_second: "nach~ü~est|nach~ü~st", "nach~ü~et|nach~ü~t";
        subordinate_subjunctive_i_i_third: "nach~ü~e", "nach~ü~en";
        imperative: "~a~ nach|~a~e nach", "~a~t nach";
        infinitive: "nach~a~en";
        zu_infinitive: "nachzu~a~en";
        participle: "nach~a~end", "nachge~a~en";
    ),
    // Aufführen: 5 words (auffüllen, aufflammen, aufführen, auffrischen, aufeinanderfolgen).
    conjugation!("aufführen", 5, "au~f~en";
        present_first: "~~e auf", "~~en auf";
        present_second: "~~st auf", "~~t auf";
        present_third: "~~t auf", "~~en auf";
        preterite_first: "~~te auf", "~~ten auf";
        preterite_second: "~~test auf", "~~tet auf";
        preterite_third: "~~te auf", "~~ten auf";
        subjunctive_i_first: "~~e auf", "~~en auf";
        subjunctive_i_second: "~~est auf", "~~et auf";
        subjunctive_i_third: "~~e auf", referred;
        subjunctive_i_i_first: "~~te auf", "~~ten auf";
        subjunctive_i_i_second: "~~test auf", "~~tet auf";
        subjunctive_i_i_third: "~~te auf", "~~ten auf";
        subordinate_present_first: "au~f~e", "au~f~en";
        subordinate_present_second: "au~f~st", "au~f~t";
        subordinate_present_third: "au~f~t", "au~f~en";
        subordinate_preterite_first: "au~f~te", "au~f~ten";
        subordinate_preterite_second: "au~f~test", "au~f~tet";
        subordinate_preterite_third: "au~f~te", "au~f~ten";
        subordinate_subjunctive_i_first: "au~f~e", "au~f~en";
        subordinate_subjunctive_i_second: "au~f~est", "au~f~et";
        subordinate_subjunctive_i_third: "au~f~e", referred;
        subordinate_subjunctive_i_i_first: "au~f~te", "au~f~ten";
        subordinate_subjunctive_i_i_second: "au~f~test", "au~f~tet";
        subordinate_subjunctive_i_i_third: "au~f~te", "au~f~ten";
        imperative: "~~ auf|~~e auf", "~~t auf";
        infinitive: "au~f~en";
        zu_infinitive: "au~zuf~en";
        participle: "au~f~end", "au~gef~t";
    ),
    // Freimachen: 5 words (freimachen, freipressen, freistellen, freikaufen, freilegen).
    conjugation!("freimachen", 5, "frei~en";
        present_first: "~e frei", "~en frei";
        present_second: "~st frei", "~t frei";
        present_third: "~t frei", "~en frei";
        preterite_first: "~te frei", "~ten frei";
        preterite_second: "~test frei", "~tet frei";
        preterite_third: "~te frei", "~ten frei";
        subjunctive_i_first: "~e frei", "~en frei";
        subjunctive_i_second: "~est frei", "~et frei";
        subjunctive_i_third: "~e frei", referred;
        subjunctive_i_i_first: "~te frei", "~ten frei";
        subjunctive_i_i_second: "~test frei", "~tet frei";
        subjunctive_i_i_third: "~te frei", "~ten frei";
        subordinate_present_first: "frei~e", "frei~en";
        subordinate_present_second: "frei~st", "frei~t";
        subordinate_present_third: "frei~t", "frei~en";
        subordinate_preterite_first: "frei~te", "frei~ten";
        subordinate_preterite_second: "frei~test", "frei~tet";
        subordinate_preterite_third: "frei~te", "frei~ten";
        subordinate_subjunctive_i_first: "frei~e", "frei~en";
        subordinate_subjunctive_i_second: "frei~est", "frei~et";
        subordinate_subjunctive_i_third: "frei~e", referred;
        subordinate_subjunctive_i_i_first: "frei~te", "frei~ten";
        subordinate_subjunctive_i_i_second: "frei~test", "frei~tet";
        subordinate_subjunctive_i_i_third: "frei~te", "frei~ten";
        imperative: "~ frei|~e frei", "~t frei";
        infinitive: "frei~en";
        zu_infinitive: "freizu~en";
        participle: "frei~end", "freige~t";
    ),
    // Vorsingen: 5 words (vordringen, vorfinden, vorsingen, vortrinken, vorspringen).
    conjugation!("vorsingen", 5, "vor~i~en";
        present_first: "~i~e vor", "~i~en vor";
        present_second: "~i~st vor", "~i~t vor";
        present_third: "~i~t vor", "~i~en vor";
        preterite_first: "~a~ vor", "~a~en vor";
        preterite_second: "~a~st vor", "~a~t vor";
        preterite_third: "~a~ vor", "~a~en vor";
        subjunctive_i_first: "~i~e vor", "~i~en vor";
        subjunctive_i_second: "~i~est vor", "~i~et vor";
        subjunctive_i_third: "~i~e vor", referred;
        subjunctive_i_i_first: "~ä~e vor", "~ä~en vor";
        subjunctive_i_i_second: "~ä~est vor|~ä~st vor", "~ä~et vor|~ä~t vor";
        subjunctive_i_i_third: "~ä~e vor", "~ä~en vor";
        subordinate_present_first: "vor~i~e", "vor~i~en";
        subordinate_present_second: "vor~i~st", "vor~i~t";
        subordinate_present_third: "vor~i~t", "vor~i~en";
        subordinate_preterite_first: "vor~a~", "vor~a~en";
        subordinate_preterite_second: "vor~a~st", "vor~a~t";
        subordinate_preterite_third: "vor~a~", "vor~a~en";
        subordinate_subjunctive_i_first: "vor~i~e", "vor~i~en";
        subordinate_subjunctive_i_second: "vor~i~est", "vor~i~et";
        subordinate_subjunctive_i_third: "vor~i~e", referred;
        subordinate_subjunctive_i_i_first: "vor~ä~e", "vor~ä~en";
        subordinate_subjunctive_i_i_second: "vor~ä~est|vor~ä~st", "vor~ä~et|vor~ä~t";
        subordinate_subjunctive_i_i_third: "vor~ä~e", "vor~ä~en";
        imperative: "~i~ vor|~i~e vor", "~i~t vor";
        infinitive: "vor~i~en";
        zu_infinitive: "vorzu~i~en";
        participle: "vor~i~end", "vorge~u~en";
    ),
    // Hochrechnen: 5 words (hochschleudern, hochrechnen, hochkrempeln, hocharbeiten, hochstapeln).
    conjugation!("hochrechnen", 5, "hoch~e~n";
        present_first: "~~e hoch|~e~e hoch|~e~ hoch", "~e~n hoch";
        present_second: "~e~st hoch", "~e~t hoch";
        present_third: "~e~t hoch", "~e~n hoch";
        preterite_first: "~e~te hoch", "~e~ten hoch";
        preterite_second: "~e~test hoch", "~e~tet hoch";
        preterite_third: "~e~te hoch", "~e~ten hoch";
        subjunctive_i_first: "~e~e hoch|~~e hoch", "~e~n hoch";
        subjunctive_i_second: "~e~est hoch|~~est hoch", "~e~et hoch|~~et hoch";
        subjunctive_i_third: "~e~e hoch|~~e hoch", referred;
        subjunctive_i_i_first: "~e~te hoch", "~e~ten hoch";
        subjunctive_i_i_second: "~e~test hoch", "~e~tet hoch";
        subjunctive_i_i_third: "~e~te hoch", "~e~ten hoch";
        subordinate_present_first: "hoch~~e|hoch~e~e|hoch~e~", "hoch~e~n";
        subordinate_present_second: "hoch~e~st", "hoch~e~t";
        subordinate_present_third: "hoch~e~t", "hoch~e~n";
        subordinate_preterite_first: "hoch~e~te", "hoch~e~ten";
        subordinate_preterite_second: "hoch~e~test", "hoch~e~tet";
        subordinate_preterite_third: "hoch~e~te", "hoch~e~ten";
        subordinate_subjunctive_i_first: "hoch~e~e|hoch~~e", "hoch~e~n";
        subordinate_subjunctive_i_second: "hoch~e~est|hoch~~est", "hoch~e~et|hoch~~et";
        subordinate_subjunctive_i_third: "hoch~e~e|hoch~~e", referred;
        subordinate_subjunctive_i_i_first: "hoch~e~te", "hoch~e~ten";
        subordinate_subjunctive_i_i_second: "hoch~e~test", "hoch~e~tet";
        subordinate_subjunctive_i_i_third: "hoch~e~te", "hoch~e~ten";
        imperative: "~~e hoch|~e~ hoch|~e~e hoch", "~e~t hoch";
        infinitive: "hoch~e~n";
        zu_infinitive: "hochzu~e~n";
        participle: "hoch~e~nd", "hochge~e~t";
    ),
    // Umwälzen: 5 words (umsetzen, umwälzen, umstürzen, umpflanzen, umnutzen).
    conjugation!("umwälzen", 5, "um~en";
        present_first: "~e um", "~en um";
        present_second: "~t um", "~t um";
        present_third: "~t um", "~en um";
        preterite_first: "~te um", "~ten um";
        preterite_second: "~test um", "~tet um";
        preterite_third: "~te um", "~ten um";
        subjunctive_i_first: "~e um", "~en um";
        subjunctive_i_second: "~est um", "~et um";
        subjunctive_i_third: "~e um", referred;
        subjunctive_i_i_first: "~te um", "~ten um";
        subjunctive_i_i_second: "~test um", "~tet um";
        subjunctive_i_i_third: "~te um", "~ten um";
        subordinate_present_first: "um~e", "um~en";
        subordinate_present_second: "um~t", "um~t";
        subordinate_present_third: "um~t", "um~en";
        subordinate_preterite_first: "um~te", "um~ten";
        subordinate_preterite_second: "um~test", "um~tet";
        subordinate_preterite_third: "um~te", "um~ten";
        subordinate_subjunctive_i_first: "um~e", "um~en";
        subordinate_subjunctive_i_second: "um~est", "um~et";
        subordinate_subjunctive_i_third: "um~e", referred;
        subordinate_subjunctive_i_i_first: "um~te", "um~ten";
        subordinate_subjunctive_i_i_second: "um~test", "um~tet";
        subordinate_subjunctive_i_i_third: "um~te", "um~ten";
        imperative: "~ um|~e um", "~t um";
        infinitive: "um~en";
        zu_infinitive: "umzu~en";
        participle: "um~end", "umge~t";
    ),
    // Austreiben: 5 words (ausweisen, ausscheiden, ausleihen, ausbleiben, austreiben).
    conjugation!("austreiben", 5, "aus~e~~en";
        present_first: "~e~~e aus", "~e~~en aus";
        present_second: "~e~~st aus", "~e~~t aus";
        present_third: "~e~~t aus", "~e~~en aus";
        preterite_first: "~~e~ aus", "~~e~en aus";
        preterite_second: "~~e~st aus", "~~e~t aus";
        preterite_third: "~~e~ aus", "~~e~en aus";
        subjunctive_i_first: "~e~~e aus", "~e~~en aus";
        subjunctive_i_second: "~e~~est aus", "~e~~et aus";
        subjunctive_i_third: "~e~~e aus", referred;
        subjunctive_i_i_first: "~~e~e aus", "~~e~en aus";
        subjunctive_i_i_second: "~~e~est aus|~~e~st aus", "~~e~et aus|~~e~t aus";
        subjunctive_i_i_third: "~~e~e aus", "~~e~en aus";
        subordinate_present_first: "aus~e~~e", "aus~e~~en";
        subordinate_present_second: "aus~e~~st", "aus~e~~t";
        subordinate_present_third: "aus~e~~t", "aus~e~~en";
        subordinate_preterite_first: "aus~~e~", "aus~~e~en";
        subordinate_preterite_second: "aus~~e~st", "aus~~e~t";
        subordinate_preterite_third: "aus~~e~", "aus~~e~en";
        subordinate_subjunctive_i_first: "aus~e~~e", "aus~e~~en";
        subordinate_subjunctive_i_second: "aus~e~~est", "aus~e~~et";
        subordinate_subjunctive_i_third: "aus~e~~e", referred;
        subordinate_subjunctive_i_i_first: "aus~~e~e", "aus~~e~en";
        subordinate_subjunctive_i_i_second: "aus~~e~est|aus~~e~st", "aus~~e~et|aus~~e~t";
        subordinate_subjunctive_i_i_third: "aus~~e~e", "aus~~e~en";
        imperative: "~e~~ aus|~e~~e aus", "~e~~t aus";
        infinitive: "aus~e~~en";
        zu_infinitive: "auszu~e~~en";
        participle: "aus~e~~end", "ausge~~e~en";
    ),
    // Abrechnen: 5 words (abzeichnen, abrechnen, abtrocknen, abordnen, abspacen).
    conjugation!("abrechnen", 5, "ab~n";
        present_first: "~ ab", "~n ab";
        present_second: "~st ab", "~t ab";
        present_third: "~t ab", "~n ab";
        preterite_first: "~te ab", "~ten ab";
        preterite_second: "~test ab", "~tet ab";
        preterite_third: "~te ab", "~ten ab";
        subjunctive_i_first: "~ ab", "~n ab";
        subjunctive_i_second: "~st ab", "~t ab";
        subjunctive_i_third: "~ ab", referred;
        subjunctive_i_i_first: "~te ab", "~ten ab";
        subjunctive_i_i_second: "~test ab", "~tet ab";
        subjunctive_i_i_third: "~te ab", "~ten ab";
        subordinate_present_first: "ab~", "ab~n";
        subordinate_present_second: "ab~st", "ab~t";
        subordinate_present_third: "ab~t", "ab~n";
        subordinate_preterite_first: "ab~te", "ab~ten";
        subordinate_preterite_second: "ab~test", "ab~tet";
        subordinate_preterite_third: "ab~te", "ab~ten";
        subordinate_subjunctive_i_first: "ab~", "ab~n";
        subordinate_subjunctive_i_second: "ab~st", "ab~t";
        subordinate_subjunctive_i_third: "ab~", referred;
        subordinate_subjunctive_i_i_first: "ab~te", "ab~ten";
        subordinate_subjunctive_i_i_second: "ab~test", "ab~tet";
        subordinate_subjunctive_i_i_third: "ab~te", "ab~ten";
        imperative: "~ ab", "~t ab";
        infinitive: "ab~n";
        zu_infinitive: "abzu~n";
        participle: "ab~nd", "abge~t";
    ),
    // Vorauseilen: 5 words (vorauseilen, davoneilen, herbeieilen, entgegeneilen, vorbeieilen).
    conjugation!("vorauseilen", 5, "~eilen";
        present_first: "eile ~", "eilen ~";
        present_second: "eilst ~", "eilt ~";
        present_third: "eilt ~", "eilen ~";
        preterite_first: "eilte ~", "eilten ~";
        preterite_second: "eiltest ~", "eiltet ~";
        preterite_third: "eilte ~", "eilten ~";
        subjunctive_i_first: "eile ~", "eilen ~";
        subjunctive_i_second: "eilest ~", "eilet ~";
        subjunctive_i_third: "eile ~", referred;
        subjunctive_i_i_first: "eilte ~", "eilten ~";
        subjunctive_i_i_second: "eiltest ~", "eiltet ~";
        subjunctive_i_i_third: "eilte ~", "eilten ~";
        subordinate_present_first: "~eile", "~eilen";
        subordinate_present_second: "~eilst", "~eilt";
        subordinate_present_third: "~eilt", "~eilen";
        subordinate_preterite_first: "~eilte", "~eilten";
        subordinate_preterite_second: "~eiltest", "~eiltet";
        subordinate_preterite_third: "~eilte", "~eilten";
        subordinate_subjunctive_i_first: "~eile", "~eilen";
        subordinate_subjunctive_i_second: "~eilest", "~eilet";
        subordinate_subjunctive_i_third: "~eile", referred;
        subordinate_subjunctive_i_i_first: "~eilte", "~eilten";
        subordinate_subjunctive_i_i_second: "~eiltest", "~eiltet";
        subordinate_subjunctive_i_i_third: "~eilte", "~eilten";
        imperative: "eil ~|eile ~", "eilt ~";
        infinitive: "~eilen";
        zu_infinitive: "~zueilen";
        participle: "~eilend", "~geeilt";
    ),
    // Nacherzählen: 5 words (nachfinanzieren, nachverfolgen, nacherzählen, nachproduzieren, nachbestellen).
    conjugation!("nacherzählen", 5, "nach~en";
        present_first: "~e nach", "~en nach";
        present_second: "~st nach", "~t nach";
        present_third: "~t nach", "~en nach";
        preterite_first: "~te nach", "~ten nach";
        preterite_second: "~test nach", "~tet nach";
        preterite_third: "~te nach", "~ten nach";
        subjunctive_i_first: "~e nach", "~en nach";
        subjunctive_i_second: "~est nach", "~et nach";
        subjunctive_i_third: "~e nach", referred;
        subjunctive_i_i_first: "~te nach", "~ten nach";
        subjunctive_i_i_second: "~test nach", "~tet nach";
        subjunctive_i_i_third: "~te nach", "~ten nach";
        subordinate_present_first: "nach~e", "nach~en";
        subordinate_present_second: "nach~st", "nach~t";
        subordinate_present_third: "nach~t", "nach~en";
        subordinate_preterite_first: "nach~te", "nach~ten";
        subordinate_preterite_second: "nach~test", "nach~tet";
        subordinate_preterite_third: "nach~te", "nach~ten";
        subordinate_subjunctive_i_first: "nach~e", "nach~en";
        subordinate_subjunctive_i_second: "nach~est", "nach~et";
        subordinate_subjunctive_i_third: "nach~e", referred;
        subordinate_subjunctive_i_i_first: "nach~te", "nach~ten";
        subordinate_subjunctive_i_i_second: "nach~test", "nach~tet";
        subordinate_subjunctive_i_i_third: "nach~te", "nach~ten";
        imperative: "~ nach|~e nach", "~t nach";
        infinitive: "nach~en";
        zu_infinitive: "nachzu~en";
        participle: "nach~end", "nach~t";
    ),
    // Emporsteigen: 5 words (emporstreben, emporsteigen, emporblicken, emporheben, emporschnellen).
    conjugation!("emporsteigen", 5, "empor~en";
        present_first: "~e empor", "~en empor";
        present_second: "~st empor", "~t empor";
        present_third: "~t empor", "~en empor";
        preterite_first: "~te empor", "~ten empor";
        preterite_second: "~test empor", "~tet empor";
        preterite_third: "~te empor", "~ten empor";
        subjunctive_i_first: "~e empor", "~en empor";
        subjunctive_i_second: "~est empor", "~et empor";
        subjunctive_i_third: "~e empor", referred;
        subjunctive_i_i_first: "~te empor", "~ten empor";
        subjunctive_i_i_second: "~test empor", "~tet empor";
        subjunctive_i_i_third: "~te empor", "~ten empor";
        subordinate_present_first: "empor~e", "empor~en";
        subordinate_present_second: "empor~st", "empor~t";
        subordinate_present_third: "empor~t", "empor~en";
        subordinate_preterite_first: "empor~te", "empor~ten";
        subordinate_preterite_second: "empor~test", "empor~tet";
        subordinate_preterite_third: "empor~te", "empor~ten";
        subordinate_subjunctive_i_first: "empor~e", "empor~en";
        subordinate_subjunctive_i_second: "empor~est", "empor~et";
        subordinate_subjunctive_i_third: "empor~e", referred;
        subordinate_subjunctive_i_i_first: "empor~te", "empor~ten";
        subordinate_subjunctive_i_i_second: "empor~test", "empor~tet";
        subordinate_subjunctive_i_i_third: "empor~te", "empor~ten";
        imperative: "~ empor|~e empor", "~t empor";
        infinitive: "empor~en";
        zu_infinitive: "emporzu~en";
        participle: "empor~end", "emporge~t";
    ),
    // Antrinken: 5 words (anbinden, antrinken, anspringen, ansinnen, anklingen).
    conjugation!("antrinken", 5, "an~i~en";
        present_first: "~i~e an", "~i~en an";
        present_second: "~i~st an", "~i~t an";
        present_third: "~i~t an", "~i~en an";
        preterite_first: "~a~ an", "~a~en an";
        preterite_second: "~a~st an", "~a~t an";
        preterite_third: "~a~ an", "~a~en an";
        subjunctive_i_first: "~i~e an", "~i~en an";
        subjunctive_i_second: "~i~est an", "~i~et an";
        subjunctive_i_third: "~i~e an", referred;
        subjunctive_i_i_first: "~ä~e an", "~ä~en an";
        subjunctive_i_i_second: "~ä~est an|~ä~st an", "~ä~et an|~ä~t an";
        subjunctive_i_i_third: "~ä~e an", "~ä~en an";
        subordinate_present_first: "an~i~e", "an~i~en";
        subordinate_present_second: "an~i~st", "an~i~t";
        subordinate_present_third: "an~i~t", "an~i~en";
        subordinate_preterite_first: "an~a~", "an~a~en";
        subordinate_preterite_second: "an~a~st", "an~a~t";
        subordinate_preterite_third: "an~a~", "an~a~en";
        subordinate_subjunctive_i_first: "an~i~e", "an~i~en";
        subordinate_subjunctive_i_second: "an~i~est", "an~i~et";
        subordinate_subjunctive_i_third: "an~i~e", referred;
        subordinate_subjunctive_i_i_first: "an~ä~e", "an~ä~en";
        subordinate_subjunctive_i_i_second: "an~ä~est|an~ä~st", "an~ä~et|an~ä~t";
        subordinate_subjunctive_i_i_third: "an~ä~e", "an~ä~en";
        imperative: "~i~ an|~i~e an", "~i~t an";
        infinitive: "an~i~en";
        zu_infinitive: "anzu~i~en";
        participle: "an~i~end", "ange~u~en";
    ),
    // Abheben: 5 words (abheben, abdreschen, abscheren, abmelken, abschwellen).
    conjugation!("abheben", 5, "ab~e~en";
        present_first: "~e~e ab", "~e~en ab";
        present_second: "~i~st ab", "~e~t ab";
        present_third: "~i~t ab", "~e~en ab";
        preterite_first: "~o~ ab", "~o~en ab";
        preterite_second: "~o~st ab", "~o~t ab";
        preterite_third: "~o~ ab", "~o~en ab";
        subjunctive_i_first: "~e~e ab", "~e~en ab";
        subjunctive_i_second: "~e~est ab", "~e~et ab";
        subjunctive_i_third: "~e~e ab", referred;
        subjunctive_i_i_first: "~ö~e ab", "~ö~en ab";
        subjunctive_i_i_second: "~ö~est ab|~ö~st ab", "~ö~et ab|~ö~t ab";
        subjunctive_i_i_third: "~ö~e ab", "~ö~en ab";
        subordinate_present_first: "ab~e~e", "ab~e~en";
        subordinate_present_second: "ab~i~st", "ab~e~t";
        subordinate_present_third: "ab~i~t", "ab~e~en";
        subordinate_preterite_first: "ab~o~", "ab~o~en";
        subordinate_preterite_second: "ab~o~st", "ab~o~t";
        subordinate_preterite_third: "ab~o~", "ab~o~en";
        subordinate_subjunctive_i_first: "ab~e~e", "ab~e~en";
        subordinate_subjunctive_i_second: "ab~e~est", "ab~e~et";
        subordinate_subjunctive_i_third: "ab~e~e", referred;
        subordinate_subjunctive_i_i_first: "ab~ö~e", "ab~ö~en";
        subordinate_subjunctive_i_i_second: "ab~ö~est|ab~ö~st", "ab~ö~et|ab~ö~t";
        subordinate_subjunctive_i_i_third: "ab~ö~e", "ab~ö~en";
        imperative: "~i~ ab", "~e~t ab";
        infinitive: "ab~e~en";
        zu_infinitive: "abzu~e~en";
        participle: "ab~e~end", "abge~o~en";
    ),
    // Vorbilden: 5 words (vorbereiten, vorbeten, vorwalten, vorleisten, vorbilden).
    conjugation!("vorbilden", 5, "vor~en";
        present_first: "~e vor", "~en vor";
        present_second: "~est vor", "~et vor";
        present_third: "~et vor", "~en vor";
        preterite_first: "~ete vor", "~eten vor";
        preterite_second: "~etest vor", "~etet vor";
        preterite_third: "~ete vor", "~eten vor";
        subjunctive_i_first: "~e vor", "~en vor";
        subjunctive_i_second: "~est vor", "~et vor";
        subjunctive_i_third: "~e vor", referred;
        subjunctive_i_i_first: "~ete vor", "~eten vor";
        subjunctive_i_i_second: "~etest vor", "~etet vor";
        subjunctive_i_i_third: "~ete vor", "~eten vor";
        subordinate_present_first: "vor~e", "vor~en";
        subordinate_present_second: "vor~est", "vor~et";
        subordinate_present_third: "vor~et", "vor~en";
        subordinate_preterite_first: "vor~ete", "vor~eten";
        subordinate_preterite_second: "vor~etest", "vor~etet";
        subordinate_preterite_third: "vor~ete", "vor~eten";
        subordinate_subjunctive_i_first: "vor~e", "vor~en";
        subordinate_subjunctive_i_second: "vor~est", "vor~et";
        subordinate_subjunctive_i_third: "vor~e", referred;
        subordinate_subjunctive_i_i_first: "vor~ete", "vor~eten";
        subordinate_subjunctive_i_i_second: "vor~etest", "vor~etet";
        subordinate_subjunctive_i_i_third: "vor~ete", "vor~eten";
        imperative: "~ vor|~e vor", "~et vor";
        infinitive: "vor~en";
        zu_infinitive: "vorzu~en";
        participle: "vor~end", "vorge~et";
    ),
    // Wölken: 5 words (schwanen, knospen, wölken, schwojen, ähnlichschauen).
    conjugation!("wölken", 5, "~en";
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
        zu_infinitive: "sich zu ~en";
        participle: "~end", "ge~t";
    ),
    // Nachempfinden: 5 words (nachempfinden, nachsinnen, nachspringen, nachsingen, nachklingen).
    conjugation!("nachempfinden", 5, "nach~i~en";
        present_first: "~i~e nach", "~i~en nach";
        present_second: "~i~st nach", "~i~t nach";
        present_third: "~i~t nach", "~i~en nach";
        preterite_first: "~a~ nach", "~a~en nach";
        preterite_second: "~a~st nach", "~a~t nach";
        preterite_third: "~a~ nach", "~a~en nach";
        subjunctive_i_first: "~i~e nach", "~i~en nach";
        subjunctive_i_second: "~i~est nach", "~i~et nach";
        subjunctive_i_third: "~i~e nach", referred;
        subjunctive_i_i_first: "~ä~e nach", "~ä~en nach";
        subjunctive_i_i_second: "~ä~est nach|~ä~st nach", "~ä~et nach|~ä~t nach";
        subjunctive_i_i_third: "~ä~e nach", "~ä~en nach";
        subordinate_present_first: "nach~i~e", "nach~i~en";
        subordinate_present_second: "nach~i~st", "nach~i~t";
        subordinate_present_third: "nach~i~t", "nach~i~en";
        subordinate_preterite_first: "nach~a~", "nach~a~en";
        subordinate_preterite_second: "nach~a~st", "nach~a~t";
        subordinate_preterite_third: "nach~a~", "nach~a~en";
        subordinate_subjunctive_i_first: "nach~i~e", "nach~i~en";
        subordinate_subjunctive_i_second: "nach~i~est", "nach~i~et";
        subordinate_subjunctive_i_third: "nach~i~e", referred;
        subordinate_subjunctive_i_i_first: "nach~ä~e", "nach~ä~en";
        subordinate_subjunctive_i_i_second: "nach~ä~est|nach~ä~st", "nach~ä~et|nach~ä~t";
        subordinate_subjunctive_i_i_third: "nach~ä~e", "nach~ä~en";
        imperative: "~i~ nach|~i~e nach", "~i~t nach";
        infinitive: "nach~i~en";
        zu_infinitive: "nachzu~i~en";
        participle: "nach~i~end", "nachge~u~en";
    ),
    // Niederspritzen: 5 words (niederschießen, niederstürzen, niederreissen, niedersetzen, niederspritzen).
    conjugation!("niederspritzen", 5, "nieder~en";
        present_first: "~e nieder", "~en nieder";
        present_second: "~t nieder", "~t nieder";
        present_third: "~t nieder", "~en nieder";
        preterite_first: "~te nieder", "~ten nieder";
        preterite_second: "~test nieder", "~tet nieder";
        preterite_third: "~te nieder", "~ten nieder";
        subjunctive_i_first: "~e nieder", "~en nieder";
        subjunctive_i_second: "~est nieder", "~et nieder";
        subjunctive_i_third: "~e nieder", referred;
        subjunctive_i_i_first: "~te nieder", "~ten nieder";
        subjunctive_i_i_second: "~test nieder", "~tet nieder";
        subjunctive_i_i_third: "~te nieder", "~ten nieder";
        subordinate_present_first: "nieder~e", "nieder~en";
        subordinate_present_second: "nieder~t", "nieder~t";
        subordinate_present_third: "nieder~t", "nieder~en";
        subordinate_preterite_first: "nieder~te", "nieder~ten";
        subordinate_preterite_second: "nieder~test", "nieder~tet";
        subordinate_preterite_third: "nieder~te", "nieder~ten";
        subordinate_subjunctive_i_first: "nieder~e", "nieder~en";
        subordinate_subjunctive_i_second: "nieder~est", "nieder~et";
        subordinate_subjunctive_i_third: "nieder~e", referred;
        subordinate_subjunctive_i_i_first: "nieder~te", "nieder~ten";
        subordinate_subjunctive_i_i_second: "nieder~test", "nieder~tet";
        subordinate_subjunctive_i_i_third: "nieder~te", "nieder~ten";
        imperative: "~ nieder|~e nieder", "~t nieder";
        infinitive: "nieder~en";
        zu_infinitive: "niederzu~en";
        participle: "nieder~end", "niederge~t";
    ),
    // Hineinbewegen: 5 words (hineininterpretieren, hineinoptieren, hineinversetzen, hineinprojizieren, hineinbewegen).
    conjugation!("hineinbewegen", 5, "hinein~en";
        present_first: "~e hinein", "~en hinein";
        present_second: "~st hinein", "~t hinein";
        present_third: "~t hinein", "~en hinein";
        preterite_first: "~te hinein", "~ten hinein";
        preterite_second: "~test hinein", "~tet hinein";
        preterite_third: "~te hinein", "~ten hinein";
        subjunctive_i_first: "~e hinein", "~en hinein";
        subjunctive_i_second: "~est hinein", "~et hinein";
        subjunctive_i_third: "~e hinein", referred;
        subjunctive_i_i_first: "~te hinein", "~ten hinein";
        subjunctive_i_i_second: "~test hinein", "~tet hinein";
        subjunctive_i_i_third: "~te hinein", "~ten hinein";
        subordinate_present_first: "hinein~e", "hinein~en";
        subordinate_present_second: "hinein~st", "hinein~t";
        subordinate_present_third: "hinein~t", "hinein~en";
        subordinate_preterite_first: "hinein~te", "hinein~ten";
        subordinate_preterite_second: "hinein~test", "hinein~tet";
        subordinate_preterite_third: "hinein~te", "hinein~ten";
        subordinate_subjunctive_i_first: "hinein~e", "hinein~en";
        subordinate_subjunctive_i_second: "hinein~est", "hinein~et";
        subordinate_subjunctive_i_third: "hinein~e", referred;
        subordinate_subjunctive_i_i_first: "hinein~te", "hinein~ten";
        subordinate_subjunctive_i_i_second: "hinein~test", "hinein~tet";
        subordinate_subjunctive_i_i_third: "hinein~te", "hinein~ten";
        imperative: "~ hinein|~e hinein", "~t hinein";
        infinitive: "hinein~en";
        zu_infinitive: "hineinzu~en";
        participle: "hinein~end", "hinein~t";
    ),
    // Akklimatisieren: 5 words (geziemen, akklimatisieren, sputen, generalüberholen, versündigen).
    conjugation!("akklimatisieren", 5, "~en";
        present_first: "~e", "~en";
        present_second: "~st", "~t";
        present_third: "~t", "~en";
        preterite_first: "~te", "~ten";
        preterite_second: "~test", "~tet";
        preterite_third: "~te", "~ten";
        subjunctive_i_first: "~e", "~en";
        subjunctive_i_second: "~est", "~et";
        subjunctive_i_third: "~e", referred;
        subjunctive_i_i_first: "~te", "~ten";
        subjunctive_i_i_second: "~test", "~tet";
        subjunctive_i_i_third: "~te", "~ten";
        subordinate_present_first: "mich ~e", "uns ~en";
        subordinate_present_second: "dich ~st", "euch ~t";
        subordinate_present_third: "sich ~t", "sich ~en";
        subordinate_preterite_first: "mich ~te", "uns ~ten";
        subordinate_preterite_second: "dich ~test", "euch ~tet";
        subordinate_preterite_third: "sich ~te", "sich ~ten";
        subordinate_subjunctive_i_first: "mich ~e", "uns ~en";
        subordinate_subjunctive_i_second: "dich ~est", "euch ~et";
        subordinate_subjunctive_i_third: "sich ~e", "sich ~en";
        subordinate_subjunctive_i_i_first: "mich ~te", "uns ~ten";
        subordinate_subjunctive_i_i_second: "dich ~test", "euch ~tet";
        subordinate_subjunctive_i_i_third: "sich ~te", "sich ~ten";
        imperative: "~", "~t";
        infinitive: "~en";
        zu_infinitive: "sich zu ~en";
        participle: "~end", "~t";
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
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~e~te", "~e~ten";
        subordinate_subjunctive_i_i_second: "~e~test", "~e~tet";
        subordinate_subjunctive_i_i_third: "~e~te", "~e~ten";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~a~t";
    ),
    // Greifen: 4 words (pfeifen, greifen, kneifen, schleifen).
    conjugation!("greifen", 4, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~f", "~~fen";
        preterite_second: "~~fst", "~~ft";
        preterite_third: "~~f", "~~fen";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~fe", "~~fen";
        subordinate_subjunctive_i_i_second: "~~fest|~~fst", "~~fet|~~ft";
        subordinate_subjunctive_i_i_third: "~~fe", "~~fen";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~~fen";
    ),
    // Halten: 4 words (braten, halten, raten, rathen).
    conjugation!("halten", 4, "~a~en";
        present_first: "~a~e", "~a~en";
        present_second: "~ä~st", "~a~et";
        present_third: "~ä~", "~a~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~est|~ie~st", "~ie~et";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~a~e", "~a~en";
        subjunctive_i_second: "~a~est", "~a~et";
        subjunctive_i_third: "~a~e", referred;
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
        subordinate_subjunctive_i_third: "~a~e", referred;
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~a~|~a~e", "~a~et";
        infinitive: "~a~en";
        zu_infinitive: "abzu~e~n";
        participle: "~a~end", "ge~a~en";
    ),
    // Abschießen: 4 words (abfließen, abschließen, abschießen, abgießen).
    conjugation!("abschießen", 4, "ab~ießen";
        present_first: "~ieße ab", "~ießen ab";
        present_second: "~ießt ab", "~ießt ab";
        present_third: "~ießt ab", "~ießen ab";
        preterite_first: "~oss ab", "~ossen ab";
        preterite_second: "~ossest ab|~osst ab", "~osst ab";
        preterite_third: "~oss ab", "~ossen ab";
        subjunctive_i_first: "~ieße ab", "~ießen ab";
        subjunctive_i_second: "~ießest ab", "~ießet ab";
        subjunctive_i_third: "~ieße ab", referred;
        subjunctive_i_i_first: "~össe ab", "~össen ab";
        subjunctive_i_i_second: "~össest ab|~össt ab", "~össet ab|~össt ab";
        subjunctive_i_i_third: "~össe ab", "~össen ab";
        subordinate_present_first: "ab~ieße", "ab~ießen";
        subordinate_present_second: "ab~ießt", "ab~ießt";
        subordinate_present_third: "ab~ießt", "ab~ießen";
        subordinate_preterite_first: "ab~oss", "ab~ossen";
        subordinate_preterite_second: "ab~ossest|ab~osst", "ab~osst";
        subordinate_preterite_third: "ab~oss", "ab~ossen";
        subordinate_subjunctive_i_first: "ab~ieße", "ab~ießen";
        subordinate_subjunctive_i_second: "ab~ießest", "ab~ießet";
        subordinate_subjunctive_i_third: "ab~ieße", referred;
        subordinate_subjunctive_i_i_first: "ab~össe", "ab~össen";
        subordinate_subjunctive_i_i_second: "ab~össest|ab~össt", "ab~össet|ab~össt";
        subordinate_subjunctive_i_i_third: "ab~össe", "ab~össen";
        imperative: "~ieß ab|~ieße ab", "~ießt ab";
        infinitive: "ab~ießen";
        zu_infinitive: "abzu~ießen";
        participle: "ab~ießend", "abge~ossen";
    ),
    // Abbiegen: 4 words (abbiegen, abfliegen, abwiegen, abschieben).
    conjugation!("abbiegen", 4, "ab~ie~en";
        present_first: "~ie~e ab", "~ie~en ab";
        present_second: "~ie~st ab", "~ie~t ab";
        present_third: "~ie~t ab", "~ie~en ab";
        preterite_first: "~o~ ab", "~o~en ab";
        preterite_second: "~o~st ab", "~o~t ab";
        preterite_third: "~o~ ab", "~o~en ab";
        subjunctive_i_first: "~ie~e ab", "~ie~en ab";
        subjunctive_i_second: "~ie~est ab", "~ie~et ab";
        subjunctive_i_third: "~ie~e ab", referred;
        subjunctive_i_i_first: "~ö~e ab", "~ö~en ab";
        subjunctive_i_i_second: "~ö~est ab|~ö~st ab", "~ö~et ab|~ö~t ab";
        subjunctive_i_i_third: "~ö~e ab", "~ö~en ab";
        subordinate_present_first: "ab~ie~e", "ab~ie~en";
        subordinate_present_second: "ab~ie~st", "ab~ie~t";
        subordinate_present_third: "ab~ie~t", "ab~ie~en";
        subordinate_preterite_first: "ab~o~", "ab~o~en";
        subordinate_preterite_second: "ab~o~st", "ab~o~t";
        subordinate_preterite_third: "ab~o~", "ab~o~en";
        subordinate_subjunctive_i_first: "ab~ie~e", "ab~ie~en";
        subordinate_subjunctive_i_second: "ab~ie~est", "ab~ie~et";
        subordinate_subjunctive_i_third: "ab~ie~e", referred;
        subordinate_subjunctive_i_i_first: "ab~ö~e", "ab~ö~en";
        subordinate_subjunctive_i_i_second: "ab~ö~est|ab~ö~st", "ab~ö~et|ab~ö~t";
        subordinate_subjunctive_i_i_third: "ab~ö~e", "ab~ö~en";
        imperative: "~ie~ ab|~ie~e ab", "~ie~t ab";
        infinitive: "ab~ie~en";
        zu_infinitive: "abzu~ie~en";
        participle: "ab~ie~end", "abge~o~en";
    ),
    // Giessen: 4 words (schliessen, giessen, fliessen, schiessen).
    conjugation!("giessen", 4, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~t", "~ie~t";
        present_third: "~ie~t", "~ie~en";
        preterite_first: "~o~", "~o~en";
        preterite_second: "~o~est|~o~t", "~o~t";
        preterite_third: "~o~", "~o~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", referred;
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
        subordinate_subjunctive_i_third: "~ie~e", referred;
        subordinate_subjunctive_i_i_first: "~ö~e", "~ö~en";
        subordinate_subjunctive_i_i_second: "~ö~est|~ö~t", "~ö~et|~ö~t";
        subordinate_subjunctive_i_i_third: "~ö~e", "~ö~en";
        imperative: "~ie~|~ie~e", "~ie~t";
        infinitive: "~ie~en";
        zu_infinitive: "abzu~e~n";
        participle: "~ie~end", "ge~o~en";
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
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~e~|~e~e", "~e~et";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~~ten";
    ),
    // Hinausgehen: 4 words (hinausgehen, hinaufgehen, hindurchgehen, hinweggehen).
    conjugation!("hinausgehen", 4, "~~gehen";
        present_first: "ge~e h~", "ge~en h~";
        present_second: "ge~st h~", "ge~t h~";
        present_third: "ge~t h~", "ge~en h~";
        preterite_first: "ging ~~", "gingen ~~";
        preterite_second: "gingst ~~", "gingt ~~";
        preterite_third: "ging ~~", "gingen ~~";
        subjunctive_i_first: "ge~e h~", "ge~en h~";
        subjunctive_i_second: "ge~est h~", "ge~et h~";
        subjunctive_i_third: "ge~e h~", referred;
        subjunctive_i_i_first: "ginge ~~", "gingen ~~";
        subjunctive_i_i_second: "gingest ~~|gingst ~~", "ginget ~~|gingt ~~";
        subjunctive_i_i_third: "ginge ~~", "gingen ~~";
        subordinate_present_first: "~~gehe", "~~gehen";
        subordinate_present_second: "~~gehst", "~~geht";
        subordinate_present_third: "~~geht", "~~gehen";
        subordinate_preterite_first: "~~ging", "~~gingen";
        subordinate_preterite_second: "~~gingst", "~~gingt";
        subordinate_preterite_third: "~~ging", "~~gingen";
        subordinate_subjunctive_i_first: "~~gehe", "~~gehen";
        subordinate_subjunctive_i_second: "~~gehest", "~~gehet";
        subordinate_subjunctive_i_third: "~~gehe", referred;
        subordinate_subjunctive_i_i_first: "~~ginge", "~~gingen";
        subordinate_subjunctive_i_i_second: "~~gingest|~~gingst", "~~ginget|~~gingt";
        subordinate_subjunctive_i_i_third: "~~ginge", "~~gingen";
        imperative: "ge~ h~|ge~e h~", "ge~t h~";
        infinitive: "~~gehen";
        zu_infinitive: "~~zugehen";
        participle: "~~gehend", "~~gegangen";
    ),
    // Schleichen: 4 words (weichen, schleichen, streichen, gleichen).
    conjugation!("schleichen", 4, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~st", "~e~t";
        present_third: "~e~t", "~e~en";
        preterite_first: "~~", "~~en";
        preterite_second: "~~st", "~~t";
        preterite_third: "~~", "~~en";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
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
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~~e", "~~en";
        subordinate_subjunctive_i_i_second: "~~est|~~st", "~~et|~~t";
        subordinate_subjunctive_i_i_third: "~~e", "~~en";
        imperative: "~e~|~e~e", "~e~t";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "ge~~en";
    ),
    // Hinwegfegen: 4 words (hinwegtäuschen, hinwegschauen, hinwegfegen, hinwegraffen).
    conjugation!("hinwegfegen", 4, "hinweg~en";
        present_first: "~e hinweg", "~en hinweg";
        present_second: "~st hinweg", "~t hinweg";
        present_third: "~t hinweg", "~en hinweg";
        preterite_first: "~te hinweg", "~ten hinweg";
        preterite_second: "~test hinweg", "~tet hinweg";
        preterite_third: "~te hinweg", "~ten hinweg";
        subjunctive_i_first: "~e hinweg", "~en hinweg";
        subjunctive_i_second: "~est hinweg", "~et hinweg";
        subjunctive_i_third: "~e hinweg", referred;
        subjunctive_i_i_first: "~te hinweg", "~ten hinweg";
        subjunctive_i_i_second: "~test hinweg", "~tet hinweg";
        subjunctive_i_i_third: "~te hinweg", "~ten hinweg";
        subordinate_present_first: "hinweg~e", "hinweg~en";
        subordinate_present_second: "hinweg~st", "hinweg~t";
        subordinate_present_third: "hinweg~t", "hinweg~en";
        subordinate_preterite_first: "hinweg~te", "hinweg~ten";
        subordinate_preterite_second: "hinweg~test", "hinweg~tet";
        subordinate_preterite_third: "hinweg~te", "hinweg~ten";
        subordinate_subjunctive_i_first: "hinweg~e", "hinweg~en";
        subordinate_subjunctive_i_second: "hinweg~est", "hinweg~et";
        subordinate_subjunctive_i_third: "hinweg~e", referred;
        subordinate_subjunctive_i_i_first: "hinweg~te", "hinweg~ten";
        subordinate_subjunctive_i_i_second: "hinweg~test", "hinweg~tet";
        subordinate_subjunctive_i_i_third: "hinweg~te", "hinweg~ten";
        imperative: "~ hinweg|~e hinweg", "~t hinweg";
        infinitive: "hinweg~en";
        zu_infinitive: "hinwegzu~en";
        participle: "hinweg~end", "hinwegge~t";
    ),
    // Anzeigen: 4 words (anzeigen, anzapfen, anzählen, anzahlen).
    conjugation!("anzeigen", 4, "an~~en";
        present_first: "~~e an", "~~en an";
        present_second: "~~st an", "~~t an";
        present_third: "~~t an", "~~en an";
        preterite_first: "~~te an", "~~ten an";
        preterite_second: "~~test an", "~~tet an";
        preterite_third: "~~te an", "~~ten an";
        subjunctive_i_first: "~~e an", "~~en an";
        subjunctive_i_second: "~~est an", "~~et an";
        subjunctive_i_third: "~~e an", referred;
        subjunctive_i_i_first: "~~te an", "~~ten an";
        subjunctive_i_i_second: "~~test an", "~~tet an";
        subjunctive_i_i_third: "~~te an", "~~ten an";
        subordinate_present_first: "an~~e", "an~~en";
        subordinate_present_second: "an~~st", "an~~t";
        subordinate_present_third: "an~~t", "an~~en";
        subordinate_preterite_first: "an~~te", "an~~ten";
        subordinate_preterite_second: "an~~test", "an~~tet";
        subordinate_preterite_third: "an~~te", "an~~ten";
        subordinate_subjunctive_i_first: "an~~e", "an~~en";
        subordinate_subjunctive_i_second: "an~~est", "an~~et";
        subordinate_subjunctive_i_third: "an~~e", referred;
        subordinate_subjunctive_i_i_first: "an~~te", "an~~ten";
        subordinate_subjunctive_i_i_second: "an~~test", "an~~tet";
        subordinate_subjunctive_i_i_third: "an~~te", "an~~ten";
        imperative: "~~ an|~~e an", "~~t an";
        infinitive: "an~~en";
        zu_infinitive: "an~uz~en";
        participle: "an~~end", "ange~~t";
    ),
    // Vortragen: 4 words (vorschlagen, vortragen, vorfahren, vorladen).
    conjugation!("vortragen", 4, "vor~a~en";
        present_first: "~a~e vor", "~a~en vor";
        present_second: "~ä~st vor", "~a~t vor";
        present_third: "~ä~t vor", "~a~en vor";
        preterite_first: "~u~ vor", "~u~en vor";
        preterite_second: "~u~st vor", "~u~t vor";
        preterite_third: "~u~ vor", "~u~en vor";
        subjunctive_i_first: "~a~e vor", "~a~en vor";
        subjunctive_i_second: "~a~est vor", "~a~et vor";
        subjunctive_i_third: "~a~e vor", referred;
        subjunctive_i_i_first: "~ü~e vor", "~ü~en vor";
        subjunctive_i_i_second: "~ü~est vor|~ü~st vor", "~ü~et vor|~ü~t vor";
        subjunctive_i_i_third: "~ü~e vor", "~ü~en vor";
        subordinate_present_first: "vor~a~e", "vor~a~en";
        subordinate_present_second: "vor~ä~st", "vor~a~t";
        subordinate_present_third: "vor~ä~t", "vor~a~en";
        subordinate_preterite_first: "vor~u~", "vor~u~en";
        subordinate_preterite_second: "vor~u~st", "vor~u~t";
        subordinate_preterite_third: "vor~u~", "vor~u~en";
        subordinate_subjunctive_i_first: "vor~a~e", "vor~a~en";
        subordinate_subjunctive_i_second: "vor~a~est", "vor~a~et";
        subordinate_subjunctive_i_third: "vor~a~e", referred;
        subordinate_subjunctive_i_i_first: "vor~ü~e", "vor~ü~en";
        subordinate_subjunctive_i_i_second: "vor~ü~est|vor~ü~st", "vor~ü~et|vor~ü~t";
        subordinate_subjunctive_i_i_third: "vor~ü~e", "vor~ü~en";
        imperative: "~a~ vor|~a~e vor", "~a~t vor";
        infinitive: "vor~a~en";
        zu_infinitive: "vorzu~a~en";
        participle: "vor~a~end", "vorge~a~en";
    ),
    // Rufen: 4 words (rufen, berufen, widerrufen, verrufen).
    conjugation!("rufen", 4, "~u~en";
        present_first: "~u~e", "~u~en";
        present_second: "~u~st", "~u~t";
        present_third: "~u~t", "~u~en";
        preterite_first: "~ie~", "~ie~en";
        preterite_second: "~ie~st", "~ie~t";
        preterite_third: "~ie~", "~ie~en";
        subjunctive_i_first: "~u~e", "~u~en";
        subjunctive_i_second: "~u~est", "~u~et";
        subjunctive_i_third: "~u~e", referred;
        subjunctive_i_i_first: "~ie~e", "~ie~en";
        subjunctive_i_i_second: "~ie~est|~ie~st", "~ie~et|~ie~t";
        subjunctive_i_i_third: "~ie~e", "~ie~en";
        subordinate_present_first: "~u~e", "~u~en";
        subordinate_present_second: "~u~st", "~u~t";
        subordinate_present_third: "~u~t", "~u~en";
        subordinate_preterite_first: "~ie~", "~ie~en";
        subordinate_preterite_second: "~ie~st", "~ie~t";
        subordinate_preterite_third: "~ie~", "~ie~en";
        subordinate_subjunctive_i_first: "~u~e", "~u~en";
        subordinate_subjunctive_i_second: "~u~est", "~u~et";
        subordinate_subjunctive_i_third: "~u~e", referred;
        subordinate_subjunctive_i_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_i_second: "~ie~est|~ie~st", "~ie~et|~ie~t";
        subordinate_subjunctive_i_i_third: "~ie~e", "~ie~en";
        imperative: "~u~|~u~e", "~u~t";
        infinitive: "~u~en";
        zu_infinitive: "abzu~e~n";
        participle: "~u~end", "~u~en";
    ),
    // Zuweisen: 4 words (zuschreiben, zuweisen, zutreiben, zubleiben).
    conjugation!("zuweisen", 4, "zu~e~~en";
        present_first: "~e~~e zu", "~e~~en zu";
        present_second: "~e~~st zu", "~e~~t zu";
        present_third: "~e~~t zu", "~e~~en zu";
        preterite_first: "~~e~ zu", "~~e~en zu";
        preterite_second: "~~e~st zu", "~~e~t zu";
        preterite_third: "~~e~ zu", "~~e~en zu";
        subjunctive_i_first: "~e~~e zu", "~e~~en zu";
        subjunctive_i_second: "~e~~est zu", "~e~~et zu";
        subjunctive_i_third: "~e~~e zu", referred;
        subjunctive_i_i_first: "~~e~e zu", "~~e~en zu";
        subjunctive_i_i_second: "~~e~est zu", "~~e~et zu|~~e~t zu";
        subjunctive_i_i_third: "~~e~e zu", "~~e~en zu";
        subordinate_present_first: "zu~e~~e", "zu~e~~en";
        subordinate_present_second: "zu~e~~st", "zu~e~~t";
        subordinate_present_third: "zu~e~~t", "zu~e~~en";
        subordinate_preterite_first: "zu~~e~", "zu~~e~en";
        subordinate_preterite_second: "zu~~e~st", "zu~~e~t";
        subordinate_preterite_third: "zu~~e~", "zu~~e~en";
        subordinate_subjunctive_i_first: "zu~e~~e", "zu~e~~en";
        subordinate_subjunctive_i_second: "zu~e~~est", "zu~e~~et";
        subordinate_subjunctive_i_third: "zu~e~~e", referred;
        subordinate_subjunctive_i_i_first: "zu~~e~e", "zu~~e~en";
        subordinate_subjunctive_i_i_second: "zu~~e~est", "zu~~e~et|zu~~e~t";
        subordinate_subjunctive_i_i_third: "zu~~e~e", "zu~~e~en";
        imperative: "~e~~ zu|~e~~e zu", "~e~~t zu";
        infinitive: "zu~e~~en";
        zu_infinitive: "zuzu~e~~en";
        participle: "zu~e~~end", "zuge~~e~en";
    ),
    // Liegen: 4 words (liegen, unterliegen, erliegen, obliegen).
    conjugation!("liegen", 4, "~ie~en";
        present_first: "~ie~e", "~ie~en";
        present_second: "~ie~st", "~ie~t";
        present_third: "~ie~t", "~ie~en";
        preterite_first: "~a~", "~a~en";
        preterite_second: "~a~st", "~a~t";
        preterite_third: "~a~", "~a~en";
        subjunctive_i_first: "~ie~e", "~ie~en";
        subjunctive_i_second: "~ie~est", "~ie~et";
        subjunctive_i_third: "~ie~e", referred;
        subjunctive_i_i_first: "~ä~e", "~ä~en";
        subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subjunctive_i_i_third: "~ä~e", "~ä~en";
        subordinate_present_first: "~ie~e", "~ie~en";
        subordinate_present_second: "~ie~st", "~ie~t";
        subordinate_present_third: "~ie~t", "~ie~en";
        subordinate_preterite_first: "~a~", "~a~en";
        subordinate_preterite_second: "~a~st", "~a~t";
        subordinate_preterite_third: "~a~", "~a~en";
        subordinate_subjunctive_i_first: "~ie~e", "~ie~en";
        subordinate_subjunctive_i_second: "~ie~est", "~ie~et";
        subordinate_subjunctive_i_third: "~ie~e", referred;
        subordinate_subjunctive_i_i_first: "~ä~e", "~ä~en";
        subordinate_subjunctive_i_i_second: "~ä~est|~ä~st", "~ä~et|~ä~t";
        subordinate_subjunctive_i_i_third: "~ä~e", "~ä~en";
        imperative: "~ie~|~ie~e", "~ie~t";
        infinitive: "~ie~en";
        zu_infinitive: "abzu~e~n";
        participle: "~ie~end", "~e~en";
    ),
    // Einordnen: 4 words (einordnen, einatmen, einsegnen, einrechnen).
    conjugation!("einordnen", 4, "ein~n";
        present_first: "~ ein", "~n ein";
        present_second: "~st ein", "~t ein";
        present_third: "~t ein", "~n ein";
        preterite_first: "~te ein", "~ten ein";
        preterite_second: "~test ein", "~tet ein";
        preterite_third: "~te ein", "~ten ein";
        subjunctive_i_first: "~ ein", "~n ein";
        subjunctive_i_second: "~st ein", "~t ein";
        subjunctive_i_third: "~ ein", referred;
        subjunctive_i_i_first: "~te ein", "~ten ein";
        subjunctive_i_i_second: "~test ein", "~tet ein";
        subjunctive_i_i_third: "~te ein", "~ten ein";
        subordinate_present_first: "ein~", "ein~n";
        subordinate_present_second: "ein~st", "ein~t";
        subordinate_present_third: "ein~t", "ein~n";
        subordinate_preterite_first: "ein~te", "ein~ten";
        subordinate_preterite_second: "ein~test", "ein~tet";
        subordinate_preterite_third: "ein~te", "ein~ten";
        subordinate_subjunctive_i_first: "ein~", "ein~n";
        subordinate_subjunctive_i_second: "ein~st", "ein~t";
        subordinate_subjunctive_i_third: "ein~", referred;
        subordinate_subjunctive_i_i_first: "ein~te", "ein~ten";
        subordinate_subjunctive_i_i_second: "ein~test", "ein~tet";
        subordinate_subjunctive_i_i_third: "ein~te", "ein~ten";
        imperative: "~ ein", "~t ein";
        infinitive: "ein~n";
        zu_infinitive: "einzu~n";
        participle: "ein~nd", "einge~t";
    ),
    // Zuschieben: 4 words (zufrieren, zuschieben, zufliegen, zufliessen).
    conjugation!("zuschieben", 4, "zu~ie~en";
        present_first: "~ie~e zu", "~ie~en zu";
        present_second: "~ie~st zu", "~ie~t zu";
        present_third: "~ie~t zu", "~ie~en zu";
        preterite_first: "~o~ zu", "~o~en zu";
        preterite_second: "~o~st zu", "~o~t zu";
        preterite_third: "~o~ zu", "~o~en zu";
        subjunctive_i_first: "~ie~e zu", "~ie~en zu";
        subjunctive_i_second: "~ie~est zu", "~ie~et zu";
        subjunctive_i_third: "~ie~e zu", referred;
        subjunctive_i_i_first: "~ö~e zu", "~ö~en zu";
        subjunctive_i_i_second: "~ö~est zu|~ö~st zu", "~ö~et zu|~ö~t zu";
        subjunctive_i_i_third: "~ö~e zu", "~ö~en zu";
        subordinate_present_first: "zu~ie~e", "zu~ie~en";
        subordinate_present_second: "zu~ie~st", "zu~ie~t";
        subordinate_present_third: "zu~ie~t", "zu~ie~en";
        subordinate_preterite_first: "zu~o~", "zu~o~en";
        subordinate_preterite_second: "zu~o~st", "zu~o~t";
        subordinate_preterite_third: "zu~o~", "zu~o~en";
        subordinate_subjunctive_i_first: "zu~ie~e", "zu~ie~en";
        subordinate_subjunctive_i_second: "zu~ie~est", "zu~ie~et";
        subordinate_subjunctive_i_third: "zu~ie~e", referred;
        subordinate_subjunctive_i_i_first: "zu~ö~e", "zu~ö~en";
        subordinate_subjunctive_i_i_second: "zu~ö~est|zu~ö~st", "zu~ö~et|zu~ö~t";
        subordinate_subjunctive_i_i_third: "zu~ö~e", "zu~ö~en";
        imperative: "~ie~ zu|~ie~e zu", "~ie~t zu";
        infinitive: "zu~ie~en";
        zu_infinitive: "zuzu~ie~en";
        participle: "zu~ie~end", "zuge~o~en";
    ),
    // Durchzappen: 4 words (durchzwängen, durchzählen, durchzwicken, durchzappen).
    conjugation!("durchzappen", 4, "durch~~en";
        present_first: "~~e durch", "~~en durch";
        present_second: "~~st durch", "~~t durch";
        present_third: "~~t durch", "~~en durch";
        preterite_first: "~~te durch", "~~ten durch";
        preterite_second: "~~test durch", "~~tet durch";
        preterite_third: "~~te durch", "~~ten durch";
        subjunctive_i_first: "~~e durch", "~~en durch";
        subjunctive_i_second: "~~est durch", "~~et durch";
        subjunctive_i_third: "~~e durch", referred;
        subjunctive_i_i_first: "~~te durch", "~~ten durch";
        subjunctive_i_i_second: "~~test durch", "~~tet durch";
        subjunctive_i_i_third: "~~te durch", "~~ten durch";
        subordinate_present_first: "durch~~e", "durch~~en";
        subordinate_present_second: "durch~~st", "durch~~t";
        subordinate_present_third: "durch~~t", "durch~~en";
        subordinate_preterite_first: "durch~~te", "durch~~ten";
        subordinate_preterite_second: "durch~~test", "durch~~tet";
        subordinate_preterite_third: "durch~~te", "durch~~ten";
        subordinate_subjunctive_i_first: "durch~~e", "durch~~en";
        subordinate_subjunctive_i_second: "durch~~est", "durch~~et";
        subordinate_subjunctive_i_third: "durch~~e", referred;
        subordinate_subjunctive_i_i_first: "durch~~te", "durch~~ten";
        subordinate_subjunctive_i_i_second: "durch~~test", "durch~~tet";
        subordinate_subjunctive_i_i_third: "durch~~te", "durch~~ten";
        imperative: "~~ durch|~~e durch", "~~t durch";
        infinitive: "durch~~en";
        zu_infinitive: "durch~uz~en";
        participle: "durch~~end", "durchge~~t";
    ),
    // Einschießen: 4 words (einschließen, eingießen, einfließen, einschießen).
    conjugation!("einschießen", 4, "ein~ießen";
        present_first: "~ieße ein", "~ießen ein";
        present_second: "~ießt ein", "~ießt ein";
        present_third: "~ießt ein", "~ießen ein";
        preterite_first: "~oss ein", "~ossen ein";
        preterite_second: "~ossest ein|~osst ein", "~osst ein";
        preterite_third: "~oss ein", "~ossen ein";
        subjunctive_i_first: "~ieße ein", "~ießen ein";
        subjunctive_i_second: "~ießest ein", "~ießet ein";
        subjunctive_i_third: "~ieße ein", referred;
        subjunctive_i_i_first: "~össe ein", "~össen ein";
        subjunctive_i_i_second: "~össest ein|~össt ein", "~össet ein|~össt ein";
        subjunctive_i_i_third: "~össe ein", "~össen ein";
        subordinate_present_first: "ein~ieße", "ein~ießen";
        subordinate_present_second: "ein~ießt", "ein~ießt";
        subordinate_present_third: "ein~ießt", "ein~ießen";
        subordinate_preterite_first: "ein~oss", "ein~ossen";
        subordinate_preterite_second: "ein~ossest|ein~osst", "ein~osst";
        subordinate_preterite_third: "ein~oss", "ein~ossen";
        subordinate_subjunctive_i_first: "ein~ieße", "ein~ießen";
        subordinate_subjunctive_i_second: "ein~ießest", "ein~ießet";
        subordinate_subjunctive_i_third: "ein~ieße", referred;
        subordinate_subjunctive_i_i_first: "ein~össe", "ein~össen";
        subordinate_subjunctive_i_i_second: "ein~össest|ein~össt", "ein~össet|ein~össt";
        subordinate_subjunctive_i_i_third: "ein~össe", "ein~össen";
        imperative: "~ieß ein|~ieße ein", "~ießt ein";
        infinitive: "ein~ießen";
        zu_infinitive: "einzu~ießen";
        participle: "ein~ießend", "einge~ossen";
    ),
    // Dahinleben: 4 words (dahinraffen, dahinleben, dahinsiechen, dahinmähen).
    conjugation!("dahinleben", 4, "dahin~en";
        present_first: "~e dahin", "~en dahin";
        present_second: "~st dahin", "~t dahin";
        present_third: "~t dahin", "~en dahin";
        preterite_first: "~te dahin", "~ten dahin";
        preterite_second: "~test dahin", "~tet dahin";
        preterite_third: "~te dahin", "~ten dahin";
        subjunctive_i_first: "~e dahin", "~en dahin";
        subjunctive_i_second: "~est dahin", "~et dahin";
        subjunctive_i_third: "~e dahin", referred;
        subjunctive_i_i_first: "~te dahin", "~ten dahin";
        subjunctive_i_i_second: "~test dahin", "~tet dahin";
        subjunctive_i_i_third: "~te dahin", "~ten dahin";
        subordinate_present_first: "dahin~e", "dahin~en";
        subordinate_present_second: "dahin~st", "dahin~t";
        subordinate_present_third: "dahin~t", "dahin~en";
        subordinate_preterite_first: "dahin~te", "dahin~ten";
        subordinate_preterite_second: "dahin~test", "dahin~tet";
        subordinate_preterite_third: "dahin~te", "dahin~ten";
        subordinate_subjunctive_i_first: "dahin~e", "dahin~en";
        subordinate_subjunctive_i_second: "dahin~est", "dahin~et";
        subordinate_subjunctive_i_third: "dahin~e", referred;
        subordinate_subjunctive_i_i_first: "dahin~te", "dahin~ten";
        subordinate_subjunctive_i_i_second: "dahin~test", "dahin~tet";
        subordinate_subjunctive_i_i_third: "dahin~te", "dahin~ten";
        imperative: "~ dahin|~e dahin", "~t dahin";
        infinitive: "dahin~en";
        zu_infinitive: "dahinzu~en";
        participle: "dahin~end", "dahinge~t";
    ),
    // Abbüßen: 4 words (abbremsen, abblitzen, abblassen, abbüßen).
    conjugation!("abbüßen", 4, "a~b~en";
        present_first: "~~e ab", "~~en ab";
        present_second: "~~t ab", "~~t ab";
        present_third: "~~t ab", "~~en ab";
        preterite_first: "~~te ab", "~~ten ab";
        preterite_second: "~~test ab", "~~tet ab";
        preterite_third: "~~te ab", "~~ten ab";
        subjunctive_i_first: "~~e ab", "~~en ab";
        subjunctive_i_second: "~~est ab", "~~et ab";
        subjunctive_i_third: "~~e ab", referred;
        subjunctive_i_i_first: "~~te ab", "~~ten ab";
        subjunctive_i_i_second: "~~test ab", "~~tet ab";
        subjunctive_i_i_third: "~~te ab", "~~ten ab";
        subordinate_present_first: "a~b~e", "a~b~en";
        subordinate_present_second: "a~b~t", "a~b~t";
        subordinate_present_third: "a~b~t", "a~b~en";
        subordinate_preterite_first: "a~b~te", "a~b~ten";
        subordinate_preterite_second: "a~b~test", "a~b~tet";
        subordinate_preterite_third: "a~b~te", "a~b~ten";
        subordinate_subjunctive_i_first: "a~b~e", "a~b~en";
        subordinate_subjunctive_i_second: "a~b~est", "a~b~et";
        subordinate_subjunctive_i_third: "a~b~e", referred;
        subordinate_subjunctive_i_i_first: "a~b~te", "a~b~ten";
        subordinate_subjunctive_i_i_second: "a~b~test", "a~b~tet";
        subordinate_subjunctive_i_i_third: "a~b~te", "a~b~ten";
        imperative: "~~ ab|~~e ab", "~~t ab";
        infinitive: "a~b~en";
        zu_infinitive: "a~zub~en";
        participle: "a~b~end", "a~geb~t";
    ),
    // Herausragen: 4 words (hinausragen, herausragen, hervorragen, emporragen).
    conjugation!("herausragen", 4, "~ragen";
        present_first: "rage ~", "ragen ~";
        present_second: "ragst ~", "ragt ~";
        present_third: "ragt ~", "ragen ~";
        preterite_first: "ragte ~", "ragten ~";
        preterite_second: "ragtest ~", "ragtet ~";
        preterite_third: "ragte ~", "ragten ~";
        subjunctive_i_first: "rage ~", "ragen ~";
        subjunctive_i_second: "ragest ~", "raget ~";
        subjunctive_i_third: "rage ~", referred;
        subjunctive_i_i_first: "ragte ~", "ragten ~";
        subjunctive_i_i_second: "ragtest ~", "ragtet ~";
        subjunctive_i_i_third: "ragte ~", "ragten ~";
        subordinate_present_first: "~rage", "~ragen";
        subordinate_present_second: "~ragst", "~ragt";
        subordinate_present_third: "~ragt", "~ragen";
        subordinate_preterite_first: "~ragte", "~ragten";
        subordinate_preterite_second: "~ragtest", "~ragtet";
        subordinate_preterite_third: "~ragte", "~ragten";
        subordinate_subjunctive_i_first: "~rage", "~ragen";
        subordinate_subjunctive_i_second: "~ragest", "~raget";
        subordinate_subjunctive_i_third: "~rage", referred;
        subordinate_subjunctive_i_i_first: "~ragte", "~ragten";
        subordinate_subjunctive_i_i_second: "~ragtest", "~ragtet";
        subordinate_subjunctive_i_i_third: "~ragte", "~ragten";
        imperative: "rag ~|rage ~", "ragt ~";
        infinitive: "~ragen";
        zu_infinitive: "~zuragen";
        participle: "~ragend", "~geragt";
    ),
    // Klarmachen: 4 words (klarstellen, klarmachen, klarspülen, klarträumen).
    conjugation!("klarmachen", 4, "klar~en";
        present_first: "~e klar", "~en klar";
        present_second: "~st klar", "~t klar";
        present_third: "~t klar", "~en klar";
        preterite_first: "~te klar", "~ten klar";
        preterite_second: "~test klar", "~tet klar";
        preterite_third: "~te klar", "~ten klar";
        subjunctive_i_first: "~e klar", "~en klar";
        subjunctive_i_second: "~est klar", "~et klar";
        subjunctive_i_third: "~e klar", referred;
        subjunctive_i_i_first: "~te klar", "~ten klar";
        subjunctive_i_i_second: "~test klar", "~tet klar";
        subjunctive_i_i_third: "~te klar", "~ten klar";
        subordinate_present_first: "klar~e", "klar~en";
        subordinate_present_second: "klar~st", "klar~t";
        subordinate_present_third: "klar~t", "klar~en";
        subordinate_preterite_first: "klar~te", "klar~ten";
        subordinate_preterite_second: "klar~test", "klar~tet";
        subordinate_preterite_third: "klar~te", "klar~ten";
        subordinate_subjunctive_i_first: "klar~e", "klar~en";
        subordinate_subjunctive_i_second: "klar~est", "klar~et";
        subordinate_subjunctive_i_third: "klar~e", referred;
        subordinate_subjunctive_i_i_first: "klar~te", "klar~ten";
        subordinate_subjunctive_i_i_second: "klar~test", "klar~tet";
        subordinate_subjunctive_i_i_third: "klar~te", "klar~ten";
        imperative: "~ klar|~e klar", "~t klar";
        infinitive: "klar~en";
        zu_infinitive: "klarzu~en";
        participle: "klar~end", "klarge~t";
    ),
    // Herunterdrücken: 4 words (draufdrücken, zusammendrücken, hinunterdrücken, herunterdrücken).
    conjugation!("herunterdrücken", 4, "~drücken";
        present_first: "drücke ~", "drücken ~";
        present_second: "drückst ~", "drückt ~";
        present_third: "drückt ~", "drücken ~";
        preterite_first: "drückte ~", "drückten ~";
        preterite_second: "drücktest ~", "drücktet ~";
        preterite_third: "drückte ~", "drückten ~";
        subjunctive_i_first: "drücke ~", "drücken ~";
        subjunctive_i_second: "drückest ~", "drücket ~";
        subjunctive_i_third: "drücke ~", referred;
        subjunctive_i_i_first: "drückte ~", "drückten ~";
        subjunctive_i_i_second: "drücktest ~", "drücktet ~";
        subjunctive_i_i_third: "drückte ~", "drückten ~";
        subordinate_present_first: "~drücke", "~drücken";
        subordinate_present_second: "~drückst", "~drückt";
        subordinate_present_third: "~drückt", "~drücken";
        subordinate_preterite_first: "~drückte", "~drückten";
        subordinate_preterite_second: "~drücktest", "~drücktet";
        subordinate_preterite_third: "~drückte", "~drückten";
        subordinate_subjunctive_i_first: "~drücke", "~drücken";
        subordinate_subjunctive_i_second: "~drückest", "~drücket";
        subordinate_subjunctive_i_third: "~drücke", referred;
        subordinate_subjunctive_i_i_first: "~drückte", "~drückten";
        subordinate_subjunctive_i_i_second: "~drücktest", "~drücktet";
        subordinate_subjunctive_i_i_third: "~drückte", "~drückten";
        imperative: "drück ~|drücke ~", "drückt ~";
        infinitive: "~drücken";
        zu_infinitive: "~zudrücken";
        participle: "~drückend", "~gedrückt";
    ),
    // Miterleben: 4 words (mitfinanzieren, miterleben, mitdiskutieren, mitbestimmen).
    conjugation!("miterleben", 4, "mit~en";
        present_first: "~e mit", "~en mit";
        present_second: "~st mit", "~t mit";
        present_third: "~t mit", "~en mit";
        preterite_first: "~te mit", "~ten mit";
        preterite_second: "~test mit", "~tet mit";
        preterite_third: "~te mit", "~ten mit";
        subjunctive_i_first: "~e mit", "~en mit";
        subjunctive_i_second: "~est mit", "~et mit";
        subjunctive_i_third: "~e mit", referred;
        subjunctive_i_i_first: "~te mit", "~ten mit";
        subjunctive_i_i_second: "~test mit", "~tet mit";
        subjunctive_i_i_third: "~te mit", "~ten mit";
        subordinate_present_first: "mit~e", "mit~en";
        subordinate_present_second: "mit~st", "mit~t";
        subordinate_present_third: "mit~t", "mit~en";
        subordinate_preterite_first: "mit~te", "mit~ten";
        subordinate_preterite_second: "mit~test", "mit~tet";
        subordinate_preterite_third: "mit~te", "mit~ten";
        subordinate_subjunctive_i_first: "mit~e", "mit~en";
        subordinate_subjunctive_i_second: "mit~est", "mit~et";
        subordinate_subjunctive_i_third: "mit~e", referred;
        subordinate_subjunctive_i_i_first: "mit~te", "mit~ten";
        subordinate_subjunctive_i_i_second: "mit~test", "mit~tet";
        subordinate_subjunctive_i_i_third: "mit~te", "mit~ten";
        imperative: "~ mit|~e mit", "~t mit";
        infinitive: "mit~en";
        zu_infinitive: "mitzu~en";
        participle: "mit~end", "mit~t";
    ),
    // Anreißen: 4 words (anbeißen, anschmeißen, anreißen, anscheißen).
    conjugation!("anreißen", 4, "an~e~ßen";
        present_first: "~e~ße an", "~e~ßen an";
        present_second: "~e~ßt an", "~e~ßt an";
        present_third: "~e~ßt an", "~e~ßen an";
        preterite_first: "~~ss an", "~~ssen an";
        preterite_second: "~~ssest an|~~sst an", "~~sst an";
        preterite_third: "~~ss an", "~~ssen an";
        subjunctive_i_first: "~e~ße an", "~e~ßen an";
        subjunctive_i_second: "~e~ßest an", "~e~ßet an";
        subjunctive_i_third: "~e~ße an", referred;
        subjunctive_i_i_first: "~~sse an", "~~ssen an";
        subjunctive_i_i_second: "~~ssest an|~~sst an", "~~sset an|~~sst an";
        subjunctive_i_i_third: "~~sse an", "~~ssen an";
        subordinate_present_first: "an~e~ße", "an~e~ßen";
        subordinate_present_second: "an~e~ßt", "an~e~ßt";
        subordinate_present_third: "an~e~ßt", "an~e~ßen";
        subordinate_preterite_first: "an~~ss", "an~~ssen";
        subordinate_preterite_second: "an~~ssest|an~~sst", "an~~sst";
        subordinate_preterite_third: "an~~ss", "an~~ssen";
        subordinate_subjunctive_i_first: "an~e~ße", "an~e~ßen";
        subordinate_subjunctive_i_second: "an~e~ßest", "an~e~ßet";
        subordinate_subjunctive_i_third: "an~e~ße", referred;
        subordinate_subjunctive_i_i_first: "an~~sse", "an~~ssen";
        subordinate_subjunctive_i_i_second: "an~~ssest|an~~sst", "an~~sset|an~~sst";
        subordinate_subjunctive_i_i_third: "an~~sse", "an~~ssen";
        imperative: "~e~ß an|~e~ße an", "~e~ßt an";
        infinitive: "an~e~ßen";
        zu_infinitive: "anzu~e~ßen";
        participle: "an~e~ßend", "ange~~ssen";
    ),
    // Besagen: 4 words (gereuen, bedünken, besagen, bewölken).
    conjugation!("besagen", 4, "~en";
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
        zu_infinitive: "sich zu ~en";
        participle: "~end", "~t";
    ),
    // Exen: 3 words (eisen, exen, erzen).
    conjugation!("exen", 3, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~t", "~~t";
        present_third: "~~t", "~~en";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~|~~e", "~~t";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "g~e~t";
    ),
    // Zusammenkehren: 3 words (zurückkehren, zusammenkehren, wiederkehren).
    conjugation!("zusammenkehren", 3, "~kehren";
        present_first: "kehre ~", "kehren ~";
        present_second: "kehrst ~", "kehrt ~";
        present_third: "kehrt ~", "kehren ~";
        preterite_first: "kehrte ~", "kehrten ~";
        preterite_second: "kehrtest ~", "kehrtet ~";
        preterite_third: "kehrte ~", "kehrten ~";
        subjunctive_i_first: "kehre ~", "kehren ~";
        subjunctive_i_second: "kehrest ~", "kehret ~";
        subjunctive_i_third: "kehre ~", referred;
        subjunctive_i_i_first: "kehrte ~", "kehrten ~";
        subjunctive_i_i_second: "kehrtest ~", "kehrtet ~";
        subjunctive_i_i_third: "kehrte ~", "kehrten ~";
        subordinate_present_first: "~kehre", "~kehren";
        subordinate_present_second: "~kehrst", "~kehrt";
        subordinate_present_third: "~kehrt", "~kehren";
        subordinate_preterite_first: "~kehrte", "~kehrten";
        subordinate_preterite_second: "~kehrtest", "~kehrtet";
        subordinate_preterite_third: "~kehrte", "~kehrten";
        subordinate_subjunctive_i_first: "~kehre", "~kehren";
        subordinate_subjunctive_i_second: "~kehrest", "~kehret";
        subordinate_subjunctive_i_third: "~kehre", referred;
        subordinate_subjunctive_i_i_first: "~kehrte", "~kehrten";
        subordinate_subjunctive_i_i_second: "~kehrtest", "~kehrtet";
        subordinate_subjunctive_i_i_third: "~kehrte", "~kehrten";
        imperative: "kehr ~|kehre ~", "kehrt ~";
        infinitive: "~kehren";
        zu_infinitive: "~zukehren";
        participle: "~kehrend", "~gekehrt";
    ),
    // Erden: 3 words (erden, ernten, enden).
    conjugation!("erden", 3, "~~en";
        present_first: "~~e", "~~en";
        present_second: "~~est", "~~et";
        present_third: "~~et", "~~en";
        preterite_first: "~~ete", "~~eten";
        preterite_second: "~~etest", "~~etet";
        preterite_third: "~~ete", "~~eten";
        subjunctive_i_first: "~~e", "~~en";
        subjunctive_i_second: "~~est", "~~et";
        subjunctive_i_third: "~~e", referred;
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
        subordinate_subjunctive_i_third: "~~e", referred;
        subordinate_subjunctive_i_i_first: "~~ete", "~~eten";
        subordinate_subjunctive_i_i_second: "~~etest", "~~etet";
        subordinate_subjunctive_i_i_third: "~~ete", "~~eten";
        imperative: "~~|~~e", "~~et";
        infinitive: "~~en";
        zu_infinitive: "abzu~e~n";
        participle: "~~end", "g~e~et";
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
        subjunctive_i_third: "mache ~~", referred;
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
        subordinate_subjunctive_i_third: "~~mache", referred;
        subordinate_subjunctive_i_i_first: "~~machte", "~~machten";
        subordinate_subjunctive_i_i_second: "~~machtest", "~~machtet";
        subordinate_subjunctive_i_i_third: "~~machte", "~~machten";
        imperative: "mach ~~|mache ~~", "macht ~~";
        infinitive: "~~machen";
        zu_infinitive: "~~zumachen";
        participle: "~~machend", "~~gemacht";
    ),
    // Verwenden: 3 words (verwenden, bewenden, übersenden).
    conjugation!("verwenden", 3, "~e~en";
        present_first: "~e~e", "~e~en";
        present_second: "~e~est", "~e~et";
        present_third: "~e~et", "~e~en";
        preterite_first: "~e~ete|~a~te", "~e~eten|~a~ten";
        preterite_second: "~e~etest|~a~test", "~e~etet|~a~tet";
        preterite_third: "~e~ete|~a~te", "~e~eten|~a~ten";
        subjunctive_i_first: "~e~e", "~e~en";
        subjunctive_i_second: "~e~est", "~e~et";
        subjunctive_i_third: "~e~e", referred;
        subjunctive_i_i_first: "~e~ete|~a~te", "~e~eten|~a~ten";
        subjunctive_i_i_second: "~e~etest|~a~test", "~e~etet|~a~tet";
        subjunctive_i_i_third: "~e~ete|~a~te", "~e~eten|~a~ten";
        subordinate_present_first: "~e~e", "~e~en";
        subordinate_present_second: "~e~est", "~e~et";
        subordinate_present_third: "~e~et", "~e~en";
        subordinate_preterite_first: "~e~ete|~a~te", "~e~eten|~a~ten";
        subordinate_preterite_second: "~e~etest|~a~test", "~e~etet|~a~tet";
        subordinate_preterite_third: "~e~ete|~a~te", "~e~eten|~a~ten";
        subordinate_subjunctive_i_first: "~e~e", "~e~en";
        subordinate_subjunctive_i_second: "~e~est", "~e~et";
        subordinate_subjunctive_i_third: "~e~e", referred;
        subordinate_subjunctive_i_i_first: "~e~ete|~a~te", "~e~eten|~a~ten";
        subordinate_subjunctive_i_i_second: "~e~etest|~a~test", "~e~etet|~a~tet";
        subordinate_subjunctive_i_i_third: "~e~ete|~a~te", "~e~eten|~a~ten";
        imperative: "~e~|~e~e", "~e~et";
        infinitive: "~e~en";
        zu_infinitive: "abzu~e~n";
        participle: "~e~end", "~e~et|~a~t";
    ),
    // Ausschreiben: 3 words (aussteigen, ausschreiben, ausschweigen).
    conjugation!("ausschreiben", 3, "au~s~e~~en";
        present_first: "~~e~~e aus", "~~e~~en aus";
        present_second: "~~e~~st aus", "~~e~~t aus";
        present_third: "~~e~~t aus", "~~e~~en aus";
        preterite_first: "~~~e~ aus", "~~~e~en aus";
        preterite_second: "~~~e~st aus", "~~~e~t aus";
        preterite_third: "~~~e~ aus", "~~~e~en aus";
        subjunctive_i_first: "~~e~~e aus", "~~e~~en aus";
        subjunctive_i_second: "~~e~~est aus", "~~e~~et aus";
        subjunctive_i_third: "~~e~~e aus", referred;
        subjunctive_i_i_first: "~~~e~e aus", "~~~e~en aus";
        subjunctive_i_i_second: "~~~e~est aus|~~~e~st aus", "~~~e~et aus|~~~e~t aus";
        subjunctive_i_i_third: "~~~e~e aus", "~~~e~en aus";
        subordinate_present_first: "au~s~e~~e", "au~s~e~~en";
        subordinate_present_second: "au~s~e~~st", "au~s~e~~t";
        subordinate_present_third: "au~s~e~~t", "au~s~e~~en";
        subordinate_preterite_first: "au~s~~e~", "au~s~~e~en";
        subordinate_preterite_second: "au~s~~e~st", "au~s~~e~t";
        subordinate_preterite_third: "au~s~~e~", "au~s~~e~en";
        subordinate_subjunctive_i_first: "au~s~e~~e", "au~s~e~~en";
        subordinate_subjunctive_i_second: "au~s~e~~est", "au~s~e~~et";
        subordinate_subjunctive_i_third: "au~s~e~~e", referred;
        subordinate_subjunctive_i_i_first: "au~s~~e~e", "au~s~~e~en";
        subordinate_subjunctive_i_i_second: "au~s~~e~est|au~s~~e~st", "au~s~~e~et|au~s~~e~t";
        subordinate_subjunctive_i_i_third: "au~s~~e~e", "au~s~~e~en";
        imperative: "~~e~~ aus|~~e~~e aus", "~~e~~t aus";
        infinitive: "au~s~e~~en";
        zu_infinitive: "au~zus~e~~en";
        participle: "au~s~e~~end", "au~ges~~e~en";
    ),
    // Umschreiben: 3 words (umsteigen, umschreiben, umtreiben).
    conjugation!("umschreiben", 3, "um~e~~en";
        present_first: "~e~~e um", "~e~~en um";
        present_second: "~e~~st um", "~e~~t um";
        present_third: "~e~~t um", "~e~~en um";
        preterite_first: "~~e~ um", "~~e~en um";
        preterite_second: "~~e~st um", "~~e~t um";
        preterite_third: "~~e~ um", "~~e~en um";
        subjunctive_i_first: "~e~~e um", "~e~~en um";
        subjunctive_i_second: "~e~~est um", "~e~~et um";
        subjunctive_i_third: "~e~~e um", referred;
        subjunctive_i_i_first: "~~e~e um", "~~e~en um";
        subjunctive_i_i_second: "~~e~est um|~~e~st um", "~~e~et um|~~e~t um";
        subjunctive_i_i_third: "~~e~e um", "~~e~en um";
        subordinate_present_first: "um~e~~e", "um~e~~en";
        subordinate_present_second: "um~e~~st", "um~e~~t";
        subordinate_present_third: "um~e~~t", "um~e~~en";
        subordinate_preterite_first: "um~~e~", "um~~e~en";
        subordinate_preterite_second: "um~~e~st", "um~~e~t";
        subordinate_preterite_third: "um~~e~", "um~~e~en";
        subordinate_subjunctive_i_first: "um~e~~e", "um~e~~en";
        subordinate_subjunctive_i_second: "um~e~~est", "um~e~~et";
        subordinate_subjunctive_i_third: "um~e~~e", referred;
        subordinate_subjunctive_i_i_first: "um~~e~e", "um~~e~en";
        subordinate_subjunctive_i_i_second: "um~~e~est|um~~e~st", "um~~e~et|um~~e~t";
        subordinate_subjunctive_i_i_third: "um~~e~e", "um~~e~en";
        imperative: "~e~~ um|~e~~e um", "~e~~t um";
        infinitive: "um~e~~en";
        zu_infinitive: "umzu~e~~en";
        participle: "um~e~~end", "umge~~e~en";
    ),
    // Einzahlen: 3 words (einzahlen, einzäunen, einzwicken).
    conjugation!("einzahlen", 3, "ein~~en";
        present_first: "~~e ein", "~~en ein";
        present_second: "~~st ein", "~~t ein";
        present_third: "~~t ein", "~~en ein";
        preterite_first: "~~te ein", "~~ten ein";
        preterite_second: "~~test ein", "~~tet ein";
        preterite_third: "~~te ein", "~~ten ein";
        subjunctive_i_first: "~~e ein", "~~en ein";
        subjunctive_i_second: "~~est ein", "~~et ein";
        subjunctive_i_third: "~~e ein", referred;
        subjunctive_i_i_first: "~~te ein", "~~ten ein";
        subjunctive_i_i_second: "~~test ein", "~~tet ein";
        subjunctive_i_i_third: "~~te ein", "~~ten ein";
        subordinate_present_first: "ein~~e", "ein~~en";
        subordinate_present_second: "ein~~st", "ein~~t";
        subordinate_present_third: "ein~~t", "ein~~en";
        subordinate_preterite_first: "ein~~te", "ein~~ten";
        subordinate_preterite_second: "ein~~test", "ein~~tet";
        subordinate_preterite_third: "ein~~te", "ein~~ten";
        subordinate_subjunctive_i_first: "ein~~e", "ein~~en";
        subordinate_subjunctive_i_second: "ein~~est", "ein~~et";
        subordinate_subjunctive_i_third: "ein~~e", referred;
        subordinate_subjunctive_i_i_first: "ein~~te", "ein~~ten";
        subordinate_subjunctive_i_i_second: "ein~~test", "ein~~tet";
        subordinate_subjunctive_i_i_third: "ein~~te", "ein~~ten";
        imperative: "~~ ein|~~e ein", "~~t ein";
        infinitive: "ein~~en";
        zu_infinitive: "ein~uz~en";
        participle: "ein~~end", "einge~~t";
    ),
    // Abbeeren: 3 words (abbestellen, abbezahlen, abbeeren).
    conjugation!("abbeeren", 3, "a~b~en";
        present_first: "~~e ab", "~~en ab";
        present_second: "~~st ab", "~~t ab";
        present_third: "~~t ab", "~~en ab";
        preterite_first: "~~te ab", "~~ten ab";
        preterite_second: "~~test ab", "~~tet ab";
        preterite_third: "~~te ab", "~~ten ab";
        subjunctive_i_first: "~~e ab", "~~en ab";
        subjunctive_i_second: "~~est ab", "~~et ab";
        subjunctive_i_third: "~~e ab", referred;
        subjunctive_i_i_first: "~~te ab", "~~ten ab";
        subjunctive_i_i_second: "~~test ab", "~~tet ab";
        subjunctive_i_i_third: "~~te ab", "~~ten ab";
        subordinate_present_first: "a~b~e", "a~b~en";
        subordinate_present_second: "a~b~st", "a~b~t";
        subordinate_present_third: "a~b~t", "a~b~en";
        subordinate_preterite_first: "a~b~te", "a~b~ten";
        subordinate_preterite_second: "a~b~test", "a~b~tet";
        subordinate_preterite_third: "a~b~te", "a~b~ten";
        subordinate_subjunctive_i_first: "a~b~e", "a~b~en";
        subordinate_subjunctive_i_second: "a~b~est", "a~b~et";
        subordinate_subjunctive_i_third: "a~b~e", referred;
        subordinate_subjunctive_i_i_first: "a~b~te", "a~b~ten";
        subordinate_subjunctive_i_i_second: "a~b~test", "a~b~tet";
        subordinate_subjunctive_i_i_third: "a~b~te", "a~b~ten";
        imperative: "~~ ab|~~e ab", "~~t ab";
        infinitive: "a~b~en";
        zu_infinitive: "a~zub~en";
        participle: "a~b~end", "a~b~t";
    ),
    // Ebnen: 3 words (ebnen, eignen, eintrocknen).
    conjugation!("ebnen", 3, "~~n";
        present_first: "~~", "~~n";
        present_second: "~~st", "~~t";
        present_third: "~~t", "~~n";
        preterite_first: "~~te", "~~ten";
        preterite_second: "~~test", "~~tet";
        preterite_third: "~~te", "~~ten";
        subjunctive_i_first: "~~", "~~n";
        subjunctive_i_second: "~~st", "~~t";
        subjunctive_i_third: "~~", referred;
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
        subordinate_subjunctive_i_third: "~~", referred;
        subordinate_subjunctive_i_i_first: "~~te", "~~ten";
        subordinate_subjunctive_i_i_second: "~~test", "~~tet";
        subordinate_subjunctive_i_i_third: "~~te", "~~ten";
        imperative: "~~", "~~t";
        infinitive: "~~n";
        zu_infinitive: "abzu~e~n";
        participle: "~~nd", "g~e~t";
    ),
    // Mitklingen: 3 words (mitschwingen, mitsingen, mitklingen).
    conjugation!("mitklingen", 3, "mit~i~en";
        present_first: "~i~e mit", "~i~en mit";
        present_second: "~i~st mit", "~i~t mit";
        present_third: "~i~t mit", "~i~en mit";
        preterite_first: "~a~ mit", "~a~en mit";
        preterite_second: "~a~st mit", "~a~t mit";
        preterite_third: "~a~ mit", "~a~en mit";
        subjunctive_i_first: "~i~e mit", "~i~en mit";
        subjunctive_i_second: "~i~est mit", "~i~et mit";
        subjunctive_i_third: "~i~e mit", referred;
        subjunctive_i_i_first: "~ä~e mit", "~ä~en mit";
        subjunctive_i_i_second: "~ä~est mit|~ä~st mit", "~ä~et mit|~ä~t mit";
        subjunctive_i_i_third: "~ä~e mit", "~ä~en mit";
        subordinate_present_first: "mit~i~e", "mit~i~en";
        subordinate_present_second: "mit~i~st", "mit~i~t";
        subordinate_present_third: "mit~i~t", "mit~i~en";
        subordinate_preterite_first: "mit~a~", "mit~a~en";
        subordinate_preterite_second: "mit~a~st", "mit~a~t";
        subordinate_preterite_third: "mit~a~", "mit~a~en";
        subordinate_subjunctive_i_first: "mit~i~e", "mit~i~en";
        subordinate_subjunctive_i_second: "mit~i~est", "mit~i~et";
        subordinate_subjunctive_i_third: "mit~i~e", referred;
        subordinate_subjunctive_i_i_first: "mit~ä~e", "mit~ä~en";
        subordinate_subjunctive_i_i_second: "mit~ä~est|mit~ä~st", "mit~ä~et|mit~ä~t";
        subordinate_subjunctive_i_i_third: "mit~ä~e", "mit~ä~en";
        imperative: "~i~ mit|~i~e mit", "~i~t mit";
        infinitive: "mit~i~en";
        zu_infinitive: "mitzu~i~en";
        participle: "mit~i~end", "mitge~u~en";
    ),
    // Überschreiben: 3 words (übertreiben, überschreiben, übersteigen).
    conjugation!("überschreiben", 3, "über~e~~en";
        present_first: "über~e~~e", "über~e~~en";
        present_second: "über~e~~st", "über~e~~t";
        present_third: "über~e~~t", "über~e~~en";
        preterite_first: "über~~e~", "über~~e~en";
        preterite_second: "über~~e~st", "über~~e~t";
        preterite_third: "über~~e~", "über~~e~en";
        subjunctive_i_first: "über~e~~e", "über~e~~en";
        subjunctive_i_second: "über~e~~est", "über~e~~et";
        subjunctive_i_third: "über~e~~e", referred;
        subjunctive_i_i_first: "über~~e~e", "über~~e~en";
        subjunctive_i_i_second: "über~~e~est", "über~~e~et";
        subjunctive_i_i_third: "über~~e~e", "über~~e~en";
        subordinate_present_first: "über~e~~e", "über~e~~en";
        subordinate_present_second: "über~e~~st", "über~e~~t";
        subordinate_present_third: "über~e~~t", "über~e~~en";
        subordinate_preterite_first: "über~~e~", "über~~e~en";
        subordinate_preterite_second: "über~~e~st", "über~~e~t";
        subordinate_preterite_third: "über~~e~", "über~~e~en";
        subordinate_subjunctive_i_first: "über~e~~e", "über~e~~en";
        subordinate_subjunctive_i_second: "über~e~~est", "über~e~~et";
        subordinate_subjunctive_i_third: "über~e~~e", referred;
        subordinate_subjunctive_i_i_first: "über~~e~e", "über~~e~en";
        subordinate_subjunctive_i_i_second: "über~~e~est", "über~~e~et";
        subordinate_subjunctive_i_i_third: "über~~e~e", "über~~e~en";
        imperative: "über~e~~", "über~e~~t";
        infinitive: "über~e~~en";
        zu_infinitive: "zu über~e~~en";
        participle: "über~e~~end", "über~~e~en";
    ),
    // Auffordern: 3 words (auffordern, auffädeln, aufflackern).
    conjugation!("auffordern", 3, "au~f~e~n";
        present_first: "~~~e auf|~~e~e auf|~~e~ auf", "~~e~n auf";
        present_second: "~~e~st auf", "~~e~t auf";
        present_third: "~~e~t auf", "~~e~n auf";
        preterite_first: "~~e~te auf", "~~e~ten auf";
        preterite_second: "~~e~test auf", "~~e~tet auf";
        preterite_third: "~~e~te auf", "~~e~ten auf";
        subjunctive_i_first: "~~e~e auf|~~~e auf", "~~e~n auf";
        subjunctive_i_second: "~~e~est auf|~~~est auf", "~~e~et auf|~~~et auf";
        subjunctive_i_third: "~~e~e auf|~~~e auf", referred;
        subjunctive_i_i_first: "~~e~te auf", "~~e~ten auf";
        subjunctive_i_i_second: "~~e~test auf", "~~e~tet auf";
        subjunctive_i_i_third: "~~e~te auf", "~~e~ten auf";
        subordinate_present_first: "au~f~~e|au~f~e~e|au~f~e~", "au~f~e~n";
        subordinate_present_second: "au~f~e~st", "au~f~e~t";
        subordinate_present_third: "au~f~e~t", "au~f~e~n";
        subordinate_preterite_first: "au~f~e~te", "au~f~e~ten";
        subordinate_preterite_second: "au~f~e~test", "au~f~e~tet";
        subordinate_preterite_third: "au~f~e~te", "au~f~e~ten";
        subordinate_subjunctive_i_first: "au~f~e~e|au~f~~e", "au~f~e~n";
        subordinate_subjunctive_i_second: "au~f~e~est|au~f~~est", "au~f~e~et|au~f~~et";
        subordinate_subjunctive_i_third: "au~f~e~e|au~f~~e", referred;
        subordinate_subjunctive_i_i_first: "au~f~e~te", "au~f~e~ten";
        subordinate_subjunctive_i_i_second: "au~f~e~test", "au~f~e~tet";
        subordinate_subjunctive_i_i_third: "au~f~e~te", "au~f~e~ten";
        imperative: "~~~e auf|~~e~ auf|~~e~e auf", "~~e~t auf";
        infinitive: "au~f~e~n";
        zu_infinitive: "au~zuf~e~n";
        participle: "au~f~e~nd", "au~gef~e~t";
    ),
    // Abfliessen: 3 words (abfliessen, abschliessen, abschiessen).
    conjugation!("abfliessen", 3, "ab~ie~en";
        present_first: "~ie~e ab", "~ie~en ab";
        present_second: "~ie~t ab", "~ie~t ab";
        present_third: "~ie~t ab", "~ie~en ab";
        preterite_first: "~o~ ab", "~o~en ab";
        preterite_second: "~o~est ab|~o~t ab", "~o~t ab";
        preterite_third: "~o~ ab", "~o~en ab";
        subjunctive_i_first: "~ie~e ab", "~ie~en ab";
        subjunctive_i_second: "~ie~est ab", "~ie~et ab";
        subjunctive_i_third: "~ie~e ab", referred;
        subjunctive_i_i_first: "~ö~e ab", "~ö~en ab";
        subjunctive_i_i_second: "~ö~est ab|~ö~t ab", "~ö~et ab|~ö~t ab";
        subjunctive_i_i_third: "~ö~e ab", "~ö~en ab";
        subordinate_present_first: "ab~ie~e", "ab~ie~en";
        subordinate_present_second: "ab~ie~t", "ab~ie~t";
        subordinate_present_third: "ab~ie~t", "ab~ie~en";
        subordinate_preterite_first: "ab~o~", "ab~o~en";
        subordinate_preterite_second: "ab~o~est|ab~o~t", "ab~o~t";
        subordinate_preterite_third: "ab~o~", "ab~o~en";
        subordinate_subjunctive_i_first: "ab~ie~e", "ab~ie~en";
        subordinate_subjunctive_i_second: "ab~ie~est", "ab~ie~et";
        subordinate_subjunctive_i_third: "ab~ie~e", referred;
        subordinate_subjunctive_i_i_first: "ab~ö~e", "ab~ö~en";
        subordinate_subjunctive_i_i_second: "ab~ö~est|ab~ö~t", "ab~ö~et|ab~ö~t";
        subordinate_subjunctive_i_i_third: "ab~ö~e", "ab~ö~en";
        imperative: "~ie~ ab|~ie~e ab", "~ie~t ab";
        infinitive: "ab~ie~en";
        zu_infinitive: "abzu~ie~en";
        participle: "ab~ie~end", "abge~o~en";
    ),
    // Aussortieren: 3 words (ausstaffieren, ausspionieren, aussortieren).
    conjugation!("aussortieren", 3, "au~s~en";
        present_first: "~~e aus", "~~en aus";
        present_second: "~~st aus", "~~t aus";
        present_third: "~~t aus", "~~en aus";
        preterite_first: "~~te aus", "~~ten aus";
        preterite_second: "~~test aus", "~~tet aus";
        preterite_third: "~~te aus", "~~ten aus";
        subjunctive_i_first: "~~e aus", "~~en aus";
        subjunctive_i_second: "~~est aus", "~~et aus";
        subjunctive_i_third: "~~e aus", referred;
        subjunctive_i_i_first: "~~te aus", "~~ten aus";
        subjunctive_i_i_second: "~~test aus", "~~tet aus";
        subjunctive_i_i_third: "~~te aus", "~~ten aus";
        subordinate_present_first: "au~s~e", "au~s~en";
        subordinate_present_second: "au~s~st", "au~s~t";
        subordinate_present_third: "au~s~t", "au~s~en";
        subordinate_preterite_first: "au~s~te", "au~s~ten";
        subordinate_preterite_second: "au~s~test", "au~s~tet";
        subordinate_preterite_third: "au~s~te", "au~s~ten";
        subordinate_subjunctive_i_first: "au~s~e", "au~s~en";
        subordinate_subjunctive_i_second: "au~s~est", "au~s~et";
        subordinate_subjunctive_i_third: "au~s~e", referred;
        subordinate_subjunctive_i_i_first: "au~s~te", "au~s~ten";
        subordinate_subjunctive_i_i_second: "au~s~test", "au~s~tet";
        subordinate_subjunctive_i_i_third: "au~s~te", "au~s~ten";
        imperative: "~~ aus|~~e aus", "~~t aus";
        infinitive: "au~s~en";
        zu_infinitive: "au~zus~en";
        participle: "au~s~end", "au~s~t";
    ),
    // Außbauen: 3 words (außlegen, außbauen, außdehnen).
    conjugation!("außbauen", 3, "auß~en";
        present_first: "~e auß", "~en auß";
        present_second: "~st auß", "~t auß";
        present_third: "~t auß", "~en auß";
        preterite_first: "~te auß", "~ten auß";
        preterite_second: "~test auß", "~tet auß";
        preterite_third: "~te auß", "~ten auß";
        subjunctive_i_first: "~e auß", "~en auß";
        subjunctive_i_second: "~est auß", "~et auß";
        subjunctive_i_third: "~e auß", referred;
        subjunctive_i_i_first: "~te auß", "~ten auß";
        subjunctive_i_i_second: "~test auß", "~tet auß";
        subjunctive_i_i_third: "~te auß", "~ten auß";
        subordinate_present_first: "auß~e", "auß~en";
        subordinate_present_second: "auß~st", "auß~t";
        subordinate_present_third: "auß~t", "auß~en";
        subordinate_preterite_first: "auß~te", "auß~ten";
        subordinate_preterite_second: "auß~test", "auß~tet";
        subordinate_preterite_third: "auß~te", "auß~ten";
        subordinate_subjunctive_i_first: "auß~e", "auß~en";
        subordinate_subjunctive_i_second: "auß~est", "auß~et";
        subordinate_subjunctive_i_third: "auß~e", referred;
        subordinate_subjunctive_i_i_first: "auß~te", "auß~ten";
        subordinate_subjunctive_i_i_second: "auß~test", "auß~tet";
        subordinate_subjunctive_i_i_third: "auß~te", "auß~ten";
        imperative: "~ auß|~e auß", "~t auß";
        infinitive: "auß~en";
        zu_infinitive: "außzu~en";
        participle: "auß~end", "außge~t";
    ),
    // Zusammenleben: 3 words (weiterleben, zusammenleben, wiederaufleben).
    conjugation!("zusammenleben", 3, "~leben";
        present_first: "lebe ~", "leben ~";
        present_second: "lebst ~", "lebt ~";
        present_third: "lebt ~", "leben ~";
        preterite_first: "lebte ~", "lebten ~";
        preterite_second: "lebtest ~", "lebtet ~";
        preterite_third: "lebte ~", "lebten ~";
        subjunctive_i_first: "lebe ~", "leben ~";
        subjunctive_i_second: "lebest ~", "lebet ~";
        subjunctive_i_third: "lebe ~", referred;
        subjunctive_i_i_first: "lebte ~", "lebten ~";
        subjunctive_i_i_second: "lebtest ~", "lebtet ~";
        subjunctive_i_i_third: "lebte ~", "lebten ~";
        subordinate_present_first: "~lebe", "~leben";
        subordinate_present_second: "~lebst", "~lebt";
        subordinate_present_third: "~lebt", "~leben";
        subordinate_preterite_first: "~lebte", "~lebten";
        subordinate_preterite_second: "~lebtest", "~lebtet";
        subordinate_preterite_third: "~lebte", "~lebten";
        subordinate_subjunctive_i_first: "~lebe", "~leben";
        subordinate_subjunctive_i_second: "~lebest", "~lebet";
        subordinate_subjunctive_i_third: "~lebe", referred;
        subordinate_subjunctive_i_i_first: "~lebte", "~lebten";
        subordinate_subjunctive_i_i_second: "~lebtest", "~lebtet";
        subordinate_subjunctive_i_i_third: "~lebte", "~lebten";
        imperative: "leb ~|lebe ~", "lebt ~";
        infinitive: "~leben";
        zu_infinitive: "~zuleben";
        participle: "~lebend", "~gelebt";
    ),
    // Angucken: 3 words (angucken, angrillen, angaffen).
    conjugation!("angucken", 3, "an~~en";
        present_first: "~~e an", "~~en an";
        present_second: "~~st an", "~~t an";
        present_third: "~~t an", "~~en an";
        preterite_first: "~~te an", "~~ten an";
        preterite_second: "~~test an", "~~tet an";
        preterite_third: "~~te an", "~~ten an";
        subjunctive_i_first: "~~e an", "~~en an";
        subjunctive_i_second: "~~est an", "~~et an";
        subjunctive_i_third: "~~e an", referred;
        subjunctive_i_i_first: "~~te an", "~~ten an";
        subjunctive_i_i_second: "~~test an", "~~tet an";
        subjunctive_i_i_third: "~~te an", "~~ten an";
        subordinate_present_first: "an~~e", "an~~en";
        subordinate_present_second: "an~~st", "an~~t";
        subordinate_present_third: "an~~t", "an~~en";
        subordinate_preterite_first: "an~~te", "an~~ten";
        subordinate_preterite_second: "an~~test", "an~~tet";
        subordinate_preterite_third: "an~~te", "an~~ten";
        subordinate_subjunctive_i_first: "an~~e", "an~~en";
        subordinate_subjunctive_i_second: "an~~est", "an~~et";
        subordinate_subjunctive_i_third: "an~~e", referred;
        subordinate_subjunctive_i_i_first: "an~~te", "an~~ten";
        subordinate_subjunctive_i_i_second: "an~~test", "an~~tet";
        subordinate_subjunctive_i_i_third: "an~~te", "an~~ten";
        imperative: "~~ an|~~e an", "~~t an";
        infinitive: "an~~en";
        zu_infinitive: "anzu~~en";
        participle: "an~~end", "an~eg~t";
    ),
    // Aufgleisen: 3 words (aufglänzen, aufgleisen, aufgrunzen).
    conjugation!("aufgleisen", 3, "auf~~en";
        present_first: "~~e auf", "~~en auf";
        present_second: "~~t auf", "~~t auf";
        present_third: "~~t auf", "~~en auf";
        preterite_first: "~~te auf", "~~ten auf";
        preterite_second: "~~test auf", "~~tet auf";
        preterite_third: "~~te auf", "~~ten auf";
        subjunctive_i_first: "~~e auf", "~~en auf";
        subjunctive_i_second: "~~est auf", "~~et auf";
        subjunctive_i_third: "~~e auf", referred;
        subjunctive_i_i_first: "~~te auf", "~~ten auf";
        subjunctive_i_i_second: "~~test auf", "~~tet auf";
        subjunctive_i_i_third: "~~te auf", "~~ten auf";
        subordinate_present_first: "auf~~e", "auf~~en";
        subordinate_present_second: "auf~~t", "auf~~t";
        subordinate_present_third: "auf~~t", "auf~~en";
        subordinate_preterite_first: "auf~~te", "auf~~ten";
        subordinate_preterite_second: "auf~~test", "auf~~tet";
        subordinate_preterite_third: "auf~~te", "auf~~ten";
        subordinate_subjunctive_i_first: "auf~~e", "auf~~en";
        subordinate_subjunctive_i_second: "auf~~est", "auf~~et";
        subordinate_subjunctive_i_third: "auf~~e", referred;
        subordinate_subjunctive_i_i_first: "auf~~te", "auf~~ten";
        subordinate_subjunctive_i_i_second: "auf~~test", "auf~~tet";
        subordinate_subjunctive_i_i_third: "auf~~te", "auf~~ten";
        imperative: "~~ auf|~~e auf", "~~t auf";
        infinitive: "auf~~en";
        zu_infinitive: "aufzu~~en";
        participle: "auf~~end", "auf~eg~t";
    ),
    // Weichkochen: 3 words (weichspülen, weichkochen, weichklopfen).
    conjugation!("weichkochen", 3, "weich~en";
        present_first: "~e weich", "~en weich";
        present_second: "~st weich", "~t weich";
        present_third: "~t weich", "~en weich";
        preterite_first: "~te weich", "~ten weich";
        preterite_second: "~test weich", "~tet weich";
        preterite_third: "~te weich", "~ten weich";
        subjunctive_i_first: "~e weich", "~en weich";
        subjunctive_i_second: "~est weich", "~et weich";
        subjunctive_i_third: "~e weich", referred;
        subjunctive_i_i_first: "~te weich", "~ten weich";
        subjunctive_i_i_second: "~test weich", "~tet weich";
        subjunctive_i_i_third: "~te weich", "~ten weich";
        subordinate_present_first: "weich~e", "weich~en";
        subordinate_present_second: "weich~st", "weich~t";
        subordinate_present_third: "weich~t", "weich~en";
        subordinate_preterite_first: "weich~te", "weich~ten";
        subordinate_preterite_second: "weich~test", "weich~tet";
        subordinate_preterite_third: "weich~te", "weich~ten";
        subordinate_subjunctive_i_first: "weich~e", "weich~en";
        subordinate_subjunctive_i_second: "weich~est", "weich~et";
        subordinate_subjunctive_i_third: "weich~e", referred;
        subordinate_subjunctive_i_i_first: "weich~te", "weich~ten";
        subordinate_subjunctive_i_i_second: "weich~test", "weich~tet";
        subordinate_subjunctive_i_i_third: "weich~te", "weich~ten";
        imperative: "~ weich|~e weich", "~t weich";
        infinitive: "weich~en";
        zu_infinitive: "weichzu~en";
        participle: "weich~end", "weichge~t";
    ),
    // Herumfahren: 3 words (herumfahren, herumschlagen, herumtragen).
    conjugation!("herumfahren", 3, "herum~a~en";
        present_first: "~a~e herum", "~a~en herum";
        present_second: "~ä~st herum", "~a~t herum";
        present_third: "~ä~t herum", "~a~en herum";
        preterite_first: "~u~ herum", "~u~en herum";
        preterite_second: "~u~st herum", "~u~t herum";
        preterite_third: "~u~ herum", "~u~en herum";
        subjunctive_i_first: "~a~e herum", "~a~en herum";
        subjunctive_i_second: "~a~est herum", "~a~et herum";
        subjunctive_i_third: "~a~e herum", referred;
        subjunctive_i_i_first: "~ü~e herum", "~ü~en herum";
        subjunctive_i_i_second: "~ü~est herum|~ü~st herum", "~ü~et herum|~ü~t herum";
        subjunctive_i_i_third: "~ü~e herum", "~ü~en herum";
        subordinate_present_first: "herum~a~e", "herum~a~en";
        subordinate_present_second: "herum~ä~st", "herum~a~t";
        subordinate_present_third: "herum~ä~t", "herum~a~en";
        subordinate_preterite_first: "herum~u~", "herum~u~en";
        subordinate_preterite_second: "herum~u~st", "herum~u~t";
        subordinate_preterite_third: "herum~u~", "herum~u~en";
        subordinate_subjunctive_i_first: "herum~a~e", "herum~a~en";
        subordinate_subjunctive_i_second: "herum~a~est", "herum~a~et";
        subordinate_subjunctive_i_third: "herum~a~e", referred;
        subordinate_subjunctive_i_i_first: "herum~ü~e", "herum~ü~en";
        subordinate_subjunctive_i_i_second: "herum~ü~est|herum~ü~st", "herum~ü~et|herum~ü~t";
        subordinate_subjunctive_i_i_third: "herum~ü~e", "herum~ü~en";
        imperative: "~a~ herum|~a~e herum", "~a~t herum";
        infinitive: "herum~a~en";
        zu_infinitive: "herumzu~a~en";
        participle: "herum~a~end", "herumge~a~en";
    ),
    // Zubewegen: 3 words (zugehören, zubewegen, zuerteilen).
    conjugation!("zubewegen", 3, "zu~en";
        present_first: "~e zu", "~en zu";
        present_second: "~st zu", "~t zu";
        present_third: "~t zu", "~en zu";
        preterite_first: "~te zu", "~ten zu";
        preterite_second: "~test zu", "~tet zu";
        preterite_third: "~te zu", "~ten zu";
        subjunctive_i_first: "~e zu", "~en zu";
        subjunctive_i_second: "~est zu", "~et zu";
        subjunctive_i_third: "~e zu", referred;
        subjunctive_i_i_first: "~te zu", "~ten zu";
        subjunctive_i_i_second: "~test zu", "~tet zu";
        subjunctive_i_i_third: "~te zu", "~ten zu";
        subordinate_present_first: "zu~e", "zu~en";
        subordinate_present_second: "zu~st", "zu~t";
        subordinate_present_third: "zu~t", "zu~en";
        subordinate_preterite_first: "zu~te", "zu~ten";
        subordinate_preterite_second: "zu~test", "zu~tet";
        subordinate_preterite_third: "zu~te", "zu~ten";
        subordinate_subjunctive_i_first: "zu~e", "zu~en";
        subordinate_subjunctive_i_second: "zu~est", "zu~et";
        subordinate_subjunctive_i_third: "zu~e", referred;
        subordinate_subjunctive_i_i_first: "zu~te", "zu~ten";
        subordinate_subjunctive_i_i_second: "zu~test", "zu~tet";
        subordinate_subjunctive_i_i_third: "zu~te", "zu~ten";
        imperative: "~ zu|~e zu", "~t zu";
        infinitive: "zu~en";
        zu_infinitive: "zuzu~en";
        participle: "zu~end", "zu~t";
    ),
    // Heraushalten: 3 words (heraushalten, herunterhalten, heraufhalten).
    conjugation!("heraushalten", 3, "~~~halten";
        present_first: "~alt~ he~", "~alt~n he~";
        present_second: "~ältst h~~", "~alt~t he~";
        present_third: "~ält h~~", "~alt~n he~";
        preterite_first: "~i~lt he~", "~i~lten he~";
        preterite_second: "~i~ltest he~|~i~ltst he~", "~i~ltet he~";
        preterite_third: "~i~lt he~", "~i~lten he~";
        subjunctive_i_first: "~alt~ he~", "~alt~n he~";
        subjunctive_i_second: "~alt~st he~", "~alt~t he~";
        subjunctive_i_third: "~alt~ he~", referred;
        subjunctive_i_i_first: "~i~lte he~", "~i~lten he~";
        subjunctive_i_i_second: "~i~ltest he~", "~i~ltet he~";
        subjunctive_i_i_third: "~i~lte he~", "~i~lten he~";
        subordinate_present_first: "~~~halte", "~~~halten";
        subordinate_present_second: "~~~hältst", "~~~haltet";
        subordinate_present_third: "~~~hält", "~~~halten";
        subordinate_preterite_first: "~~~hielt", "~~~hielten";
        subordinate_preterite_second: "~~~hieltest|~~~hieltst", "~~~hieltet";
        subordinate_preterite_third: "~~~hielt", "~~~hielten";
        subordinate_subjunctive_i_first: "~~~halte", "~~~halten";
        subordinate_subjunctive_i_second: "~~~haltest", "~~~haltet";
        subordinate_subjunctive_i_third: "~~~halte", referred;
        subordinate_subjunctive_i_i_first: "~~~hielte", "~~~hielten";
        subordinate_subjunctive_i_i_second: "~~~hieltest", "~~~hieltet";
        subordinate_subjunctive_i_i_third: "~~~hielte", "~~~hielten";
        imperative: "~alt h~~|~alt~ he~", "~alt~t he~";
        infinitive: "~~~halten";
        zu_infinitive: "~~~zuhalten";
        participle: "~~~haltend", "~~~gehalten";
    ),
    // Zusammenräumen: 3 words (zusammenräumen, zurückräumen, hinterherräumen).
    conjugation!("zusammenräumen", 3, "~räumen";
        present_first: "räume ~", "räumen ~";
        present_second: "räumst ~", "räumt ~";
        present_third: "räumt ~", "räumen ~";
        preterite_first: "räumte ~", "räumten ~";
        preterite_second: "räumtest ~", "räumtet ~";
        preterite_third: "räumte ~", "räumten ~";
        subjunctive_i_first: "räume ~", "räumen ~";
        subjunctive_i_second: "räumest ~", "räumet ~";
        subjunctive_i_third: "räume ~", referred;
        subjunctive_i_i_first: "räumte ~", "räumten ~";
        subjunctive_i_i_second: "räumtest ~", "räumtet ~";
        subjunctive_i_i_third: "räumte ~", "räumten ~";
        subordinate_present_first: "~räume", "~räumen";
        subordinate_present_second: "~räumst", "~räumt";
        subordinate_present_third: "~räumt", "~räumen";
        subordinate_preterite_first: "~räumte", "~räumten";
        subordinate_preterite_second: "~räumtest", "~räumtet";
        subordinate_preterite_third: "~räumte", "~räumten";
        subordinate_subjunctive_i_first: "~räume", "~räumen";
        subordinate_subjunctive_i_second: "~räumest", "~räumet";
        subordinate_subjunctive_i_third: "~räume", referred;
        subordinate_subjunctive_i_i_first: "~räumte", "~räumten";
        subordinate_subjunctive_i_i_second: "~räumtest", "~räumtet";
        subordinate_subjunctive_i_i_third: "~räumte", "~räumten";
        imperative: "räum ~|räume ~", "räumt ~";
        infinitive: "~räumen";
        zu_infinitive: "~zuräumen";
        participle: "~räumend", "~geräumt";
    ),
];
