use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['k', '\u{11}', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['e', '\u{13}', 'i', 'w', 'l', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 20);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							super::push(out, 21);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 6);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 15);
					match s {
						[] => true,
						['\u{12}', 'o', '\u{1}', s @ ..] => {
							super::push(out, 22);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 8);
			s.is_empty()
		}
		['ś', s @ ..] => {
			super::push(out, 9);
			s.is_empty()
		}
		_ => false,
	}
}

