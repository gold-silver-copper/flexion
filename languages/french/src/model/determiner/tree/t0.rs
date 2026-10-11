use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 20);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 21);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 7);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8);
					match s {
						[] => true,
						['n', 'i', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 22);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['l', 'e', 'u', 'q', 'é', '\u{1}', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['i', 'd', 'n', s @ ..] => {
							super::push(out, 23);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14);
					match s {
						[] => true,
						['t', 'i', 'd', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 26);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 27);
									s.is_empty()
								}
								['r', 'u', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 28);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 15);
					match s {
						[] => true,
						['u', '-', 't', 'e', s @ ..] => {
							super::push(out, 24);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 16);
					match s {
						[] => true,
						['e', 't', 'i', 'd', 's', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 25);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

