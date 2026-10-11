use flexion::Path;

/// Words ending -xis (3 syllables).
pub(crate) fn n3246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3026);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 6060);
			match s {
				[] => true,
				['a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 8521);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word congius (2 syllables).
pub(crate) fn n6730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6240);
	s.is_empty()
}

/// The word trollius (2 syllables).
pub(crate) fn n6731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6241);
	s.is_empty()
}

/// The word bermius (2 syllables).
pub(crate) fn n6732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6242);
	s.is_empty()
}

/// Words ending -nius (2 syllables).
pub(crate) fn n6733(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 10133);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 10134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 8616);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word celsius (2 syllables).
pub(crate) fn n6734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6243);
	s.is_empty()
}

/// Words ending -kus (2 syllables).
pub(crate) fn n3421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3192);
	match s {
		[] => true,
		['n', 'u', 'w', '\u{1}', s @ ..] => {
			super::push(out, 6244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lus (2 syllables).
pub(crate) fn n3422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3193);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6245);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 8617);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 8618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6246);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8619);
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8620);
					s.is_empty()
				}
				['y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6247);
			match s {
				[] => true,
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6248);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 10135);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 10136);
							s.is_empty()
						}
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 10137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 6249);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6250);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mus (2 syllables).
pub(crate) fn n3423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3194);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6252);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 8624);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 8625);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 8626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 6253);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6254);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 8627);
					s.is_empty()
				}
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6255);
			s.is_empty()
		}
		['s', 'a', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6256);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nus (2 syllables).
pub(crate) fn n3424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3195);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6257);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 8629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 8630);
					s.is_empty()
				}
				['o', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6258);
			s.is_empty()
		}
		['l', 'u', 'v', '\u{1}', s @ ..] => {
			super::push(out, 6259);
			s.is_empty()
		}
		['n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6260);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6261);
			s.is_empty()
		}
		['r', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 6262);
			s.is_empty()
		}
		['u', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 6263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ous (2 syllables).
pub(crate) fn n3425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3196);
	s.is_empty()
}

/// Words ending -pus (2 syllables).
pub(crate) fn n3426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3197);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8632);
					s.is_empty()
				}
				['i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 6264);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6265);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6266);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6267);
			match s {
				[] => true,
				['e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8635);
					s.is_empty()
				}
				['i', 'e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6268);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8637);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8638);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rus (2 syllables).
pub(crate) fn n3427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3198);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6269);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 8639);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 8640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 6270);
			s.is_empty()
		}
		['o', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6271);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8641);
					s.is_empty()
				}
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8642);
					s.is_empty()
				}
				['u', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8644);
					s.is_empty()
				}
				['s', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10138);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 10139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', 't', '\u{1}', s @ ..] => {
					super::push(out, 8645);
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 8646);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8647);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sus (2 syllables).
pub(crate) fn n3428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3199);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6272);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6273);
			s.is_empty()
		}
		['i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 6274);
			s.is_empty()
		}
		['p', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 6275);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 't', '\u{1}', s @ ..] => {
					super::push(out, 8648);
					s.is_empty()
				}
				['y', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 8649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'y', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 8650);
					s.is_empty()
				}
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8651);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tus (2 syllables).
pub(crate) fn n3429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3200);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6276);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 10140);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 10141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8652);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 8654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6277);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10143);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 10144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8655);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 10145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8656);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10146);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 10147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8657);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 10148);
							s.is_empty()
						}
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 10149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 6278);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6279);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 8658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6280);
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8659);
					s.is_empty()
				}
				['e', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8660);
					s.is_empty()
				}
				['y', 'x', '\u{1}', s @ ..] => {
					super::push(out, 8661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word lituus (2 syllables).
pub(crate) fn n3430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3201);
	s.is_empty()
}

/// Words ending -vus (2 syllables).
pub(crate) fn n3431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3202);
	match s {
		[] => true,
		['a', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6281);
			s.is_empty()
		}
		['r', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xus (2 syllables).
pub(crate) fn n3432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3203);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6283);
			match s {
				[] => true,
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hus (3 syllables).
pub(crate) fn n3438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3209);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6305);
			match s {
				[] => true,
				['i', 'd', 'd', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8691);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10161);
							s.is_empty()
						}
						['y', 'h', 'r', 'o', 'h', 't', 'i', 'n', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 's', 'w', 'e', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8692);
					s.is_empty()
				}
				['y', 'n', 'o', 'n', 'i', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'e', 'g', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8694);
					s.is_empty()
				}
				['m', 'o', 'g', 'o', 'r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 8695);
					s.is_empty()
				}
				['o', 'r', 't', 's', 'o', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8696);
					s.is_empty()
				}
				['r', 'o', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6306);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10163);
							s.is_empty()
						}
						['n', 'g', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'o', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8698);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8699);
					match s {
						[] => true,
						['a', 'c', 'i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 10166);
							s.is_empty()
						}
						['k', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 10167);
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

/// Words ending -ius (3 syllables).
pub(crate) fn n3439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3210);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6307);
			match s {
				[] => true,
				['e', 's', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8700);
					s.is_empty()
				}
				['o', 'c', 'a', 'n', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6308);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6309);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8702);
					match s {
						[] => true,
						['b', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10168);
							s.is_empty()
						}
						['f', 'o', 't', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8703);
					s.is_empty()
				}
				['y', 't', 's', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 8704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6310);
			match s {
				[] => true,
				['a', 'r', 'c', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8705);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8706);
					s.is_empty()
				}
				['o', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6311);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'e', 'c', 'e', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10170);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10171);
							match s {
								[] => true,
								['t', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 10857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 10172);
							s.is_empty()
						}
						['r', 't', 's', 'i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10173);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 10858);
									s.is_empty()
								}
								['t', 'i', 'g', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 10859);
									s.is_empty()
								}
								['x', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 10860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'q', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10174);
							s.is_empty()
						}
						['v', 'i', 'd', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 10175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6312);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8708);
					match s {
						[] => true,
						['g', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6313);
			match s {
				[] => true,
				['a', 'l', 'u', 'k', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 8709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'u', 's', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 6314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lus (3 syllables).
pub(crate) fn n3441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3212);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', 'e', 'a', 'd', 'o', 'g', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8710);
					s.is_empty()
				}
				['h', 'p', 'e', 'c', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10177);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 8711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'n', 'i', 'c', 'o', 't', 'o', '\u{1}', s @ ..] => {
			super::push(out, 6315);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6316);
			match s {
				[] => true,
				['b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 8712);
					s.is_empty()
				}
				['g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8714);
					s.is_empty()
				}
				['p', 's', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8715);
					s.is_empty()
				}
				['t', 'u', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 8716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6317);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8717);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10861);
									match s {
										[] => true,
										['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11221);
											s.is_empty()
										}
										['i', 's', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 11222);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 11223);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'c', s @ ..] => {
					super::push(out, 8718);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', 'a', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8720);
					s.is_empty()
				}
				['r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6318);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8722);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8723);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10179);
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10863);
									s.is_empty()
								}
								['v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10864);
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
		['t', 'a', 'o', 'c', 'l', 'a', 'z', 't', 'e', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 6319);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6320);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10180);
							s.is_empty()
						}
						['n', 'u', 'n', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 10182);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 10183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8724);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10865);
									s.is_empty()
								}
								['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 8725);
					match s {
						[] => true,
						['l', 'o', 'v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10184);
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

/// Words ending -mus (3 syllables).
pub(crate) fn n3442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3213);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8726);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8727);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10867);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 10868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'o', 'p', 'o', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6321);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', 'o', 'b', 'r', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8729);
					s.is_empty()
				}
				['y', 'r', 'o', 'b', 'r', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8731);
					s.is_empty()
				}
				['l', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8732);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8733);
					match s {
						[] => true,
						['i', 'l', 'l', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10185);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 10869);
									match s {
										[] => true,
										['c', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11224);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8734);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8735);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 10186);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 10871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 10187);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10872);
									match s {
										[] => true,
										['l', 'a', 'r', 'e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 11225);
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
		['o', s @ ..] => {
			super::push(out, 6322);
			match s {
				[] => true,
				['r', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8737);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', 'o', 'l', 'l', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 10188);
							s.is_empty()
						}
						['r', 'o', 'h', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8738);
					s.is_empty()
				}
				['l', 'g', 'n', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nus (3 syllables).
pub(crate) fn n3443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3214);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6323);
			match s {
				[] => true,
				['d', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8740);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8741);
					s.is_empty()
				}
				['r', 'o', 'n', 'g', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8742);
					s.is_empty()
				}
				['t', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6324);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8744);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8745);
					s.is_empty()
				}
				['n', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8746);
					s.is_empty()
				}
				['x', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'a', 'e', 'a', 'l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 6325);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6326);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10191);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8748);
					match s {
						[] => true,
						['c', 'e', 't', s @ ..] => {
							super::push(out, 10193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8749);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8750);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10194);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 10195);
							s.is_empty()
						}
						['u', 'r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 10196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'l', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 'c', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8752);
					s.is_empty()
				}
				['i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 't', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6327);
			s.is_empty()
		}
		['r', 'e', 't', 'a', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6328);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vous (3 syllables).
pub(crate) fn n6848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6331);
	match s {
		[] => true,
		['e', 'd', 'n', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 8756);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 8757);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pus (3 syllables).
pub(crate) fn n3445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3216);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 6332);
			s.is_empty()
		}
		['m', 'a', 'c', 'o', 'p', 'p', 'i', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8758);
					s.is_empty()
				}
				['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6333);
			match s {
				[] => true,
				['c', 's', 'i', 'p', 'e', 'r', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8760);
					s.is_empty()
				}
				['g', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'i', s @ ..] => {
			match s {
				[] => true,
				['h', 'o', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8762);
					s.is_empty()
				}
				['r', 'e', 't', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6334);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10198);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 10874);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11226);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 10199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8764);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6335);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rus (3 syllables).
pub(crate) fn n3446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3217);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6336);
			match s {
				[] => true,
				['b', 'r', 'a', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8765);
					s.is_empty()
				}
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8766);
					s.is_empty()
				}
				['h', 't', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8767);
					s.is_empty()
				}
				['l', 'y', 't', '\u{1}', s @ ..] => {
					super::push(out, 8768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6337);
			match s {
				[] => true,
				['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8769);
					s.is_empty()
				}
				['l', 'c', 'u', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 8770);
					s.is_empty()
				}
				['m', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8771);
					s.is_empty()
				}
				['t', 'u', '\u{1}', s @ ..] => {
					super::push(out, 8772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'a', 'g', 'e', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6338);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6339);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 8773);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10201);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 10875);
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
		['o', s @ ..] => {
			super::push(out, 6340);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10202);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 10876);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['l', 'l', 'a', 'h', 'p', '\u{1}', s @ ..] => {
											super::push(out, 11227);
											s.is_empty()
										}
										['o', 'z', '\u{1}', s @ ..] => {
											super::push(out, 11228);
											s.is_empty()
										}
										['t', 's', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 11229);
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
				['k', 'i', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8774);
					s.is_empty()
				}
				['l', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8775);
					s.is_empty()
				}
				['p', 'l', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8776);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8777);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8778);
					match s {
						[] => true,
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 10204);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6341);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8779);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10206);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10877);
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
		['t', 's', 'e', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6342);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6343);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10878);
									s.is_empty()
								}
								['a', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', 'a', 'l', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10880);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11230);
											s.is_empty()
										}
										['y', 'k', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11231);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 11232);
											s.is_empty()
										}
										['n', 'a', 'r', 'y', 't', '\u{1}', s @ ..] => {
											super::push(out, 11233);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 'h', 't', 'h', 'c', 'i', '\u{1}', s @ ..] => {
									super::push(out, 10881);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 8780);
					s.is_empty()
				}
				['n', 'e', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 6344);
			match s {
				[] => true,
				['g', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8782);
					s.is_empty()
				}
				['p', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sus (3 syllables).
pub(crate) fn n3447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3218);
	match s {
		[] => true,
		['a', 'g', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6345);
			s.is_empty()
		}
		['e', 'n', 'o', 's', 'r', 'e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6346);
			s.is_empty()
		}
		['i', 't', 'y', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6347);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6348);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6349);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6350);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8785);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8786);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10207);
							match s {
								[] => true,
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10882);
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10883);
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
		['s', s @ ..] => {
			super::push(out, 6351);
			match s {
				[] => true,
				['a', 'n', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8787);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8788);
					match s {
						[] => true,
						['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10208);
							s.is_empty()
						}
						['d', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 'r', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 8789);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8790);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 10884);
									s.is_empty()
								}
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10885);
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

/// Words ending -tus (3 syllables).
pub(crate) fn n3448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3219);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6352);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8791);
					match s {
						[] => true,
						['f', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10210);
							s.is_empty()
						}
						['o', 'r', 't', 's', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10211);
							s.is_empty()
						}
						['u', 'd', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 10212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10213);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8792);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8793);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10215);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10886);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 11234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 10887);
									s.is_empty()
								}
								['p', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8794);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8795);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10219);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8796);
					match s {
						[] => true,
						['n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8797);
					s.is_empty()
				}
				['l', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8798);
					s.is_empty()
				}
				['o', 'f', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8799);
					s.is_empty()
				}
				['p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6353);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10222);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10223);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10889);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 11235);
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
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8801);
					s.is_empty()
				}
				['l', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8802);
					s.is_empty()
				}
				['m', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8803);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10224);
							s.is_empty()
						}
						['n', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 10225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8804);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 10226);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10227);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10890);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 'p', 'e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10229);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'g', 'n', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 6354);
			s.is_empty()
		}
		['n', 'a', 'l', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6355);
			s.is_empty()
		}
		['o', 'n', s @ ..] => {
			super::push(out, 6356);
			s.is_empty()
		}
		['p', 'e', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6357);
			s.is_empty()
		}
		['r', 'o', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6358);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8806);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 8807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6359);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xus (3 syllables).
pub(crate) fn n3450(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6360);
			s.is_empty()
		}
		['o', 'i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -plantula (3 syllables).
pub(crate) fn n7447(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9088);
			s.is_empty()
		}
		['u', 'e', '\u{1}', s @ ..] => {
			super::push(out, 9089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtula (3 syllables).
pub(crate) fn n7449(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9090);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9091);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 10355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stula (3 syllables).
pub(crate) fn n7450(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'l', 'b', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 9092);
			s.is_empty()
		}
		['i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9093);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9094);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word guttula (3 syllables).
pub(crate) fn n7451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6873);
	s.is_empty()
}

/// Words ending -vula (3 syllables).
pub(crate) fn n4178(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 6874);
			s.is_empty()
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 6875);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cidium (3 syllables).
pub(crate) fn n10463(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'c', 'o', 'r', 'a', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 10541);
			s.is_empty()
		}
		['n', 'o', '\u{1}', s @ ..] => {
			super::push(out, 10542);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word amidium (3 syllables).
pub(crate) fn n10464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9556);
	s.is_empty()
}

/// Words ending -nidium (3 syllables).
pub(crate) fn n10465(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10543);
			match s {
				[] => true,
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11009);
					s.is_empty()
				}
				['t', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11010);
					s.is_empty()
				}
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 11011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 10544);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10545);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 11012);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pidium (3 syllables).
pub(crate) fn n10466(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 10546);
			s.is_empty()
		}
		['s', 'a', '\u{1}', s @ ..] => {
			super::push(out, 10547);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ridium (3 syllables).
pub(crate) fn n10467(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 't', 'p', '\u{1}', s @ ..] => {
			super::push(out, 10548);
			s.is_empty()
		}
		['h', 'p', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 10549);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10550);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11013);
					s.is_empty()
				}
				['m', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 11014);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -asidium (3 syllables).
pub(crate) fn n10468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9557);
	s.is_empty()
}

/// Words ending -tidium (3 syllables).
pub(crate) fn n10469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9558);
	match s {
		[] => true,
		['l', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 10551);
			s.is_empty()
		}
		['s', 'y', 'c', 'o', 'e', 'l', 'g', '\u{1}', s @ ..] => {
			super::push(out, 10552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ndium (3 syllables).
pub(crate) fn n8424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7740);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9559);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11015);
							s.is_empty()
						}
						['i', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11267);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', 'h', 'c', 's', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 11268);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11269);
									s.is_empty()
								}
								['o', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 11270);
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
		['u', s @ ..] => {
			super::push(out, 9560);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odium (3 syllables).
pub(crate) fn n8425(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', 'a', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 9561);
			s.is_empty()
		}
		['g', 'y', 'l', '\u{1}', s @ ..] => {
			super::push(out, 9562);
			s.is_empty()
		}
		['h', 'r', 'l', 'y', 'l', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 9563);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 10553);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10554);
					s.is_empty()
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 's', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'p', 'o', s @ ..] => {
					super::push(out, 10556);
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 10557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10558);
					s.is_empty()
				}
				['m', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10559);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10560);
					match s {
						[] => true,
						['c', 'y', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11016);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11017);
							match s {
								[] => true,
								['e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11271);
									s.is_empty()
								}
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 11272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 10561);
					match s {
						[] => true,
						['h', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11019);
							s.is_empty()
						}
						['l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 9564);
			s.is_empty()
		}
		['x', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 9565);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rdium (3 syllables).
pub(crate) fn n8426(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'c', s @ ..] => {
			match s {
				[] => true,
				['i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 10562);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 11021);
							s.is_empty()
						}
						['r', 't', 'c', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 11022);
							s.is_empty()
						}
						['y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 9566);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word preludium (3 syllables).
pub(crate) fn n8427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7741);
	s.is_empty()
}

/// Words ending -gium (3 syllables).
pub(crate) fn n5589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5191);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7742);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7743);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10563);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7744);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 11024);
									s.is_empty()
								}
								['r', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'r', 'e', 't', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9568);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10566);
							s.is_empty()
						}
						['r', 'e', 't', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 10567);
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

/// Words ending -hium (3 syllables).
pub(crate) fn n5590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5192);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7745);
			match s {
				[] => true,
				['n', 'i', 'r', 'y', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 9569);
					s.is_empty()
				}
				['y', 'd', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7746);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9571);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10568);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 11026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lium (3 syllables).
pub(crate) fn n5591(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', 'p', 'a', 'n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9573);
					s.is_empty()
				}
				['n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9574);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7747);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10569);
							match s {
								[] => true,
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 11027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10570);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11273);
											s.is_empty()
										}
										['o', 's', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 11274);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 11028);
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
		['i', s @ ..] => {
			super::push(out, 7748);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10571);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9577);
					match s {
						[] => true,
						['p', 'o', 'd', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 'i', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9578);
					s.is_empty()
				}
				['y', 'r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7749);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9580);
					s.is_empty()
				}
				['f', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9581);
					s.is_empty()
				}
				['p', 'o', 'm', 'r', 'e', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 9582);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9583);
					s.is_empty()
				}
				['s', 'o', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9584);
					s.is_empty()
				}
				['t', 'i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9585);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 9586);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7750);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9587);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mium (3 syllables).
pub(crate) fn n5592(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'l', 'a', 'h', 't', s @ ..] => {
			match s {
				[] => true,
				['i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9589);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', 'c', 'l', 'y', 'h', 't', 'e', 'm', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 7751);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7752);
			match s {
				[] => true,
				['d', 'u', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 9591);
					s.is_empty()
				}
				['r', 'p', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9592);
					s.is_empty()
				}
				['s', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9594);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9595);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7753);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9596);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10576);
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

/// Words ending -nium (3 syllables).
pub(crate) fn n5593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5193);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7754);
			match s {
				[] => true,
				['l', 'u', 'c', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9597);
					s.is_empty()
				}
				['p', 'e', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9598);
					s.is_empty()
				}
				['r', 'c', s @ ..] => {
					super::push(out, 9599);
					match s {
						[] => true,
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10577);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', 's', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 11029);
									s.is_empty()
								}
								['n', 'h', 'c', 'n', 'a', 'l', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 11030);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11031);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11032);
									match s {
										[] => true,
										['c', 'e', '\u{1}', s @ ..] => {
											super::push(out, 11275);
											s.is_empty()
										}
										['n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 11276);
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											super::push(out, 11277);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9600);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 9601);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10579);
							s.is_empty()
						}
						['s', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', s @ ..] => {
					super::push(out, 9602);
					match s {
						[] => true,
						['e', 'h', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10581);
							s.is_empty()
						}
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9603);
					match s {
						[] => true,
						['y', 'h', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11033);
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 11034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 's', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 7755);
			s.is_empty()
		}
		['h', 'c', 'i', 't', 'r', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 7756);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7757);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9605);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10583);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11035);
									match s {
										[] => true,
										['r', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 11278);
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
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 10584);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10586);
							match s {
								[] => true,
								['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11036);
									s.is_empty()
								}
								['o', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 11037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10587);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'l', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9606);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 9607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7758);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', 'i', s @ ..] => {
							match s {
								[] => true,
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 11038);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 11039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'i', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11040);
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 11279);
											s.is_empty()
										}
										['g', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11280);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 11281);
											s.is_empty()
										}
										['c', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 11282);
											s.is_empty()
										}
										['t', 'n', 'e', 'c', '\u{1}', s @ ..] => {
											super::push(out, 11283);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 'i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10589);
							s.is_empty()
						}
						['t', 'n', 'e', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11041);
									s.is_empty()
								}
								['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 11042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10590);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9608);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10592);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 11043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 10593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9609);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 9610);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9611);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10594);
							s.is_empty()
						}
						['r', 'a', 'l', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'p', s @ ..] => super::t4::n10536(s, out),
				['i', 'l', 'e', 'h', '\u{1}', s @ ..] => super::t4::n10537(s, out),
				['k', 'r', s @ ..] => super::t4::n10538(s, out),
				['l', s @ ..] => super::t4::n10539(s, out),
				['m', s @ ..] => super::t4::n10540(s, out),
				['r', s @ ..] => super::t4::n10541(s, out),
				['s', s @ ..] => super::t4::n10542(s, out),
				['x', s @ ..] => super::t4::n10543(s, out),
				['z', s @ ..] => super::t4::n10544(s, out),
				_ => false,
			}
		}
		['r', 'e', s @ ..] => super::t4::n8452(s, out),
		['u', 'l', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => super::t4::n8453(s, out),
		['y', 'g', s @ ..] => super::t4::n8454(s, out),
		_ => false,
	}
}

/// Words ending -pium (3 syllables).
pub(crate) fn n5594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5194);
	match s {
		[] => true,
		['l', 'a', 'c', 's', 'i', 'r', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7761);
			s.is_empty()
		}
		['o', 'r', 't', 'c', 'e', '\u{1}', s @ ..] => {
			super::push(out, 7762);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7763);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 9623);
					s.is_empty()
				}
				['s', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9624);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rium (3 syllables).
pub(crate) fn n5595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5195);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7764);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'l', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 10603);
							s.is_empty()
						}
						['m', 'u', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10604);
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 10606);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 11051);
									s.is_empty()
								}
								['l', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11052);
									s.is_empty()
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 11284);
											s.is_empty()
										}
										['e', 't', '\u{1}', s @ ..] => {
											super::push(out, 11285);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'p', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9625);
					s.is_empty()
				}
				['i', 'd', 'i', 'h', 'p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 9626);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'n', 'a', 'v', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10608);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 11053);
									s.is_empty()
								}
								['t', 'p', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 11054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 't', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 10609);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 10610);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10612);
							match s {
								[] => true,
								['s', 'l', s @ ..] => {
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 11286);
											s.is_empty()
										}
										['y', 'l', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11287);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10614);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 11055);
									s.is_empty()
								}
								['e', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 11056);
									s.is_empty()
								}
								['h', 'p', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 11057);
									s.is_empty()
								}
								['l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11058);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 11059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 11060);
									s.is_empty()
								}
								['h', 'p', 'l', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 11061);
									s.is_empty()
								}
								['m', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11062);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11288);
											s.is_empty()
										}
										['u', '\u{1}', s @ ..] => {
											super::push(out, 11289);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'g', 'n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10615);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9627);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 10617);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11064);
									s.is_empty()
								}
								['t', 'p', 'o', 'd', 'i', 'p', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10618);
							s.is_empty()
						}
						['r', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11066);
									s.is_empty()
								}
								['a', 'u', 'q', 'a', '\u{1}', s @ ..] => {
									super::push(out, 11067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'o', 'n', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11068);
									s.is_empty()
								}
								['p', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', 'c', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10620);
							s.is_empty()
						}
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 10621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', 'a', 's', '\u{1}', s @ ..] => super::t4::n11689(s, out),
						['e', s @ ..] => super::t4::n11690(s, out),
						['i', s @ ..] => super::t4::n11691(s, out),
						['n', 'e', s @ ..] => super::t4::n11692(s, out),
						['p', 'e', 'r', '\u{1}', s @ ..] => super::t4::n11693(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t4::n10562(s, out),
				['v', s @ ..] => super::t4::n10563(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t4::n8460(s, out),
		['d', s @ ..] => super::t4::n8461(s, out),
		['e', s @ ..] => super::t4::n8462(s, out),
		['i', 'l', 'e', 'd', '\u{1}', s @ ..] => super::t4::n8463(s, out),
		['o', s @ ..] => super::t4::n8464(s, out),
		['t', s @ ..] => super::t4::n8465(s, out),
		['u', s @ ..] => super::t4::n8466(s, out),
		['y', 't', 'r', 'a', 'm', '\u{1}', s @ ..] => super::t4::n8467(s, out),
		_ => false,
	}
}

/// Words ending -sium (3 syllables).
pub(crate) fn n5596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5196);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'm', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 9639);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9640);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9641);
					match s {
						[] => true,
						['m', 'y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7772);
			s.is_empty()
		}
		['n', 'e', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7773);
			s.is_empty()
		}
		['o', 'p', 'm', 'y', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9642);
					s.is_empty()
				}
				['i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9643);
					s.is_empty()
				}
				['r', 'e', 'b', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'a', 't', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7774);
			s.is_empty()
		}
		['y', 'm', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7775);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tium (3 syllables).
pub(crate) fn n5597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5197);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', 'r', 'h', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9645);
					s.is_empty()
				}
				['m', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9646);
					s.is_empty()
				}
				['r', 'c', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7776);
			match s {
				[] => true,
				['p', 's', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'o', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9649);
					s.is_empty()
				}
				['r', 't', 's', 'o', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 't', 's', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 7777);
			s.is_empty()
		}
		['s', 'a', 'r', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 7778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -quium (3 syllables).
pub(crate) fn n5598(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 7779);
			s.is_empty()
		}
		['o', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 7780);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vium (3 syllables).
pub(crate) fn n5599(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 9651);
					s.is_empty()
				}
				['x', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 9652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10648);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 10649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9653);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 10650);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 10651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 9654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zium (3 syllables).
pub(crate) fn n5600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5198);
	match s {
		[] => true,
		['e', 'p', 'a', 'r', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9655);
					s.is_empty()
				}
				['o', 'h', 'p', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 9656);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word haemoprotozoan (3 syllables).
pub(crate) fn n12551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11378);
	s.is_empty()
}

/// The word hemoprotozoan (3 syllables).
pub(crate) fn n12552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11379);
	s.is_empty()
}

/// The word menaulion (3 syllables).
pub(crate) fn n10858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9892);
	s.is_empty()
}

/// The word koukoulion (3 syllables).
pub(crate) fn n10859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9893);
	s.is_empty()
}

/// Words ending -mion (3 syllables).
pub(crate) fn n5903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5463);
	match s {
		[] => true,
		['h', 't', 's', 'i', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8095);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8096);
			match s {
				[] => true,
				['r', 'c', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nion (3 syllables).
pub(crate) fn n5904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5464);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8098);
			match s {
				[] => true,
				['r', 'u', s @ ..] => {
					super::push(out, 9895);
					match s {
						[] => true,
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10736);
							s.is_empty()
						}
						['o', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8099);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8100);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9896);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 's', 'u', 'e', 'l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 8101);
			s.is_empty()
		}
		['m', 'i', 'l', 'o', s @ ..] => {
			super::push(out, 8102);
			match s {
				[] => true,
				['n', 'i', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9897);
					s.is_empty()
				}
				['p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['i', 'k', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 9899);
					s.is_empty()
				}
				['m', 'i', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9900);
					s.is_empty()
				}
				['t', 'u', 'o', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8103);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pion (3 syllables).
pub(crate) fn n5905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5465);
	match s {
		[] => true,
		['l', 'o', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9902);
					s.is_empty()
				}
				['k', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', s @ ..] => {
			super::push(out, 8105);
			match s {
				[] => true,
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9904);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rion (3 syllables).
pub(crate) fn n5906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5466);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8106);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 10739);
							s.is_empty()
						}
						['p', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9905);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9906);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 10741);
							s.is_empty()
						}
						['p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9907);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9908);
					s.is_empty()
				}
				['v', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8107);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', 'o', 'l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 10743);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10744);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 11134);
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
		['e', s @ ..] => {
			super::push(out, 8108);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['p', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 10745);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 11135);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 11136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 10746);
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9910);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10748);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10749);
							s.is_empty()
						}
						['n', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10750);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 11138);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 11140);
									s.is_empty()
								}
								['i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10751);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11142);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 11317);
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
		['i', s @ ..] => {
			super::push(out, 8109);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9911);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10752);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 11143);
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 11144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11145);
									s.is_empty()
								}
								['t', 'y', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'o', 't', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 't', 'r', 'a', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10754);
							s.is_empty()
						}
						['e', 't', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8110);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8111);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8112);
			s.is_empty()
		}
		['y', 't', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 8113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sion (3 syllables).
pub(crate) fn n5907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5467);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8114);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9913);
					match s {
						[] => true,
						['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 10756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'm', 'y', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9914);
					s.is_empty()
				}
				['t', 's', 'o', 'n', 'o', 'c', 'i', '\u{1}', s @ ..] => {
					super::push(out, 9915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', 'o', 'm', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 8115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tion (3 syllables).
pub(crate) fn n5908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5468);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8116);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9916);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10757);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 11147);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11318);
											match s {
												[] => true,
												['y', s @ ..] => {
													super::push(out, 11380);
													match s {
														[] => true,
														['s', 's', 'u', 'p', '\u{1}', s @ ..] => super::t4::n12569(s, out),
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
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9917);
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8117);
			match s {
				[] => true,
				['s', 'o', 'p', 'i', s @ ..] => {
					super::push(out, 9918);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 10759);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 11148);
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

/// Words ending -ahedron (3 syllables).
pub(crate) fn n11861(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', 'e', 'd', s @ ..] => {
			match s {
				[] => true,
				['i', 'a', 'k', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 11319);
					s.is_empty()
				}
				['n', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 11320);
					s.is_empty()
				}
				['o', 'd', s @ ..] => {
					super::push(out, 11321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11381);
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'e', 'd', 'o', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11404);
											s.is_empty()
										}
										['i', 's', 'o', 'c', 'i', '\u{1}', s @ ..] => {
											super::push(out, 11405);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'i', 'b', 'm', 'o', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 11382);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'm', 'o', 'h', 'r', '\u{1}', s @ ..] => {
									super::push(out, 11393);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 11394);
									s.is_empty()
								}
								['m', 'e', 'h', 'i', 's', 'o', 'c', 'i', '\u{1}', s @ ..] => {
									super::push(out, 11395);
									s.is_empty()
								}
								['s', 'o', 'c', 'i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11406);
											s.is_empty()
										}
										['b', 'm', 'o', 'h', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11411);
													s.is_empty()
												}
												['i', 'd', '\u{1}', s @ ..] => {
													super::push(out, 11412);
													s.is_empty()
												}
												_ => false,
											}
										}
										['c', 'e', 'd', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 11407);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'i', 'k', 'a', 'y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11383);
							s.is_empty()
						}
						['y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', 'y', 'm', '\u{1}', s @ ..] => {
			super::push(out, 11155);
			s.is_empty()
		}
		['k', 'e', 'd', 'i', 'a', 'k', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 11322);
					s.is_empty()
				}
				['s', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 11323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 't', 'e', 't', s @ ..] => {
			match s {
				[] => true,
				['i', 's', 'o', 'c', 'i', '\u{1}', s @ ..] => {
					super::push(out, 11324);
					s.is_empty()
				}
				['r', 'e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 11325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 11385);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11396);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 11408);
											s.is_empty()
										}
										['s', 'o', 'c', 'i', '\u{1}', s @ ..] => {
											super::push(out, 11409);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'b', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11397);
									s.is_empty()
								}
								['s', 'i', s @ ..] => {
									match s {
										[] => true,
										['k', 'a', s @ ..] => {
											match s {
												[] => true,
												['i', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 11413);
													s.is_empty()
												}
												['x', 'e', 'h', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 11414);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 't', '\u{1}', s @ ..] => {
											super::push(out, 11410);
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 11326);
					s.is_empty()
				}
				['n', 'o', 'c', 'a', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11327);
					s.is_empty()
				}
				['u', 'm', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 11386);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'm', 'o', 'h', 'r', '\u{1}', s @ ..] => {
									super::push(out, 11399);
									s.is_empty()
								}
								['s', 'o', 'c', 'i', '\u{1}', s @ ..] => {
									super::push(out, 11400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'k', 'a', s @ ..] => {
							match s {
								[] => true,
								['r', 't', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 11401);
									s.is_empty()
								}
								['t', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 11402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 11329);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ohedron (3 syllables).
pub(crate) fn n11863(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 't', 'r', 'o', '\u{1}', s @ ..] => {
			super::push(out, 11156);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11330);
					s.is_empty()
				}
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 11331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 11332);
					s.is_empty()
				}
				['o', 'g', 'a', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'i', 'e', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 11157);
			s.is_empty()
		}
		['t', 'i', 'r', 'y', 'p', '\u{1}', s @ ..] => {
			super::push(out, 11158);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word subpolyhedron (3 syllables).
pub(crate) fn n12314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11159);
	s.is_empty()
}

/// Words ending -opolyhedron (3 syllables).
pub(crate) fn n12315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11160);
	s.is_empty()
}

/// The word polyedron (3 syllables).
pub(crate) fn n10911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9933);
	s.is_empty()
}

/// Words ending -ndron (3 syllables).
pub(crate) fn n8914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8151);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9934);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dehydron (3 syllables).
pub(crate) fn n8915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8152);
	s.is_empty()
}

/// Words ending -eron (3 syllables).
pub(crate) fn n5938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5493);
	match s {
		[] => true,
		['f', 'i', 'n', 'i', 'm', 'a', 'r', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 8153);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 8154);
			match s {
				[] => true,
				['o', 'm', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8155);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 8156);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9936);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'x', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10767);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['a', 'x', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11161);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['c', 'y', 'n', '\u{1}', s @ ..] => {
											super::push(out, 11334);
											s.is_empty()
										}
										['h', 'c', 'y', 'n', '\u{1}', s @ ..] => {
											super::push(out, 11335);
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
				['i', 'p', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10768);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10769);
							s.is_empty()
						}
						['t', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8157);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8158);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10771);
							s.is_empty()
						}
						['s', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 10772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9938);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 10773);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 11162);
									s.is_empty()
								}
								['m', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11164);
									s.is_empty()
								}
								['d', 'i', 'p', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11165);
									s.is_empty()
								}
								['h', 't', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 11166);
									s.is_empty()
								}
								['r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 11167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9939);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gron (3 syllables).
pub(crate) fn n5939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5494);
	s.is_empty()
}

/// Words ending -hron (3 syllables).
pub(crate) fn n5940(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8159);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', 'a', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9940);
					s.is_empty()
				}
				['o', 'b', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iron (3 syllables).
pub(crate) fn n5941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5495);
	s.is_empty()
}

/// Words ending -oron (3 syllables).
pub(crate) fn n5942(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8160);
			s.is_empty()
		}
		['h', 'p', s @ ..] => {
			super::push(out, 8161);
			match s {
				[] => true,
				['a', 'i', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9942);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['o', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10774);
							s.is_empty()
						}
						['r', 'p', 'o', 'e', 'o', 'm', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 10775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'k', 'a', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rron (3 syllables).
pub(crate) fn n5943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5496);
	s.is_empty()
}

/// Words ending -tron (3 syllables).
pub(crate) fn n5944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5497);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8163);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'r', 'o', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 9943);
					s.is_empty()
				}
				['l', 'a', 'h', 'p', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9944);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 9945);
					match s {
						[] => true,
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', 'h', 'r', 'o', 'n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9946);
					s.is_empty()
				}
				['l', 'a', 't', 'n', 'o', 'c', 'e', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8164);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8165);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9948);
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', '\u{1}', s @ ..] => {
									super::push(out, 11168);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 11336);
											s.is_empty()
										}
										['s', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 11337);
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 11338);
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
		['y', s @ ..] => {
			super::push(out, 8166);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9949);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10777);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11169);
									s.is_empty()
								}
								['m', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 11170);
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

/// Words ending -uron (3 syllables).
pub(crate) fn n5945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5498);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9950);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 10778);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 10779);
							match s {
								[] => true,
								['o', 'x', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9951);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word hyperpyron (3 syllables).
pub(crate) fn n5946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5499);
	s.is_empty()
}

/// The word encephaloradiculitis (3 syllables).
pub(crate) fn n12565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11387);
	s.is_empty()
}

/// The word pyodermitis (3 syllables).
pub(crate) fn n11093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10095);
	s.is_empty()
}

/// Words ending -ymitis (3 syllables).
pub(crate) fn n11094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10096);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', 'd', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 11204);
					s.is_empty()
				}
				['n', 'e', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 11205);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nitis (3 syllables).
pub(crate) fn n9291(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10097);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11206);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11207);
							match s {
								[] => true,
								['i', 'g', 'n', 'a', 'h', 'p', 'm', 'y', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11353);
									s.is_empty()
								}
								['r', 'a', 'h', 'p', 'e', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 11354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'h', 'p', 'e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 10832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10098);
			match s {
				[] => true,
				['d', 'n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 10833);
					s.is_empty()
				}
				['g', 'a', 'v', 'o', 'v', 'l', 'u', 'v', '\u{1}', s @ ..] => {
					super::push(out, 10834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10099);
			match s {
				[] => true,
				['d', 'n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 10835);
					s.is_empty()
				}
				['i', 'r', 'o', 'h', 'c', 'b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 10836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ritis (3 syllables).
pub(crate) fn n9292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8506);
	match s {
		[] => true,
		['e', 't', 'n', 'e', 'o', 'r', 't', 's', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 10100);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['p', 'e', 'n', 'o', s @ ..] => {
					super::push(out, 10837);
					match s {
						[] => true,
						['l', 'u', s @ ..] => {
							super::push(out, 11209);
							match s {
								[] => true,
								['b', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 11355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', 'b', 'o', 't', 'n', 'o', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 11210);
							s.is_empty()
						}
						['r', 'a', 'd', 'n', 'o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 11211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10101);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10838);
					match s {
						[] => true,
						['m', 'o', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'o', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 10839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sitis (3 syllables).
pub(crate) fn n9293(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 10102);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10840);
					match s {
						[] => true,
						['b', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 11213);
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 11214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'm', 'y', 'l', 'o', 'p', 'o', 't', 'a', 'm', 'r', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 10841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10103);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 10842);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11356);
									s.is_empty()
								}
								['o', 'n', 'i', 'h', 'r', '\u{1}', s @ ..] => {
									super::push(out, 11357);
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

/// Words ending -titis (3 syllables).
pub(crate) fn n9294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8507);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'c', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10843);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 11215);
							s.is_empty()
						}
						['r', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11358);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 10844);
					s.is_empty()
				}
				['r', 'e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 10845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10104);
			match s {
				[] => true,
				['y', 'c', 'e', s @ ..] => {
					super::push(out, 10846);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11216);
							match s {
								[] => true,
								['o', 'h', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11388);
											s.is_empty()
										}
										['o', 'i', 'g', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 11389);
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
		_ => false,
	}
}

/// Words ending -vitis (3 syllables).
pub(crate) fn n9295(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 10105);
			s.is_empty()
		}
		['o', 'n', 'y', 's', 'o', s @ ..] => {
			super::push(out, 10106);
			match s {
				[] => true,
				['r', 'h', 't', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10847);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltis (3 syllables).
pub(crate) fn n6528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6053);
	match s {
		[] => true,
		['a', 's', 'p', 'o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8508);
			s.is_empty()
		}
		['e', 'c', 'g', 'n', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 8509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -otis (3 syllables).
pub(crate) fn n6529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6054);
	match s {
		[] => true,
		['n', 'a', 'h', 'p', 'e', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 8510);
			s.is_empty()
		}
		['s', 'o', 'y', 'm', '\u{1}', s @ ..] => {
			super::push(out, 8511);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stis (3 syllables).
pub(crate) fn n6530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6055);
	match s {
		[] => true,
		['e', 't', 'o', 'v', 'o', '\u{1}', s @ ..] => {
			super::push(out, 8512);
			s.is_empty()
		}
		['o', 'r', 'g', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8513);
			s.is_empty()
		}
		['y', 'c', 'o', 'c', 'r', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 8514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttis (3 syllables).
pub(crate) fn n6531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6056);
	match s {
		[] => true,
		['a', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8515);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8516);
			match s {
				[] => true,
				['l', 'g', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10848);
							s.is_empty()
						}
						['i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10849);
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

/// Words ending -utis (3 syllables).
pub(crate) fn n6532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6057);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8517);
			match s {
				[] => true,
				['s', 'a', 'y', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 10107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'f', 'a', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 8518);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word botrytis (3 syllables).
pub(crate) fn n6533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6058);
	s.is_empty()
}

/// Words ending -phonium (3 syllables).
pub(crate) fn n10536(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 10596);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10597);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word helionium (3 syllables).
pub(crate) fn n10537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9612);
	s.is_empty()
}

/// Words ending -rkonium (3 syllables).
pub(crate) fn n10538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9613);
	match s {
		[] => true,
		['a', 'u', 'q', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11044);
					s.is_empty()
				}
				['o', 'h', 't', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 11045);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lonium (3 syllables).
pub(crate) fn n10539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9614);
	s.is_empty()
}

/// Words ending -monium (3 syllables).
pub(crate) fn n10540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9615);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10598);
			match s {
				[] => true,
				['a', 'd', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11046);
					s.is_empty()
				}
				['r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'd', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 10599);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10600);
			match s {
				[] => true,
				['a', 'l', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11048);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ronium (3 syllables).
pub(crate) fn n10541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9616);
	s.is_empty()
}

/// Words ending -sonium (3 syllables).
pub(crate) fn n10542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9617);
	s.is_empty()
}

/// Words ending -xonium (3 syllables).
pub(crate) fn n10543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9618);
	s.is_empty()
}

/// Words ending -zonium (3 syllables).
pub(crate) fn n10544(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11049);
					s.is_empty()
				}
				['r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 10601);
			s.is_empty()
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 10602);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ernium (3 syllables).
pub(crate) fn n8452(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 9619);
			s.is_empty()
		}
		['t', 's', 'i', 'l', 'l', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 9620);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word interlunium (3 syllables).
pub(crate) fn n8453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7759);
	s.is_empty()
}

/// Words ending -gynium (3 syllables).
pub(crate) fn n8454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7760);
	match s {
		[] => true,
		['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9621);
			s.is_empty()
		}
		['o', 'd', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9622);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sanatarium (3 syllables).
pub(crate) fn n11689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10622);
	s.is_empty()
}

/// Words ending -etarium (3 syllables).
pub(crate) fn n11690(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 11070);
			match s {
				[] => true,
				['a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11290);
					s.is_empty()
				}
				['g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 11071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itarium (3 syllables).
pub(crate) fn n11691(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', 'r', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 11072);
			s.is_empty()
		}
		['n', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 11073);
			s.is_empty()
		}
		['u', 'q', 's', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 11074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -entarium (3 syllables).
pub(crate) fn n11692(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', 'a', 'm', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 11075);
			s.is_empty()
		}
		['p', 'r', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 11076);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word reptarium (3 syllables).
pub(crate) fn n11693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10623);
	s.is_empty()
}

/// Words ending -uarium (3 syllables).
pub(crate) fn n10562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9628);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 11077);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 11078);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11292);
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

/// Words ending -varium (3 syllables).
pub(crate) fn n10563(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 11079);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 11080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 10624);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -brium (3 syllables).
pub(crate) fn n8460(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'l', 'i', 'u', 'q', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10626);
					s.is_empty()
				}
				['n', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 10627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'y', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 9629);
			s.is_empty()
		}
		['o', 'r', 'p', 'p', 'o', '\u{1}', s @ ..] => {
			super::push(out, 9630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -drium (3 syllables).
pub(crate) fn n8461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7765);
	match s {
		[] => true,
		['n', 'a', 'n', 'e', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 9631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -erium (3 syllables).
pub(crate) fn n8462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7766);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 9632);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 10628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'm', 'i', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 9633);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9634);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10629);
					s.is_empty()
				}
				['i', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10630);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10631);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 11293);
									s.is_empty()
								}
								['n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 11294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 10632);
					s.is_empty()
				}
				['y', 'd', 'o', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10633);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word delirium (3 syllables).
pub(crate) fn n8463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7767);
	s.is_empty()
}

/// Words ending -orium (3 syllables).
pub(crate) fn n8464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7768);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10634);
					s.is_empty()
				}
				['y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 't', 'o', 's', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9635);
			s.is_empty()
		}
		['n', 'i', 'l', 'c', s @ ..] => {
			match s {
				[] => true,
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10636);
					s.is_empty()
				}
				['n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 10637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 10638);
					s.is_empty()
				}
				['m', 'e', '\u{1}', s @ ..] => {
					super::push(out, 10639);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10640);
					match s {
						[] => true,
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11081);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10641);
					s.is_empty()
				}
				['n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 10642);
					s.is_empty()
				}
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'u', 't', 's', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11083);
							s.is_empty()
						}
						['c', 'i', 'n', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 11084);
							s.is_empty()
						}
						['m', 'e', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11085);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11086);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', 'e', '\u{1}', s @ ..] => {
											super::push(out, 11374);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11375);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'l', 'c', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 11295);
									s.is_empty()
								}
								['m', 'y', 'g', '\u{1}', s @ ..] => {
									super::push(out, 11296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 11087);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11297);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['l', 'p', 'x', 'e', '\u{1}', s @ ..] => {
											super::push(out, 11376);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 11377);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11088);
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 11299);
									s.is_empty()
								}
								['r', 'e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 11300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 10644);
					match s {
						[] => true,
						['u', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 11089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11090);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', '\u{1}', s @ ..] => {
									super::push(out, 11301);
									s.is_empty()
								}
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 11302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11092);
							s.is_empty()
						}
						['n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 11093);
							s.is_empty()
						}
						['r', 'b', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 11095);
							s.is_empty()
						}
						['e', 'v', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11096);
							s.is_empty()
						}
						['o', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', 'g', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11098);
							s.is_empty()
						}
						['y', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11100);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 11101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'o', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 9636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trium (3 syllables).
pub(crate) fn n8465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7769);
	match s {
		[] => true,
		['a', 'b', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 9637);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urium (3 syllables).
pub(crate) fn n8466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7770);
	match s {
		[] => true,
		['e', 'n', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9638);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word martyrium (3 syllables).
pub(crate) fn n8467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7771);
	s.is_empty()
}

/// The word pussyification (3 syllables).
pub(crate) fn n12569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11391);
	s.is_empty()
}

