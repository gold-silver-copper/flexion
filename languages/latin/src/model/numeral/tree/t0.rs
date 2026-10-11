use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', 'l', 'l', 'u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 11);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8);
							match s {
								[] => true,
								['o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 9);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 10);
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
		_ => false,
	}
}

