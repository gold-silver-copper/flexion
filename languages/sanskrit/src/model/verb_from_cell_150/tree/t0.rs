use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ः', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['क', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 2);
					s.is_empty()
				}
				['घ', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 3);
					s.is_empty()
				}
				['ा', 'व', '\u{94d}', 'स', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 4);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['ो', s @ ..] => {
							super::push(out, 10);
							match s {
								[] => true,
								['म', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 31);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', 'र', s @ ..] => {
							match s {
								[] => true,
								['द', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 32);
									s.is_empty()
								}
								['व', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 33);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ट', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 11);
							s.is_empty()
						}
						['च', s @ ..] => {
							super::push(out, 12);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 13);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 14);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 15);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 16);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 17);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						['ष', s @ ..] => {
							super::push(out, 20);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									super::push(out, 34);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ह', s @ ..] => {
							super::push(out, 21);
							s.is_empty()
						}
						['ा', s @ ..] => {
							super::push(out, 22);
							s.is_empty()
						}
						['ी', s @ ..] => {
							super::push(out, 23);
							match s {
								[] => true,
								['त', s @ ..] => {
									super::push(out, 35);
									s.is_empty()
								}
								['द', s @ ..] => {
									super::push(out, 36);
									s.is_empty()
								}
								['न', s @ ..] => {
									super::push(out, 37);
									s.is_empty()
								}
								['र', s @ ..] => {
									super::push(out, 38);
									s.is_empty()
								}
								['व', s @ ..] => {
									super::push(out, 39);
									s.is_empty()
								}
								['ष', s @ ..] => {
									super::push(out, 40);
									match s {
										[] => true,
										['ऐ', s @ ..] => {
											super::push(out, 47);
											s.is_empty()
										}
										['ा', s @ ..] => {
											super::push(out, 48);
											s.is_empty()
										}
										['\u{948}', s @ ..] => {
											super::push(out, 49);
											s.is_empty()
										}
										['ौ', s @ ..] => {
											super::push(out, 50);
											s.is_empty()
										}
										['\u{94d}', s @ ..] => {
											super::push(out, 51);
											match s {
												[] => true,
												['क', s @ ..] => {
													super::push(out, 60);
													match s {
														[] => true,
														['ा', s @ ..] => super::t0::n68(s, out),
														['\u{94d}', s @ ..] => super::t0::n69(s, out),
														_ => false,
													}
												}
												['र', s @ ..] => {
													super::push(out, 61);
													match s {
														[] => true,
														['ा', s @ ..] => super::t0::n70(s, out),
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
									super::push(out, 41);
									match s {
										[] => true,
										['\u{902}', s @ ..] => {
											super::push(out, 52);
											s.is_empty()
										}
										['ा', s @ ..] => {
											super::push(out, 53);
											s.is_empty()
										}
										['\u{94d}', s @ ..] => super::t0::n58(s, out),
										_ => false,
									}
								}
								['ह', s @ ..] => super::t0::n44(s, out),
								_ => false,
							}
						}
						['\u{942}', s @ ..] => super::t0::n26(s, out),
						['\u{947}', s @ ..] => super::t0::n27(s, out),
						['ो', s @ ..] => super::t0::n28(s, out),
						['\u{94d}', s @ ..] => super::t0::n29(s, out),
						_ => false,
					}
				}
				['न', s @ ..] => super::t0::n9(s, out),
				['र', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ाक्षीत्.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['ध', s @ ..] => {
			super::push(out, 68);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 69);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 70);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 71);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्क्षीत्.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	s.is_empty()
}

/// Words ending -ार्षीत्.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	s.is_empty()
}

/// Words ending -्सीत्.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['त', s @ ..] => {
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 66);
					s.is_empty()
				}
				['ौ', s @ ..] => {
					super::push(out, 67);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 72);
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

/// Words ending -हीत्.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['र', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['\u{94d}', 'ग', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 62);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ूत्.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['भ', s @ ..] => {
			super::push(out, 43);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ेत्.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['छ', '\u{94d}', 'च', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 44);
			s.is_empty()
		}
		['भ', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 45);
			s.is_empty()
		}
		['र', '\u{94d}', 'श', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 46);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ोत्.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['र', s @ ..] => {
			match s {
				[] => true,
				['अ', '\u{1}', s @ ..] => {
					super::push(out, 56);
					s.is_empty()
				}
				['\u{94d}', 'श', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 57);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -्त्.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['र', s @ ..] => {
			match s {
				[] => true,
				['आ', '\u{1}', s @ ..] => {
					super::push(out, 58);
					s.is_empty()
				}
				['व', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 59);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -न्.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['ग', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 28);
			s.is_empty()
		}
		['त', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 29);
			s.is_empty()
		}
		['र', '\u{94d}', 'क', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 30);
			s.is_empty()
		}
		_ => false,
	}
}

