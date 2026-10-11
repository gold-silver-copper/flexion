use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['s', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				['t', 's', 'e', 'u', 'q', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13);
							s.is_empty()
						}
						['u', 'q', 'a', '\u{1}', s @ ..] => {
							super::push(out, 14);
							s.is_empty()
						}
						_ => false,
					}
				}
				['á', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 4);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['c', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['n', 'u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21);
									s.is_empty()
								}
								['g', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 22);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', 'u', 'v', '\u{1}', s @ ..] => {
							super::push(out, 17);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 23);
									s.is_empty()
								}
								['á', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 24);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'u', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 20);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6);
			s.is_empty()
		}
		['x', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 7);
			s.is_empty()
		}
		_ => false,
	}
}

