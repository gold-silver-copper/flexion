//! Turning a (lemma, form) pair into an edit the runtime can replay on other
//! words: an optional added prefix, a stripped and an added suffix, and a
//! stress rule relative to that split.

use flexion::Spelling;
use flexion::format::Stress;
use flexion::write::EditSpec;

/// The edit from `base` (the lemma's letters, stressed on vowel
/// `lemma_stress`) to `form`. `lemma_cell` marks the lemma's own cell, whose
/// stress is described from the word's end so it can be predicted for words
/// the caller didn't stress.
pub fn edit(
    sp: &Spelling<'_>,
    base: &str,
    lemma_stress: Option<usize>,
    form: &str,
    lemma_cell: bool,
) -> EditSpec {
    let read = sp.read(form);
    let l: Vec<char> = base.chars().collect();
    let f: Vec<char> = read.base.chars().collect();
    let common = l
        .iter()
        .zip(&f)
        .take_while(|(a, b)| sp.fold_char(**a) == sp.fold_char(**b))
        .count();

    // A prefix was added when a long start of the lemma reappears later in
    // the form (beautiful → more beautiful, machen → gemacht).
    let mut split = (0usize, common, common); // (prefix length in form, kept lemma chars, form index after kept)
    for k in (common.saturating_add(1).max(3)..=l.len()).rev() {
        let Some(needle) = l.get(..k) else {
            continue;
        };
        let found = (1..f.len().saturating_sub(k).saturating_add(1))
            .find(|&at| f.get(at..at.saturating_add(k)).is_some_and(|w| w == needle));
        if let Some(at) = found {
            split = (at, k, at.saturating_add(k));
            break;
        }
    }
    let (plen, kept_len, after) = split;
    let collect = |s: Option<&[char]>| s.map(|c| c.iter().collect::<String>()).unwrap_or_default();
    let prefix = collect(f.get(..plen));
    let kept = collect(l.get(..kept_len));
    let strip = collect(l.get(kept_len..));
    let add = collect(f.get(after..));

    let stress = rule(
        sp,
        &prefix,
        &kept,
        &add,
        lemma_stress,
        read.stress,
        lemma_cell,
    );
    let stress = if sp.place(stress, lemma_stress, &prefix, &kept, &add) == read.stress {
        stress
    } else {
        Stress::Unmarked
    };
    EditSpec {
        prefix,
        strip,
        add,
        stress,
    }
}

fn rule(
    sp: &Spelling<'_>,
    prefix: &str,
    kept: &str,
    _add: &str,
    lemma_stress: Option<usize>,
    stress: Option<usize>,
    lemma_cell: bool,
) -> Stress {
    let Some(s) = stress else {
        return Stress::Unmarked;
    };
    let vp = sp.vowel_count(prefix);
    let vk = sp.vowel_count(kept);
    let small = |n: usize| u8::try_from(n).ok();
    if !lemma_cell
        && let Some(ls) = lemma_stress
        && ls < vk
        && s == vp.saturating_add(ls)
    {
        return Stress::SameAsLemma;
    }
    if s >= vp.saturating_add(vk) {
        return small(s - vp - vk).map_or(Stress::Unmarked, Stress::Added);
    }
    if s >= vp {
        let from_start = s - vp;
        let from_end = vk.saturating_sub(1).saturating_sub(from_start);
        if !lemma_cell && from_end != 0 && from_start == 0 {
            return Stress::KeptFromStart(0);
        }
        return small(from_end).map_or(Stress::Unmarked, Stress::KeptFromEnd);
    }
    small(s).map_or(Stress::Unmarked, Stress::Prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn en() -> Spelling<'static> {
        Spelling {
            fold_case: true,
            mark_monosyllables: false,
            stress_mark: None,
            vowels: "aeiouy",
            vowel_set: flexion::VowelSet::new("aeiouy"),
            implicit: "",
            fold_from: "",
            fold_to: "",
            strip_marks: "",
        }
    }

    fn ru() -> Spelling<'static> {
        Spelling {
            fold_case: true,
            mark_monosyllables: true,
            stress_mark: Some('\u{301}'),
            vowels: "аеёиоуыэюя",
            vowel_set: flexion::VowelSet::new("аеёиоуыэюя"),
            implicit: "ё",
            fold_from: "ё",
            fold_to: "е",
            strip_marks: "\u{300}",
        }
    }

    fn e(prefix: &str, strip: &str, add: &str, stress: Stress) -> EditSpec {
        EditSpec {
            prefix: prefix.into(),
            strip: strip.into(),
            add: add.into(),
            stress,
        }
    }

    #[test]
    fn suffix_and_prefix_edits() {
        let sp = en();
        assert_eq!(
            edit(&sp, "city", None, "cities", false),
            e("", "y", "ies", Stress::Unmarked)
        );
        assert_eq!(
            edit(&sp, "sing", None, "sang", false),
            e("", "ing", "ang", Stress::Unmarked)
        );
        assert_eq!(
            edit(&sp, "beautiful", None, "more beautiful", false),
            e("more ", "", "", Stress::Unmarked)
        );
        assert_eq!(
            edit(&sp, "good", None, "better", false),
            e("", "good", "better", Stress::Unmarked)
        );
    }

    #[test]
    fn stress_rules() {
        let sp = ru();
        // соба́ка → соба́ке: stress stays where the lemma has it.
        assert_eq!(
            edit(&sp, "собака", Some(1), "соба\u{301}ке", false).stress,
            Stress::SameAsLemma
        );
        // стол → стола́: on the ending.
        assert_eq!(
            edit(&sp, "стол", Some(0), "стола\u{301}", false).stress,
            Stress::Added(0)
        );
        // рука́ → ру́ки: onto the stem's last vowel.
        assert_eq!(
            edit(&sp, "рука", Some(1), "ру\u{301}ки", false).stress,
            Stress::KeptFromEnd(0)
        );
        // голова́ → го́ловы: back to the stem's first vowel.
        assert_eq!(
            edit(&sp, "голова", Some(2), "го\u{301}ловы", false).stress,
            Stress::KeptFromStart(0)
        );
        // The lemma cell counts from the word's end.
        assert_eq!(
            edit(&sp, "собака", Some(1), "соба\u{301}ка", true).stress,
            Stress::KeptFromEnd(1)
        );
    }
}
