//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ий`, `сведущий`: the stem and `ий`). A stem in two pieces (`~~ый`, `уместный`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom_m` (anim, inan)
/// `nom_f` (anim, inan)
/// `nom_n` (anim, inan)
/// `nom_pl` (anim, inan)
/// `gen_m` (anim, inan)
/// `gen_f` (anim, inan)
/// `gen_n` (anim, inan)
/// `gen_pl` (anim, inan)
/// `dat_m` (anim, inan)
/// `dat_f` (anim, inan)
/// `dat_n` (anim, inan)
/// `dat_pl` (anim, inan)
/// `acc_m` (anim, inan)
/// `acc_f` (anim, inan)
/// `acc_n` (anim, inan)
/// `acc_pl` (anim, inan)
/// `ins_m` (anim, inan)
/// `ins_f` (anim, inan)
/// `ins_n` (anim, inan)
/// `ins_pl` (anim, inan)
/// `prep_m` (anim, inan)
/// `prep_f` (anim, inan)
/// `prep_n` (anim, inan)
/// `prep_pl` (anim, inan)
/// `short_adjective` (m, f, n, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(nom_pl: $c6:tt, $c7:tt;)?
     $(gen_m: $c8:tt, $c9:tt;)?
     $(gen_f: $c10:tt, $c11:tt;)?
     $(gen_n: $c12:tt, $c13:tt;)?
     $(gen_pl: $c14:tt, $c15:tt;)?
     $(dat_m: $c16:tt, $c17:tt;)?
     $(dat_f: $c18:tt, $c19:tt;)?
     $(dat_n: $c20:tt, $c21:tt;)?
     $(dat_pl: $c22:tt, $c23:tt;)?
     $(acc_m: $c24:tt, $c25:tt;)?
     $(acc_f: $c26:tt, $c27:tt;)?
     $(acc_n: $c28:tt, $c29:tt;)?
     $(acc_pl: $c30:tt, $c31:tt;)?
     $(ins_m: $c32:tt, $c33:tt;)?
     $(ins_f: $c34:tt, $c35:tt;)?
     $(ins_n: $c36:tt, $c37:tt;)?
     $(ins_pl: $c38:tt, $c39:tt;)?
     $(prep_m: $c40:tt, $c41:tt;)?
     $(prep_f: $c42:tt, $c43:tt;)?
     $(prep_n: $c44:tt, $c45:tt;)?
     $(prep_pl: $c46:tt, $c47:tt;)?
     $(short_adjective: $c48:tt, $c49:tt, $c50:tt, $c51:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "~"`: `~` after every pattern).
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
    /// Like `сведущий`: 24,417 words.
    Сведущий,
    /// Like `уместный`: 10,805 words.
    Уместный,
    /// Like `устарелый`: 9,357 words.
    Устарелый,
    /// Like `пылкий`: 8,007 words.
    Пылкий,
    /// Like `снижающийся`: 5,127 words.
    Снижающийся,
    /// Like `подверженный`: 3,768 words.
    Подверженный,
    /// Like `вольный`: 2,508 words.
    Вольный,
    /// Like `казнённый`: 994 words.
    Казнённый,
    /// Like `прямой`: 658 words.
    Прямой,
    /// Like `непринуждённый`: 489 words.
    Непринуждённый,
    /// Like `показной`: 433 words.
    Показной,
    /// Like `тайный`: 353 words.
    Тайный,
    /// Like `лисий`: 224 words.
    Лисий,
    /// Like `средний`: 174 words.
    Средний,
    /// Like `горький`: 132 words.
    Горький,
    /// Like `иродов`: 64 words.
    Иродов,
    /// Like `бойкий`: 24 words.
    Бойкий,
    /// Like `куцый`: 19 words.
    Куцый,
    /// Like `дядин`: 14 words.
    Дядин,
    /// Like `синий`: 13 words.
    Синий,
    /// Like `рад`: 12 words.
    Рад,
    /// Like `острый`: 10 words.
    Острый,
    /// Like `тяжёлый`: 9 words.
    Тяжёлый,
    /// Like `длинношеий`: 8 words.
    Длинношеий,
    /// Like `злой`: 7 words.
    Злой,
    /// Like `чёрный`: 6 words.
    Чёрный,
    /// Like `теплонагревательный`: 6 words.
    Теплонагревательный,
    /// Like `горячий`: 6 words.
    Горячий,
    /// Like `должен`: 5 words.
    Должен,
    /// Like `тяжкий`: 5 words.
    Тяжкий,
    /// Like `лёгкий`: 5 words.
    Лёгкий,
    /// Like `зелёный`: 4 words.
    Зелёный,
    /// Like `достойный`: 4 words.
    Достойный,
    /// Like `высокий`: 3 words.
    Высокий,
    /// Like `дешёвый`: 3 words.
    Дешёвый,
    /// Like `катин`: 3 words.
    Катин,
    /// Like `крутенек`: 3 words.
    Крутенек,
    /// Like `жерёбый`: 3 words.
    Жерёбый,
    /// Like `сукотая`: 3 words.
    Сукотая,
    /// Like `трёхгодовой`: 2 words.
    Трёхгодовой,
    /// Like `прищепленный`: 1 words.
    Прищепленный,
    /// Like `трёхоперационный`: 1 words.
    Трёхоперационный,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 42] = [
        Declension::Сведущий,
        Declension::Уместный,
        Declension::Устарелый,
        Declension::Пылкий,
        Declension::Снижающийся,
        Declension::Подверженный,
        Declension::Вольный,
        Declension::Казнённый,
        Declension::Прямой,
        Declension::Непринуждённый,
        Declension::Показной,
        Declension::Тайный,
        Declension::Лисий,
        Declension::Средний,
        Declension::Горький,
        Declension::Иродов,
        Declension::Бойкий,
        Declension::Куцый,
        Declension::Дядин,
        Declension::Синий,
        Declension::Рад,
        Declension::Острый,
        Declension::Тяжёлый,
        Declension::Длинношеий,
        Declension::Злой,
        Declension::Чёрный,
        Declension::Теплонагревательный,
        Declension::Горячий,
        Declension::Должен,
        Declension::Тяжкий,
        Declension::Лёгкий,
        Declension::Зелёный,
        Declension::Достойный,
        Declension::Высокий,
        Declension::Дешёвый,
        Declension::Катин,
        Declension::Крутенек,
        Declension::Жерёбый,
        Declension::Сукотая,
        Declension::Трёхгодовой,
        Declension::Прищепленный,
        Declension::Трёхоперационный,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 42] = [
    // Сведущий: 24,417 words (деревообрабатывающий, близорукий, великий, будущий, лесообрабатывающий, настоящий).
    declension!("сведущий", 24417, "~ий";
    nom_m: _, "~ий";
    nom_f: "~ая", _;
    nom_n: "~ее", _;
    nom_pl: "~ие", _;
    gen_m: "~его", _;
    gen_f: "~ей", _;
    gen_pl: "~их", _;
    dat_m: "~ему", _;
    dat_pl: "~им", _;
    acc_m: _, "~ий";
    acc_f: "~ую", _;
    ins_f: "~ей|~ею", "~ей|~ею";
    ins_pl: "~ими", _;
    prep_m: "~ем", _;
    short_adjective: "~", "~а", "~е", "~и";
    ),
    // Уместный: 10,805 words (славный, крохотный, деревянный, древесный, лесоводственный, электронный).
    declension!("уместный", 10805, "~~ый";
    nom_m: _, "~~ый";
    nom_f: "~~ая", _;
    nom_n: "~~ое", _;
    nom_pl: "~~ые", _;
    gen_m: "~~ого", _;
    gen_f: "~~ой", _;
    gen_pl: "~~ых", _;
    dat_m: "~~ому", _;
    dat_pl: "~~ым", _;
    acc_m: _, "~~ый";
    acc_f: "~~ую", _;
    ins_f: "~~ой|~~ою", "~~ой|~~ою";
    ins_pl: "~~ыми", _;
    prep_m: "~~ом", _;
    short_adjective: "~е~", "~~а", "~~о", "~~ы";
    ),
    // Устарелый: 9,357 words (браный, слабый, ласковый, деревянистый, лесистый, лесотундровый). Like Сведущий, except in 15 cells: nom_m inan, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, gen_pl anim, ….
    declension!("устарелый", 9357, "~ый", like Сведущий;
    nom_m: _, "~ый";
    nom_n: "~ое", _;
    nom_pl: "~ые", _;
    gen_m: "~ого", _;
    gen_f: "~ой", _;
    gen_pl: "~ых", _;
    dat_m: "~ому", _;
    dat_pl: "~ым", _;
    acc_m: _, "~ый";
    ins_f: "~ой|~ою", "~ой|~ою";
    ins_pl: "~ыми", _;
    prep_m: "~ом", _;
    short_adjective: _, _, "~о", "~ы";
    ),
    // Пылкий: 8,007 words (русский, тайский, жалкий, лесоводческий, лесохимический, лесотехнический). Like Уместный, except in 10 cells: nom_m inan, nom_pl anim, gen_pl anim, dat_pl anim, acc_m inan, ins_f anim, ….
    declension!("пылкий", 8007, "~~ий", like Уместный;
    nom_m: _, "~~ий";
    nom_pl: "~~ие", _;
    gen_pl: "~~их", _;
    dat_pl: "~~им", _;
    acc_m: _, "~~ий";
    ins_f: derived, derived;
    ins_pl: "~~ими", _;
    short_adjective: "~о~", _, _, "~~и";
    ),
    // Снижающийся: 5,127 words (снижающийся, невзрывающийся, неразорвавшийся, учащийся, трудящийся, полуразвалившийся). Like Сведущий with `~` after each pattern, except in 4 cells: short_adjective m, short_adjective f, short_adjective n, short_adjective pl.
    declension!("снижающийся", 5127, "~ий~", like Сведущий with "~";
    short_adjective: derived, derived, derived, derived;
    ),
    // Подверженный: 3,768 words (использованный, количественный, ученный, неприготовленный, непривязанный, неупакованный). Like Устарелый, except in 15 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("подверженный", 3768, "~ный", like Устарелый;
    nom_m: _, "~ный";
    nom_f: "~ная", _;
    nom_n: "~ное", _;
    nom_pl: "~ные", _;
    gen_m: "~ного", _;
    gen_f: "~ной", _;
    gen_pl: "~ных", _;
    dat_m: "~ному", _;
    dat_pl: "~ным", _;
    acc_m: _, "~ный";
    acc_f: "~ную", _;
    ins_f: "~ной|~ною", "~ной|~ною";
    ins_pl: "~ными", _;
    prep_m: "~ном", _;
    ),
    // Вольный: 2,508 words (лесопильный, недельный, лесозаготовительный, примирительный, древесноугольный, дровокольный). Like Уместный, except in 18 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("вольный", 2508, "~ь~ый", like Уместный;
    nom_m: _, "~ь~ый";
    nom_f: "~ь~ая", _;
    nom_n: "~ь~ое", _;
    nom_pl: "~ь~ые", _;
    gen_m: "~ь~ого", _;
    gen_f: "~ь~ой", _;
    gen_pl: "~ь~ых", _;
    dat_m: "~ь~ому", _;
    dat_pl: "~ь~ым", _;
    acc_m: _, "~ь~ый";
    acc_f: "~ь~ую", _;
    ins_f: "~ь~ой|~ь~ою", "~ь~ой|~ь~ою";
    ins_pl: "~ь~ыми", _;
    prep_m: "~ь~ом", _;
    short_adjective: _, "~ь~а", "~ь~о", "~ь~ы";
    ),
    // Казнённый: 994 words (соединённый, обызвествлённый, одушевлённый, объединённый, приобретённый, неприкреплённый). Like Уместный, except in 19 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("казнённый", 994, "~ё~ный", like Уместный;
    nom_m: _, "~ё~ный";
    nom_f: "~ё~ная", _;
    nom_n: "~ё~ное", _;
    nom_pl: "~ё~ные", _;
    gen_m: "~ё~ного", _;
    gen_f: "~ё~ной", _;
    gen_pl: "~ё~ных", _;
    dat_m: "~ё~ному", _;
    dat_pl: "~ё~ным", _;
    acc_m: _, "~ё~ный";
    acc_f: "~ё~ную", _;
    ins_f: "~ё~ной|~ё~ною", "~ё~ной|~ё~ною";
    ins_pl: "~ё~ными", _;
    prep_m: "~ё~ном", _;
    short_adjective: "~ё~", "~е~а", "~е~о", "~е~ы";
    ),
    // Прямой: 658 words (большой, баской, морской, городской, лесовой, былой). Like Устарелый, except in 2 cells: nom_m inan, acc_m inan.
    declension!("прямой", 658, "~ой", like Устарелый;
    nom_m: _, "~ой";
    acc_m: _, "~ой";
    ),
    // Непринуждённый: 489 words (сверхъестественный, неодушевлённый, несдержанный, неожиданный, недисциплинированный, непредвиденный). Like Подверженный, except in 3 cells: short_adjective f, short_adjective n, short_adjective pl.
    declension!("непринуждённый", 489, "~ный", like Подверженный;
    short_adjective: _, "~на", "~но", "~ны";
    ),
    // Показной: 433 words (лесной, лесостепной, дровяной, губной, стальной, нефтяной). Like Уместный, except in 4 cells: nom_m inan, acc_m inan, ins_f anim, ins_f inan.
    declension!("показной", 433, "~~ой", like Уместный;
    nom_m: _, "~~ой";
    acc_m: _, "~~ой";
    ins_f: derived, derived;
    ),
    // Тайный: 353 words (покойный, аварийный, спокойный, семейный, случайный, партийный). Like Уместный, except in 18 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("тайный", 353, "~й~ый", like Уместный;
    nom_m: _, "~й~ый";
    nom_f: "~й~ая", _;
    nom_n: "~й~ое", _;
    nom_pl: "~й~ые", _;
    gen_m: "~й~ого", _;
    gen_f: "~й~ой", _;
    gen_pl: "~й~ых", _;
    dat_m: "~й~ому", _;
    dat_pl: "~й~ым", _;
    acc_m: _, "~й~ый";
    acc_f: "~й~ую", _;
    ins_f: "~й~ой|~й~ою", "~й~ой|~й~ою";
    ins_pl: "~й~ыми", _;
    prep_m: "~й~ом", _;
    short_adjective: _, "~й~а", "~й~о", "~й~ы";
    ),
    // Лисий: 224 words (бабий, лесничий, третий, собачий, заячий, рыбий). Like Иродов with `ий` after each pattern, except in 13 cells: nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, gen_pl anim, ….
    declension!("лисий", 224, "~ий", like Иродов with "ий";
    nom_f: "~ья", _;
    nom_n: "~ье", _;
    nom_pl: "~ьи", _;
    gen_m: "~ьего", _;
    gen_f: "~ьей", _;
    gen_pl: "~ьих", _;
    dat_m: "~ьему", _;
    dat_pl: "~ьим", _;
    acc_f: "~ью", _;
    ins_f: "~ьей|~ьею", "~ьей|~ьею";
    ins_pl: "~ьими", _;
    prep_m: "~ьем", _;
    ),
    // Средний: 174 words (субботний, средний, последний, задний, предпоследний, посторонний). Like Пылкий, except in 10 cells: nom_f anim, nom_n anim, gen_m anim, gen_f anim, dat_m anim, acc_f anim, ….
    declension!("средний", 174, "~~ий", like Пылкий;
    nom_f: "~~яя", _;
    nom_n: "~~ее", _;
    gen_m: "~~его", _;
    gen_f: "~~ей", _;
    dat_m: "~~ему", _;
    acc_f: "~~юю", _;
    prep_m: "~~ем", _;
    short_adjective: "~е~", "~~я", "~~е", _;
    ),
    // Горький: 132 words (горький, серенький, пёстренький, тоненький, синенький, коротенький). Like Вольный, except in 7 cells: nom_m inan, nom_pl anim, gen_pl anim, dat_pl anim, acc_m inan, ins_pl anim, ….
    declension!("горький", 132, "~ь~ий", like Вольный;
    nom_m: _, "~ь~ий";
    nom_pl: "~ь~ие", _;
    gen_pl: "~ь~их", _;
    dat_pl: "~ь~им", _;
    acc_m: _, "~ь~ий";
    ins_pl: "~ь~ими", _;
    short_adjective: _, _, _, "~ь~и";
    ),
    // Иродов: 64 words (сукин, христов, адамов, хуев, булев, отцов). Like Устарелый, except in 12 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, dat_m anim, ….
    declension!("иродов", 64, "~", like Устарелый;
    nom_m: _, "~";
    nom_f: "~а", _;
    nom_n: "~о", _;
    nom_pl: "~ы", _;
    gen_m: "~а", _;
    dat_m: "~у", _;
    acc_m: _, "~";
    acc_f: "~у", _;
    short_adjective: derived, derived, derived, derived;
    ),
    // Бойкий: 24 words (бойкий, клейкий, стойкий, износостойкий, огнестойкий, морозостойкий). Like Тайный, except in 7 cells: nom_m inan, nom_pl anim, gen_pl anim, dat_pl anim, acc_m inan, ins_pl anim, ….
    declension!("бойкий", 24, "~й~ий", like Тайный;
    nom_m: _, "~й~ий";
    nom_pl: "~й~ие", _;
    gen_pl: "~й~их", _;
    dat_pl: "~й~им", _;
    acc_m: _, "~й~ий";
    ins_pl: "~й~ими", _;
    short_adjective: _, _, _, "~й~и";
    ),
    // Куцый: 19 words (бледнолицый, куцый, круглолицый, пухлолицый, багроволицый, белолицый). Like Сведущий, except in 7 cells: nom_m inan, nom_pl anim, gen_pl anim, dat_pl anim, acc_m inan, ins_pl anim, ….
    declension!("куцый", 19, "~ый", like Сведущий;
    nom_m: _, "~ый";
    nom_pl: "~ые", _;
    gen_pl: "~ых", _;
    dat_pl: "~ым", _;
    acc_m: _, "~ый";
    ins_pl: "~ыми", _;
    short_adjective: _, _, _, "~ы";
    ),
    // Дядин: 14 words (мамин, женин, никитин, дядин, бабушкин, папин). Like Иродов, except in 2 cells: gen_m anim, dat_m anim.
    declension!("дядин", 14, "~", like Иродов;
    gen_m: "~ого", _;
    dat_m: "~ому", _;
    ),
    // Синий: 13 words (синий, дальний, лососий, тёмно-синий, крокодилий, сверхдальний). Like Сведущий, except in 4 cells: nom_f anim, acc_f anim, short_adjective m, short_adjective f.
    declension!("синий", 13, "~ий", like Сведущий;
    nom_f: "~яя", _;
    acc_f: "~юю", _;
    short_adjective: "~ь", "~я", _, _;
    ),
    // Рад: 12 words (широк, мал, рад, велик, намерен, наслышан). Like Устарелый, except in 15 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("рад", 12, "~", like Устарелый;
    nom_m: _, "~";
    nom_f: derived, _;
    nom_n: derived, _;
    nom_pl: derived, _;
    gen_m: derived, _;
    gen_f: derived, _;
    gen_pl: derived, _;
    dat_m: derived, _;
    dat_pl: derived, _;
    acc_m: _, "~";
    acc_f: derived, _;
    ins_f: derived, derived;
    ins_pl: derived, _;
    prep_m: derived, _;
    ),
    // Острый: 10 words (острый, неумный, умный, хитрый, шустрый, нехитрый). Like Уместный, except in 3 cells: short_adjective m, short_adjective n, short_adjective pl.
    declension!("острый", 10, "~~ый", like Уместный;
    short_adjective: "~ё~", _, "~~о|~~о", "~~ы|~~ы";
    ),
    // Тяжёлый: 9 words (жёлтый, твёрдый, мёртвый, тяжёлый, пёстрый, полумёртвый). Like Казнённый, except in 15 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("тяжёлый", 9, "~ё~ый", like Казнённый;
    nom_m: _, "~ё~ый";
    nom_f: "~ё~ая", _;
    nom_n: "~ё~ое", _;
    nom_pl: "~ё~ые", _;
    gen_m: "~ё~ого", _;
    gen_f: "~ё~ой", _;
    gen_pl: "~ё~ых", _;
    dat_m: "~ё~ому", _;
    dat_pl: "~ё~ым", _;
    acc_m: _, "~ё~ый";
    acc_f: "~ё~ую", _;
    ins_f: "~ё~ой|~ё~ою", "~ё~ой|~ё~ою";
    ins_pl: "~ё~ыми", _;
    prep_m: "~ё~ом", _;
    ),
    // Длинношеий: 8 words (длинношеий, голошеий, долгошеий, короткошеий, кривошеий, толстошеий). Like Синий, except in 1 cell: short_adjective m.
    declension!("длинношеий", 8, "~ий", like Синий;
    short_adjective: "~й", _, _, _;
    ),
    // Злой: 7 words (злой, смешной, сплошной, вскрышной, незлой, пресмешной). Like Уместный, except in 3 cells: nom_m inan, acc_m inan, short_adjective m.
    declension!("злой", 7, "~~ой", like Уместный;
    nom_m: _, "~~ой";
    acc_m: _, "~~ой";
    short_adjective: "~о~", _, _, _;
    ),
    // Чёрный: 6 words (чёрный, тёмный, тёплый, полутёмный, нечёрный, белочёрный).
    declension!("чёрный", 6, "~ё~~ый";
    nom_m: _, "~ё~~ый";
    nom_f: "~ё~~ая", _;
    nom_n: "~ё~~ое", _;
    nom_pl: "~ё~~ые", _;
    gen_m: "~ё~~ого", _;
    gen_f: "~ё~~ой", _;
    gen_pl: "~ё~~ых", _;
    dat_m: "~ё~~ому", _;
    dat_pl: "~ё~~ым", _;
    acc_m: _, "~ё~~ый";
    acc_f: "~ё~~ую", _;
    ins_f: "~ё~~ой|~ё~~ою", "~ё~~ой|~ё~~ою";
    ins_pl: "~ё~~ыми", _;
    prep_m: "~ё~~ом", _;
    short_adjective: "~ё~е~", "~е~~а", "~е~~о", "~е~~ы";
    ),
    // Теплонагревательный: 6 words (сланцевый, нахлыстовый, дёгтевый, народно-разговорный, теплонагревательный, малоуютный). Like Уместный, except in 4 cells: acc_m anim, acc_pl anim, ins_f anim, ins_f inan.
    declension!("теплонагревательный", 6, "~~ый", like Уместный;
    acc_m: "~~ый", _;
    acc_pl: "~~ые", _;
    ins_f: derived, derived;
    ),
    // Горячий: 6 words (хороший, горячий, свежий, нехороший, прехороший, наихороший). Like Сведущий, except in 1 cell: short_adjective n.
    declension!("горячий", 6, "~ий", like Сведущий;
    short_adjective: _, _, "~о", _;
    ),
    // Должен: 5 words (должен, тесен, грешен, длинен, годен). Like Рад with `е~` after each pattern, except in 4 cells: acc_m inan, short_adjective f, short_adjective n, short_adjective pl.
    declension!("должен", 5, "~е~", like Рад with "е~";
    acc_m: _, derived;
    short_adjective: _, "~~а", "~~о|~~о", "~~ы|~~ы";
    ),
    // Тяжкий: 5 words (тяжкий, лёжкий, нелёжкий, нетяжкий, сторожкий). Like Пылкий, except in 3 cells: ins_f anim, ins_f inan, short_adjective m.
    declension!("тяжкий", 5, "~~ий", like Пылкий;
    ins_f: "~~ой|~~ою", "~~ой|~~ою";
    short_adjective: "~е~", _, _, _;
    ),
    // Лёгкий: 5 words (лёгкий, жёсткий, нелёгкий, прелёгкий, сверхлёгкий). Like Чёрный, except in 8 cells: nom_m inan, nom_pl anim, gen_pl anim, dat_pl anim, acc_m inan, ins_pl anim, ….
    declension!("лёгкий", 5, "~ё~~ий", like Чёрный;
    nom_m: _, "~ё~~ий";
    nom_pl: "~ё~~ие", _;
    gen_pl: "~ё~~их", _;
    dat_pl: "~ё~~им", _;
    acc_m: _, "~ё~~ий";
    ins_pl: "~ё~~ими", _;
    short_adjective: "~ё~о~", _, _, "~е~~и";
    ),
    // Зелёный: 4 words (зелёный, весёлый, невесёлый, бешено-весёлый). Like Тяжёлый, except in 2 cells: short_adjective m, short_adjective pl.
    declension!("зелёный", 4, "~ё~ый", like Тяжёлый;
    short_adjective: "~е~", _, _, "~е~ы|~е~ы";
    ),
    // Достойный: 4 words (достойный, ейный, недостойный, сухостойный). Like Тайный, except in 1 cell: short_adjective m.
    declension!("достойный", 4, "~й~ый", like Тайный;
    short_adjective: "~и~", _, _, _;
    ),
    // Высокий: 3 words (противосвёртывающий, всемилостивейший, бывалошний). Like Дядин with `` after each pattern (`ий` after ~), except in 11 cells: nom_f anim, nom_n anim, nom_pl anim, gen_pl anim, dat_pl anim, acc_f anim, ….
    declension!("высокий", 3, "~ий", like Дядин with "", "ий" after "~";
    nom_f: "~ая", _;
    nom_n: "~ое", _;
    nom_pl: "~ие", _;
    gen_pl: "~их", _;
    dat_pl: "~им", _;
    acc_f: "~ую", _;
    ins_pl: "~ими", _;
    short_adjective: "~", "~а", "~о", "~и";
    ),
    // Дешёвый: 3 words (дешёвый, недешёвый, ультрадешёвый). Like Уместный, except in 19 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("дешёвый", 3, "~ешё~ый", like Уместный;
    nom_m: _, "~ешё~ый";
    nom_f: "~ешё~ая", _;
    nom_n: "~ешё~ое", _;
    nom_pl: "~ешё~ые", _;
    gen_m: "~ешё~ого", _;
    gen_f: "~ешё~ой", _;
    gen_pl: "~ешё~ых", _;
    dat_m: "~ешё~ому", _;
    dat_pl: "~ешё~ым", _;
    acc_m: _, "~ешё~ый";
    acc_f: "~ешё~ую", _;
    ins_f: "~ешё~ой|~ешё~ою", "~ешё~ой|~ешё~ою";
    ins_pl: "~ешё~ыми", _;
    prep_m: "~ешё~ом", _;
    short_adjective: "~ёше~", "~еше~а", "~ёше~о", "~ёше~ы";
    ),
    // Катин: 3 words (сашин, катин, танин). Like Иродов, except in 6 cells: nom_n anim, acc_m inan, acc_pl inan, ins_f anim, ins_f inan, prep_m anim.
    declension!("катин", 3, "~", like Иродов;
    nom_n: derived, _;
    acc_m: _, "~а";
    acc_pl: _, "~ых";
    ins_f: derived, derived;
    prep_m: "~е", _;
    ),
    // Крутенек: 3 words (крутенек, радёхонек, радёшенек). Like Рад with `е~` after each pattern, except in 3 cells: short_adjective f, short_adjective n, short_adjective pl.
    declension!("крутенек", 3, "~е~", like Рад with "е~";
    short_adjective: _, "~ь~а", "~ь~о", "~ь~и";
    ),
    // Жерёбый: 3 words (успешливый, посажёный, жерёбый). Like Устарелый, except in 2 cells: acc_m inan, acc_pl inan.
    declension!("жерёбый", 3, "~ый", like Устарелый;
    acc_m: _, "~ого|~ый";
    acc_pl: _, "~ых";
    ),
    // Сукотая: 3 words (бесприданная, сукотная, сукотая). Like Иродов with `` after each pattern (`ая` after ~), except in 10 cells: nom_f anim, nom_n anim, nom_pl anim, gen_m anim, dat_m anim, acc_m inan, ….
    declension!("сукотая", 3, "~ая", like Иродов with "", "ая" after "~";
    nom_f: "~ая", _;
    nom_n: derived, _;
    nom_pl: "~ые", _;
    gen_m: derived, _;
    dat_m: derived, _;
    acc_m: _, derived;
    acc_f: "~ую", _;
    prep_m: derived, _;
    short_adjective: _, "~а", _, "~ы";
    ),
    // Трёхгодовой: 2 words (трёхгодовой, трёхчасовой). Like Снижающийся, except in 15 cells: nom_m inan, nom_f anim, nom_n anim, nom_pl anim, gen_m anim, gen_f anim, ….
    declension!("трёхгодовой", 2, "~ё~ой", like Снижающийся;
    nom_m: _, "~е~ой";
    nom_f: "~е~ая", _;
    nom_n: "~е~ое", _;
    nom_pl: "~е~ые", _;
    gen_m: "~е~ого", _;
    gen_f: "~е~ой", _;
    gen_pl: "~е~ых", _;
    dat_m: "~е~ому", _;
    dat_pl: "~е~ым", _;
    acc_m: _, "~е~ой";
    acc_f: "~е~ую", _;
    ins_f: "~е~ой|~е~ою", "~е~ой|~е~ою";
    ins_pl: "~е~ыми", _;
    prep_m: "~е~ом", _;
    ),
    // Прищепленный: 1 words (прищепленный). Like Казнённый, except in 5 cells: ins_f anim, ins_f inan, short_adjective f, short_adjective n, short_adjective pl.
    declension!("прищепленный", 1, "~е~ный", like Казнённый;
    ins_f: derived, derived;
    short_adjective: _, "~ё~а", "~ё~о", "~ё~ы";
    ),
    // Трёхоперационный: 1 words (трёхоперационный). Like Трёхгодовой, except in 2 cells: nom_m inan, acc_m inan.
    declension!("трёхоперационный", 1, "~ё~ый", like Трёхгодовой;
    nom_m: _, "~е~ый";
    acc_m: _, "~е~ый";
    ),
];
