use flexion::Path;

/// Words ending -ntus.
pub(crate) fn n1577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1557);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2618);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['g', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3339);
							s.is_empty()
						}
						['u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -motus.
pub(crate) fn n2661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2622);
	s.is_empty()
}

/// The word potus.
pub(crate) fn n2662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2623);
	s.is_empty()
}

/// Words ending -ptus.
pub(crate) fn n1579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1559);
	match s {
		[] => true,
		['i', 'r', 'c', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2624);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtus.
pub(crate) fn n1580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1560);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2626);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2627);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2628);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2629);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2630);
			s.is_empty()
		}
		['y', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stus.
pub(crate) fn n1581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1561);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2633);
			match s {
				[] => true,
				['a', 'p', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3177);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				['l', 'e', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 3179);
					s.is_empty()
				}
				['p', 'm', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 3180);
					s.is_empty()
				}
				['s', 'p', 'm', 'i', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2634);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 2635);
			s.is_empty()
		}
		['u', 'g', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 2636);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2637);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttus.
pub(crate) fn n1582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1562);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2638);
			s.is_empty()
		}
		['e', 'l', 'l', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2639);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2640);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utus.
pub(crate) fn n1583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1563);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2642);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 2643);
			s.is_empty()
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 2644);
			s.is_empty()
		}
		['h', 'a', 'm', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2645);
			s.is_empty()
		}
		['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2646);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2647);
			s.is_empty()
		}
		['u', 'q', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 2648);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -extus.
pub(crate) fn n1584(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['s', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2649);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytus.
pub(crate) fn n1585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1564);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2651);
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2652);
			s.is_empty()
		}
		_ => false,
	}
}

