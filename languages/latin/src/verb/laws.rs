//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`), on a letter a `%` copies (`°`), on a class
//! letter an ending writes (harmony, by the nearest letter of a tier before
//! it as written), or inside a stem that alternates (`ˇ`, by what follows
//! it): every form a conjugation builds goes through them, in order, so one conjugation
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
        law2 = [("er", "b") => "er", ("er", "i") => "er", ("er", "l") => "er", ("er", "n") => "er", ("er", "p") => "er", ("er", "r") => "er", ("er", "s") => "¦", ("er", "t") => "er", ("u", "b") => "ū¦", ("u", "i") => "ū¦", ("u", "l") => "ū¦", ("u", "n") => "ū¦", ("u", "p") => "ū¦", ("u", "r") => "ū¦", ("u", "s") => "ū¦", ("u", "t") => "ū¦"];
        law5 = [("n", "f") => "n", ("n", "i") => "¦", ("n", "n") => "n", ("n", "r") => "n", ("n", "s") => "n", ("n", "t") => "n", ("um", "f") => "um", ("um", "i") => "um", ("um", "n") => "um", ("um", "r") => "ū¦", ("um", "s") => "um", ("um", "t") => "um", ("un", "f") => "ū¦", ("un", "i") => "un", ("un", "n") => "un", ("un", "r") => "un", ("un", "s") => "un", ("un", "t") => "ū¦", ("ī", "f") => "ī", ("ī", "i") => "ī", ("ī", "n") => "ī", ("ī", "r") => "ī", ("ī", "s") => "ē¦", ("ī", "t") => "ī", ("ō", "f") => "ō", ("ō", "i") => "ō", ("ō", "n") => "i¦", ("ō", "r") => "ō", ("ō", "s") => "ō", ("ō", "t") => "ō"];
    }
    rules {
        // ēs+s → +u: 109 words (crūdēscō, vigēscō, cānēscō, senēscō), 0 forms wrong.
        law0: "ēs+s" => "+u";
        // d+t → +s: 73 words (ōdī, lūdō, rādō, ēlūdō), 31 forms wrong.
        law1: "d+t" => "+s";
        // u → ū after [b|i|l|n|p|r|s|t]; er → ∅ after s in alternating stems (ˇ), before +.[e|i|u|ē|ī|ū], +.tu[m|s], +.t[ī|ū], .+[c|s|x]: 41 words (nuō, minuō, innuō, annuō), 0 forms wrong.
        law2: "{x:[er|u]}" => "{x|law2[l]}" / "{l:[b|i|l|n|p|r|s|t]}" _ "+{![+|ˇ|¦|°|‹]}{[e|i|u|ē|ī|ū]}{![ˇ]*}ˇ$" | "+{![+|ˇ|¦|°|‹]}{[t]}{[u]}{[m|s]}{![ˇ]*}ˇ$" | "+{![+|ˇ|¦|°|‹]}{[t]}{[ī|ū]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[c|s|x]}{![ˇ]*}ˇ$";
        // d+um → +sum: 15 words (comprēndō, prēndō, reprēndō, apprēndō), 0 forms wrong.
        law3: "d+um" => "+sum";
        // d+s → +s: 69 words (lūdō, vādō, rādō, ēlūdō), 10 forms wrong.
        law4: "d+s" => "+s";
        // n → ∅ after i; un → ū after [f|t]; um → ū after r; ō → i after n; ī → ē after s in alternating stems (ˇ), before +[c|t], +d.r[a|u|ā|ī|ō], +d.ri[m|n|t], +d.s., +e.[a|ā|ī], +e.i.., +i..[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.r[a|u|ā|ī|ō], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.ri[m|n|t], [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.s., [^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]+.[u|ī|ū]: 30 words (conōscō, dēsinō, cōnfingō, effingō), 0 forms wrong.
        law5: "{x:[n|um|un|ī|ō]}" => "{x|law5[l]}" / "{l:[f|i|n|r|s|t]}" _ "+{[c|t]}{![ˇ]*}ˇ$" | "+{[d]}{![+|ˇ|¦|°|‹]}{[r]}{[a|u|ā|ī|ō]}{![ˇ]*}ˇ$" | "+{[d]}{![+|ˇ|¦|°|‹]}{[r]}{[i]}{[m|n|t]}{![ˇ]*}ˇ$" | "+{[d]}{![+|ˇ|¦|°|‹]}{[s]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{[a|ā|ī]}{![ˇ]*}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}+{![+|ˇ|¦|°|‹]}{[r]}{[a|u|ā|ī|ō]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}+{![+|ˇ|¦|°|‹]}{[r]}{[i]}{[m|n|t]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}+{![+|ˇ|¦|°|‹]}{[s]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}+{![+|ˇ|¦|°|‹]}{[u|ī|ū]}{![ˇ]*}ˇ$";
        // sc+s → +v: 47 words (nōscō, crēscō, dēcrēscō, accrēscō), 0 forms wrong.
        law6: "sc+s" => "+v";
        // c+s → +x: 6 words (speciō, amplector, complector, circumplector), 0 forms wrong.
        law7: "c+s" => "+x";
        // ūc+u → ū+x: 20 words (lūceō, allūceō, collūceō, dīlūceō), 0 forms wrong.
        law8: "ūc+u" => "ū+x";
        // b+t → +pt: 30 words (prōscrībō, nūbō, scrībō, dēscrībō), 0 forms wrong.
        law9: "b+t" => "+pt";
        // sc+t → +t: 37 words (nōscō, crēscō, dēcrēscō, accrēscō), 0 forms wrong.
        law10: "sc+t" => "+t";
        // lg+u → l+s: 18 words (fulgeō, algeō, mulgeō, affulgeō), 0 forms wrong.
        law11: "lg+u" => "l+s";
        // rr+s → rr+: 17 words (verrō, concurrō, recurrō, succurrō), 0 forms wrong.
        law12: "rr+s" => "rr+";
        // rr+t → r+s: 17 words (verrō, concurrō, recurrō, succurrō), 0 forms wrong.
        law13: "rr+t" => "r+s";
        // ec+i → ec+ after a vowel earlier in the stem: 13 words (bisecō, dissecō, intersecō, resecō), 0 forms wrong.
        law14: "ec+i" => "ec+" / "{[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}{![a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]*}" _ "";
        // m+s → m+ps: 10 words (sūmō, cōnsūmō, resūmō, assūmō), 0 forms wrong.
        law15: "m+s" => "m+ps";
        // m+tū → m+ptū: 10 words (sūmō, cōnsūmō, resūmō, assūmō), 0 forms wrong.
        law16: "m+tū" => "m+ptū";
        // ru+s → ru+: 14 words (ruō, congruō, gruō, prōruō), 0 forms wrong.
        law17: "ru+s" => "ru+";
        // fī+ct → fī+x after a vowel earlier in the stem: 12 words (praefīgō, īnfīgō, cōnfīgō, dēfīgō), 0 forms wrong.
        law18: "fī+ct" => "fī+x" / "{[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}{![a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]*}" _ "";
        // g+c → +c: 6 words (tāgō, sūgō, tragō, discingō), 0 forms wrong.
        law19: "g+c" => "+c";
        // ūr+s → +uss: 11 words (ūrō, combūrō, deūrō, exūrō), 0 forms wrong.
        law20: "ūr+s" => "+uss";
        // ūr+t → +ust: 11 words (ūrō, combūrō, deūrō, exūrō), 0 forms wrong.
        law21: "ūr+t" => "+ust";
        // ex+s → ex+u: 10 words (texō, attexō, contexō, intexō), 0 forms wrong.
        law22: "ex+s" => "ex+u";
        // ūm+t → ūm+pt: 10 words (sūmō, cōnsūmō, resūmō, assūmō), 0 forms wrong.
        law23: "ūm+t" => "ūm+pt";
        // i+su → +īsu: 7 words (videō, invideō, revideō, prōvideō), 0 forms wrong.
        law24: "i+su" => "+īsu";
        // i+sī → +īsī: 7 words (videō, invideō, revideō, prōvideō), 0 forms wrong.
        law25: "i+sī" => "+īsī";
        // i+sū → +īsū: 7 words (videō, invideō, revideō, prōvideō), 0 forms wrong.
        law26: "i+sū" => "+īsū";
        // b+id → b+: 8 words (bibō, lambō, adbibō, ēbibō), 0 forms wrong.
        law27: "b+id" => "b+";
        // oc+it → oc+t after a vowel earlier in the stem: 7 words (addoceō, condoceō, dēdoceō, ēdoceō), 9 forms wrong.
        law28: "oc+it" => "oc+t" / "{[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]}{![a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ]*}" _ "";
        // ēt+it → ē+ns: 8 words (admētior, dīmētior, remētior, commētior), 3 forms wrong.
        law29: "ēt+it" => "ē+ns";
        // īr+s → ī+sīv: 8 words (requīrō, inquīrō, conquīrō, exquīrō), 0 forms wrong.
        law30: "īr+s" => "ī+sīv";
        // īr+t → ī+sīt: 8 words (requīrō, inquīrō, conquīrō, exquīrō), 0 forms wrong.
        law31: "īr+t" => "ī+sīt";
        // qu+c → +c: 7 words (coquō, concoquō, discoquō, excoquō), 0 forms wrong.
        law32: "qu+c" => "+c";
        // qu+x → +x: 7 words (coquō, concoquō, discoquō, excoquō), 0 forms wrong.
        law33: "qu+x" => "+x";
        // an+s → +āns: 6 words (scandō, prandeō, oppandō, superscandō), 0 forms wrong.
        law34: "an+s" => "+āns";
        // l+āc → l+ēc: 6 words (colligō, ēligō, sēligō, dēligō), 0 forms wrong.
        law35: "l+āc" => "l+ēc";
        // t+ēg → t+ig: 5 words (attingō, contingō, adtingō, obtingō), 0 forms wrong.
        law36: "t+ēg" => "t+ig";
        // v+ēd → v+īd: 6 words (videō, invideō, revideō, prōvideō), 0 forms wrong.
        law37: "v+ēd" => "v+īd";
        // ol+t → +ult: 5 words (colō, recolō, excolō, circumcolō), 0 forms wrong.
        law38: "ol+t" => "+ult";
        // ss+t → ss+īt: 5 words (arcessō, lacessō, capessō, facessō), 0 forms wrong.
        law39: "ss+t" => "ss+īt";
        // [ā|e]+t → +st after p before [u|ī|ū]: 5 words (pāscō, pāscor, dēpāscō, dispescō), 1 forms wrong.
        law40: "{x:[ā|e]}+t" => "{x}+st" / "{[p]}" _ "{[u|ī|ū]}";
    }
};
