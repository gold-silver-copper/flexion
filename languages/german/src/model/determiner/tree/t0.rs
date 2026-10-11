use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['\u{13}', 'i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 24);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 25);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 9);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							super::push(out, 10);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 11);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 's', '\u{1}', s @ ..] => {
									super::push(out, 16);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 17);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{12}', s @ ..] => {
							super::push(out, 12);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 18);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 19);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 27);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 20);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 21);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 28);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 29);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ß', s @ ..] => {
									super::push(out, 22);
									match s {
										[] => true,
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 30);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 13);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 23);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

