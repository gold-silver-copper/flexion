use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ι', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['ω', s @ ..] => {
					match s {
						[] => true,
						['ι', 'σ', 'α', 'λ', 'π', 'α', 'σ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 11);
							s.is_empty()
						}
						['λ', 'η', 'λ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 12);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 13);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 23);
									match s {
										[] => true,
										['η', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 31);
											s.is_empty()
										}
										['υ', 'ο', '\u{1}', s @ ..] => {
											super::push(out, 32);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 24);
									s.is_empty()
								}
								['φ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 25);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 14);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26);
									s.is_empty()
								}
								['σ', 'ο', '\u{1}', s @ ..] => {
									super::push(out, 27);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 8);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 16);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 28);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', 'ι', 'ε', 'κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 17);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 20);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 29);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 30);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 33);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 34);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36);
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
		['υ', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 22);
							match s {
								[] => true,
								['υ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 37);
													s.is_empty()
												}
												['ε', s @ ..] => {
													super::push(out, 38);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n43(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ω', s @ ..] => {
											super::push(out, 35);
											match s {
												[] => true,
												['ε', '\u{1}', s @ ..] => {
													super::push(out, 39);
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
		['ω', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word εαυτου.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	s.is_empty()
}

