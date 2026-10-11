use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ि', 'त', 'य', '\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['ष', s @ ..] => {
					super::push(out, 1);
					match s {
						[] => true,
						['ए', s @ ..] => {
							super::push(out, 3);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 4);
							match s {
								[] => true,
								['ख', s @ ..] => {
									super::push(out, 10);
									match s {
										[] => true,
										['\u{947}', s @ ..] => {
											super::push(out, 21);
											s.is_empty()
										}
										_ => false,
									}
								}
								['च', s @ ..] => {
									super::push(out, 11);
									match s {
										[] => true,
										['\u{94d}', s @ ..] => {
											super::push(out, 22);
											s.is_empty()
										}
										_ => false,
									}
								}
								['द', s @ ..] => {
									super::push(out, 12);
									match s {
										[] => true,
										['व', s @ ..] => {
											super::push(out, 23);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 34);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['म', s @ ..] => {
									super::push(out, 13);
									match s {
										[] => true,
										['ग', 'व', s @ ..] => {
											super::push(out, 24);
											match s {
												[] => true,
												['अ', '\u{1}', s @ ..] => {
													super::push(out, 35);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['य', s @ ..] => {
									super::push(out, 14);
									match s {
										[] => true,
										['ग', s @ ..] => {
											super::push(out, 25);
											match s {
												[] => true,
												['\u{94d}', s @ ..] => {
													super::push(out, 36);
													s.is_empty()
												}
												_ => false,
											}
										}
										['प', s @ ..] => {
											super::push(out, 26);
											match s {
												[] => true,
												['ा', 'ड', s @ ..] => {
													super::push(out, 37);
													match s {
														[] => true,
														['\u{94d}', 'ड', 'उ', '\u{1}', s @ ..] => super::t0::n43(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['म', s @ ..] => {
											super::push(out, 27);
											match s {
												[] => true,
												['आ', s @ ..] => {
													super::push(out, 38);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n44(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ष', s @ ..] => {
											super::push(out, 28);
											match s {
												[] => true,
												['\u{94d}', s @ ..] => {
													super::push(out, 39);
													match s {
														[] => true,
														['क', 'त', '\u{1}', s @ ..] => super::t0::n45(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['र', s @ ..] => {
									super::push(out, 15);
									match s {
										[] => true,
										['घ', s @ ..] => {
											super::push(out, 29);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 40);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['व', s @ ..] => {
									super::push(out, 16);
									match s {
										[] => true,
										['अ', s @ ..] => {
											super::push(out, 30);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 41);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ह', s @ ..] => {
									super::push(out, 17);
									match s {
										[] => true,
										['\u{94d}', s @ ..] => {
											super::push(out, 31);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ो', s @ ..] => {
							super::push(out, 5);
							match s {
								[] => true,
								['द', s @ ..] => {
									super::push(out, 18);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				['स', s @ ..] => {
					super::push(out, 2);
					match s {
						[] => true,
						['\u{902}', s @ ..] => {
							super::push(out, 6);
							match s {
								[] => true,
								['ग', s @ ..] => {
									super::push(out, 19);
									match s {
										[] => true,
										['ि', 'ध', 'अ', '\u{1}', s @ ..] => {
											super::push(out, 33);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ा', s @ ..] => {
							super::push(out, 7);
							match s {
								[] => true,
								['ञ', '\u{94d}', 'ज', '\u{1}', s @ ..] => {
									super::push(out, 20);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 8);
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

/// The word उड्डापयिष्यति.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	s.is_empty()
}

/// The word आमयिष्यति.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	s.is_empty()
}

/// The word तक्षयिष्यति.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	s.is_empty()
}

