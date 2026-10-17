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
        // i+e → +ī: 910 words (servius, aquārius, pius, asclēpius), 16 forms wrong.
        law0: "i+e" => "+ī";
        // ī+ī → ī+: 552 words (padī, darī, geminī, dānī), 22 forms wrong.
        law1: "ī+ī" => "ī+";
        // ī+ō → +ō: 551 words (padī, darī, geminī, dānī), 25 forms wrong.
        law2: "ī+ō" => "+ō";
        // ia+e → ia+: 13 words (tiberias, abantias, jūlias, īlias), 0 forms wrong.
        law3: "ia+e" => "ia+";
        // [n|r|l|u|t|o|d]+nī → +ī: 141 words (saskatchewan, charōn, rāchēl, ōrīōn), 2 forms wrong.
        law4: "{x:[n|r|l|u|t|o|d]}+nī" => "{x}+ī";
        // [n|r|l|t|o|d]+ne → +e: 141 words (saskatchewan, charōn, rāchēl, ōrīōn), 4 forms wrong.
        law5: "{x:[n|r|l|t|o|d]}+ne" => "{x}+e";
        // [n|r|l|t|o|d]+ni → +i: 141 words (saskatchewan, charōn, rāchēl, ōrīōn), 6 forms wrong.
        law6: "{x:[n|r|l|t|o|d]}+ni" => "{x}+i";
        // sa+e → sa+: 6 words (asa, charbrusa, elusa, namusa), 0 forms wrong.
        law7: "sa+e" => "sa+";
        // ū+t → +unt: 8 words (hiericūs, daphnūs, elaeūs, phȳcūs), 0 forms wrong.
        law8: "ū+t" => "+unt";
        // ēn+t → +ent: 5 words (clēmēns, valēns, ūfēns, arcēns), 0 forms wrong.
        law9: "ēn+t" => "+ent";
    }
};
