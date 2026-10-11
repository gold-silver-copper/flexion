use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ι', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', 'σ', 'σ', 'ε', 'τ', 'α', s @ ..] => {
									super::push(out, 8);
									s.is_empty()
								}
								['υ', 'σ', 'ι', 'π', '\u{1}', s @ ..] => {
									super::push(out, 9);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['ε', 'ρ', s @ ..] => {
							super::push(out, 7);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

