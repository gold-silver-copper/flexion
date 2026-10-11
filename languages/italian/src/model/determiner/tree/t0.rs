use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['i', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['c', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 9);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 10);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

