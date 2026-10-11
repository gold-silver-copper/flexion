use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ι', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['ο', 'ι', 'ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 12);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 8);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 13);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 20);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 21);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 14);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 15);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 26);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 27);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', 'α', 'λ', 'π', 'α', 'σ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 22);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', 'ι', 'λ', 'η', 'π', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 16);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 17);
							match s {
								[] => true,
								['ε', 'τ', s @ ..] => {
									match s {
										[] => true,
										['α', 'κ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 28);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 29);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 30);
											match s {
												[] => true,
												['φ', 'μ', 'α', '\u{1}', s @ ..] => {
													super::push(out, 32);
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
						['σ', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 19);
							match s {
								[] => true,
								['α', 'τ', 'ο', 'τ', 'υ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 23);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 24);
									match s {
										[] => true,
										['α', 'κ', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 31);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 25);
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
		['υ', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 5);
			s.is_empty()
		}
		_ => false,
	}
}

