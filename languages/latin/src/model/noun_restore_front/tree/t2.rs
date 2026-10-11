use flexion::Path;

/// Words ending -vnoc.
pub(crate) fn n2043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1994);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4238);
			match s {
				[] => true,
				['l', 'e', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4239);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4240);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6151);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 5665);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word sutylesorp.
pub(crate) fn n6561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6276);
	s.is_empty()
}

/// The word murtemisorp.
pub(crate) fn n5271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5071);
	s.is_empty()
}

/// Words ending -osorp.
pub(crate) fn n5272(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5963);
			s.is_empty()
		}
		['d', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5964);
			s.is_empty()
		}
		['p', 'o', 'g', 'r', 'a', 'p', 'h', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5965);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word satirepsorp.
pub(crate) fn n5273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5072);
	s.is_empty()
}

/// Words ending -tsorp.
pub(crate) fn n5274(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5966);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5967);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -torp.
pub(crate) fn n3318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3210);
	match s {
		[] => true,
		['a', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 5073);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5968);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6277);
									s.is_empty()
								}
								['r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 6278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'e', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 5074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vorp.
pub(crate) fn n3319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3211);
	s.is_empty()
}

/// Words ending -xorp.
pub(crate) fn n3320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3212);
	s.is_empty()
}

