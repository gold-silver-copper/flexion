//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`), on a letter a `%` copies (`°`), on a class
//! letter an ending writes (harmony, by the nearest letter of a tier before
//! it as written), or inside a stem that alternates (`ˇ`, by what follows
//! it): every form a declension builds goes through them, in order, so one declension
//! serves words whose stems change its endings. Each with the words whose
//! forms it gives, and how many of the dictionary's forms it gets wrong
//! (those are listed in `irregular.rs`). Written in `sound-law`'s notation
//! (`"target" => "replacement" / "before" _ "after"`; `{x|law3}`, a
//! captured letter through that law's map) and compiled when this crate is
//! built.

use flexion::sound_law::Program;

/// The laws, in order.
pub(super) const LAWS: Program<'static> = flexion::laws! {
    classes {}
    rules {
        // k+y → k+i: 22 words (orka, mokka, siarka, szermierka), 0 forms wrong.
        law0: "k+y" => "k+i";
        // k+z → +c: 15 words (mokka, krzemionka, krowianka, czerwonka), 0 forms wrong.
        law1: "k+z" => "+c";
        // [r|c]+i → +z: 1,060 words (barter, zero, spacer, war), 17 forms wrong.
        law2: "{x:[r|c]}+i" => "{x}+z";
        // d+i → d+zi: 651 words (pseudo, standard, absurd, keyboard), 0 forms wrong.
        law3: "d+i" => "d+zi";
        // d+ze → d+zie: 33 words (nerd, steward, bastard, druid), 0 forms wrong.
        law4: "d+ze" => "d+zie";
        // s+ci → +ści: 5 words (angst, oset, chrzęst, rzęst), 0 forms wrong.
        law5: "s+ci" => "+ści";
        // l+y → l+i: 396 words (bel, cebula, nauczyciel, pula), 0 forms wrong.
        law6: "l+y" => "l+i";
        // i+y → i+: 402 words (paulownia, śródmózgowie, stulecie, próżnia), 0 forms wrong.
        law7: "i+y" => "i+";
        // st+z → +ści: 40 words (kapusta, pasta, lista, fiesta), 0 forms wrong.
        law8: "st+z" => "+ści";
        // g+y → g+i: 149 words (manga, omega, fuga, toga), 0 forms wrong.
        law9: "g+y" => "g+i";
        // t+z → +ci: 351 words (arietta, kobieta, herbata, rata), 0 forms wrong.
        law10: "t+z" => "+ci";
        // ł+i → +l: 232 words (muskuł, muł, pedał, brokuł), 0 forms wrong.
        law11: "ł+i" => "+l";
        // h+i → h+: 35 words (echo, parch, rancho, ucho), 0 forms wrong.
        law12: "h+i" => "h+";
        // l+ie → l+u: 77 words (industrial, benzyl, earl, amyl), 0 forms wrong.
        law13: "l+ie" => "l+u";
        // y+eg → +eg: 49 words (prosty, pierwszy, luty, czwarty), 0 forms wrong.
        law14: "y+eg" => "+eg";
        // g+z → +dz: 133 words (podłoga, manga, omega, fuga), 11 forms wrong.
        law15: "g+z" => "+dz";
        // h+ze → h+u: 149 words (loch, mach, mech, brach), 0 forms wrong.
        law16: "h+ze" => "h+u";
        // c+s → +s: 128 words (mucha, marcha, pacha, locha), 0 forms wrong.
        law17: "c+s" => "+s";
        // ż+i → ż+y: 67 words (reportaż, witraż, makijaż, szantaż), 0 forms wrong.
        law18: "ż+i" => "ż+y";
        // ś+e → +sie: 35 words (tatuś, miś, zośka, foryś), 0 forms wrong.
        law19: "ś+e" => "+sie";
        // j+y → j+e: 19 words (rój, gronostaj, nastrój, zdrój), 0 forms wrong.
        law20: "j+y" => "j+e";
        // e+m → +ym: 20 words (dane, awaryjne, okrytonasienne, nagonasienne), 0 forms wrong.
        law21: "e+m" => "+ym";
        // e+ch → +ych after a vowel earlier in the stem: 20 words (dane, awaryjne, okrytonasienne, nagonasienne), 0 forms wrong.
        law22: "e+ch" => "+ych" / "{[a|e|i|o|u|y|ó|ą|ę]}{![a|e|i|o|u|y|ó|ą|ę]*}" _ "";
        // j+ie → j+u: 10 words (samuraj, wuj, jajo, gronostaj), 0 forms wrong.
        law23: "j+ie" => "j+u";
        // ci+i → ci+: 15 words (ciuchcia, ciufcia, gacie, córuńcia), 0 forms wrong.
        law24: "ci+i" => "ci+";
        // d+zy → d+zi: 29 words (nerd, steward, bastard, druid), 0 forms wrong.
        law25: "d+zy" => "d+zi";
        // s+l → +śl: 16 words (zmysł, umysł, pomysł, przemysł), 0 forms wrong.
        law26: "s+l" => "+śl";
        // ź+e → +zie: 20 words (łabędź, niedźwiedź, śledź, piędź), 0 forms wrong.
        law27: "ź+e" => "+zie";
        // si+i → si+: 7 words (pusia, trusia, strojnisia, rąsia), 0 forms wrong.
        law28: "si+i" => "si+";
        // ź+i → +zi: 22 words (łabędź, niedźwiedź, śledź, piędź), 0 forms wrong.
        law29: "ź+i" => "+zi";
        // ź+a → +zia: 15 words (łabędź, niedźwiedź, śledź, piędź), 0 forms wrong.
        law30: "ź+a" => "+zia";
        // ź+o → +zio: 15 words (łabędź, niedźwiedź, śledź, piędź), 0 forms wrong.
        law31: "ź+o" => "+zio";
        // j+ze → j+u: 18 words (rój, nastrój, zdrój, bój), 0 forms wrong.
        law32: "j+ze" => "j+u";
        // zy+e → z+e: 6 words (pierwszy, pieszy, starszy, tutejszy), 0 forms wrong.
        law33: "zy+e" => "z+e";
        // dz+i → dz+y: 11 words (bryndza, rdza, sadza, wiedza), 0 forms wrong.
        law34: "dz+i" => "dz+y";
        // yg+ie → y+dze after a vowel earlier in the stem: 10 words (ostryga, łodyga, kwadryga, intryga), 1 forms wrong.
        law35: "yg+ie" => "y+dze" / "{[a|e|i|o|u|y|ó|ą|ę]}{![a|e|i|o|u|y|ó|ą|ę]*}" _ "";
        // zi+i → zi+: 5 words (buzia, młodzie, bazia, gandzia), 0 forms wrong.
        law36: "zi+i" => "zi+";
        // lg+ie → l+dze: 10 words (alga, wilga, felga, folga), 1 forms wrong.
        law37: "lg+ie" => "l+dze";
        // zd+z → +źdz: 9 words (pizda, drozd, uzda, bruzda), 0 forms wrong.
        law38: "zd+z" => "+źdz";
        // ch+zi → +sz: 7 words (monarcha, oligarcha, patriarcha, hierarcha), 0 forms wrong.
        law39: "ch+zi" => "+sz";
        // z+l → +źl: 7 words (węzeł, gzło, giezło, gruzeł), 0 forms wrong.
        law40: "z+l" => "+źl";
    }
};
