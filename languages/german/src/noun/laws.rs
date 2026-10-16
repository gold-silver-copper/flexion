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
    agreements {
        law11 = [("is", "b") => "is", ("is", "c") => "is", ("is", "d") => "is", ("is", "e") => "is", ("is", "g") => "is", ("is", "i") => "is", ("is", "k") => "is", ("is", "l") => "¦", ("is", "m") => "is", ("is", "n") => "is", ("is", "p") => "is", ("is", "r") => "is", ("is", "s") => "¦", ("is", "t") => "is", ("is", "u") => "is", ("is", "v") => "is", ("is", "x") => "¦", ("is", "z") => "is", ("is", "ä") => "is", ("o", "b") => "o", ("o", "c") => "o", ("o", "d") => "¦", ("o", "e") => "o", ("o", "g") => "o", ("o", "i") => "¦", ("o", "k") => "¦", ("o", "l") => "¦", ("o", "m") => "o", ("o", "n") => "¦", ("o", "p") => "¦", ("o", "r") => "o", ("o", "s") => "o", ("o", "t") => "¦", ("o", "u") => "o", ("o", "v") => "o", ("o", "x") => "o", ("o", "z") => "¦", ("o", "ä") => "o", ("um", "b") => "¦", ("um", "c") => "¦", ("um", "d") => "¦", ("um", "e") => "¦", ("um", "g") => "¦", ("um", "i") => "¦", ("um", "k") => "¦", ("um", "l") => "¦", ("um", "m") => "¦", ("um", "n") => "¦", ("um", "p") => "um", ("um", "r") => "¦", ("um", "s") => "¦", ("um", "t") => "¦", ("um", "u") => "¦", ("um", "v") => "¦", ("um", "x") => "um", ("um", "z") => "um", ("um", "ä") => "¦", ("us", "b") => "¦", ("us", "c") => "¦", ("us", "d") => "¦", ("us", "e") => "¦", ("us", "g") => "¦", ("us", "i") => "¦", ("us", "k") => "¦", ("us", "l") => "¦", ("us", "m") => "¦", ("us", "n") => "¦", ("us", "p") => "¦", ("us", "r") => "¦", ("us", "s") => "¦", ("us", "t") => "¦", ("us", "u") => "us", ("us", "v") => "us", ("us", "x") => "us", ("us", "z") => "us", ("us", "ä") => "us"];
        law33 = [("k", "a") => "k", ("k", "e") => "k", ("k", "h") => "k", ("k", "i") => "z¦", ("k", "k") => "k", ("k", "l") => "k", ("k", "n") => "k", ("k", "o") => "k", ("k", "r") => "k", ("k", "t") => "k", ("k", "x") => "k", ("on", "a") => "on", ("on", "e") => "on", ("on", "h") => "¦", ("on", "i") => "¦", ("on", "k") => "¦", ("on", "l") => "¦", ("on", "n") => "¦", ("on", "o") => "¦", ("on", "r") => "¦", ("on", "t") => "¦", ("on", "x") => "¦", ("s", "a") => "s", ("s", "e") => "¦", ("s", "h") => "s", ("s", "i") => "s", ("s", "k") => "s", ("s", "l") => "s", ("s", "n") => "ti¦", ("s", "o") => "¦", ("s", "r") => "s", ("s", "t") => "s", ("s", "x") => "s", ("ß", "a") => "ss¦", ("ß", "e") => "ss¦", ("ß", "h") => "ß", ("ß", "i") => "ss¦", ("ß", "k") => "ß", ("ß", "l") => "ß", ("ß", "n") => "ß", ("ß", "o") => "ss¦", ("ß", "r") => "ß", ("ß", "t") => "ß", ("ß", "x") => "ß"];
    }
    rules {
        // t+n → t+en: 4,521 words (wissenschaft, welt, schlucht, abart), 0 forms wrong.
        law0: "t+n" => "t+en";
        // g+n → g+en: 8,443 words (bestellung, teilung, abteilung, zerstörung), 0 forms wrong.
        law1: "g+n" => "g+en";
        // in+en → in+nen: 7,250 words (ärztin, freundin, ehegattin, gattin), 1,119 forms wrong.
        law2: "in+en" => "in+nen";
        // en+n → en+: 4,896 words (possessivpronomen, garten, boden, rappen), 0 forms wrong.
        law3: "en+n" => "en+";
        // on+n → on+en: 2,211 words (auktion, definition, aberration, abrasion), 0 forms wrong.
        law4: "on+n" => "on+en";
        // h+n → h+en: 26 words (milch, couch, blötsch, kirchweih), 0 forms wrong.
        law5: "h+n" => "h+en";
        // k+n → k+en: 647 words (gebrauchsmusik, verbrauchsmusik, depositenbank, datenbank), 0 forms wrong.
        law6: "k+n" => "k+en";
        // m+n → m+en: 266 words (farm, form, norm, reform), 0 forms wrong.
        law7: "m+n" => "m+en";
        // d+n → d+en: 127 words (maid, schuld, gegend, jagd), 0 forms wrong.
        law8: "d+n" => "d+en";
        // z+n → z+en: 445 words (terz, nelkenwurz, milz, kadenz), 0 forms wrong.
        law9: "z+n" => "z+en";
        // s+n → s+en: 20 words (eins, geiss, stewardess, sechs), 0 forms wrong.
        law10: "s+n" => "s+en";
        // um → ∅ after [b|c|d|e|g|i|k|l|m|n|r|s|t|u|v|ä]; us → ∅ after [b|c|d|e|g|i|k|l|m|n|p|r|s|t]; o → ∅ after [d|i|k|l|n|p|t|z]; is → ∅ after [l|s|x] in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$, +[a|e|i|o|u|y|ä|ö|ü]n: 1,182 words (impressum, vakuum, amaryllis, elysium), 17 forms wrong.
        law11: "{x:[is|o|um|us]}" => "{x|law11[l]}" / "{l:[b|c|d|e|g|i|k|l|m|n|p|r|s|t|u|v|x|z|ä]}" _ "+{[a|e|i|o|u|y|ä|ö|ü]}ˇ$" | "+{[a|e|i|o|u|y|ä|ö|ü]}{[n]}{![ˇ]*}ˇ$";
        // ur+n → ur+en: 726 words (architektur, natur, spur, temperatur), 0 forms wrong.
        law12: "ur+n" => "ur+en";
        // f+n → f+en: 10 words (elf, zwölf, fünf, startelf), 0 forms wrong.
        law13: "f+n" => "f+en";
        // or+n → or+en: 5 words (isolator, retardationsfaktor, retardierungsfaktor, punktionisator), 1 forms wrong.
        law14: "or+n" => "or+en";
        // i+n → i+en: 620 words (drei, lei, zwei, partei), 9 forms wrong.
        law15: "i+n" => "i+en";
        // o+e → o+s: 682 words (ufo, velo, moskito, abo), 25 forms wrong.
        law16: "o+e" => "o+s";
        // a+n → a+s: 414 words (digitalkamera, diaspora, bosna, aba), 8 forms wrong.
        law17: "a+n" => "a+s";
        // hr+n → hr+en: 209 words (uhr, wehr, gefahr, feuerwehr), 0 forms wrong.
        law18: "hr+n" => "hr+en";
        // ss+s → ss+es: 41 words (cross, miss, dünnschiss, beschiss), 0 forms wrong.
        law19: "ss+s" => "ss+es";
        // al+n → al+en: 6 words (moral, qual, todesqual, höllenqual), 0 forms wrong.
        law20: "al+n" => "al+en";
        // hl+n → hl+en: 203 words (zahl, wahl, auswahl, ordnungszahl), 4 forms wrong.
        law21: "hl+n" => "hl+en";
        // ll+n → ll+en: 5 words (null, nachtigall, mamsell, margell), 0 forms wrong.
        law22: "ll+n" => "ll+en";
        // an+n → an+en: 6 words (membran, zellmembran, tulipan, thylakoidmembran), 0 forms wrong.
        law23: "an+n" => "an+en";
        // y+n → y+s: 103 words (city, lady, party, lobby), 0 forms wrong.
        law24: "y+n" => "y+s";
        // ß+s → +ss: 108 words (haß, biß, naß, schiß), 0 forms wrong.
        law25: "ß+s" => "+ss";
        // er+er → er+: 44 words (bruder, stiefbruder, halbbruder, blutsbruder), 4 forms wrong.
        law26: "er+er" => "er+";
        // u+n → u+en: 176 words (putzfrau, frau, ehefrau, au), 0 forms wrong.
        law27: "u+n" => "u+en";
        // p+n → p+s: 38 words (soap, app, op, krepp), 0 forms wrong.
        law28: "p+n" => "p+s";
        // x+n → x+en: 47 words (box, phalanx, sandbox, kühlbox), 0 forms wrong.
        law29: "x+n" => "x+en";
        // y+e → y+s: 171 words (tommy, baby, boy, brandy), 10 forms wrong.
        law30: "y+e" => "y+s";
        // hn+n → hn+en: 138 words (u-bahn, autobahn, bahn, straßenbahn), 0 forms wrong.
        law31: "hn+n" => "hn+en";
        // ür+n → ür+en: 140 words (tür, falltür, hintertür, kür), 0 forms wrong.
        law32: "ür+n" => "ür+en";
        // on → ∅ after [h|i|k|l|n|o|r|t|x]; s → ∅ after [e|o]; ß → ss after [a|e|i|o]; s → ti after n; k → z after i in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$: 130 words (lexikon, macron, mitochondrion, oxymoron), 4 forms wrong.
        law33: "{x:[k|on|s|ß]}" => "{x|law33[l]}" / "{l:[a|e|h|i|k|l|n|o|r|t|x]}" _ "+{[a|e|i|o|u|y|ä|ö|ü]}ˇ$";
        // é+en → é+s: 71 words (café, exposé, soufflé, kommuniqué), 0 forms wrong.
        law34: "é+en" => "é+s";
        // é+n → é+s: 7 words (macramé, rosé, consommé, croisé), 0 forms wrong.
        law35: "é+n" => "é+s";
        // é+e → é+s: 71 words (café, exposé, soufflé, kommuniqué), 0 forms wrong.
        law36: "é+e" => "é+s";
        // w+n → w+s: 45 words (show, talkshow, one-man-show, castingshow), 0 forms wrong.
        law37: "w+n" => "w+s";
        // o+i → +i: 6 words (mafioso, exkasso, clavicembalo, guerrigliero), 0 forms wrong.
        law38: "o+i" => "+i";
        // ds+en → ds+: 34 words (fonds, shakehands, dachfonds, indexfonds), 0 forms wrong.
        law39: "ds+en" => "ds+";
        // ki+en → ki+s: 9 words (kaki, schoki, brocki, raki), 0 forms wrong.
        law40: "ki+en" => "ki+s";
        // mi+e → mi+s: 33 words (sashimi, gummi, kaugummi, schaumgummi), 0 forms wrong.
        law41: "mi+e" => "mi+s";
        // c+n → c+s: 6 words (franc, comic, cfa-franc, disc), 0 forms wrong.
        law42: "c+n" => "c+s";
        // e+i → +i: 27 words (generale, korporale, gondoliere, genitale), 0 forms wrong.
        law43: "e+i" => "+i";
        // ip+en → ip+ien: 27 words (partizip, prinzip, programmierprinzip, äquivalenzprinzip), 3 forms wrong.
        law44: "ip+en" => "ip+ien";
        // ok+en → ok+s: 26 words (lok, notebook, e-book, look), 0 forms wrong.
        law45: "ok+en" => "ok+s";
        // fi+en → fi+s: 25 words (profi, sofi, fiffi, fuffi), 0 forms wrong.
        law46: "fi+en" => "fi+s";
        // ok+e → ok+s: 20 words (notebook, e-book, look, webhook), 1 forms wrong.
        law47: "ok+e" => "ok+s";
        // un+n → un+en: 9 words (neun, pikneun, herzneun, herz-neun), 0 forms wrong.
        law48: "un+n" => "un+en";
        // w+e → w+s: 20 words (bungalow, interview, marshmallow, cashflow), 0 forms wrong.
        law49: "w+e" => "w+s";
        // fi+e → fi+s: 19 words (profi, fiffi, fuffi, doofi), 0 forms wrong.
        law50: "fi+e" => "fi+s";
        // wn+en → wn+s: 14 words (clown, countdown, count-down, klinikclown), 0 forms wrong.
        law51: "wn+en" => "wn+s";
        // wn+n → wn+s: 5 words (boomtown, showdown, chinatown, shutdown), 0 forms wrong.
        law52: "wn+n" => "wn+s";
        // bi+en → bi+s: 17 words (alibi, rabbi, abi, kohlrabi), 0 forms wrong.
        law53: "bi+en" => "bi+s";
        // ln+n → ln+: 16 words (röteln, bratkartoffeln, porzeln, katzenlächeln), 0 forms wrong.
        law54: "ln+n" => "ln+";
        // fa+e → fa+s: 15 words (sofa, mofa, schlafsofa, plüschsofa), 0 forms wrong.
        law55: "fa+e" => "fa+s";
        // ka+e → ka+s: 15 words (wodka, mokka, diademsifaka, alpaka), 0 forms wrong.
        law56: "ka+e" => "ka+s";
        // ts+en → ts+: 15 words (hotpants, shorts, peanuts, boxershorts), 2 forms wrong.
        law57: "ts+en" => "ts+";
        // bi+e → bi+s: 14 words (alibi, rabbi, abi, kohlrabi), 0 forms wrong.
        law58: "bi+e" => "bi+s";
        // ns+ie → n+zie: 14 words (stimulans, ingrediens, formans, adjuvans), 0 forms wrong.
        law59: "ns+ie" => "n+zie";
        // wn+e → wn+s: 14 words (clown, countdown, count-down, klinikclown), 0 forms wrong.
        law60: "wn+e" => "wn+s";
        // ü+en → ü+s: 13 words (menü, parvenü, untermenü, dreigängemenü), 0 forms wrong.
        law61: "ü+en" => "ü+s";
        // gn+en → gn+s: 12 words (design, webdesign, straßendesign, solomon-vier-gruppen-design), 0 forms wrong.
        law62: "gn+en" => "gn+s";
        // ü+e → ü+s: 13 words (menü, parvenü, untermenü, dreigängemenü), 0 forms wrong.
        law63: "ü+e" => "ü+s";
        // ga+e → ga+s: 12 words (omega, tanga, kiga, arakanga), 0 forms wrong.
        law64: "ga+e" => "ga+s";
        // gn+e → gn+s: 12 words (design, webdesign, straßendesign, solomon-vier-gruppen-design), 0 forms wrong.
        law65: "gn+e" => "gn+s";
        // ku+en → ku+s: 12 words (akku, seppuku, doku, otaku), 0 forms wrong.
        law66: "ku+en" => "ku+s";
        // ae+n → ae+: 11 words (australopithecinae, dryopithecinae, boinae, boidae), 0 forms wrong.
        law67: "ae+n" => "ae+";
        // ö+n → ö+en: 11 words (diarrhö, bö, amenorrhö, urethrorrhö), 0 forms wrong.
        law68: "ö+n" => "ö+en";
        // bu+en → bu+s after a vowel earlier in the stem: 10 words (tabu, zebu, sprachtabu, babu), 0 forms wrong.
        law69: "bu+en" => "bu+s" / "{[a|e|i|o|u|y|ä|ö|ü]}{![a|e|i|o|u|y|ä|ö|ü]*}" _ "";
        // dj+n → dj+s: 9 words (trance-dj, hiphop-dj, hip-hop-dj, house-dj), 0 forms wrong.
        law70: "dj+n" => "dj+s";
        // ic+en → ic+s: 8 words (tic, dickpic, fancomic, actioncomic), 0 forms wrong.
        law71: "ic+en" => "ic+s";
        // ah+e → ah+s: 9 words (schah, ajatollah, schahinschah, hisbollah), 0 forms wrong.
        law72: "ah+e" => "ah+s";
        // bu+e → bu+s after a vowel earlier in the stem: 9 words (tabu, zebu, sprachtabu, babu), 0 forms wrong.
        law73: "bu+e" => "bu+s" / "{[a|e|i|o|u|y|ä|ö|ü]}{![a|e|i|o|u|y|ä|ö|ü]*}" _ "";
        // ja+e → ja+s: 9 words (halleluja, ninja, maharaja, alleluja), 0 forms wrong.
        law74: "ja+e" => "ja+s";
        // wa+e → wa+s: 9 words (schwa, a-schwa, tiefschwa, bar-mizwa), 0 forms wrong.
        law75: "wa+e" => "wa+s";
        // ic+e → ic+s: 8 words (tic, dickpic, fancomic, actioncomic), 0 forms wrong.
        law76: "ic+e" => "ic+s";
        // ku+e → ku+s: 8 words (akku, otaku, bhikku, nunchaku), 0 forms wrong.
        law77: "ku+e" => "ku+s";
        // ou+en → ou+s: 7 words (tennou, clou, bijou, filou), 0 forms wrong.
        law78: "ou+en" => "ou+s";
        // ru+en → ru+s: 7 words (guru, cru, kaninchenkänguru, börsenguru), 0 forms wrong.
        law79: "ru+en" => "ru+s";
        // cs+en → cs+: 6 words (abenomics, basics, paralympics, lyrics), 0 forms wrong.
        law80: "cs+en" => "cs+";
        // [h|y|s|j|i]+ä → +ot: 6 words (aliya, alija, chanukkia, mesusa), 0 forms wrong.
        law81: "{x:[h|y|s|j|i]}+ä" => "{x}+ot";
        // hu+en → hu+s: 6 words (uhu, tohuwabohu, virginiauhu, bhikschu), 0 forms wrong.
        law82: "hu+en" => "hu+s";
        // ou+e → ou+s: 6 words (tennou, clou, bijou, filou), 0 forms wrong.
        law83: "ou+e" => "ou+s";
        // ru+e → ru+s: 6 words (guru, kaninchenkänguru, börsenguru, breitkopfkänguru), 0 forms wrong.
        law84: "ru+e" => "ru+s";
        // a+i → +i: 5 words (isba, stotinka, papirossa, naliwka), 0 forms wrong.
        law85: "a+i" => "+i";
        // ba+e → ba+s: 5 words (samba, kiba, kahba, rambazamba), 0 forms wrong.
        law86: "ba+e" => "ba+s";
        // hu+e → hu+s: 5 words (uhu, tohuwabohu, virginiauhu, bhikschu), 0 forms wrong.
        law87: "hu+e" => "hu+s";
    }
};
