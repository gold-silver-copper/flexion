use flexion::Path;

/// Words ending -vesco.
pub(crate) fn n1298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1188);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2652);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4126);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 5022);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5378);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 4128);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5379);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5466);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5380);
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
			super::push(out, 2653);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2654);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2655);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4131);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5381);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5382);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5467);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 5383);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5468);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5384);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5469);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 5470);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 5485);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n8253(s, out),
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
				['u', s @ ..] => {
					super::push(out, 4132);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5024);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5025);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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

/// Words ending -tericio.
pub(crate) fn n5895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4416);
	match s {
		[] => true,
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rricio.
pub(crate) fn n3831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3119);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4417);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sicio.
pub(crate) fn n1815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1604);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3120);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word interequito.
pub(crate) fn n8249(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -tresco.
pub(crate) fn n3121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2638);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4102);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4103);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 5007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uresco.
pub(crate) fn n3122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2639);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4104);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4105);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4106);
			match s {
				[] => true,
				['a', 'm', 'e', s @ ..] => {
					super::push(out, 5008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// The word confervesco.
pub(crate) fn n8253(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

