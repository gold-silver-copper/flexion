//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 48 prefixes and 52 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix { letters: Cow::Borrowed("अ\u{902}स"), separable: 0, inseparable: 4, words: 4 },
        Prefix { letters: Cow::Borrowed("अधि"), separable: 0, inseparable: 5, words: 6 },
        Prefix { letters: Cow::Borrowed("अन\u{941}"), separable: 0, inseparable: 16, words: 16 },
        Prefix { letters: Cow::Borrowed("अप"), separable: 0, inseparable: 16, words: 16 },
        Prefix { letters: Cow::Borrowed("अभि"), separable: 0, inseparable: 11, words: 11 },
        Prefix { letters: Cow::Borrowed("अव"), separable: 0, inseparable: 12, words: 12 },
        Prefix { letters: Cow::Borrowed("आदि"), separable: 0, inseparable: 6, words: 6 },
        Prefix { letters: Cow::Borrowed("इन\u{94d}द\u{94d}र"), separable: 0, inseparable: 5, words: 6 },
        Prefix { letters: Cow::Borrowed("उद\u{94d}"), separable: 0, inseparable: 9, words: 10 },
        Prefix { letters: Cow::Borrowed("उप"), separable: 0, inseparable: 23, words: 26 },
        Prefix { letters: Cow::Borrowed("कर"), separable: 0, inseparable: 19, words: 19 },
        Prefix { letters: Cow::Borrowed("करि"), separable: 0, inseparable: 19, words: 20 },
        Prefix { letters: Cow::Borrowed("कर\u{94d}"), separable: 0, inseparable: 18, words: 18 },
        Prefix { letters: Cow::Borrowed("काम"), separable: 0, inseparable: 4, words: 4 },
        Prefix { letters: Cow::Borrowed("क\u{941}"), separable: 0, inseparable: 41, words: 41 },
        Prefix { letters: Cow::Borrowed("गो"), separable: 0, inseparable: 20, words: 20 },
        Prefix { letters: Cow::Borrowed("चक\u{94d}र"), separable: 0, inseparable: 7, words: 7 },
        Prefix { letters: Cow::Borrowed("चन\u{94d}द\u{94d}र"), separable: 0, inseparable: 6, words: 7 },
        Prefix { letters: Cow::Borrowed("त\u{94d}"), separable: 0, inseparable: 23, words: 23 },
        Prefix { letters: Cow::Borrowed("त\u{94d}रि"), separable: 0, inseparable: 3, words: 6 },
        Prefix { letters: Cow::Borrowed("दन\u{94d}त"), separable: 0, inseparable: 4, words: 4 },
        Prefix { letters: Cow::Borrowed("द\u{941}र\u{94d}"), separable: 0, inseparable: 5, words: 5 },
        Prefix { letters: Cow::Borrowed("द\u{947}व"), separable: 0, inseparable: 10, words: 10 },
        Prefix { letters: Cow::Borrowed("द\u{94d}"), separable: 0, inseparable: 32, words: 35 },
        Prefix { letters: Cow::Borrowed("नि"), separable: 0, inseparable: 40, words: 43 },
        Prefix { letters: Cow::Borrowed("परि"), separable: 0, inseparable: 17, words: 17 },
        Prefix { letters: Cow::Borrowed("पाद"), separable: 0, inseparable: 7, words: 8 },
        Prefix { letters: Cow::Borrowed("पित\u{943}"), separable: 0, inseparable: 8, words: 8 },
        Prefix { letters: Cow::Borrowed("प\u{94d}"), separable: 0, inseparable: 19, words: 19 },
        Prefix { letters: Cow::Borrowed("प\u{94d}र"), separable: 0, inseparable: 91, words: 97 },
        Prefix { letters: Cow::Borrowed("प\u{94d}रति"), separable: 0, inseparable: 9, words: 13 },
        Prefix { letters: Cow::Borrowed("प\u{94d}रा"), separable: 0, inseparable: 19, words: 21 },
        Prefix { letters: Cow::Borrowed("बह\u{941}"), separable: 0, inseparable: 4, words: 4 },
        Prefix { letters: Cow::Borrowed("ब\u{94d}रह\u{94d}म"), separable: 0, inseparable: 6, words: 7 },
        Prefix { letters: Cow::Borrowed("भ\u{942}"), separable: 0, inseparable: 13, words: 13 },
        Prefix { letters: Cow::Borrowed("महा"), separable: 0, inseparable: 11, words: 13 },
        Prefix { letters: Cow::Borrowed("राज"), separable: 0, inseparable: 13, words: 14 },
        Prefix { letters: Cow::Borrowed("वि"), separable: 0, inseparable: 86, words: 89 },
        Prefix { letters: Cow::Borrowed("व\u{948}"), separable: 0, inseparable: 16, words: 16 },
        Prefix { letters: Cow::Borrowed("श\u{94d}"), separable: 0, inseparable: 45, words: 48 },
        Prefix { letters: Cow::Borrowed("स\u{902}"), separable: 0, inseparable: 51, words: 53 },
        Prefix { letters: Cow::Borrowed("सत\u{94d}"), separable: 0, inseparable: 10, words: 10 },
        Prefix { letters: Cow::Borrowed("समा"), separable: 0, inseparable: 4, words: 4 },
        Prefix { letters: Cow::Borrowed("सम\u{94d}"), separable: 0, inseparable: 9, words: 12 },
        Prefix { letters: Cow::Borrowed("सि\u{902}ह"), separable: 0, inseparable: 8, words: 8 },
        Prefix { letters: Cow::Borrowed("स\u{941}"), separable: 0, inseparable: 21, words: 25 },
        Prefix { letters: Cow::Borrowed("स\u{942}र\u{94d}य"), separable: 0, inseparable: 5, words: 5 },
        Prefix { letters: Cow::Borrowed("स\u{94d}"), separable: 0, inseparable: 117, words: 120 },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("अधिवास"), Separation::Neither),
        (Cow::Borrowed("इन\u{94d}द\u{94d}रधन\u{941}स\u{94d}"), Separation::Neither),
        (Cow::Borrowed("उद\u{94d}द\u{947}श\u{94d}य"), Separation::Neither),
        (Cow::Borrowed("उपकरण"), Separation::Neither),
        (Cow::Borrowed("उपर\u{942}पक"), Separation::Neither),
        (Cow::Borrowed("उपवास"), Separation::Neither),
        (Cow::Borrowed("करिक\u{941}म\u{94d}भ"), Separation::Neither),
        (Cow::Borrowed("चन\u{94d}द\u{94d}रतारक"), Separation::Neither),
        (Cow::Borrowed("त\u{94d}रिकोण"), Separation::Inseparable),
        (Cow::Borrowed("त\u{94d}रिज\u{94d}या"), Separation::Inseparable),
        (Cow::Borrowed("त\u{94d}रिरत\u{94d}न"), Separation::Inseparable),
        (Cow::Borrowed("द\u{94d}र\u{941}ह\u{94d}"), Separation::Neither),
        (Cow::Borrowed("द\u{94d}वार"), Separation::Neither),
        (Cow::Borrowed("द\u{94d}वार\u{94d}"), Separation::Neither),
        (Cow::Borrowed("निखर\u{94d}व"), Separation::Neither),
        (Cow::Borrowed("निभ\u{943}त"), Separation::Neither),
        (Cow::Borrowed("निवास"), Separation::Neither),
        (Cow::Borrowed("पादकन\u{94d}द\u{941}क"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रकोप"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रतिपत\u{94d}ति"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रतिपद\u{94d}"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रतिबिम\u{94d}ब"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रतिव\u{947}श\u{94d}य"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रधान"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रबन\u{94d}ध"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रभ\u{942}त"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रम\u{941}ख"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रवाह"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}राक\u{943}त"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}राचीन"), Separation::Neither),
        (Cow::Borrowed("ब\u{94d}रह\u{94d}मदार\u{941}"), Separation::Neither),
        (Cow::Borrowed("महाभारत"), Separation::Neither),
        (Cow::Borrowed("महामात\u{94d}र"), Separation::Neither),
        (Cow::Borrowed("राजलक\u{94d}ष\u{94d}मी"), Separation::Neither),
        (Cow::Borrowed("वितरण"), Separation::Neither),
        (Cow::Borrowed("विराम"), Separation::Neither),
        (Cow::Borrowed("विरोचन"), Separation::Neither),
        (Cow::Borrowed("श\u{94d}रमण"), Separation::Neither),
        (Cow::Borrowed("श\u{94d}वन\u{94d}"), Separation::Neither),
        (Cow::Borrowed("श\u{94d}वास"), Separation::Neither),
        (Cow::Borrowed("स\u{902}वर\u{94d}धन"), Separation::Neither),
        (Cow::Borrowed("स\u{902}विधान"), Separation::Neither),
        (Cow::Borrowed("सम\u{94d}पत\u{94d}ति"), Separation::Neither),
        (Cow::Borrowed("सम\u{94d}पद\u{94d}"), Separation::Neither),
        (Cow::Borrowed("सम\u{94d}प\u{94d}रदाय"), Separation::Neither),
        (Cow::Borrowed("स\u{941}दर\u{94d}शन"), Separation::Neither),
        (Cow::Borrowed("स\u{941}पर\u{94d}ण"), Separation::Neither),
        (Cow::Borrowed("स\u{941}वर\u{94d}ण"), Separation::Neither),
        (Cow::Borrowed("स\u{941}ह\u{943}द\u{94d}"), Separation::Neither),
        (Cow::Borrowed("स\u{94d}तोक"), Separation::Neither),
        (Cow::Borrowed("स\u{94d}वर\u{94d}"), Separation::Neither),
        (Cow::Borrowed("स\u{94d}वस\u{94d}ति"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // NOM_SG
        Before, // NOM_DU
        Before, // NOM_PL
        Before, // ACC_SG
        Before, // ACC_DU
        Before, // ACC_PL
        Before, // INS_SG
        Before, // INS_DU
        Before, // INS_PL
        Before, // DAT_SG
        Before, // DAT_DU
        Before, // DAT_PL
        Before, // ABL_SG
        Before, // ABL_DU
        Before, // ABL_PL
        Before, // GEN_SG
        Before, // GEN_DU
        Before, // GEN_PL
        Before, // LOC_SG
        Before, // LOC_DU
        Before, // LOC_PL
        Before, // VOC_SG
        Before, // VOC_DU
        Before, // VOC_PL
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // NOM_SG
        Cow::Borrowed(""), // NOM_DU
        Cow::Borrowed(""), // NOM_PL
        Cow::Borrowed(""), // ACC_SG
        Cow::Borrowed(""), // ACC_DU
        Cow::Borrowed(""), // ACC_PL
        Cow::Borrowed(""), // INS_SG
        Cow::Borrowed(""), // INS_DU
        Cow::Borrowed(""), // INS_PL
        Cow::Borrowed(""), // DAT_SG
        Cow::Borrowed(""), // DAT_DU
        Cow::Borrowed(""), // DAT_PL
        Cow::Borrowed(""), // ABL_SG
        Cow::Borrowed(""), // ABL_DU
        Cow::Borrowed(""), // ABL_PL
        Cow::Borrowed(""), // GEN_SG
        Cow::Borrowed(""), // GEN_DU
        Cow::Borrowed(""), // GEN_PL
        Cow::Borrowed(""), // LOC_SG
        Cow::Borrowed(""), // LOC_DU
        Cow::Borrowed(""), // LOC_PL
        Cow::Borrowed(""), // VOC_SG
        Cow::Borrowed(""), // VOC_DU
        Cow::Borrowed(""), // VOC_PL
    ]),
};
