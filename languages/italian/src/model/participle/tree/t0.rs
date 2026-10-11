use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['o', 't', 'a', 'd', 'i', 'f', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['c', 'i', 'd', 'n', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

