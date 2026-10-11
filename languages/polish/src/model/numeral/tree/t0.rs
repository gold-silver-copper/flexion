use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['i', 'c', 'ś', 'e', 'i', 'w', 'd', '\u{1}', s @ ..] => {
							super::push(out, 14);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['e', 'i', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 15);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'd', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 7);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				['y', 'r', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 16);
							s.is_empty()
						}
						['ó', 't', 'w', '\u{1}', s @ ..] => {
							super::push(out, 17);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ć', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11);
					match s {
						[] => true,
						['i', 'c', 'ś', 'a', 'n', 'a', 'w', 'd', '\u{1}', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['e', 's', 'm', 'e', 'i', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

