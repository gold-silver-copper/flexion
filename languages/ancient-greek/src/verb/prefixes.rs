//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front, takes off the front of the rest's form
//! what the cell says, and meets it as `junctions` says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 28 prefixes and 260 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("αμφι"),
            separable: 0,
            inseparable: 7,
            words: 10,
        },
        Prefix {
            letters: Cow::Borrowed("ανα"),
            separable: 0,
            inseparable: 49,
            words: 65,
        },
        Prefix {
            letters: Cow::Borrowed("αντι"),
            separable: 0,
            inseparable: 11,
            words: 15,
        },
        Prefix {
            letters: Cow::Borrowed("απο"),
            separable: 0,
            inseparable: 56,
            words: 67,
        },
        Prefix {
            letters: Cow::Borrowed("δι"),
            separable: 0,
            inseparable: 4,
            words: 23,
        },
        Prefix {
            letters: Cow::Borrowed("δια"),
            separable: 0,
            inseparable: 38,
            words: 63,
        },
        Prefix {
            letters: Cow::Borrowed("εγ"),
            separable: 0,
            inseparable: 11,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("εισ"),
            separable: 0,
            inseparable: 6,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("εκ"),
            separable: 0,
            inseparable: 42,
            words: 54,
        },
        Prefix {
            letters: Cow::Borrowed("εμ"),
            separable: 0,
            inseparable: 16,
            words: 21,
        },
        Prefix {
            letters: Cow::Borrowed("εν"),
            separable: 0,
            inseparable: 21,
            words: 35,
        },
        Prefix {
            letters: Cow::Borrowed("επι"),
            separable: 0,
            inseparable: 63,
            words: 82,
        },
        Prefix {
            letters: Cow::Borrowed("εσ"),
            separable: 0,
            inseparable: 4,
            words: 5,
        },
        Prefix {
            letters: Cow::Borrowed("κατα"),
            separable: 0,
            inseparable: 36,
            words: 59,
        },
        Prefix {
            letters: Cow::Borrowed("μετα"),
            separable: 0,
            inseparable: 14,
            words: 18,
        },
        Prefix {
            letters: Cow::Borrowed("ξυγ"),
            separable: 0,
            inseparable: 5,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("ξυμ"),
            separable: 0,
            inseparable: 8,
            words: 11,
        },
        Prefix {
            letters: Cow::Borrowed("ξυν"),
            separable: 0,
            inseparable: 7,
            words: 21,
        },
        Prefix {
            letters: Cow::Borrowed("παρα"),
            separable: 0,
            inseparable: 21,
            words: 32,
        },
        Prefix {
            letters: Cow::Borrowed("περι"),
            separable: 0,
            inseparable: 18,
            words: 30,
        },
        Prefix {
            letters: Cow::Borrowed("προ"),
            separable: 0,
            inseparable: 44,
            words: 65,
        },
        Prefix {
            letters: Cow::Borrowed("προσ"),
            separable: 0,
            inseparable: 27,
            words: 44,
        },
        Prefix {
            letters: Cow::Borrowed("συ"),
            separable: 0,
            inseparable: 17,
            words: 17,
        },
        Prefix {
            letters: Cow::Borrowed("συγ"),
            separable: 0,
            inseparable: 10,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("συμ"),
            separable: 0,
            inseparable: 15,
            words: 20,
        },
        Prefix {
            letters: Cow::Borrowed("συν"),
            separable: 0,
            inseparable: 15,
            words: 39,
        },
        Prefix {
            letters: Cow::Borrowed("υπερ"),
            separable: 0,
            inseparable: 4,
            words: 7,
        },
        Prefix {
            letters: Cow::Borrowed("υπο"),
            separable: 0,
            inseparable: 22,
            words: 29,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("αμφιεπω"), Separation::Neither),
        (Cow::Borrowed("αμφικαλυπτω"), Separation::Neither),
        (Cow::Borrowed("αμφιστημι"), Separation::Neither),
        (Cow::Borrowed("αναβλαστανω"), Separation::Neither),
        (Cow::Borrowed("αναβοαω"), Separation::Neither),
        (Cow::Borrowed("αναγγελλω"), Separation::Neither),
        (Cow::Borrowed("αναγκαζω"), Separation::Neither),
        (Cow::Borrowed("αναειρω"), Separation::Neither),
        (Cow::Borrowed("αναινομαι"), Separation::Neither),
        (Cow::Borrowed("αναιρεω"), Separation::Neither),
        (Cow::Borrowed("αναισσω"), Separation::Neither),
        (Cow::Borrowed("ανακαλεω"), Separation::Neither),
        (Cow::Borrowed("αναλαμβανω"), Separation::Neither),
        (Cow::Borrowed("αναλισκω"), Separation::Neither),
        (Cow::Borrowed("αναποδιζω"), Separation::Neither),
        (Cow::Borrowed("αναπτω"), Separation::Neither),
        (Cow::Borrowed("ανατρεχω"), Separation::Neither),
        (Cow::Borrowed("αναχρονιζομαι"), Separation::Neither),
        (Cow::Borrowed("αναψυχω"), Separation::Neither),
        (Cow::Borrowed("αντιαζω"), Separation::Neither),
        (Cow::Borrowed("αντικαθιστημι"), Separation::Neither),
        (Cow::Borrowed("αντιλεγω"), Separation::Neither),
        (Cow::Borrowed("αντιπαρατασσομαι"), Separation::Neither),
        (Cow::Borrowed("αποδυω"), Separation::Neither),
        (Cow::Borrowed("αποθραυω"), Separation::Neither),
        (Cow::Borrowed("αποικιζω"), Separation::Neither),
        (Cow::Borrowed("αποκαλυπτω"), Separation::Neither),
        (Cow::Borrowed("αποκωλυω"), Separation::Neither),
        (Cow::Borrowed("απολαμβανω"), Separation::Neither),
        (Cow::Borrowed("απολεγω"), Separation::Neither),
        (Cow::Borrowed("απολυτροω"), Separation::Neither),
        (Cow::Borrowed("αποπερδομαι"), Separation::Neither),
        (Cow::Borrowed("απορεω"), Separation::Neither),
        (Cow::Borrowed("αποστατεω"), Separation::Neither),
        (Cow::Borrowed("διαβαινω"), Separation::Neither),
        (Cow::Borrowed("διαβαλλω"), Separation::Neither),
        (Cow::Borrowed("διαγω"), Separation::Inseparable),
        (Cow::Borrowed("διαγωνιζομαι"), Separation::Neither),
        (Cow::Borrowed("διαζω"), Separation::Inseparable),
        (Cow::Borrowed("διαζωννυμι"), Separation::Neither),
        (Cow::Borrowed("διαιρεω"), Separation::Neither),
        (Cow::Borrowed("διαιρω"), Separation::Neither),
        (Cow::Borrowed("διαιταω"), Separation::Neither),
        (Cow::Borrowed("διακειμαι"), Separation::Neither),
        (Cow::Borrowed("διακονεω"), Separation::Neither),
        (Cow::Borrowed("διαλεγω"), Separation::Neither),
        (Cow::Borrowed("διαλειπω"), Separation::Neither),
        (Cow::Borrowed("διαμηνυω"), Separation::Neither),
        (Cow::Borrowed("διανυω"), Separation::Neither),
        (Cow::Borrowed("διαπερθω"), Separation::Neither),
        (Cow::Borrowed("διαπυνθανομαι"), Separation::Neither),
        (Cow::Borrowed("διαρκεω"), Separation::Neither),
        (Cow::Borrowed("διαρπαζω"), Separation::Neither),
        (Cow::Borrowed("διασευομαι"), Separation::Neither),
        (Cow::Borrowed("διασκευαζω"), Separation::Neither),
        (Cow::Borrowed("διασκοπεω"), Separation::Neither),
        (Cow::Borrowed("διασωζω"), Separation::Neither),
        (Cow::Borrowed("διατιθημι"), Separation::Neither),
        (Cow::Borrowed("διαφερω"), Separation::Neither),
        (Cow::Borrowed("διαφορεω"), Separation::Neither),
        (Cow::Borrowed("διαχεω"), Separation::Neither),
        (Cow::Borrowed("διεκπλεω"), Separation::Inseparable),
        (Cow::Borrowed("διυλιζω"), Separation::Inseparable),
        (Cow::Borrowed("εγγιζω"), Separation::Neither),
        (Cow::Borrowed("εγειρω"), Separation::Neither),
        (Cow::Borrowed("εγκαθιστημι"), Separation::Neither),
        (Cow::Borrowed("εγκαλεω"), Separation::Neither),
        (Cow::Borrowed("εγκαταλεγω"), Separation::Neither),
        (Cow::Borrowed("εισαγω"), Separation::Neither),
        (Cow::Borrowed("εισακουω"), Separation::Neither),
        (Cow::Borrowed("εισαναβαινω"), Separation::Neither),
        (Cow::Borrowed("εισερχομαι"), Separation::Neither),
        (Cow::Borrowed("εισοραω"), Separation::Neither),
        (Cow::Borrowed("εκγαμιζω"), Separation::Neither),
        (Cow::Borrowed("εκδιωκω"), Separation::Neither),
        (Cow::Borrowed("εκδυω"), Separation::Neither),
        (Cow::Borrowed("εκκαλεω"), Separation::Neither),
        (Cow::Borrowed("εκκρυπτω"), Separation::Neither),
        (Cow::Borrowed("εκλεγω"), Separation::Neither),
        (Cow::Borrowed("εκλυτροω"), Separation::Neither),
        (Cow::Borrowed("εκρεω"), Separation::Neither),
        (Cow::Borrowed("εκτεινω"), Separation::Neither),
        (Cow::Borrowed("εκτιθημι"), Separation::Neither),
        (Cow::Borrowed("εκφερω"), Separation::Neither),
        (Cow::Borrowed("εκχεω"), Separation::Neither),
        (Cow::Borrowed("εμβαλλω"), Separation::Neither),
        (Cow::Borrowed("εμπαιζω"), Separation::Neither),
        (Cow::Borrowed("εμπηγνυμι"), Separation::Neither),
        (Cow::Borrowed("εμπιμπρημι"), Separation::Neither),
        (Cow::Borrowed("εμπολαω"), Separation::Neither),
        (Cow::Borrowed("εναιρω"), Separation::Neither),
        (Cow::Borrowed("εναντιοομαι"), Separation::Neither),
        (Cow::Borrowed("εναριζω"), Separation::Neither),
        (Cow::Borrowed("εναττικιζω"), Separation::Neither),
        (Cow::Borrowed("ενδεω"), Separation::Neither),
        (Cow::Borrowed("ενδυω"), Separation::Neither),
        (Cow::Borrowed("ενειμι"), Separation::Neither),
        (Cow::Borrowed("ενιημι"), Separation::Neither),
        (Cow::Borrowed("ενιστημι"), Separation::Neither),
        (Cow::Borrowed("εννυμι"), Separation::Neither),
        (Cow::Borrowed("ενοικεω"), Separation::Neither),
        (Cow::Borrowed("ενοραω"), Separation::Neither),
        (Cow::Borrowed("εντυγχανω"), Separation::Neither),
        (Cow::Borrowed("ενυπνιαζω"), Separation::Neither),
        (Cow::Borrowed("επιγιγνωσκω"), Separation::Neither),
        (Cow::Borrowed("επικαλεω"), Separation::Neither),
        (Cow::Borrowed("επιλαμβανω"), Separation::Neither),
        (Cow::Borrowed("επιληθω"), Separation::Neither),
        (Cow::Borrowed("επιμειδαω"), Separation::Neither),
        (Cow::Borrowed("επιμελεομαι"), Separation::Neither),
        (Cow::Borrowed("επιμιμνησκομαι"), Separation::Neither),
        (Cow::Borrowed("επιπειθομαι"), Separation::Neither),
        (Cow::Borrowed("επισευω"), Separation::Neither),
        (Cow::Borrowed("επισκεπτομαι"), Separation::Neither),
        (Cow::Borrowed("επισκευαζω"), Separation::Neither),
        (Cow::Borrowed("επισκοπεω"), Separation::Neither),
        (Cow::Borrowed("επισταμαι"), Separation::Neither),
        (Cow::Borrowed("επισυναπτω"), Separation::Neither),
        (Cow::Borrowed("επιτιθημι"), Separation::Neither),
        (Cow::Borrowed("επιτρεχω"), Separation::Neither),
        (Cow::Borrowed("επιφαινω"), Separation::Neither),
        (Cow::Borrowed("επιφερω"), Separation::Neither),
        (Cow::Borrowed("επιχειρεω"), Separation::Neither),
        (Cow::Borrowed("εσθιω"), Separation::Neither),
        (Cow::Borrowed("καταβαινω"), Separation::Neither),
        (Cow::Borrowed("καταβοαω"), Separation::Neither),
        (Cow::Borrowed("καταδιωκω"), Separation::Neither),
        (Cow::Borrowed("καταδυω"), Separation::Neither),
        (Cow::Borrowed("καταθνησκω"), Separation::Neither),
        (Cow::Borrowed("καταικιζω"), Separation::Neither),
        (Cow::Borrowed("καταιρω"), Separation::Neither),
        (Cow::Borrowed("κατακαλεω"), Separation::Neither),
        (Cow::Borrowed("κατακειμαι"), Separation::Neither),
        (Cow::Borrowed("κατακλαω"), Separation::Neither),
        (Cow::Borrowed("καταλαμβανω"), Separation::Neither),
        (Cow::Borrowed("καταλεγω"), Separation::Neither),
        (Cow::Borrowed("καταμυω"), Separation::Neither),
        (Cow::Borrowed("καταναγκαζω"), Separation::Neither),
        (Cow::Borrowed("καταπαιδεραστεω"), Separation::Neither),
        (Cow::Borrowed("καταπετροω"), Separation::Neither),
        (Cow::Borrowed("καταργεω"), Separation::Neither),
        (Cow::Borrowed("κατασκηνοω"), Separation::Neither),
        (Cow::Borrowed("καταστρεφω"), Separation::Neither),
        (Cow::Borrowed("κατατιθημι"), Separation::Neither),
        (Cow::Borrowed("κατατρεχω"), Separation::Neither),
        (Cow::Borrowed("καταφρονεω"), Separation::Neither),
        (Cow::Borrowed("καταχραομαι"), Separation::Neither),
        (Cow::Borrowed("μετακαλεω"), Separation::Neither),
        (Cow::Borrowed("μεταλαγχανω"), Separation::Neither),
        (Cow::Borrowed("μεταλαμβανω"), Separation::Neither),
        (Cow::Borrowed("μεταφερω"), Separation::Neither),
        (Cow::Borrowed("ξυγκειμαι"), Separation::Neither),
        (Cow::Borrowed("ξυγκινδυνευω"), Separation::Neither),
        (Cow::Borrowed("ξυμμενω"), Separation::Neither),
        (Cow::Borrowed("ξυμπολεμεω"), Separation::Neither),
        (Cow::Borrowed("ξυμφερω"), Separation::Neither),
        (Cow::Borrowed("ξυναλλαττω"), Separation::Inseparable),
        (Cow::Borrowed("ξυναρχω"), Separation::Inseparable),
        (Cow::Borrowed("ξυναφιστημι"), Separation::Inseparable),
        (Cow::Borrowed("ξυνεξερχομαι"), Separation::Inseparable),
        (Cow::Borrowed("ξυνετεω"), Separation::Inseparable),
        (Cow::Borrowed("ξυνθαπτω"), Separation::Inseparable),
        (Cow::Borrowed("ξυντιθημι"), Separation::Inseparable),
        (Cow::Borrowed("παραβαινω"), Separation::Neither),
        (Cow::Borrowed("παραγγελλω"), Separation::Neither),
        (Cow::Borrowed("παραδιδωμι"), Separation::Neither),
        (Cow::Borrowed("παραινεω"), Separation::Neither),
        (Cow::Borrowed("παραιτεομαι"), Separation::Neither),
        (Cow::Borrowed("παρακαλεω"), Separation::Neither),
        (Cow::Borrowed("παραλαμβανω"), Separation::Neither),
        (Cow::Borrowed("παραλλασσω"), Separation::Neither),
        (Cow::Borrowed("παραμελεω"), Separation::Neither),
        (Cow::Borrowed("παρατιθημι"), Separation::Neither),
        (Cow::Borrowed("παρατυγχανω"), Separation::Neither),
        (Cow::Borrowed("περιειμι"), Separation::Neither),
        (Cow::Borrowed("περιεπω"), Separation::Neither),
        (Cow::Borrowed("περιεχω"), Separation::Neither),
        (Cow::Borrowed("περιθαλπω"), Separation::Neither),
        (Cow::Borrowed("περιιστημι"), Separation::Neither),
        (Cow::Borrowed("περικλειω"), Separation::Neither),
        (Cow::Borrowed("περιοραω"), Separation::Neither),
        (Cow::Borrowed("περισκοπεω"), Separation::Neither),
        (Cow::Borrowed("περισσευω"), Separation::Neither),
        (Cow::Borrowed("περιτειχιζω"), Separation::Neither),
        (Cow::Borrowed("περιτρεχω"), Separation::Neither),
        (Cow::Borrowed("περιφερω"), Separation::Neither),
        (Cow::Borrowed("προβαλλω"), Separation::Neither),
        (Cow::Borrowed("προειμι"), Separation::Neither),
        (Cow::Borrowed("προειπον"), Separation::Neither),
        (Cow::Borrowed("προερχομαι"), Separation::Neither),
        (Cow::Borrowed("προετοιμαζω"), Separation::Neither),
        (Cow::Borrowed("προιημι"), Separation::Neither),
        (Cow::Borrowed("προιστημι"), Separation::Neither),
        (Cow::Borrowed("προισχω"), Separation::Neither),
        (Cow::Borrowed("προκαλεω"), Separation::Neither),
        (Cow::Borrowed("προκαταλαμβανω"), Separation::Neither),
        (Cow::Borrowed("προκειμαι"), Separation::Neither),
        (Cow::Borrowed("προλαμβανω"), Separation::Neither),
        (Cow::Borrowed("προμηνυω"), Separation::Neither),
        (Cow::Borrowed("προοιδα"), Separation::Neither),
        (Cow::Borrowed("προοραω"), Separation::Neither),
        (Cow::Borrowed("προοριζω"), Separation::Neither),
        (Cow::Borrowed("προοφειλω"), Separation::Neither),
        (Cow::Borrowed("προσαγορευω"), Separation::Neither),
        (Cow::Borrowed("προσαγω"), Separation::Neither),
        (Cow::Borrowed("προσαυδαω"), Separation::Neither),
        (Cow::Borrowed("προσειμι"), Separation::Neither),
        (Cow::Borrowed("προσειπον"), Separation::Neither),
        (Cow::Borrowed("προσερχομαι"), Separation::Neither),
        (Cow::Borrowed("προσευχομαι"), Separation::Neither),
        (Cow::Borrowed("προσεχω"), Separation::Neither),
        (Cow::Borrowed("προσηκω"), Separation::Neither),
        (Cow::Borrowed("προσκαλεω"), Separation::Neither),
        (Cow::Borrowed("προσκυνεω"), Separation::Neither),
        (Cow::Borrowed("προσλαμβανω"), Separation::Neither),
        (Cow::Borrowed("προσμειγνυμι"), Separation::Neither),
        (Cow::Borrowed("προσοικεω"), Separation::Neither),
        (Cow::Borrowed("προσφερω"), Separation::Neither),
        (Cow::Borrowed("προσφωνεω"), Separation::Neither),
        (Cow::Borrowed("προσχωρεω"), Separation::Neither),
        (Cow::Borrowed("προτερεω"), Separation::Neither),
        (Cow::Borrowed("προυφειλω"), Separation::Neither),
        (Cow::Borrowed("προφερω"), Separation::Neither),
        (Cow::Borrowed("προωθεω"), Separation::Neither),
        (Cow::Borrowed("συγκακουργεω"), Separation::Neither),
        (Cow::Borrowed("συγκαλεω"), Separation::Neither),
        (Cow::Borrowed("συγκειμαι"), Separation::Neither),
        (Cow::Borrowed("συγκινδυνευω"), Separation::Neither),
        (Cow::Borrowed("συγκρυπτω"), Separation::Neither),
        (Cow::Borrowed("συγχρονιζω"), Separation::Neither),
        (Cow::Borrowed("συμβουλευω"), Separation::Neither),
        (Cow::Borrowed("συμμαχεω"), Separation::Neither),
        (Cow::Borrowed("συμμαχομαι"), Separation::Neither),
        (Cow::Borrowed("συμπολεμεω"), Separation::Neither),
        (Cow::Borrowed("συμφερω"), Separation::Neither),
        (Cow::Borrowed("συναλλασσω"), Separation::Inseparable),
        (Cow::Borrowed("συνανταω"), Separation::Inseparable),
        (Cow::Borrowed("συναρχω"), Separation::Inseparable),
        (Cow::Borrowed("συναφιστημι"), Separation::Inseparable),
        (Cow::Borrowed("συνδιαλυω"), Separation::Inseparable),
        (Cow::Borrowed("συνεισκατοικεω"), Separation::Inseparable),
        (Cow::Borrowed("συνεξερχομαι"), Separation::Inseparable),
        (Cow::Borrowed("συνετεω"), Separation::Inseparable),
        (Cow::Borrowed("συνηλοω"), Separation::Inseparable),
        (Cow::Borrowed("συνθαπτω"), Separation::Inseparable),
        (Cow::Borrowed("συνθλιβω"), Separation::Inseparable),
        (Cow::Borrowed("συντασσω"), Separation::Inseparable),
        (Cow::Borrowed("συντεμνω"), Separation::Inseparable),
        (Cow::Borrowed("συντιθημι"), Separation::Inseparable),
        (Cow::Borrowed("συντριβω"), Separation::Inseparable),
        (Cow::Borrowed("υπεραττικιζω"), Separation::Neither),
        (Cow::Borrowed("υπερνικαω"), Separation::Neither),
        (Cow::Borrowed("υπερφερω"), Separation::Neither),
        (Cow::Borrowed("υπογιγνομαι"), Separation::Neither),
        (Cow::Borrowed("υποκοριζομαι"), Separation::Neither),
        (Cow::Borrowed("υπολαμβανω"), Separation::Neither),
        (Cow::Borrowed("υπολογιζομαι"), Separation::Neither),
        (Cow::Borrowed("υπονοεω"), Separation::Neither),
        (Cow::Borrowed("υποστηριζω"), Separation::Neither),
        (Cow::Borrowed("υποφερω"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // IND_ACT_1_SG
        Before, // IND_ACT_1_DU
        Before, // IND_ACT_1_PL
        Before, // IND_ACT_2_SG
        Before, // IND_ACT_2_DU
        Before, // IND_ACT_2_PL
        Before, // IND_ACT_3_SG
        Before, // IND_ACT_3_DU
        Before, // IND_ACT_3_PL
        Before, // IND_MID_1_SG
        Before, // IND_MID_1_DU
        Before, // IND_MID_1_PL
        Before, // IND_MID_2_SG
        Before, // IND_MID_2_DU
        Before, // IND_MID_2_PL
        Before, // IND_MID_3_SG
        Before, // IND_MID_3_DU
        Before, // IND_MID_3_PL
        Before, // IND_PASS_1_SG
        Before, // IND_PASS_1_DU
        Before, // IND_PASS_1_PL
        Before, // IND_PASS_2_SG
        Before, // IND_PASS_2_DU
        Before, // IND_PASS_2_PL
        Before, // IND_PASS_3_SG
        Before, // IND_PASS_3_DU
        Before, // IND_PASS_3_PL
        Before, // SUBJ_ACT_1_SG
        Before, // SUBJ_ACT_1_DU
        Before, // SUBJ_ACT_1_PL
        Before, // SUBJ_ACT_2_SG
        Before, // SUBJ_ACT_2_DU
        Before, // SUBJ_ACT_2_PL
        Before, // SUBJ_ACT_3_SG
        Before, // SUBJ_ACT_3_DU
        Before, // SUBJ_ACT_3_PL
        Before, // SUBJ_MID_1_SG
        Before, // SUBJ_MID_1_DU
        Before, // SUBJ_MID_1_PL
        Before, // SUBJ_MID_2_SG
        Before, // SUBJ_MID_2_DU
        Before, // SUBJ_MID_2_PL
        Before, // SUBJ_MID_3_SG
        Before, // SUBJ_MID_3_DU
        Before, // SUBJ_MID_3_PL
        Before, // SUBJ_PASS_1_SG
        Before, // SUBJ_PASS_1_DU
        Before, // SUBJ_PASS_1_PL
        Before, // SUBJ_PASS_2_SG
        Before, // SUBJ_PASS_2_DU
        Before, // SUBJ_PASS_2_PL
        Before, // SUBJ_PASS_3_SG
        Before, // SUBJ_PASS_3_DU
        Before, // SUBJ_PASS_3_PL
        Before, // OPT_ACT_1_SG
        Before, // OPT_ACT_1_DU
        Before, // OPT_ACT_1_PL
        Before, // OPT_ACT_2_SG
        Before, // OPT_ACT_2_DU
        Before, // OPT_ACT_2_PL
        Before, // OPT_ACT_3_SG
        Before, // OPT_ACT_3_DU
        Before, // OPT_ACT_3_PL
        Before, // OPT_MID_1_SG
        Before, // OPT_MID_1_DU
        Before, // OPT_MID_1_PL
        Before, // OPT_MID_2_SG
        Before, // OPT_MID_2_DU
        Before, // OPT_MID_2_PL
        Before, // OPT_MID_3_SG
        Before, // OPT_MID_3_DU
        Before, // OPT_MID_3_PL
        Before, // OPT_PASS_1_SG
        Before, // OPT_PASS_1_DU
        Before, // OPT_PASS_1_PL
        Before, // OPT_PASS_2_SG
        Before, // OPT_PASS_2_DU
        Before, // OPT_PASS_2_PL
        Before, // OPT_PASS_3_SG
        Before, // OPT_PASS_3_DU
        Before, // OPT_PASS_3_PL
        Before, // PRESENT_IMPERATIVE_ACT_2_SG
        Before, // PRESENT_IMPERATIVE_ACT_2_DU
        Before, // PRESENT_IMPERATIVE_ACT_2_PL
        Before, // PRESENT_IMPERATIVE_ACT_3_SG
        Before, // PRESENT_IMPERATIVE_ACT_3_DU
        Before, // PRESENT_IMPERATIVE_ACT_3_PL
        Before, // PRESENT_IMPERATIVE_MID_2_SG
        Before, // PRESENT_IMPERATIVE_MID_2_DU
        Before, // PRESENT_IMPERATIVE_MID_2_PL
        Before, // PRESENT_IMPERATIVE_MID_3_SG
        Before, // PRESENT_IMPERATIVE_MID_3_DU
        Before, // PRESENT_IMPERATIVE_MID_3_PL
        Before, // PRESENT_IMPERATIVE_PASS_2_SG
        Before, // PRESENT_IMPERATIVE_PASS_2_DU
        Before, // PRESENT_IMPERATIVE_PASS_2_PL
        Before, // PRESENT_IMPERATIVE_PASS_3_SG
        Before, // PRESENT_IMPERATIVE_PASS_3_DU
        Before, // PRESENT_IMPERATIVE_PASS_3_PL
        Before, // IMPERFECT_ACT_1_SG
        Before, // IMPERFECT_ACT_1_DU
        Before, // IMPERFECT_ACT_1_PL
        Before, // IMPERFECT_ACT_2_SG
        Before, // IMPERFECT_ACT_2_DU
        Before, // IMPERFECT_ACT_2_PL
        Before, // IMPERFECT_ACT_3_SG
        Before, // IMPERFECT_ACT_3_DU
        Before, // IMPERFECT_ACT_3_PL
        Before, // IMPERFECT_MID_1_SG
        Before, // IMPERFECT_MID_1_DU
        Before, // IMPERFECT_MID_1_PL
        Before, // IMPERFECT_MID_2_SG
        Before, // IMPERFECT_MID_2_DU
        Before, // IMPERFECT_MID_2_PL
        Before, // IMPERFECT_MID_3_SG
        Before, // IMPERFECT_MID_3_DU
        Before, // IMPERFECT_MID_3_PL
        Before, // IMPERFECT_PASS_1_SG
        Before, // IMPERFECT_PASS_1_DU
        Before, // IMPERFECT_PASS_1_PL
        Before, // IMPERFECT_PASS_2_SG
        Before, // IMPERFECT_PASS_2_DU
        Before, // IMPERFECT_PASS_2_PL
        Before, // IMPERFECT_PASS_3_SG
        Before, // IMPERFECT_PASS_3_DU
        Before, // IMPERFECT_PASS_3_PL
        Before, // FUTURE_IND_ACT_1_SG
        Before, // FUTURE_IND_ACT_1_DU
        Before, // FUTURE_IND_ACT_1_PL
        Before, // FUTURE_IND_ACT_2_SG
        Before, // FUTURE_IND_ACT_2_DU
        Before, // FUTURE_IND_ACT_2_PL
        Before, // FUTURE_IND_ACT_3_SG
        Before, // FUTURE_IND_ACT_3_DU
        Before, // FUTURE_IND_ACT_3_PL
        Before, // FUTURE_IND_MID_1_SG
        Before, // FUTURE_IND_MID_1_DU
        Before, // FUTURE_IND_MID_1_PL
        Before, // FUTURE_IND_MID_2_SG
        Before, // FUTURE_IND_MID_2_DU
        Before, // FUTURE_IND_MID_2_PL
        Before, // FUTURE_IND_MID_3_SG
        Before, // FUTURE_IND_MID_3_DU
        Before, // FUTURE_IND_MID_3_PL
        Before, // FUTURE_IND_PASS_1_SG
        Before, // FUTURE_IND_PASS_1_DU
        Before, // FUTURE_IND_PASS_1_PL
        Before, // FUTURE_IND_PASS_2_SG
        Before, // FUTURE_IND_PASS_2_DU
        Before, // FUTURE_IND_PASS_2_PL
        Before, // FUTURE_IND_PASS_3_SG
        Before, // FUTURE_IND_PASS_3_DU
        Before, // FUTURE_IND_PASS_3_PL
        Before, // FUTURE_OPT_ACT_1_SG
        Before, // FUTURE_OPT_ACT_1_DU
        Before, // FUTURE_OPT_ACT_1_PL
        Before, // FUTURE_OPT_ACT_2_SG
        Before, // FUTURE_OPT_ACT_2_DU
        Before, // FUTURE_OPT_ACT_2_PL
        Before, // FUTURE_OPT_ACT_3_SG
        Before, // FUTURE_OPT_ACT_3_DU
        Before, // FUTURE_OPT_ACT_3_PL
        Before, // FUTURE_OPT_MID_1_SG
        Before, // FUTURE_OPT_MID_1_DU
        Before, // FUTURE_OPT_MID_1_PL
        Before, // FUTURE_OPT_MID_2_SG
        Before, // FUTURE_OPT_MID_2_DU
        Before, // FUTURE_OPT_MID_2_PL
        Before, // FUTURE_OPT_MID_3_SG
        Before, // FUTURE_OPT_MID_3_DU
        Before, // FUTURE_OPT_MID_3_PL
        Before, // FUTURE_OPT_PASS_1_SG
        Before, // FUTURE_OPT_PASS_1_DU
        Before, // FUTURE_OPT_PASS_1_PL
        Before, // FUTURE_OPT_PASS_2_SG
        Before, // FUTURE_OPT_PASS_2_DU
        Before, // FUTURE_OPT_PASS_2_PL
        Before, // FUTURE_OPT_PASS_3_SG
        Before, // FUTURE_OPT_PASS_3_DU
        Before, // FUTURE_OPT_PASS_3_PL
        Before, // AORIST_IND_ACT_1_SG
        Before, // AORIST_IND_ACT_1_DU
        Before, // AORIST_IND_ACT_1_PL
        Before, // AORIST_IND_ACT_2_SG
        Before, // AORIST_IND_ACT_2_DU
        Before, // AORIST_IND_ACT_2_PL
        Before, // AORIST_IND_ACT_3_SG
        Before, // AORIST_IND_ACT_3_DU
        Before, // AORIST_IND_ACT_3_PL
        Before, // AORIST_IND_MID_1_SG
        Before, // AORIST_IND_MID_1_DU
        Before, // AORIST_IND_MID_1_PL
        Before, // AORIST_IND_MID_2_SG
        Before, // AORIST_IND_MID_2_DU
        Before, // AORIST_IND_MID_2_PL
        Before, // AORIST_IND_MID_3_SG
        Before, // AORIST_IND_MID_3_DU
        Before, // AORIST_IND_MID_3_PL
        Before, // AORIST_IND_PASS_1_SG
        Before, // AORIST_IND_PASS_1_DU
        Before, // AORIST_IND_PASS_1_PL
        Before, // AORIST_IND_PASS_2_SG
        Before, // AORIST_IND_PASS_2_DU
        Before, // AORIST_IND_PASS_2_PL
        Before, // AORIST_IND_PASS_3_SG
        Before, // AORIST_IND_PASS_3_DU
        Before, // AORIST_IND_PASS_3_PL
        Before, // AORIST_SUBJ_ACT_1_SG
        Before, // AORIST_SUBJ_ACT_1_DU
        Before, // AORIST_SUBJ_ACT_1_PL
        Before, // AORIST_SUBJ_ACT_2_SG
        Before, // AORIST_SUBJ_ACT_2_DU
        Before, // AORIST_SUBJ_ACT_2_PL
        Before, // AORIST_SUBJ_ACT_3_SG
        Before, // AORIST_SUBJ_ACT_3_DU
        Before, // AORIST_SUBJ_ACT_3_PL
        Before, // AORIST_SUBJ_MID_1_SG
        Before, // AORIST_SUBJ_MID_1_DU
        Before, // AORIST_SUBJ_MID_1_PL
        Before, // AORIST_SUBJ_MID_2_SG
        Before, // AORIST_SUBJ_MID_2_DU
        Before, // AORIST_SUBJ_MID_2_PL
        Before, // AORIST_SUBJ_MID_3_SG
        Before, // AORIST_SUBJ_MID_3_DU
        Before, // AORIST_SUBJ_MID_3_PL
        Before, // AORIST_SUBJ_PASS_1_SG
        Before, // AORIST_SUBJ_PASS_1_DU
        Before, // AORIST_SUBJ_PASS_1_PL
        Before, // AORIST_SUBJ_PASS_2_SG
        Before, // AORIST_SUBJ_PASS_2_DU
        Before, // AORIST_SUBJ_PASS_2_PL
        Before, // AORIST_SUBJ_PASS_3_SG
        Before, // AORIST_SUBJ_PASS_3_DU
        Before, // AORIST_SUBJ_PASS_3_PL
        Before, // AORIST_OPT_ACT_1_SG
        Before, // AORIST_OPT_ACT_1_DU
        Before, // AORIST_OPT_ACT_1_PL
        Before, // AORIST_OPT_ACT_2_SG
        Before, // AORIST_OPT_ACT_2_DU
        Before, // AORIST_OPT_ACT_2_PL
        Before, // AORIST_OPT_ACT_3_SG
        Before, // AORIST_OPT_ACT_3_DU
        Before, // AORIST_OPT_ACT_3_PL
        Before, // AORIST_OPT_MID_1_SG
        Before, // AORIST_OPT_MID_1_DU
        Before, // AORIST_OPT_MID_1_PL
        Before, // AORIST_OPT_MID_2_SG
        Before, // AORIST_OPT_MID_2_DU
        Before, // AORIST_OPT_MID_2_PL
        Before, // AORIST_OPT_MID_3_SG
        Before, // AORIST_OPT_MID_3_DU
        Before, // AORIST_OPT_MID_3_PL
        Before, // AORIST_OPT_PASS_1_SG
        Before, // AORIST_OPT_PASS_1_DU
        Before, // AORIST_OPT_PASS_1_PL
        Before, // AORIST_OPT_PASS_2_SG
        Before, // AORIST_OPT_PASS_2_DU
        Before, // AORIST_OPT_PASS_2_PL
        Before, // AORIST_OPT_PASS_3_SG
        Before, // AORIST_OPT_PASS_3_DU
        Before, // AORIST_OPT_PASS_3_PL
        Before, // AORIST_IMPERATIVE_ACT_2_SG
        Before, // AORIST_IMPERATIVE_ACT_2_DU
        Before, // AORIST_IMPERATIVE_ACT_2_PL
        Before, // AORIST_IMPERATIVE_ACT_3_SG
        Before, // AORIST_IMPERATIVE_ACT_3_DU
        Before, // AORIST_IMPERATIVE_ACT_3_PL
        Before, // AORIST_IMPERATIVE_MID_2_SG
        Before, // AORIST_IMPERATIVE_MID_2_DU
        Before, // AORIST_IMPERATIVE_MID_2_PL
        Before, // AORIST_IMPERATIVE_MID_3_SG
        Before, // AORIST_IMPERATIVE_MID_3_DU
        Before, // AORIST_IMPERATIVE_MID_3_PL
        Before, // AORIST_IMPERATIVE_PASS_2_SG
        Before, // AORIST_IMPERATIVE_PASS_2_DU
        Before, // AORIST_IMPERATIVE_PASS_2_PL
        Before, // AORIST_IMPERATIVE_PASS_3_SG
        Before, // AORIST_IMPERATIVE_PASS_3_DU
        Before, // AORIST_IMPERATIVE_PASS_3_PL
        Before, // PERFECT_IND_ACT_1_SG
        Before, // PERFECT_IND_ACT_1_DU
        Before, // PERFECT_IND_ACT_1_PL
        Before, // PERFECT_IND_ACT_2_SG
        Before, // PERFECT_IND_ACT_2_DU
        Before, // PERFECT_IND_ACT_2_PL
        Before, // PERFECT_IND_ACT_3_SG
        Before, // PERFECT_IND_ACT_3_DU
        Before, // PERFECT_IND_ACT_3_PL
        Before, // PERFECT_IND_MID_1_SG
        Before, // PERFECT_IND_MID_1_DU
        Before, // PERFECT_IND_MID_1_PL
        Before, // PERFECT_IND_MID_2_SG
        Before, // PERFECT_IND_MID_2_DU
        Before, // PERFECT_IND_MID_2_PL
        Before, // PERFECT_IND_MID_3_SG
        Before, // PERFECT_IND_MID_3_DU
        Before, // PERFECT_IND_MID_3_PL
        Before, // PERFECT_IND_PASS_1_SG
        Before, // PERFECT_IND_PASS_1_DU
        Before, // PERFECT_IND_PASS_1_PL
        Before, // PERFECT_IND_PASS_2_SG
        Before, // PERFECT_IND_PASS_2_DU
        Before, // PERFECT_IND_PASS_2_PL
        Before, // PERFECT_IND_PASS_3_SG
        Before, // PERFECT_IND_PASS_3_DU
        Before, // PERFECT_IND_PASS_3_PL
        Before, // PERFECT_SUBJ_ACT_1_SG
        Before, // PERFECT_SUBJ_ACT_1_DU
        Before, // PERFECT_SUBJ_ACT_1_PL
        Before, // PERFECT_SUBJ_ACT_2_SG
        Before, // PERFECT_SUBJ_ACT_2_DU
        Before, // PERFECT_SUBJ_ACT_2_PL
        Before, // PERFECT_SUBJ_ACT_3_SG
        Before, // PERFECT_SUBJ_ACT_3_DU
        Before, // PERFECT_SUBJ_ACT_3_PL
        Before, // PERFECT_SUBJ_MID_1_SG
        Before, // PERFECT_SUBJ_MID_1_DU
        Before, // PERFECT_SUBJ_MID_1_PL
        Before, // PERFECT_SUBJ_MID_2_SG
        Before, // PERFECT_SUBJ_MID_2_DU
        Before, // PERFECT_SUBJ_MID_2_PL
        Before, // PERFECT_SUBJ_MID_3_SG
        Before, // PERFECT_SUBJ_MID_3_DU
        Before, // PERFECT_SUBJ_MID_3_PL
        Before, // PERFECT_SUBJ_PASS_1_SG
        Before, // PERFECT_SUBJ_PASS_1_DU
        Before, // PERFECT_SUBJ_PASS_1_PL
        Before, // PERFECT_SUBJ_PASS_2_SG
        Before, // PERFECT_SUBJ_PASS_2_DU
        Before, // PERFECT_SUBJ_PASS_2_PL
        Before, // PERFECT_SUBJ_PASS_3_SG
        Before, // PERFECT_SUBJ_PASS_3_DU
        Before, // PERFECT_SUBJ_PASS_3_PL
        Before, // PERFECT_OPT_ACT_1_SG
        Before, // PERFECT_OPT_ACT_1_DU
        Before, // PERFECT_OPT_ACT_1_PL
        Before, // PERFECT_OPT_ACT_2_SG
        Before, // PERFECT_OPT_ACT_2_DU
        Before, // PERFECT_OPT_ACT_2_PL
        Before, // PERFECT_OPT_ACT_3_SG
        Before, // PERFECT_OPT_ACT_3_DU
        Before, // PERFECT_OPT_ACT_3_PL
        Before, // PERFECT_OPT_MID_1_SG
        Before, // PERFECT_OPT_MID_1_DU
        Before, // PERFECT_OPT_MID_1_PL
        Before, // PERFECT_OPT_MID_2_SG
        Before, // PERFECT_OPT_MID_2_DU
        Before, // PERFECT_OPT_MID_2_PL
        Before, // PERFECT_OPT_MID_3_SG
        Before, // PERFECT_OPT_MID_3_DU
        Before, // PERFECT_OPT_MID_3_PL
        Before, // PERFECT_OPT_PASS_1_SG
        Before, // PERFECT_OPT_PASS_1_DU
        Before, // PERFECT_OPT_PASS_1_PL
        Before, // PERFECT_OPT_PASS_2_SG
        Before, // PERFECT_OPT_PASS_2_DU
        Before, // PERFECT_OPT_PASS_2_PL
        Before, // PERFECT_OPT_PASS_3_SG
        Before, // PERFECT_OPT_PASS_3_DU
        Before, // PERFECT_OPT_PASS_3_PL
        Before, // PERFECT_IMPERATIVE_ACT_2_SG
        Before, // PERFECT_IMPERATIVE_ACT_2_DU
        Before, // PERFECT_IMPERATIVE_ACT_2_PL
        Before, // PERFECT_IMPERATIVE_ACT_3_SG
        Before, // PERFECT_IMPERATIVE_ACT_3_DU
        Before, // PERFECT_IMPERATIVE_ACT_3_PL
        Before, // PERFECT_IMPERATIVE_MID_2_SG
        Before, // PERFECT_IMPERATIVE_MID_2_DU
        Before, // PERFECT_IMPERATIVE_MID_2_PL
        Before, // PERFECT_IMPERATIVE_MID_3_SG
        Before, // PERFECT_IMPERATIVE_MID_3_DU
        Before, // PERFECT_IMPERATIVE_MID_3_PL
        Before, // PERFECT_IMPERATIVE_PASS_2_SG
        Before, // PERFECT_IMPERATIVE_PASS_2_DU
        Before, // PERFECT_IMPERATIVE_PASS_2_PL
        Before, // PERFECT_IMPERATIVE_PASS_3_SG
        Before, // PERFECT_IMPERATIVE_PASS_3_DU
        Before, // PERFECT_IMPERATIVE_PASS_3_PL
        Before, // PLUPERFECT_ACT_1_SG
        Before, // PLUPERFECT_ACT_1_DU
        Before, // PLUPERFECT_ACT_1_PL
        Before, // PLUPERFECT_ACT_2_SG
        Before, // PLUPERFECT_ACT_2_DU
        Before, // PLUPERFECT_ACT_2_PL
        Before, // PLUPERFECT_ACT_3_SG
        Before, // PLUPERFECT_ACT_3_DU
        Before, // PLUPERFECT_ACT_3_PL
        Before, // PLUPERFECT_MID_1_SG
        Before, // PLUPERFECT_MID_1_DU
        Before, // PLUPERFECT_MID_1_PL
        Before, // PLUPERFECT_MID_2_SG
        Before, // PLUPERFECT_MID_2_DU
        Before, // PLUPERFECT_MID_2_PL
        Before, // PLUPERFECT_MID_3_SG
        Before, // PLUPERFECT_MID_3_DU
        Before, // PLUPERFECT_MID_3_PL
        Before, // PLUPERFECT_PASS_1_SG
        Before, // PLUPERFECT_PASS_1_DU
        Before, // PLUPERFECT_PASS_1_PL
        Before, // PLUPERFECT_PASS_2_SG
        Before, // PLUPERFECT_PASS_2_DU
        Before, // PLUPERFECT_PASS_2_PL
        Before, // PLUPERFECT_PASS_3_SG
        Before, // PLUPERFECT_PASS_3_DU
        Before, // PLUPERFECT_PASS_3_PL
        Before, // FUTURE_PERFECT_IND_ACT_1_SG
        Before, // FUTURE_PERFECT_IND_ACT_1_DU
        Before, // FUTURE_PERFECT_IND_ACT_1_PL
        Before, // FUTURE_PERFECT_IND_ACT_2_SG
        Before, // FUTURE_PERFECT_IND_ACT_2_DU
        Before, // FUTURE_PERFECT_IND_ACT_2_PL
        Before, // FUTURE_PERFECT_IND_ACT_3_SG
        Before, // FUTURE_PERFECT_IND_ACT_3_DU
        Before, // FUTURE_PERFECT_IND_ACT_3_PL
        Before, // FUTURE_PERFECT_IND_MID_1_SG
        Before, // FUTURE_PERFECT_IND_MID_1_DU
        Before, // FUTURE_PERFECT_IND_MID_1_PL
        Before, // FUTURE_PERFECT_IND_MID_2_SG
        Before, // FUTURE_PERFECT_IND_MID_2_DU
        Before, // FUTURE_PERFECT_IND_MID_2_PL
        Before, // FUTURE_PERFECT_IND_MID_3_SG
        Before, // FUTURE_PERFECT_IND_MID_3_DU
        Before, // FUTURE_PERFECT_IND_MID_3_PL
        Before, // FUTURE_PERFECT_IND_PASS_1_SG
        Before, // FUTURE_PERFECT_IND_PASS_1_DU
        Before, // FUTURE_PERFECT_IND_PASS_1_PL
        Before, // FUTURE_PERFECT_IND_PASS_2_SG
        Before, // FUTURE_PERFECT_IND_PASS_2_DU
        Before, // FUTURE_PERFECT_IND_PASS_2_PL
        Before, // FUTURE_PERFECT_IND_PASS_3_SG
        Before, // FUTURE_PERFECT_IND_PASS_3_DU
        Before, // FUTURE_PERFECT_IND_PASS_3_PL
        Before, // FUTURE_PERFECT_OPT_ACT_1_SG
        Before, // FUTURE_PERFECT_OPT_ACT_1_DU
        Before, // FUTURE_PERFECT_OPT_ACT_1_PL
        Before, // FUTURE_PERFECT_OPT_ACT_2_SG
        Before, // FUTURE_PERFECT_OPT_ACT_2_DU
        Before, // FUTURE_PERFECT_OPT_ACT_2_PL
        Before, // FUTURE_PERFECT_OPT_ACT_3_SG
        Before, // FUTURE_PERFECT_OPT_ACT_3_DU
        Before, // FUTURE_PERFECT_OPT_ACT_3_PL
        Before, // FUTURE_PERFECT_OPT_MID_1_SG
        Before, // FUTURE_PERFECT_OPT_MID_1_DU
        Before, // FUTURE_PERFECT_OPT_MID_1_PL
        Before, // FUTURE_PERFECT_OPT_MID_2_SG
        Before, // FUTURE_PERFECT_OPT_MID_2_DU
        Before, // FUTURE_PERFECT_OPT_MID_2_PL
        Before, // FUTURE_PERFECT_OPT_MID_3_SG
        Before, // FUTURE_PERFECT_OPT_MID_3_DU
        Before, // FUTURE_PERFECT_OPT_MID_3_PL
        Before, // FUTURE_PERFECT_OPT_PASS_1_SG
        Before, // FUTURE_PERFECT_OPT_PASS_1_DU
        Before, // FUTURE_PERFECT_OPT_PASS_1_PL
        Before, // FUTURE_PERFECT_OPT_PASS_2_SG
        Before, // FUTURE_PERFECT_OPT_PASS_2_DU
        Before, // FUTURE_PERFECT_OPT_PASS_2_PL
        Before, // FUTURE_PERFECT_OPT_PASS_3_SG
        Before, // FUTURE_PERFECT_OPT_PASS_3_DU
        Before, // FUTURE_PERFECT_OPT_PASS_3_PL
        Before, // INFINITIVE_PRES_ACT
        Before, // INFINITIVE_PRES_MID
        Before, // INFINITIVE_PRES_PASS
        Before, // INFINITIVE_FUT_ACT
        Before, // INFINITIVE_FUT_MID
        Before, // INFINITIVE_FUT_PASS
        Before, // INFINITIVE_AOR_ACT
        Before, // INFINITIVE_AOR_MID
        Before, // INFINITIVE_AOR_PASS
        Before, // INFINITIVE_PERF_ACT
        Before, // INFINITIVE_PERF_MID
        Before, // INFINITIVE_PERF_PASS
        Before, // INFINITIVE_FUTP_ACT
        Before, // INFINITIVE_FUTP_MID
        Before, // INFINITIVE_FUTP_PASS
        Before, // PARTICIPLE_PRES_ACT
        Before, // PARTICIPLE_PRES_MID
        Before, // PARTICIPLE_PRES_PASS
        Before, // PARTICIPLE_FUT_ACT
        Before, // PARTICIPLE_FUT_MID
        Before, // PARTICIPLE_FUT_PASS
        Before, // PARTICIPLE_AOR_ACT
        Before, // PARTICIPLE_AOR_MID
        Before, // PARTICIPLE_AOR_PASS
        Before, // PARTICIPLE_PERF_ACT
        Before, // PARTICIPLE_PERF_MID
        Before, // PARTICIPLE_PERF_PASS
        Before, // PARTICIPLE_FUTP_ACT
        Before, // PARTICIPLE_FUTP_MID
        Before, // PARTICIPLE_FUTP_PASS
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // IND_ACT_1_SG
        Cow::Borrowed(""), // IND_ACT_1_DU
        Cow::Borrowed(""), // IND_ACT_1_PL
        Cow::Borrowed(""), // IND_ACT_2_SG
        Cow::Borrowed(""), // IND_ACT_2_DU
        Cow::Borrowed(""), // IND_ACT_2_PL
        Cow::Borrowed(""), // IND_ACT_3_SG
        Cow::Borrowed(""), // IND_ACT_3_DU
        Cow::Borrowed(""), // IND_ACT_3_PL
        Cow::Borrowed(""), // IND_MID_1_SG
        Cow::Borrowed(""), // IND_MID_1_DU
        Cow::Borrowed(""), // IND_MID_1_PL
        Cow::Borrowed(""), // IND_MID_2_SG
        Cow::Borrowed(""), // IND_MID_2_DU
        Cow::Borrowed(""), // IND_MID_2_PL
        Cow::Borrowed(""), // IND_MID_3_SG
        Cow::Borrowed(""), // IND_MID_3_DU
        Cow::Borrowed(""), // IND_MID_3_PL
        Cow::Borrowed(""), // IND_PASS_1_SG
        Cow::Borrowed(""), // IND_PASS_1_DU
        Cow::Borrowed(""), // IND_PASS_1_PL
        Cow::Borrowed(""), // IND_PASS_2_SG
        Cow::Borrowed(""), // IND_PASS_2_DU
        Cow::Borrowed(""), // IND_PASS_2_PL
        Cow::Borrowed(""), // IND_PASS_3_SG
        Cow::Borrowed(""), // IND_PASS_3_DU
        Cow::Borrowed(""), // IND_PASS_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_1_DU
        Cow::Borrowed(""), // SUBJ_ACT_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_2_DU
        Cow::Borrowed(""), // SUBJ_ACT_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_3_DU
        Cow::Borrowed(""), // SUBJ_ACT_3_PL
        Cow::Borrowed(""), // SUBJ_MID_1_SG
        Cow::Borrowed(""), // SUBJ_MID_1_DU
        Cow::Borrowed(""), // SUBJ_MID_1_PL
        Cow::Borrowed(""), // SUBJ_MID_2_SG
        Cow::Borrowed(""), // SUBJ_MID_2_DU
        Cow::Borrowed(""), // SUBJ_MID_2_PL
        Cow::Borrowed(""), // SUBJ_MID_3_SG
        Cow::Borrowed(""), // SUBJ_MID_3_DU
        Cow::Borrowed(""), // SUBJ_MID_3_PL
        Cow::Borrowed(""), // SUBJ_PASS_1_SG
        Cow::Borrowed(""), // SUBJ_PASS_1_DU
        Cow::Borrowed(""), // SUBJ_PASS_1_PL
        Cow::Borrowed(""), // SUBJ_PASS_2_SG
        Cow::Borrowed(""), // SUBJ_PASS_2_DU
        Cow::Borrowed(""), // SUBJ_PASS_2_PL
        Cow::Borrowed(""), // SUBJ_PASS_3_SG
        Cow::Borrowed(""), // SUBJ_PASS_3_DU
        Cow::Borrowed(""), // SUBJ_PASS_3_PL
        Cow::Borrowed(""), // OPT_ACT_1_SG
        Cow::Borrowed(""), // OPT_ACT_1_DU
        Cow::Borrowed(""), // OPT_ACT_1_PL
        Cow::Borrowed(""), // OPT_ACT_2_SG
        Cow::Borrowed(""), // OPT_ACT_2_DU
        Cow::Borrowed(""), // OPT_ACT_2_PL
        Cow::Borrowed(""), // OPT_ACT_3_SG
        Cow::Borrowed(""), // OPT_ACT_3_DU
        Cow::Borrowed(""), // OPT_ACT_3_PL
        Cow::Borrowed(""), // OPT_MID_1_SG
        Cow::Borrowed(""), // OPT_MID_1_DU
        Cow::Borrowed(""), // OPT_MID_1_PL
        Cow::Borrowed(""), // OPT_MID_2_SG
        Cow::Borrowed(""), // OPT_MID_2_DU
        Cow::Borrowed(""), // OPT_MID_2_PL
        Cow::Borrowed(""), // OPT_MID_3_SG
        Cow::Borrowed(""), // OPT_MID_3_DU
        Cow::Borrowed(""), // OPT_MID_3_PL
        Cow::Borrowed(""), // OPT_PASS_1_SG
        Cow::Borrowed(""), // OPT_PASS_1_DU
        Cow::Borrowed(""), // OPT_PASS_1_PL
        Cow::Borrowed(""), // OPT_PASS_2_SG
        Cow::Borrowed(""), // OPT_PASS_2_DU
        Cow::Borrowed(""), // OPT_PASS_2_PL
        Cow::Borrowed(""), // OPT_PASS_3_SG
        Cow::Borrowed(""), // OPT_PASS_3_DU
        Cow::Borrowed(""), // OPT_PASS_3_PL
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_ACT_2_SG
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_ACT_2_DU
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_ACT_2_PL
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_ACT_3_SG
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_ACT_3_DU
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_ACT_3_PL
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_MID_2_SG
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_MID_2_DU
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_MID_2_PL
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_MID_3_SG
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_MID_3_DU
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_MID_3_PL
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_PASS_2_SG
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_PASS_2_DU
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_PASS_2_PL
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_PASS_3_SG
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_PASS_3_DU
        Cow::Borrowed(""), // PRESENT_IMPERATIVE_PASS_3_PL
        Cow::Borrowed(""), // IMPERFECT_ACT_1_SG
        Cow::Borrowed(""), // IMPERFECT_ACT_1_DU
        Cow::Borrowed(""), // IMPERFECT_ACT_1_PL
        Cow::Borrowed(""), // IMPERFECT_ACT_2_SG
        Cow::Borrowed(""), // IMPERFECT_ACT_2_DU
        Cow::Borrowed(""), // IMPERFECT_ACT_2_PL
        Cow::Borrowed(""), // IMPERFECT_ACT_3_SG
        Cow::Borrowed(""), // IMPERFECT_ACT_3_DU
        Cow::Borrowed(""), // IMPERFECT_ACT_3_PL
        Cow::Borrowed(""), // IMPERFECT_MID_1_SG
        Cow::Borrowed(""), // IMPERFECT_MID_1_DU
        Cow::Borrowed(""), // IMPERFECT_MID_1_PL
        Cow::Borrowed(""), // IMPERFECT_MID_2_SG
        Cow::Borrowed(""), // IMPERFECT_MID_2_DU
        Cow::Borrowed(""), // IMPERFECT_MID_2_PL
        Cow::Borrowed(""), // IMPERFECT_MID_3_SG
        Cow::Borrowed(""), // IMPERFECT_MID_3_DU
        Cow::Borrowed(""), // IMPERFECT_MID_3_PL
        Cow::Borrowed(""), // IMPERFECT_PASS_1_SG
        Cow::Borrowed(""), // IMPERFECT_PASS_1_DU
        Cow::Borrowed(""), // IMPERFECT_PASS_1_PL
        Cow::Borrowed(""), // IMPERFECT_PASS_2_SG
        Cow::Borrowed(""), // IMPERFECT_PASS_2_DU
        Cow::Borrowed(""), // IMPERFECT_PASS_2_PL
        Cow::Borrowed(""), // IMPERFECT_PASS_3_SG
        Cow::Borrowed(""), // IMPERFECT_PASS_3_DU
        Cow::Borrowed(""), // IMPERFECT_PASS_3_PL
        Cow::Borrowed(""), // FUTURE_IND_ACT_1_SG
        Cow::Borrowed(""), // FUTURE_IND_ACT_1_DU
        Cow::Borrowed(""), // FUTURE_IND_ACT_1_PL
        Cow::Borrowed(""), // FUTURE_IND_ACT_2_SG
        Cow::Borrowed(""), // FUTURE_IND_ACT_2_DU
        Cow::Borrowed(""), // FUTURE_IND_ACT_2_PL
        Cow::Borrowed(""), // FUTURE_IND_ACT_3_SG
        Cow::Borrowed(""), // FUTURE_IND_ACT_3_DU
        Cow::Borrowed(""), // FUTURE_IND_ACT_3_PL
        Cow::Borrowed(""), // FUTURE_IND_MID_1_SG
        Cow::Borrowed(""), // FUTURE_IND_MID_1_DU
        Cow::Borrowed(""), // FUTURE_IND_MID_1_PL
        Cow::Borrowed(""), // FUTURE_IND_MID_2_SG
        Cow::Borrowed(""), // FUTURE_IND_MID_2_DU
        Cow::Borrowed(""), // FUTURE_IND_MID_2_PL
        Cow::Borrowed(""), // FUTURE_IND_MID_3_SG
        Cow::Borrowed(""), // FUTURE_IND_MID_3_DU
        Cow::Borrowed(""), // FUTURE_IND_MID_3_PL
        Cow::Borrowed(""), // FUTURE_IND_PASS_1_SG
        Cow::Borrowed(""), // FUTURE_IND_PASS_1_DU
        Cow::Borrowed(""), // FUTURE_IND_PASS_1_PL
        Cow::Borrowed(""), // FUTURE_IND_PASS_2_SG
        Cow::Borrowed(""), // FUTURE_IND_PASS_2_DU
        Cow::Borrowed(""), // FUTURE_IND_PASS_2_PL
        Cow::Borrowed(""), // FUTURE_IND_PASS_3_SG
        Cow::Borrowed(""), // FUTURE_IND_PASS_3_DU
        Cow::Borrowed(""), // FUTURE_IND_PASS_3_PL
        Cow::Borrowed(""), // FUTURE_OPT_ACT_1_SG
        Cow::Borrowed(""), // FUTURE_OPT_ACT_1_DU
        Cow::Borrowed(""), // FUTURE_OPT_ACT_1_PL
        Cow::Borrowed(""), // FUTURE_OPT_ACT_2_SG
        Cow::Borrowed(""), // FUTURE_OPT_ACT_2_DU
        Cow::Borrowed(""), // FUTURE_OPT_ACT_2_PL
        Cow::Borrowed(""), // FUTURE_OPT_ACT_3_SG
        Cow::Borrowed(""), // FUTURE_OPT_ACT_3_DU
        Cow::Borrowed(""), // FUTURE_OPT_ACT_3_PL
        Cow::Borrowed(""), // FUTURE_OPT_MID_1_SG
        Cow::Borrowed(""), // FUTURE_OPT_MID_1_DU
        Cow::Borrowed(""), // FUTURE_OPT_MID_1_PL
        Cow::Borrowed(""), // FUTURE_OPT_MID_2_SG
        Cow::Borrowed(""), // FUTURE_OPT_MID_2_DU
        Cow::Borrowed(""), // FUTURE_OPT_MID_2_PL
        Cow::Borrowed(""), // FUTURE_OPT_MID_3_SG
        Cow::Borrowed(""), // FUTURE_OPT_MID_3_DU
        Cow::Borrowed(""), // FUTURE_OPT_MID_3_PL
        Cow::Borrowed(""), // FUTURE_OPT_PASS_1_SG
        Cow::Borrowed(""), // FUTURE_OPT_PASS_1_DU
        Cow::Borrowed(""), // FUTURE_OPT_PASS_1_PL
        Cow::Borrowed(""), // FUTURE_OPT_PASS_2_SG
        Cow::Borrowed(""), // FUTURE_OPT_PASS_2_DU
        Cow::Borrowed(""), // FUTURE_OPT_PASS_2_PL
        Cow::Borrowed(""), // FUTURE_OPT_PASS_3_SG
        Cow::Borrowed(""), // FUTURE_OPT_PASS_3_DU
        Cow::Borrowed(""), // FUTURE_OPT_PASS_3_PL
        Cow::Borrowed(""), // AORIST_IND_ACT_1_SG
        Cow::Borrowed(""), // AORIST_IND_ACT_1_DU
        Cow::Borrowed(""), // AORIST_IND_ACT_1_PL
        Cow::Borrowed(""), // AORIST_IND_ACT_2_SG
        Cow::Borrowed(""), // AORIST_IND_ACT_2_DU
        Cow::Borrowed(""), // AORIST_IND_ACT_2_PL
        Cow::Borrowed(""), // AORIST_IND_ACT_3_SG
        Cow::Borrowed(""), // AORIST_IND_ACT_3_DU
        Cow::Borrowed(""), // AORIST_IND_ACT_3_PL
        Cow::Borrowed(""), // AORIST_IND_MID_1_SG
        Cow::Borrowed(""), // AORIST_IND_MID_1_DU
        Cow::Borrowed(""), // AORIST_IND_MID_1_PL
        Cow::Borrowed(""), // AORIST_IND_MID_2_SG
        Cow::Borrowed(""), // AORIST_IND_MID_2_DU
        Cow::Borrowed(""), // AORIST_IND_MID_2_PL
        Cow::Borrowed(""), // AORIST_IND_MID_3_SG
        Cow::Borrowed(""), // AORIST_IND_MID_3_DU
        Cow::Borrowed(""), // AORIST_IND_MID_3_PL
        Cow::Borrowed(""), // AORIST_IND_PASS_1_SG
        Cow::Borrowed(""), // AORIST_IND_PASS_1_DU
        Cow::Borrowed(""), // AORIST_IND_PASS_1_PL
        Cow::Borrowed(""), // AORIST_IND_PASS_2_SG
        Cow::Borrowed(""), // AORIST_IND_PASS_2_DU
        Cow::Borrowed(""), // AORIST_IND_PASS_2_PL
        Cow::Borrowed(""), // AORIST_IND_PASS_3_SG
        Cow::Borrowed(""), // AORIST_IND_PASS_3_DU
        Cow::Borrowed(""), // AORIST_IND_PASS_3_PL
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_1_SG
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_1_DU
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_1_PL
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_2_SG
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_2_DU
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_2_PL
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_3_SG
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_3_DU
        Cow::Borrowed(""), // AORIST_SUBJ_ACT_3_PL
        Cow::Borrowed(""), // AORIST_SUBJ_MID_1_SG
        Cow::Borrowed(""), // AORIST_SUBJ_MID_1_DU
        Cow::Borrowed(""), // AORIST_SUBJ_MID_1_PL
        Cow::Borrowed(""), // AORIST_SUBJ_MID_2_SG
        Cow::Borrowed(""), // AORIST_SUBJ_MID_2_DU
        Cow::Borrowed(""), // AORIST_SUBJ_MID_2_PL
        Cow::Borrowed(""), // AORIST_SUBJ_MID_3_SG
        Cow::Borrowed(""), // AORIST_SUBJ_MID_3_DU
        Cow::Borrowed(""), // AORIST_SUBJ_MID_3_PL
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_1_SG
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_1_DU
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_1_PL
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_2_SG
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_2_DU
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_2_PL
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_3_SG
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_3_DU
        Cow::Borrowed(""), // AORIST_SUBJ_PASS_3_PL
        Cow::Borrowed(""), // AORIST_OPT_ACT_1_SG
        Cow::Borrowed(""), // AORIST_OPT_ACT_1_DU
        Cow::Borrowed(""), // AORIST_OPT_ACT_1_PL
        Cow::Borrowed(""), // AORIST_OPT_ACT_2_SG
        Cow::Borrowed(""), // AORIST_OPT_ACT_2_DU
        Cow::Borrowed(""), // AORIST_OPT_ACT_2_PL
        Cow::Borrowed(""), // AORIST_OPT_ACT_3_SG
        Cow::Borrowed(""), // AORIST_OPT_ACT_3_DU
        Cow::Borrowed(""), // AORIST_OPT_ACT_3_PL
        Cow::Borrowed(""), // AORIST_OPT_MID_1_SG
        Cow::Borrowed(""), // AORIST_OPT_MID_1_DU
        Cow::Borrowed(""), // AORIST_OPT_MID_1_PL
        Cow::Borrowed(""), // AORIST_OPT_MID_2_SG
        Cow::Borrowed(""), // AORIST_OPT_MID_2_DU
        Cow::Borrowed(""), // AORIST_OPT_MID_2_PL
        Cow::Borrowed(""), // AORIST_OPT_MID_3_SG
        Cow::Borrowed(""), // AORIST_OPT_MID_3_DU
        Cow::Borrowed(""), // AORIST_OPT_MID_3_PL
        Cow::Borrowed(""), // AORIST_OPT_PASS_1_SG
        Cow::Borrowed(""), // AORIST_OPT_PASS_1_DU
        Cow::Borrowed(""), // AORIST_OPT_PASS_1_PL
        Cow::Borrowed(""), // AORIST_OPT_PASS_2_SG
        Cow::Borrowed(""), // AORIST_OPT_PASS_2_DU
        Cow::Borrowed(""), // AORIST_OPT_PASS_2_PL
        Cow::Borrowed(""), // AORIST_OPT_PASS_3_SG
        Cow::Borrowed(""), // AORIST_OPT_PASS_3_DU
        Cow::Borrowed(""), // AORIST_OPT_PASS_3_PL
        Cow::Borrowed(""), // AORIST_IMPERATIVE_ACT_2_SG
        Cow::Borrowed(""), // AORIST_IMPERATIVE_ACT_2_DU
        Cow::Borrowed(""), // AORIST_IMPERATIVE_ACT_2_PL
        Cow::Borrowed(""), // AORIST_IMPERATIVE_ACT_3_SG
        Cow::Borrowed(""), // AORIST_IMPERATIVE_ACT_3_DU
        Cow::Borrowed(""), // AORIST_IMPERATIVE_ACT_3_PL
        Cow::Borrowed(""), // AORIST_IMPERATIVE_MID_2_SG
        Cow::Borrowed(""), // AORIST_IMPERATIVE_MID_2_DU
        Cow::Borrowed(""), // AORIST_IMPERATIVE_MID_2_PL
        Cow::Borrowed(""), // AORIST_IMPERATIVE_MID_3_SG
        Cow::Borrowed(""), // AORIST_IMPERATIVE_MID_3_DU
        Cow::Borrowed(""), // AORIST_IMPERATIVE_MID_3_PL
        Cow::Borrowed(""), // AORIST_IMPERATIVE_PASS_2_SG
        Cow::Borrowed(""), // AORIST_IMPERATIVE_PASS_2_DU
        Cow::Borrowed(""), // AORIST_IMPERATIVE_PASS_2_PL
        Cow::Borrowed(""), // AORIST_IMPERATIVE_PASS_3_SG
        Cow::Borrowed(""), // AORIST_IMPERATIVE_PASS_3_DU
        Cow::Borrowed(""), // AORIST_IMPERATIVE_PASS_3_PL
        Cow::Borrowed(""), // PERFECT_IND_ACT_1_SG
        Cow::Borrowed(""), // PERFECT_IND_ACT_1_DU
        Cow::Borrowed(""), // PERFECT_IND_ACT_1_PL
        Cow::Borrowed(""), // PERFECT_IND_ACT_2_SG
        Cow::Borrowed(""), // PERFECT_IND_ACT_2_DU
        Cow::Borrowed(""), // PERFECT_IND_ACT_2_PL
        Cow::Borrowed(""), // PERFECT_IND_ACT_3_SG
        Cow::Borrowed(""), // PERFECT_IND_ACT_3_DU
        Cow::Borrowed(""), // PERFECT_IND_ACT_3_PL
        Cow::Borrowed(""), // PERFECT_IND_MID_1_SG
        Cow::Borrowed(""), // PERFECT_IND_MID_1_DU
        Cow::Borrowed(""), // PERFECT_IND_MID_1_PL
        Cow::Borrowed(""), // PERFECT_IND_MID_2_SG
        Cow::Borrowed(""), // PERFECT_IND_MID_2_DU
        Cow::Borrowed(""), // PERFECT_IND_MID_2_PL
        Cow::Borrowed(""), // PERFECT_IND_MID_3_SG
        Cow::Borrowed(""), // PERFECT_IND_MID_3_DU
        Cow::Borrowed(""), // PERFECT_IND_MID_3_PL
        Cow::Borrowed(""), // PERFECT_IND_PASS_1_SG
        Cow::Borrowed(""), // PERFECT_IND_PASS_1_DU
        Cow::Borrowed(""), // PERFECT_IND_PASS_1_PL
        Cow::Borrowed(""), // PERFECT_IND_PASS_2_SG
        Cow::Borrowed(""), // PERFECT_IND_PASS_2_DU
        Cow::Borrowed(""), // PERFECT_IND_PASS_2_PL
        Cow::Borrowed(""), // PERFECT_IND_PASS_3_SG
        Cow::Borrowed(""), // PERFECT_IND_PASS_3_DU
        Cow::Borrowed(""), // PERFECT_IND_PASS_3_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_1_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_1_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_1_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_2_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_2_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_2_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_3_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_3_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_ACT_3_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_1_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_1_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_1_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_2_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_2_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_2_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_3_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_3_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_MID_3_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_1_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_1_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_1_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_2_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_2_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_2_PL
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_3_SG
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_3_DU
        Cow::Borrowed(""), // PERFECT_SUBJ_PASS_3_PL
        Cow::Borrowed(""), // PERFECT_OPT_ACT_1_SG
        Cow::Borrowed(""), // PERFECT_OPT_ACT_1_DU
        Cow::Borrowed(""), // PERFECT_OPT_ACT_1_PL
        Cow::Borrowed(""), // PERFECT_OPT_ACT_2_SG
        Cow::Borrowed(""), // PERFECT_OPT_ACT_2_DU
        Cow::Borrowed(""), // PERFECT_OPT_ACT_2_PL
        Cow::Borrowed(""), // PERFECT_OPT_ACT_3_SG
        Cow::Borrowed(""), // PERFECT_OPT_ACT_3_DU
        Cow::Borrowed(""), // PERFECT_OPT_ACT_3_PL
        Cow::Borrowed(""), // PERFECT_OPT_MID_1_SG
        Cow::Borrowed(""), // PERFECT_OPT_MID_1_DU
        Cow::Borrowed(""), // PERFECT_OPT_MID_1_PL
        Cow::Borrowed(""), // PERFECT_OPT_MID_2_SG
        Cow::Borrowed(""), // PERFECT_OPT_MID_2_DU
        Cow::Borrowed(""), // PERFECT_OPT_MID_2_PL
        Cow::Borrowed(""), // PERFECT_OPT_MID_3_SG
        Cow::Borrowed(""), // PERFECT_OPT_MID_3_DU
        Cow::Borrowed(""), // PERFECT_OPT_MID_3_PL
        Cow::Borrowed(""), // PERFECT_OPT_PASS_1_SG
        Cow::Borrowed(""), // PERFECT_OPT_PASS_1_DU
        Cow::Borrowed(""), // PERFECT_OPT_PASS_1_PL
        Cow::Borrowed(""), // PERFECT_OPT_PASS_2_SG
        Cow::Borrowed(""), // PERFECT_OPT_PASS_2_DU
        Cow::Borrowed(""), // PERFECT_OPT_PASS_2_PL
        Cow::Borrowed(""), // PERFECT_OPT_PASS_3_SG
        Cow::Borrowed(""), // PERFECT_OPT_PASS_3_DU
        Cow::Borrowed(""), // PERFECT_OPT_PASS_3_PL
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_ACT_2_SG
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_ACT_2_DU
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_ACT_2_PL
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_ACT_3_SG
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_ACT_3_DU
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_ACT_3_PL
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_MID_2_SG
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_MID_2_DU
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_MID_2_PL
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_MID_3_SG
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_MID_3_DU
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_MID_3_PL
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_PASS_2_SG
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_PASS_2_DU
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_PASS_2_PL
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_PASS_3_SG
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_PASS_3_DU
        Cow::Borrowed(""), // PERFECT_IMPERATIVE_PASS_3_PL
        Cow::Borrowed(""), // PLUPERFECT_ACT_1_SG
        Cow::Borrowed(""), // PLUPERFECT_ACT_1_DU
        Cow::Borrowed(""), // PLUPERFECT_ACT_1_PL
        Cow::Borrowed(""), // PLUPERFECT_ACT_2_SG
        Cow::Borrowed(""), // PLUPERFECT_ACT_2_DU
        Cow::Borrowed(""), // PLUPERFECT_ACT_2_PL
        Cow::Borrowed(""), // PLUPERFECT_ACT_3_SG
        Cow::Borrowed(""), // PLUPERFECT_ACT_3_DU
        Cow::Borrowed(""), // PLUPERFECT_ACT_3_PL
        Cow::Borrowed(""), // PLUPERFECT_MID_1_SG
        Cow::Borrowed(""), // PLUPERFECT_MID_1_DU
        Cow::Borrowed(""), // PLUPERFECT_MID_1_PL
        Cow::Borrowed(""), // PLUPERFECT_MID_2_SG
        Cow::Borrowed(""), // PLUPERFECT_MID_2_DU
        Cow::Borrowed(""), // PLUPERFECT_MID_2_PL
        Cow::Borrowed(""), // PLUPERFECT_MID_3_SG
        Cow::Borrowed(""), // PLUPERFECT_MID_3_DU
        Cow::Borrowed(""), // PLUPERFECT_MID_3_PL
        Cow::Borrowed(""), // PLUPERFECT_PASS_1_SG
        Cow::Borrowed(""), // PLUPERFECT_PASS_1_DU
        Cow::Borrowed(""), // PLUPERFECT_PASS_1_PL
        Cow::Borrowed(""), // PLUPERFECT_PASS_2_SG
        Cow::Borrowed(""), // PLUPERFECT_PASS_2_DU
        Cow::Borrowed(""), // PLUPERFECT_PASS_2_PL
        Cow::Borrowed(""), // PLUPERFECT_PASS_3_SG
        Cow::Borrowed(""), // PLUPERFECT_PASS_3_DU
        Cow::Borrowed(""), // PLUPERFECT_PASS_3_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_1_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_1_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_1_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_2_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_2_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_2_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_3_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_3_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_ACT_3_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_1_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_1_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_1_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_2_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_2_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_2_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_3_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_3_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_MID_3_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_1_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_1_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_1_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_2_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_2_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_2_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_3_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_3_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_IND_PASS_3_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_1_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_1_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_1_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_2_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_2_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_2_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_3_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_3_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_ACT_3_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_1_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_1_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_1_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_2_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_2_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_2_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_3_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_3_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_MID_3_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_1_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_1_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_1_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_2_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_2_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_2_PL
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_3_SG
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_3_DU
        Cow::Borrowed(""), // FUTURE_PERFECT_OPT_PASS_3_PL
        Cow::Borrowed(""), // INFINITIVE_PRES_ACT
        Cow::Borrowed(""), // INFINITIVE_PRES_MID
        Cow::Borrowed(""), // INFINITIVE_PRES_PASS
        Cow::Borrowed(""), // INFINITIVE_FUT_ACT
        Cow::Borrowed(""), // INFINITIVE_FUT_MID
        Cow::Borrowed(""), // INFINITIVE_FUT_PASS
        Cow::Borrowed(""), // INFINITIVE_AOR_ACT
        Cow::Borrowed(""), // INFINITIVE_AOR_MID
        Cow::Borrowed(""), // INFINITIVE_AOR_PASS
        Cow::Borrowed(""), // INFINITIVE_PERF_ACT
        Cow::Borrowed(""), // INFINITIVE_PERF_MID
        Cow::Borrowed(""), // INFINITIVE_PERF_PASS
        Cow::Borrowed(""), // INFINITIVE_FUTP_ACT
        Cow::Borrowed(""), // INFINITIVE_FUTP_MID
        Cow::Borrowed(""), // INFINITIVE_FUTP_PASS
        Cow::Borrowed(""), // PARTICIPLE_PRES_ACT
        Cow::Borrowed(""), // PARTICIPLE_PRES_MID
        Cow::Borrowed(""), // PARTICIPLE_PRES_PASS
        Cow::Borrowed(""), // PARTICIPLE_FUT_ACT
        Cow::Borrowed(""), // PARTICIPLE_FUT_MID
        Cow::Borrowed(""), // PARTICIPLE_FUT_PASS
        Cow::Borrowed(""), // PARTICIPLE_AOR_ACT
        Cow::Borrowed(""), // PARTICIPLE_AOR_MID
        Cow::Borrowed(""), // PARTICIPLE_AOR_PASS
        Cow::Borrowed(""), // PARTICIPLE_PERF_ACT
        Cow::Borrowed(""), // PARTICIPLE_PERF_MID
        Cow::Borrowed(""), // PARTICIPLE_PERF_PASS
        Cow::Borrowed(""), // PARTICIPLE_FUTP_ACT
        Cow::Borrowed(""), // PARTICIPLE_FUTP_MID
        Cow::Borrowed(""), // PARTICIPLE_FUTP_PASS
    ]),
    // How an inseparable prefix meets the rest's form, by the form's first
    // letter: what the two become.
    junctions: Cow::Borrowed(&[
        (
            Cow::Borrowed("αμφι"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἀμφε"),
        ), // αμφι + ἐ → ἀμφε
        (
            Cow::Borrowed("ανα"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἀνε"),
        ), // ανα + ἐ → ἀνε
        (
            Cow::Borrowed("αντι"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἀντε"),
        ), // αντι + ἐ → ἀντε
        (
            Cow::Borrowed("απο"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἀπε"),
        ), // απο + ἐ → ἀπε
        (
            Cow::Borrowed("δι"),
            Cow::Borrowed("ἀ"),
            Cow::Borrowed("δῐᾰ"),
        ), // δι + ἀ → δῐᾰ
        (
            Cow::Borrowed("δι"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("δῐε"),
        ), // δι + ἐ → δῐε
        (
            Cow::Borrowed("δι"),
            Cow::Borrowed("ἠ"),
            Cow::Borrowed("δῐη"),
        ), // δι + ἠ → δῐη
        (
            Cow::Borrowed("δια"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("δῐε"),
        ), // δια + ἐ → δῐε
        (
            Cow::Borrowed("εγ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐνε"),
        ), // εγ + ἐ → ἐνε
        (
            Cow::Borrowed("εισ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("εἰσε"),
        ), // εισ + ἐ → εἰσε
        (
            Cow::Borrowed("εκ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐξε"),
        ), // εκ + ἐ → ἐξε
        (
            Cow::Borrowed("εμ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐνε"),
        ), // εμ + ἐ → ἐνε
        (
            Cow::Borrowed("εν"),
            Cow::Borrowed("ἀ"),
            Cow::Borrowed("ἐνᾰ"),
        ), // εν + ἀ → ἐνᾰ
        (
            Cow::Borrowed("εν"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐνε"),
        ), // εν + ἐ → ἐνε
        (
            Cow::Borrowed("εν"),
            Cow::Borrowed("ἠ"),
            Cow::Borrowed("ἐνη"),
        ), // εν + ἠ → ἐνη
        (
            Cow::Borrowed("επι"),
            Cow::Borrowed("κ"),
            Cow::Borrowed("ἐπικ"),
        ), // επι + κ → ἐπικ
        (
            Cow::Borrowed("επι"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐπε"),
        ), // επι + ἐ → ἐπε
        (
            Cow::Borrowed("επισ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐπεσ"),
        ), // επισ + ἐ → ἐπεσ
        (
            Cow::Borrowed("εσ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐσε"),
        ), // εσ + ἐ → ἐσε
        (
            Cow::Borrowed("κατα"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("κᾰτε"),
        ), // κατα + ἐ → κᾰτε
        (
            Cow::Borrowed("λα"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ἐλᾰ"),
        ), // λα + ἐ → ἐλᾰ
        (
            Cow::Borrowed("μετα"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("μετε"),
        ), // μετα + ἐ → μετε
        (
            Cow::Borrowed("ξυμ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ξῠνε"),
        ), // ξυμ + ἐ → ξῠνε
        (
            Cow::Borrowed("ξυν"),
            Cow::Borrowed("ἀ"),
            Cow::Borrowed("ξῠνᾰ"),
        ), // ξυν + ἀ → ξῠνᾰ
        (
            Cow::Borrowed("ξυν"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ξῠνε"),
        ), // ξυν + ἐ → ξῠνε
        (
            Cow::Borrowed("ξυν"),
            Cow::Borrowed("ἑ"),
            Cow::Borrowed("ξῠνε"),
        ), // ξυν + ἑ → ξῠνε
        (
            Cow::Borrowed("ξυν"),
            Cow::Borrowed("ἠ"),
            Cow::Borrowed("ξῠνη"),
        ), // ξυν + ἠ → ξῠνη
        (
            Cow::Borrowed("παρα"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("πᾰρε"),
        ), // παρα + ἐ → πᾰρε
        (
            Cow::Borrowed("περι"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("περῐε"),
        ), // περι + ἐ → περῐε
        (
            Cow::Borrowed("προ"),
            Cow::Borrowed("ἀ"),
            Cow::Borrowed("προᾰ"),
        ), // προ + ἀ → προᾰ
        (
            Cow::Borrowed("προ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("προε"),
        ), // προ + ἐ → προε
        (
            Cow::Borrowed("προ"),
            Cow::Borrowed("ἑ"),
            Cow::Borrowed("προε"),
        ), // προ + ἑ → προε
        (
            Cow::Borrowed("προ"),
            Cow::Borrowed("ἠ"),
            Cow::Borrowed("προη"),
        ), // προ + ἠ → προη
        (
            Cow::Borrowed("προκατα"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("προκᾰτε"),
        ), // προκατα + ἐ → προκᾰτε
        (
            Cow::Borrowed("προσ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("προσε"),
        ), // προσ + ἐ → προσε
        (
            Cow::Borrowed("προσ"),
            Cow::Borrowed("ἠ"),
            Cow::Borrowed("προση"),
        ), // προσ + ἠ → προση
        (
            Cow::Borrowed("συ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("σῠνε"),
        ), // συ + ἐ → σῠνε
        (
            Cow::Borrowed("συγ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("σῠνε"),
        ), // συγ + ἐ → σῠνε
        (
            Cow::Borrowed("συλ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("σῠνε"),
        ), // συλ + ἐ → σῠνε
        (
            Cow::Borrowed("συμ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("σῠνε"),
        ), // συμ + ἐ → σῠνε
        (
            Cow::Borrowed("συν"),
            Cow::Borrowed("ἀ"),
            Cow::Borrowed("σῠνᾰ"),
        ), // συν + ἀ → σῠνᾰ
        (
            Cow::Borrowed("συν"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("σῠνε"),
        ), // συν + ἐ → σῠνε
        (
            Cow::Borrowed("συν"),
            Cow::Borrowed("ἑ"),
            Cow::Borrowed("σῠνε"),
        ), // συν + ἑ → σῠνε
        (
            Cow::Borrowed("συν"),
            Cow::Borrowed("ἠ"),
            Cow::Borrowed("σῠνη"),
        ), // συν + ἠ → σῠνη
        (
            Cow::Borrowed("συν"),
            Cow::Borrowed("ἡ"),
            Cow::Borrowed("σῠνη"),
        ), // συν + ἡ → σῠνη
        (
            Cow::Borrowed("συν"),
            Cow::Borrowed("ᾠ"),
            Cow::Borrowed("σῠνῳ"),
        ), // συν + ᾠ → σῠνῳ
        (
            Cow::Borrowed("υπερ"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ὑπερε"),
        ), // υπερ + ἐ → ὑπερε
        (
            Cow::Borrowed("υπο"),
            Cow::Borrowed("ἐ"),
            Cow::Borrowed("ὑπε"),
        ), // υπο + ἐ → ὑπε
    ]),
};
