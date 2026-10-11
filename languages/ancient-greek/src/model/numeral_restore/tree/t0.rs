use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ι', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['ο', 'ι', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6);
							match s {
								[] => true,
								['ι', 'χ', 'σ', s @ ..] => {
									super::push(out, 9);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 7);
							match s {
								[] => true,
								['ο', 'κ', 'α', 'ι', s @ ..] => {
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
						['ρ', 'α', 'σ', 'σ', 'ε', 'τ', 'α', s @ ..] => {
							super::push(out, 8);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

