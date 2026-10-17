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
        law4 = [("is", "b") => "is", ("is", "c") => "is", ("is", "d") => "is", ("is", "e") => "is", ("is", "g") => "is", ("is", "i") => "is", ("is", "k") => "is", ("is", "l") => "¦", ("is", "m") => "is", ("is", "n") => "is", ("is", "p") => "is", ("is", "r") => "is", ("is", "s") => "¦", ("is", "t") => "is", ("is", "u") => "is", ("is", "v") => "is", ("is", "x") => "¦", ("is", "z") => "is", ("is", "ä") => "is", ("o", "b") => "o", ("o", "c") => "o", ("o", "d") => "¦", ("o", "e") => "o", ("o", "g") => "o", ("o", "i") => "¦", ("o", "k") => "¦", ("o", "l") => "¦", ("o", "m") => "o", ("o", "n") => "¦", ("o", "p") => "¦", ("o", "r") => "o", ("o", "s") => "o", ("o", "t") => "¦", ("o", "u") => "o", ("o", "v") => "o", ("o", "x") => "o", ("o", "z") => "¦", ("o", "ä") => "o", ("um", "b") => "¦", ("um", "c") => "¦", ("um", "d") => "¦", ("um", "e") => "¦", ("um", "g") => "¦", ("um", "i") => "¦", ("um", "k") => "¦", ("um", "l") => "¦", ("um", "m") => "¦", ("um", "n") => "¦", ("um", "p") => "um", ("um", "r") => "¦", ("um", "s") => "¦", ("um", "t") => "¦", ("um", "u") => "¦", ("um", "v") => "¦", ("um", "x") => "um", ("um", "z") => "um", ("um", "ä") => "¦", ("us", "b") => "¦", ("us", "c") => "¦", ("us", "d") => "¦", ("us", "e") => "¦", ("us", "g") => "¦", ("us", "i") => "¦", ("us", "k") => "¦", ("us", "l") => "¦", ("us", "m") => "¦", ("us", "n") => "¦", ("us", "p") => "¦", ("us", "r") => "¦", ("us", "s") => "¦", ("us", "t") => "¦", ("us", "u") => "us", ("us", "v") => "us", ("us", "x") => "us", ("us", "z") => "us", ("us", "ä") => "us"];
        law17 = [("k", "a") => "k", ("k", "e") => "k", ("k", "h") => "k", ("k", "i") => "z¦", ("k", "k") => "k", ("k", "l") => "k", ("k", "n") => "k", ("k", "o") => "k", ("k", "r") => "k", ("k", "t") => "k", ("k", "x") => "k", ("on", "a") => "on", ("on", "e") => "on", ("on", "h") => "¦", ("on", "i") => "¦", ("on", "k") => "¦", ("on", "l") => "¦", ("on", "n") => "¦", ("on", "o") => "¦", ("on", "r") => "¦", ("on", "t") => "¦", ("on", "x") => "¦", ("s", "a") => "s", ("s", "e") => "¦", ("s", "h") => "s", ("s", "i") => "s", ("s", "k") => "s", ("s", "l") => "s", ("s", "n") => "ti¦", ("s", "o") => "¦", ("s", "r") => "s", ("s", "t") => "s", ("s", "x") => "s", ("ß", "a") => "ss¦", ("ß", "e") => "ss¦", ("ß", "h") => "ß", ("ß", "i") => "ss¦", ("ß", "k") => "ß", ("ß", "l") => "ß", ("ß", "n") => "ß", ("ß", "o") => "ss¦", ("ß", "r") => "ß", ("ß", "t") => "ß", ("ß", "x") => "ß"];
    }
    rules {
        // [t|g]+n → +en: 12,964 words (wissenschaft, welt, schlucht, abart), 0 forms wrong.
        law0: "{x:[t|g]}+n" => "{x}+en";
        // in+en → in+nen: 7,250 words (ärztin, freundin, ehegattin, gattin), 1,119 forms wrong.
        law1: "in+en" => "in+nen";
        // en+n → en+: 4,896 words (possessivpronomen, garten, boden, rappen), 0 forms wrong.
        law2: "en+n" => "en+";
        // [on|h|k|m|d|z|s]+n → +en: 3,742 words (auktion, definition, aberration, abrasion), 0 forms wrong.
        law3: "{x:[on|h|k|m|d|z|s]}+n" => "{x}+en";
        // um → ∅ after [b|c|d|e|g|i|k|l|m|n|r|s|t|u|v|ä]; us → ∅ after [b|c|d|e|g|i|k|l|m|n|p|r|s|t]; o → ∅ after [d|i|k|l|n|p|t|z]; is → ∅ after [l|s|x] in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$, +[a|e|i|o|u|y|ä|ö|ü]n: 1,182 words (impressum, vakuum, amaryllis, elysium), 17 forms wrong.
        law4: "{x:[is|o|um|us]}" => "{x|law4[l]}" / "{l:[b|c|d|e|g|i|k|l|m|n|p|r|s|t|u|v|x|z|ä]}" _ "+{[a|e|i|o|u|y|ä|ö|ü]}ˇ$" | "+{[a|e|i|o|u|y|ä|ö|ü]}{[n]}{![ˇ]*}ˇ$";
        // [ur|f|or|i]+n → +en: 1,361 words (architektur, natur, spur, temperatur), 10 forms wrong.
        law5: "{x:[ur|f|or|i]}+n" => "{x}+en";
        // o+e → o+s: 682 words (ufo, velo, moskito, abo), 25 forms wrong.
        law6: "o+e" => "o+s";
        // a+n → a+s: 414 words (digitalkamera, diaspora, bosna, aba), 8 forms wrong.
        law7: "a+n" => "a+s";
        // [hr|al|hl|ll|an]+n → +en: 429 words (uhr, wehr, gefahr, feuerwehr), 4 forms wrong.
        law8: "{x:[hr|al|hl|ll|an]}+n" => "{x}+en";
        // ss+s → ss+es: 41 words (cross, miss, dünnschiss, beschiss), 0 forms wrong.
        law9: "ss+s" => "ss+es";
        // y+n → y+s: 103 words (city, lady, party, lobby), 0 forms wrong.
        law10: "y+n" => "y+s";
        // ß+s → +ss: 108 words (haß, biß, naß, schiß), 0 forms wrong.
        law11: "ß+s" => "+ss";
        // er+er → er+: 44 words (bruder, stiefbruder, halbbruder, blutsbruder), 4 forms wrong.
        law12: "er+er" => "er+";
        // u+n → u+en: 176 words (putzfrau, frau, ehefrau, au), 0 forms wrong.
        law13: "u+n" => "u+en";
        // p+n → p+s: 38 words (soap, app, op, krepp), 0 forms wrong.
        law14: "p+n" => "p+s";
        // [x|hn|ür]+n → +en: 325 words (box, phalanx, sandbox, kühlbox), 0 forms wrong.
        law15: "{x:[x|hn|ür]}+n" => "{x}+en";
        // y+e → y+s: 171 words (tommy, baby, boy, brandy), 10 forms wrong.
        law16: "y+e" => "y+s";
        // on → ∅ after [h|i|k|l|n|o|r|t|x]; s → ∅ after [e|o]; ß → ss after [a|e|i|o]; s → ti after n; k → z after i in alternating stems (ˇ), before +[a|e|i|o|u|y|ä|ö|ü]$: 130 words (lexikon, macron, mitochondrion, oxymoron), 4 forms wrong.
        law17: "{x:[k|on|s|ß]}" => "{x|law17[l]}" / "{l:[a|e|h|i|k|l|n|o|r|t|x]}" _ "+{[a|e|i|o|u|y|ä|ö|ü]}ˇ$";
        // é+en → é+s: 71 words (café, exposé, soufflé, kommuniqué), 0 forms wrong.
        law18: "é+en" => "é+s";
        // [é|w|c]+n → +s: 58 words (macramé, rosé, consommé, croisé), 0 forms wrong.
        law19: "{x:[é|w|c]}+n" => "{x}+s";
        // [é|mi]+e → +s: 104 words (café, exposé, soufflé, kommuniqué), 0 forms wrong.
        law20: "{x:[é|mi]}+e" => "{x}+s";
        // o+i → +i: 6 words (mafioso, exkasso, clavicembalo, guerrigliero), 0 forms wrong.
        law21: "o+i" => "+i";
        // ds+en → ds+: 34 words (fonds, shakehands, dachfonds, indexfonds), 0 forms wrong.
        law22: "ds+en" => "ds+";
        // ki+en → ki+s: 9 words (kaki, schoki, brocki, raki), 0 forms wrong.
        law23: "ki+en" => "ki+s";
        // e+i → +i: 27 words (generale, korporale, gondoliere, genitale), 0 forms wrong.
        law24: "e+i" => "+i";
        // ip+en → ip+ien: 27 words (partizip, prinzip, programmierprinzip, äquivalenzprinzip), 3 forms wrong.
        law25: "ip+en" => "ip+ien";
        // [ok|fi]+en → +s: 51 words (lok, notebook, e-book, look), 0 forms wrong.
        law26: "{x:[ok|fi]}+en" => "{x}+s";
        // ok+e → ok+s: 20 words (notebook, e-book, look, webhook), 1 forms wrong.
        law27: "ok+e" => "ok+s";
        // un+n → un+en: 9 words (neun, pikneun, herzneun, herz-neun), 0 forms wrong.
        law28: "un+n" => "un+en";
        // [w|fi|fa|ka|bi]+e → +s: 83 words (bungalow, interview, marshmallow, cashflow), 0 forms wrong.
        law29: "{x:[w|fi|fa|ka|bi]}+e" => "{x}+s";
        // [wn|bi]+en → +s: 31 words (clown, countdown, count-down, klinikclown), 0 forms wrong.
        law30: "{x:[wn|bi]}+en" => "{x}+s";
        // wn+n → wn+s: 5 words (boomtown, showdown, chinatown, shutdown), 0 forms wrong.
        law31: "wn+n" => "wn+s";
        // ln+n → ln+: 16 words (röteln, bratkartoffeln, porzeln, katzenlächeln), 0 forms wrong.
        law32: "ln+n" => "ln+";
        // ts+en → ts+: 15 words (hotpants, shorts, peanuts, boxershorts), 2 forms wrong.
        law33: "ts+en" => "ts+";
        // ns+ie → n+zie: 14 words (stimulans, ingrediens, formans, adjuvans), 0 forms wrong.
        law34: "ns+ie" => "n+zie";
        // [wn|ü|ga|gn]+e → +s: 51 words (clown, countdown, count-down, klinikclown), 0 forms wrong.
        law35: "{x:[wn|ü|ga|gn]}+e" => "{x}+s";
        // [ü|gn|ku]+en → +s: 37 words (menü, parvenü, untermenü, dreigängemenü), 0 forms wrong.
        law36: "{x:[ü|gn|ku]}+en" => "{x}+s";
        // ae+n → ae+: 11 words (australopithecinae, dryopithecinae, boinae, boidae), 0 forms wrong.
        law37: "ae+n" => "ae+";
        // ö+n → ö+en: 11 words (diarrhö, bö, amenorrhö, urethrorrhö), 0 forms wrong.
        law38: "ö+n" => "ö+en";
        // bu+en → bu+s after a vowel earlier in the stem: 10 words (tabu, zebu, sprachtabu, babu), 0 forms wrong.
        law39: "bu+en" => "bu+s" / "{[a|e|i|o|u|y|ä|ö|ü]}{![a|e|i|o|u|y|ä|ö|ü]*}" _ "";
        // dj+n → dj+s: 9 words (trance-dj, hiphop-dj, hip-hop-dj, house-dj), 0 forms wrong.
        law40: "dj+n" => "dj+s";
        // [ic|ou|ru]+en → +s: 22 words (tic, dickpic, fancomic, actioncomic), 0 forms wrong.
        law41: "{x:[ic|ou|ru]}+en" => "{x}+s";
        // ah+e → ah+s: 9 words (schah, ajatollah, schahinschah, hisbollah), 0 forms wrong.
        law42: "ah+e" => "ah+s";
        // bu+e → bu+s after a vowel earlier in the stem: 9 words (tabu, zebu, sprachtabu, babu), 0 forms wrong.
        law43: "bu+e" => "bu+s" / "{[a|e|i|o|u|y|ä|ö|ü]}{![a|e|i|o|u|y|ä|ö|ü]*}" _ "";
        // [ja|wa|ic|ku|ou|ru|ba|hu]+e → +s: 56 words (halleluja, ninja, maharaja, alleluja), 0 forms wrong.
        law44: "{x:[ja|wa|ic|ku|ou|ru|ba|hu]}+e" => "{x}+s";
        // cs+en → cs+: 6 words (abenomics, basics, paralympics, lyrics), 0 forms wrong.
        law45: "cs+en" => "cs+";
        // [h|y|s|j|i]+ä → +ot: 6 words (aliya, alija, chanukkia, mesusa), 0 forms wrong.
        law46: "{x:[h|y|s|j|i]}+ä" => "{x}+ot";
        // hu+en → hu+s: 6 words (uhu, tohuwabohu, virginiauhu, bhikschu), 0 forms wrong.
        law47: "hu+en" => "hu+s";
        // a+i → +i: 5 words (isba, stotinka, papirossa, naliwka), 0 forms wrong.
        law48: "a+i" => "+i";
    }
};
