use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['क', 'त', '\u{1}', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['त', 'य', '\u{94d}', 'न', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 5);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['त', '\u{1}', s @ ..] => {
							super::push(out, 7);
							s.is_empty()
						}
						['र', 'स', '\u{1}', s @ ..] => {
							super::push(out, 8);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['द', 'त', '\u{1}', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

