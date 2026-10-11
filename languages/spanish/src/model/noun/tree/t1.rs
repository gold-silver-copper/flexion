use flexion::Path;

/// Words ending -ter.
pub(crate) fn n1152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1109);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2358);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2359);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2360);
			match s {
				[] => true,
				['á', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2361);
			s.is_empty()
		}
		['h', 'g', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 2362);
			s.is_empty()
		}
		['m', 'o', 'r', 'p', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 2363);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2364);
			match s {
				[] => true,
				['e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				['u', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['í', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2365);
			s.is_empty()
		}
		['p', 'm', 'o', 'r', 'p', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 2366);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['á', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				['ó', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2367);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				['d', 'a', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				['m', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3139);
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3437);
							s.is_empty()
						}
						['í', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				['á', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				['í', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				['ú', s @ ..] => {
					super::push(out, 3144);
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2368);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2369);
			s.is_empty()
		}
		['á', s @ ..] => {
			super::push(out, 2370);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 2371);
			s.is_empty()
		}
		['í', 'j', '\u{1}', s @ ..] => {
			super::push(out, 2372);
			s.is_empty()
		}
		['ú', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2373);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uer.
pub(crate) fn n1153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', 'l', 'g', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['o', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3147);
					match s {
						[] => true,
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3440);
							s.is_empty()
						}
						['ú', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3441);
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

/// Words ending -ver.
pub(crate) fn n1154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1111);
	match s {
		[] => true,
		['l', 'ó', 'v', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2374);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2375);
			s.is_empty()
		}
		['r', 'a', 'p', 's', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2376);
			s.is_empty()
		}
		['á', 'd', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2377);
			s.is_empty()
		}
		['ó', 'l', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xer.
pub(crate) fn n1155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1112);
	match s {
		[] => true,
		['i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2379);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yer.
pub(crate) fn n1156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1113);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 2381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zer.
pub(crate) fn n1157(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2382);
			s.is_empty()
		}
		['o', 'd', 'l', 'l', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2383);
			s.is_empty()
		}
		['u', 'a', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2384);
			s.is_empty()
		}
		_ => false,
	}
}

