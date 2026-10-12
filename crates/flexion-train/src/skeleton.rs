//! A word's skeleton: the letters it keeps in every form, in order, and each
//! form written as a pattern around them (`книга`, `книг`: `~а`, `~`). This
//! is the longest-common-subsequence analysis of Ahlberg, Forsberg & Hulden
//! (2014), done form by form against the lemma.

/// A word's forms as patterns over its skeleton.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Analysis {
    /// The skeleton's pieces: runs of letters every form keeps together.
    pub stems: Vec<String>,
    /// The lemma's pattern.
    pub lemma: String,
    /// One pattern per form given, in order (`None` for a form the skeleton
    /// can't be found in, which can't happen for forms it was built from).
    pub forms: Vec<Option<String>>,
}

/// The letters of `a` kept by a longest common subsequence with `b`, and
/// where in `b` each kept letter is. Of equal-length subsequences, the one
/// matching `a`'s letters as early in `b` as possible.
fn lcs(a: &[char], b: &[char]) -> (Vec<usize>, Vec<usize>) {
    let (n, m) = (a.len(), b.len());
    let width = m.saturating_add(1);
    let mut table = vec![0u16; n.saturating_add(1).saturating_mul(width)];
    let at = |i: usize, j: usize| i.saturating_mul(width).saturating_add(j);
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let v = if a.get(i) == b.get(j) {
                table.get(at(i + 1, j + 1)).copied().unwrap_or(0).saturating_add(1)
            } else {
                let down = table.get(at(i + 1, j)).copied().unwrap_or(0);
                let right = table.get(at(i, j + 1)).copied().unwrap_or(0);
                down.max(right)
            };
            if let Some(slot) = table.get_mut(at(i, j)) {
                *slot = v;
            }
        }
    }
    let get = |i: usize, j: usize| table.get(at(i, j)).copied().unwrap_or(0);
    let (mut i, mut j) = (0, 0);
    let (mut keep, mut pos) = (Vec::new(), Vec::new());
    while i < n && j < m {
        if a.get(i) == b.get(j) && get(i, j) == get(i + 1, j + 1).saturating_add(1) {
            keep.push(i);
            pos.push(j);
            i += 1;
            j += 1;
        } else if get(i + 1, j) >= get(i, j + 1) {
            i += 1;
        } else {
            j += 1;
        }
    }
    (keep, pos)
}

/// The skeleton of `lemma` and `forms`, and every form as a pattern over it.
pub fn analyze(lemma: &str, forms: &[&str]) -> Analysis {
    let lemma_chars: Vec<char> = lemma.chars().collect();
    let all: Vec<Vec<char>> = std::iter::once(lemma_chars.clone())
        .chain(forms.iter().map(|f| f.chars().collect()))
        .collect();
    // The lemma letters every form keeps.
    let mut kept: Vec<usize> = (0..lemma_chars.len()).collect();
    for form in all.iter().skip(1) {
        let sub: Vec<char> = kept.iter().filter_map(|&i| lemma_chars.get(i).copied()).collect();
        let (keep, _) = lcs(&sub, form);
        kept = keep.iter().filter_map(|&k| kept.get(k).copied()).collect();
    }
    let sub: Vec<char> = kept.iter().filter_map(|&i| lemma_chars.get(i).copied()).collect();
    // Where each kept letter is in each word.
    let places: Vec<Option<Vec<usize>>> = all
        .iter()
        .map(|w| {
            let (keep, pos) = lcs(&sub, w);
            (keep.len() == sub.len()).then_some(pos)
        })
        .collect();
    // Kept letters stay one stem piece while they are adjacent in every word.
    let mut runs: Vec<Vec<usize>> = Vec::new();
    for k in 0..sub.len() {
        let together = k > 0
            && places.iter().flatten().all(|p| {
                matches!((p.get(k - 1), p.get(k)), (Some(&x), Some(&y)) if y == x + 1)
            });
        match runs.last_mut() {
            Some(run) if together => run.push(k),
            _ => runs.push(vec![k]),
        }
    }
    let stems: Vec<String> = runs
        .iter()
        .map(|r| r.iter().filter_map(|&k| sub.get(k)).collect())
        .collect();
    let pattern = |w: &[char], p: &[usize]| -> String {
        let mut out = String::new();
        let mut last = 0usize;
        for r in &runs {
            let (Some(&first), Some(&end)) = (r.first(), r.last()) else {
                continue;
            };
            let (Some(&s), Some(&e)) = (p.get(first), p.get(end)) else {
                continue;
            };
            out.extend(w.get(last..s).unwrap_or_default());
            out.push('~');
            last = e + 1;
        }
        out.extend(w.get(last..).unwrap_or_default());
        out
    };
    let mut patterns = all.iter().zip(&places).map(|(w, p)| p.as_ref().map(|p| pattern(w, p)));
    let lemma_pattern = patterns.next().flatten().unwrap_or_else(|| lemma.to_string());
    Analysis {
        stems,
        lemma: lemma_pattern,
        forms: patterns.collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_skeleton() {
        let a = analyze("книга", &["книги", "книг", "книгой"]);
        assert_eq!(a.stems, vec!["книг"]);
        assert_eq!(a.lemma, "~а");
        assert_eq!(a.forms, vec![Some("~и".into()), Some("~".into()), Some("~ой".into())]);
        // A fleeting vowel splits the stem.
        let a = analyze("лесок", &["леска", "лесок"]);
        assert_eq!(a.stems, vec!["лес", "к"]);
        assert_eq!(a.lemma, "~о~");
        assert_eq!(a.forms.first(), Some(&Some("~~а".to_string())));
        // A prefix.
        let a = analyze("sagen", &["gesagt", "sagt"]);
        assert_eq!(a.lemma, "~en");
        assert_eq!(a.forms, vec![Some("ge~t".into()), Some("~t".into())]);
    }
}
