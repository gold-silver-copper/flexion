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
        // k+e → k+ie: 1,368 words (beck, pik, wapiennik, malik), 22 forms wrong.
        law0: "k+e" => "k+ie";
        // e → ∅ after [b|c|d|i|j|l|m|n|p|r|s|t|w|z|ł|ż] in alternating stems (ˇ), before +.[a|e|i|ó], +.o., c+.$, c+.[^a|e|i|o|u|y|ó|ą|ę], cz+., [k|l|r|w|z]+.: 551 words (aleksander, melchizedek, marek, turek), 305 forms wrong.
        law1: "e" => "¦" / "{[b|c|d|i|j|l|m|n|p|r|s|t|w|z|ł|ż]}" _ "+{![+|ˇ|¦|°|‹]}{[a|e|i|ó]}{![ˇ]*}ˇ$" | "+{![+|ˇ|¦|°|‹]}{[o]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[c]}+{![+|ˇ|¦|°|‹]}ˇ$" | "{[c]}+{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}{![ˇ]*}ˇ$" | "{[c]}{[z]}+{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[k|l|r|w|z]}+{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // r+i → r+z: 252 words (abdera, canberra, laura, ankara), 0 forms wrong.
        law2: "r+i" => "r+z";
        // t+i → +ci: 236 words (malta, alberta, stuttgart, basilicata), 0 forms wrong.
        law3: "t+i" => "+ci";
        // s+u → s+ie: 496 words (achilles, zeus, adonis, sokrates), 24 forms wrong.
        law4: "s+u" => "s+ie";
        // ł+i → +l: 257 words (bogumiła, reguła, siła, domagała), 0 forms wrong.
        law5: "ł+i" => "+l";
        // ł+z → +l: 5 words (ciepły, cały, goły, rychły), 0 forms wrong.
        law6: "ł+z" => "+l";
        // d+i → d+zi: 302 words (edward, rwanda, uganda, svalbard), 0 forms wrong.
        law7: "d+i" => "d+zi";
        // [k|g|l]+y → +i: 562 words (lusaka, polska, alaska, nebraska), 0 forms wrong.
        law8: "{x:[k|g|l]}+y" => "{x}+i";
        // k+zi → +c: 161 words (lusaka, polska, alaska, nebraska), 0 forms wrong.
        law9: "k+zi" => "+c";
        // g+z → +dz: 56 words (kinga, olga, helga, wołga), 0 forms wrong.
        law10: "g+z" => "+dz";
        // ł+u → +le: 184 words (michał, bajkał, rafał, kozioł), 9 forms wrong.
        law11: "ł+u" => "+le";
        // n+ca → +ńca: 152 words (chowaniec, marciniec, szymaniec, wawrzyniec), 0 forms wrong.
        law12: "n+ca" => "+ńca";
        // n+co → +ńco: 152 words (chowaniec, marciniec, szymaniec, wawrzyniec), 0 forms wrong.
        law13: "n+co" => "+ńco";
        // n+có → +ńcó: 152 words (chowaniec, marciniec, szymaniec, wawrzyniec), 0 forms wrong.
        law14: "n+có" => "+ńcó";
        // l+ie → l+i: 122 words (angola, kampala, uppsala, lula), 0 forms wrong.
        law15: "l+ie" => "l+i";
        // ch+i → +sz: 95 words (macha, halacha, pascha, sacha), 0 forms wrong.
        law16: "ch+i" => "+sz";
        // [b|un|p|aw]+u → +ie: 185 words (jakub, grab, hiob, deneb), 7 forms wrong.
        law17: "{x:[b|un|p|aw]}+u" => "{x}+ie";
        // ic+ze → ic+y: 47 words (spica, podgorica, legnica, pilica), 0 forms wrong.
        law18: "ic+ze" => "ic+y";
        // s+ci → +ści: 29 words (ernest, august, augusta, drost), 2 forms wrong.
        law19: "s+ci" => "+ści";
        // ag+ie → a+dze: 38 words (malaga, praga, aga, waga), 2 forms wrong.
        law20: "ag+ie" => "a+dze";
        // ęg+ie → ę+dze: 38 words (dołęga, wałęga, potęga, wilczęga), 2 forms wrong.
        law21: "ęg+ie" => "ę+dze";
        // [sz|cz]+i → +: 27 words (kalisz, pisz, maciesza, przasnysz), 0 forms wrong.
        law22: "{x:[sz|cz]}+i" => "{x}+";
        // zn+ie → +źnie: 28 words (jaworzno, drezno, gniezno, wileńszczyzna), 1 forms wrong.
        law23: "zn+ie" => "+źnie";
        // wk+ie → w+ce: 26 words (antonówka, marchewka, śliwka, kawka), 1 forms wrong.
        law24: "wk+ie" => "w+ce";
        // ej+i → e+i: 5 words (freja, adygeja, chaldeja, amalteja), 0 forms wrong.
        law25: "ej+i" => "e+i";
        // łk+ie → ł+ce: 23 words (jaskółka, gorzałka, żyłka, nawałka), 2 forms wrong.
        law26: "łk+ie" => "ł+ce";
        // yg+ie → y+dze: 21 words (ryga, łodyga, wojdyga, fotyga), 1 forms wrong.
        law27: "yg+ie" => "y+dze";
        // zi+i → zi+: 14 words (madzia, rędzia, kuzia, kędzia), 0 forms wrong.
        law28: "zi+i" => "zi+";
        // rz+ie → rz+u: 8 words (zgierz, wieprz, sandomierz, żytomierz), 0 forms wrong.
        law29: "rz+ie" => "rz+u";
        // ug+ie → u+dze after a vowel earlier in the stem: 15 words (kaluga, cecuga, feruga, łazuga), 0 forms wrong.
        law30: "ug+ie" => "u+dze" / "{[a|e|i|o|u|y|ó|ą|ę]}{![a|e|i|o|u|y|ó|ą|ę]*}" _ "";
        // ig+ie → i+dze: 15 words (jadwiga, iga, szeliga, burliga), 1 forms wrong.
        law31: "ig+ie" => "i+dze";
        // ym+u → ym+ie after a vowel earlier in the stem: 10 words (korzym, borzym, kurzdym, nieoczym), 0 forms wrong.
        law32: "ym+u" => "ym+ie" / "{[a|e|i|o|u|y|ó|ą|ę]}{![a|e|i|o|u|y|ó|ą|ę]*}" _ "";
        // jk+ie → j+ce: 12 words (jamajka, cyrenajka, majka, sojka), 1 forms wrong.
        law33: "jk+ie" => "j+ce";
        // ż+i → ż+: 8 words (paryż, woroneż, krzyż, chociebuż), 0 forms wrong.
        law34: "ż+i" => "ż+";
        // pk+ie → p+ce: 12 words (konopka, lipka, skorupka, rzepka), 0 forms wrong.
        law35: "pk+ie" => "p+ce";
        // [ż|yc]+ze → +y: 20 words (kambodża, powierża, dzieża, giża), 0 forms wrong.
        law36: "{x:[ż|yc]}+ze" => "{x}+y";
        // ód+a → +oda: 11 words (zaród, mrzygłód, pogłód, wrzód), 1 forms wrong.
        law37: "ód+a" => "+oda";
        // [im|uz]+u → +ie: 16 words (trochim, tuzim, radzim, szulim), 0 forms wrong.
        law38: "{x:[im|uz]}+u" => "{x}+ie";
        // rz+i → rz+y: 7 words (zorza, powierza, kurza, burza), 0 forms wrong.
        law39: "rz+i" => "rz+y";
        // zd+z → +źdz: 7 words (drozd, guzda, zagozda, gorazd), 0 forms wrong.
        law40: "zd+z" => "+źdz";
        // zg+ie → z+dze: 7 words (plizga, drzazga, drzyzga, miozga), 0 forms wrong.
        law41: "zg+ie" => "z+dze";
        // ąd+u → ąd+zie: 7 words (błąd, trąd, wielbłąd, myszogląd), 0 forms wrong.
        law42: "ąd+u" => "ąd+zie";
        // dz+ie → dz+u: 6 words (grudziądz, sieradz, goniądz, inowłódz), 0 forms wrong.
        law43: "dz+ie" => "dz+u";
        // [nn|oz]+u → +ie: 11 words (wassermann, seemann, hoffmann, bachmann), 0 forms wrong.
        law44: "{x:[nn|oz]}+u" => "{x}+ie";
        // s+l → +śl: 9 words (wisła, namysło, wcisło, cisło), 0 forms wrong.
        law45: "s+l" => "+śl";
        // id+u → id+zie: 5 words (dawid, światowid, norwid, kupido), 0 forms wrong.
        law46: "id+u" => "id+zie";
        // yz+u → yz+ie: 5 words (hyz, wygryz, gryz, gmyz), 0 forms wrong.
        law47: "yz+u" => "yz+ie";
    }
};
