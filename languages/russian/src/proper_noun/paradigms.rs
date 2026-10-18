//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~а`, `слава`: the stem and `а`). A stem in two pieces (`~~а`, `камчатка`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem gen_sg: \"~- like рокпорт\"`: the slot's cells are `рокпорт`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `ins` (sg, pl)
/// `prep` (sg, pl)
/// `proper_noun_second_case` (part, loc)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem gen_sg: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(acc: $c6:tt, $c7:tt;)?
     $(ins: $c8:tt, $c9:tt;)?
     $(prep: $c10:tt, $c11:tt;)?
     $(proper_noun_second_case: $c12:tt, $c13:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "", "ь" after "~"`: ``, or `ь`
/// where that pattern ends in `~`).
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
    (animacy: $v:ident) => {
        (1, crate::Animacy::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `рокпорт`: 1,354 words.
    Рокпорт,
    /// Like `слава`: 585 words.
    Слава,
    /// Like `македония`: 234 words.
    Македония,
    /// Like `люксембург`: 125 words.
    Люксембург,
    /// Like `хиль`: 84 words.
    Хиль,
    /// Like `париж`: 69 words.
    Париж,
    /// Like `алтай`: 67 words.
    Алтай,
    /// Like `дмитрий`: 51 words.
    Дмитрий,
    /// Like `русь`: 47 words.
    Русь,
    /// Like `валя`: 44 words.
    Валя,
    /// Like `николаево`: 36 words.
    Николаево,
    /// Like `рыбы`: 31 words.
    Рыбы,
    /// Like `рея`: 28 words.
    Рея,
    /// Like `запорожье`: 27 words.
    Запорожье,
    /// Like `катюша`: 25 words.
    Катюша,
    /// Like `камчатка`: 20 words.
    Камчатка,
    /// Like `дарья`: 19 words.
    Дарья,
    /// Like `николаевская`: 14 words.
    Николаевская,
    /// Like `белоснежка`: 10 words.
    Белоснежка,
    /// Like `азов`: 10 words.
    Азов,
    /// Like `варенька`: 10 words.
    Варенька,
    /// Like `урожайное`: 10 words.
    Урожайное,
    /// Like `погребки`: 10 words.
    Погребки,
    /// Like `илия`: 9 words.
    Илия,
    /// Like `николаевский`: 9 words.
    Николаевский,
    /// Like `писание`: 8 words.
    Писание,
    /// Like `весы`: 7 words.
    Весы,
    /// Like `овен`: 7 words.
    Овен,
    /// Like `альбац`: 6 words.
    Альбац,
    /// Like `гавайи`: 6 words.
    Гавайи,
    /// Like `кремль`: 6 words.
    Кремль,
    /// Like `двина`: 6 words.
    Двина,
    /// Like `олонец`: 4 words.
    Олонец,
    /// Like `лев`: 3 words.
    Лев,
    /// Like `майка`: 3 words.
    Майка,
    /// Like `грозный`: 3 words.
    Грозный,
    /// Like `майя`: 3 words.
    Майя,
    /// Like `николаёнок`: 3 words.
    Николаёнок,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 38] = [
        Declension::Рокпорт,
        Declension::Слава,
        Declension::Македония,
        Declension::Люксембург,
        Declension::Хиль,
        Declension::Париж,
        Declension::Алтай,
        Declension::Дмитрий,
        Declension::Русь,
        Declension::Валя,
        Declension::Николаево,
        Declension::Рыбы,
        Declension::Рея,
        Declension::Запорожье,
        Declension::Катюша,
        Declension::Камчатка,
        Declension::Дарья,
        Declension::Николаевская,
        Declension::Белоснежка,
        Declension::Азов,
        Declension::Варенька,
        Declension::Урожайное,
        Declension::Погребки,
        Declension::Илия,
        Declension::Николаевский,
        Declension::Писание,
        Declension::Весы,
        Declension::Овен,
        Declension::Альбац,
        Declension::Гавайи,
        Declension::Кремль,
        Declension::Двина,
        Declension::Олонец,
        Declension::Лев,
        Declension::Майка,
        Declension::Грозный,
        Declension::Майя,
        Declension::Николаёнок,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 38] = [
    // Рокпорт: 1,354 words (смолян, христос, юпитер, ирак, рейн, заир).
    declension!("рокпорт", 1354, "~", [gender: Masculine];
    stem gen_sg: "~- like рокпорт";
    nom: "~", _;
    ),
    // Слава: 585 words (украина, луизиана, москва, канада, европа, герцеговина). Like Рокпорт, except in 4 cells: gen_ sg, gen_ pl, dat sg, ins sg.
    declension!("слава", 585, "~а", like Рокпорт;
    stem lemma: "~- like слава";
    gen_: "~ы", "~";
    dat: "~е", _;
    ins: "~ой|~ою", _;
    ),
    // Македония: 234 words (россия, боливия, босния, бразилия, колумбия, македония).
    declension!("македония", 234, "~я", [gender: Feminine];
    nom: "~я", "~и";
    gen_: "~и", "~й";
    dat: "~и", "~ям";
    acc: "~ю", _;
    ins: "~ей|~ею", _;
    prep: "~и", _;
    ),
    // Люксембург: 125 words (бангкок, архангельск, санкт-петербург, владивосток, выборг, банкок). Like Рокпорт.
    declension!("люксембург", 125, "~", [gender: Masculine, animacy: Inanimate], like Рокпорт;
    stem gen_sg: "~- like люксембург";
    ),
    // Хиль: 84 words (израиль, господь, чернобыль, брюссель, неаполь, тайвань). Like Париж with `` after each pattern (`ь` after ~), except in 3 cells: gen_ sg, dat sg, dat pl.
    declension!("хиль", 84, "~ь", [gender: Masculine], like Париж with "", "ь" after "~";
    gen_: "~я", _;
    dat: "~ю", "~ям";
    ),
    // Париж: 69 words (гринвич, париж, ла-манш, смерш, идиш, галич). Like Рокпорт, except in 3 cells: nom pl, gen_ pl, ins sg.
    declension!("париж", 69, "~", [gender: Masculine], like Рокпорт;
    nom: _, "~и";
    gen_: _, "~ей";
    ins: "~ем", _;
    ),
    // Алтай: 67 words (китай, дунай, бомбей, уругвай, авдей, андрей). Like Хиль, except in 2 cells: nom sg, gen_ pl.
    declension!("алтай", 67, "~й", [gender: Masculine], like Хиль;
    nom: "~й", _;
    gen_: _, "~ев";
    ),
    // Дмитрий: 51 words (меркурий, дмитрий, григорий, анатолий, георгий, евгений). Like Алтай, except in 1 cell: prep sg.
    declension!("дмитрий", 51, "~й", [gender: Masculine], like Алтай;
    prep: "~и", _;
    ),
    // Русь: 47 words (беларусь, сибирь, парень, казань, русь, богоматерь). Like Македония, except in 4 cells: nom sg, gen_ pl, acc sg, ins sg.
    declension!("русь", 47, "~ь", [gender: Feminine], like Македония;
    nom: "~ь", _;
    gen_: _, "~ей";
    acc: "~ь", _;
    ins: "~ью", _;
    ),
    // Валя: 44 words (аня, женя, тоня, генуя, падуя, костя). Like Македония, except in 3 cells: gen_ pl, dat sg, prep sg.
    declension!("валя", 44, "~я", like Македония;
    gen_: _, "~ь";
    dat: "~е", _;
    prep: "~е", _;
    ),
    // Николаево: 36 words (сараево, рождество, косово, кемерово, иваново, гродно). Like Рокпорт with `` after each pattern (`о` after ~), except in 3 cells: nom pl, gen_ pl, acc sg.
    declension!("николаево", 36, "~о", like Рокпорт with "", "о" after "~";
    nom: _, "~а";
    gen_: _, "~";
    acc: "~о", _;
    ),
    // Рыбы: 31 words (афины, балканы, альпы, анды, рыбы, филиппины). Like Рокпорт, except in 6 cells: nom sg, gen_ sg, gen_ pl, dat sg, ins sg, prep sg.
    declension!("рыбы", 31, "~ы", [animacy: Inanimate], like Рокпорт;
    nom: "~ы", _;
    gen_: derived, "~";
    dat: derived, _;
    ins: derived, _;
    prep: derived, _;
    ),
    // Рея: 28 words (корея, пангея, адыгея, эритрея, гвинея, прая). Like Валя, except in 1 cell: gen_ pl.
    declension!("рея", 28, "~я", [gender: Feminine], like Валя;
    gen_: _, "~й";
    ),
    // Запорожье: 27 words (солнце, закавказье, приднестровье, приморье, подмосковье, запорожье). Like Хиль, except in 4 cells: nom sg, nom pl, gen_ pl, dat pl.
    declension!("запорожье", 27, "~е", [gender: Neuter, animacy: Inanimate], like Хиль;
    nom: "~е", derived;
    gen_: _, derived;
    dat: _, derived;
    ),
    // Катюша: 25 words (камбоджа, польша, саша, авдюша, наташа, абуджа). Like Слава, except in 4 cells: nom pl, gen_ sg, acc pl, ins sg.
    declension!("катюша", 25, "~а", like Слава;
    nom: _, "~и";
    gen_: "~и", _;
    acc: _, "~";
    ins: "~ей|~ею", _;
    ),
    // Камчатка: 20 words (аляска, небраска, глинка, камчатка, чукотка, находка).
    declension!("камчатка", 20, "~~а";
    stem lemma: "~~- like слава";
    nom: _, "~~и";
    gen_: "~~и", "~о~";
    dat: "~~е", "~~ам";
    ins: "~~ой", _;
    prep: "~~е", _;
    ),
    // Дарья: 19 words (ангилья, анталья, болонья, вентимилья, наталья, софья).
    declension!("дарья", 19, "~ья", [gender: Feminine];
    nom: "~ья", "~ьи";
    gen_: "~ьи", "~ий";
    dat: "~ье", "~ьям";
    acc: "~ью", _;
    ins: "~ьей|~ьею", _;
    prep: "~ье", _;
    ),
    // Николаевская: 14 words (поднебесная, столичная, амурская, электрозаводская, киевская, новослободская).
    declension!("николаевская", 14, "~ая", [gender: Feminine, animacy: Inanimate];
    nom: "~ая", "~ые";
    gen_: "~ой", "~ых";
    dat: "~ой", "~ым";
    acc: "~ую", _;
    ins: "~ой|~ою", _;
    prep: "~ой", _;
    ),
    // Белоснежка: 10 words (снегурочка, чебурашка, белоснежка, золушка, дюймовочка, землячка). Like Камчатка, except in 2 cells: gen_ pl, ins sg.
    declension!("белоснежка", 10, "~~а", like Камчатка;
    gen_: _, "~е~";
    ins: "~~ой|~~ою", _;
    ),
    // Азов: 10 words (киров, калинин, данил, азов, уильям, царицын). Like Рокпорт, except in 2 cells: acc sg, acc pl.
    declension!("азов", 10, "~", [gender: Masculine, animacy: Animate], like Рокпорт;
    acc: "~", "~ы";
    ),
    // Варенька: 10 words (кузька, катенька, диканька, дашенька, мишенька, машенька).
    declension!("варенька", 10, "~ь~а";
    stem lemma: "~ь~- like слава";
    nom: _, "~ь~и";
    gen_: "~ь~и", "~е~";
    dat: "~ь~е", "~ь~ам";
    ins: "~ь~ой|~ь~ою", _;
    prep: "~ь~е", _;
    ),
    // Урожайное: 10 words (лазаревское, южное, каменское, снежное, кальмиусское, урожайное). Like Запорожье, except in 5 cells: nom sg, gen_ sg, dat sg, ins sg, prep sg.
    declension!("урожайное", 10, "~ое", [gender: Neuter, animacy: Inanimate], like Запорожье;
    nom: "~ое", _;
    gen_: "~ого", _;
    dat: "~ому", _;
    ins: "~ым", _;
    prep: "~ом", _;
    ),
    // Погребки: 10 words (сочи, аппалачи, любавичи, соловки, ессентуки, горки). Like Рыбы, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("погребки", 10, "~и", [animacy: Inanimate], like Рыбы;
    nom: "~и", "~и";
    gen_: _, "~ов";
    ),
    // Илия: 9 words (земля, илья, чечня, санья, амударья, аму-дарья). Like Рея, except in 1 cell: ins sg.
    declension!("илия", 9, "~я", like Рея;
    ins: "~ёй|~ёю", _;
    ),
    // Николаевский: 9 words (возничий, нижний, ыныкчанский, хмельницкий, кропивницкий, керенский). Like Урожайное, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, ins sg.
    declension!("николаевский", 9, "~ий", [gender: Masculine], like Урожайное;
    nom: "~ий", "~ие";
    gen_: _, "~их";
    dat: _, "~им";
    ins: "~им", _;
    ),
    // Писание: 8 words (умиление, евровидение, благовещение, вознесение, преполовение, родноверие). Like Дмитрий, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("писание", 8, "~е", [gender: Neuter, animacy: Inanimate], like Дмитрий;
    nom: "~е", "~я";
    gen_: _, "~й";
    ),
    // Весы: 7 words (нидерланды, близнецы, весы, штаты, каракумы, черновцы). Like Рыбы, except in 1 cell: gen_ pl.
    declension!("весы", 7, "~ы", [gender: Masculine], like Рыбы;
    gen_: _, "~ов";
    ),
    // Овен: 7 words (египет, павел, овен, донец, резец, березовец). Like Камчатка, except in 2 cells: nom sg, acc sg.
    declension!("овен", 7, "~е~", [gender: Masculine], like Камчатка;
    stem gen_sg: "~~- like рокпорт";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Альбац: 6 words (карловац, вадуц, кац, альбац, королевец, суэц). Like Рокпорт, except in 2 cells: gen_ pl, ins sg.
    declension!("альбац", 6, "~", [gender: Masculine], like Рокпорт;
    gen_: _, "~ев";
    ins: "~ем", _;
    ),
    // Гавайи: 6 words (гималаи, гавайи, пиренеи, жигули, стабии, помпеи). Like Погребки, except in 2 cells: gen_ pl, dat pl.
    declension!("гавайи", 6, "~и", [animacy: Inanimate], like Погребки;
    gen_: _, "~ев";
    dat: _, "~ям";
    ),
    // Кремль: 6 words (викисловарь, кремль, дудь, ирпень, кушнарь, коваль). Like Хиль.
    declension!("кремль", 6, "~ь", [gender: Masculine], like Хиль;
    stem gen_sg: "~- like кремль";
    ),
    // Двина: 6 words (аддис-абеба, капица, кулеба, таратута, двина, геба). Like Слава, except in 1 cell: acc pl.
    declension!("двина", 6, "~а", like Слава;
    acc: _, "~ы";
    ),
    // Олонец: 4 words (олонец, кременец, торопец, сосновец). Like Овен, except in 4 cells: nom pl, gen_ pl, dat pl, ins sg.
    declension!("олонец", 4, "~е~", [gender: Masculine, animacy: Inanimate], like Овен;
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    ins: "~~ем", _;
    ),
    // Лев: 3 words (телец, лев, стрелец). Like Варенька, except in 2 cells: nom sg, acc sg.
    declension!("лев", 3, "~е~", [gender: Masculine, animacy: Animate], like Варенька;
    stem gen_sg: "~ь~- like рокпорт";
    nom: "~е~", _;
    acc: derived, _;
    ),
    // Майка: 3 words (майка, ямайка, незнайка).
    declension!("майка", 3, "~й~а";
    stem lemma: "~й~- like слава";
    nom: _, "~й~и";
    gen_: "~й~и", "~е~";
    dat: "~й~е", "~й~ам";
    ins: "~й~ой|~й~ою", _;
    prep: "~й~е", _;
    ),
    // Грозный: 3 words (грозный, лукавый, кедровый). Like Урожайное, except in 1 cell: nom sg.
    declension!("грозный", 3, "~ый", [gender: Masculine], like Урожайное;
    nom: "~ый", _;
    ),
    // Майя: 3 words (савойя, майя, джедедайя). Like Валя, except in 1 cell: gen_ pl.
    declension!("майя", 3, "~я", like Валя;
    gen_: _, "~";
    ),
    // Николаёнок: 3 words (пятачок, дячок, николаёнок). Like Овен, except in 2 cells: nom sg, nom pl.
    declension!("николаёнок", 3, "~о~", [gender: Masculine, animacy: Animate], like Овен;
    nom: "~о~", "~~и";
    ),
];
