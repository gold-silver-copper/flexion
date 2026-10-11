use flexion::Path;

/// Words ending -id.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2135);
					match s {
						[] => true,
						['a', 't', 'h', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2136);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['r', 'h', 'o', 'e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2140);
					match s {
						[] => true,
						['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4346);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2141);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2142);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4348);
							match s {
								[] => true,
								['b', 'o', 'l', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5708);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6162);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 6163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4349);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4350);
							match s {
								[] => true,
								['o', 'n', s @ ..] => {
									super::push(out, 5709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2143);
					s.is_empty()
				}
				['r', 'a', 'g', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2144);
					s.is_empty()
				}
				['u', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				['t', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['f', 'i', 'd', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2152);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4352);
							s.is_empty()
						}
						['g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4353);
							s.is_empty()
						}
						['n', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2154);
					match s {
						[] => true,
						['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2155);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4356);
							s.is_empty()
						}
						['m', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2156);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4358);
							s.is_empty()
						}
						['d', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4359);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2157);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['u', 'm', 'e', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 543);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2160);
					match s {
						[] => true,
						['o', 'm', 'a', s @ ..] => {
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5710);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5711);
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
			super::push(out, 545);
			match s {
				[] => true,
				['c', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2162);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4361);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2163);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4363);
							s.is_empty()
						}
						['t', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2165);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4365);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5712);
									s.is_empty()
								}
								['i', 'm', s @ ..] => {
									super::push(out, 5713);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 6164);
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
					super::push(out, 2166);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2167);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4367);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5714);
									match s {
										[] => true,
										['d', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6165);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6166);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4368);
							match s {
								[] => true,
								['n', 's', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5716);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2168);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4369);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5718);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4371);
							match s {
								[] => true,
								['c', 'h', 'o', 'n', '\u{1}', s @ ..] => super::t1::n5966(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t1::n4535(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n558(s, out),
		['u', s @ ..] => super::t1::n559(s, out),
		['v', s @ ..] => super::t1::n560(s, out),
		_ => false,
	}
}

/// Words ending -od.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['t', 'o', 'r', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['r', 'a', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2179);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5723);
									s.is_empty()
								}
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6167);
											s.is_empty()
										}
										['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6168);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4376);
							s.is_empty()
						}
						['t', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2181);
					match s {
						[] => true,
						['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2182);
					match s {
						[] => true,
						['t', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4380);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4381);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 6169);
											s.is_empty()
										}
										['v', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6170);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', '\u{1}', s @ ..] => {
							super::push(out, 4382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['i', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rd.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2188);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2189);
					match s {
						[] => true,
						['p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4384);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['m', 'y', 'p', 'h', 'a', 'g', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2191);
					s.is_empty()
				}
				['p', 'a', 'x', '\u{1}', s @ ..] => {
					super::push(out, 2192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 561);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 562);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 563);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2193);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5724);
									s.is_empty()
								}
								['r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 5725);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['c', 'a', 'm', 'a', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 566);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['d', 'e', 'c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 568);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2198);
					match s {
						[] => true,
						['m', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4387);
							s.is_empty()
						}
						['t', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2199);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yd.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['o', 'd', 'e', 'c', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 571);
			s.is_empty()
		}
		['s', 'u', 'r', 'i', 'a', 'c', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ime.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 601);
	match s {
		[] => true,
		['r', '\u{1}', s @ ..] => {
			super::push(out, 2234);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pme.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['e', 'r', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2235);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2236);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2237);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ume.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['n', 'c', 't', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2239);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word syme.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	s.is_empty()
}

/// Words ending -ne.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['r', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['h', 'i', 'r', 'i', 'd', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2242);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2243);
					match s {
						[] => true,
						['m', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 608);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['r', 'v', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 610);
			match s {
				[] => true,
				['g', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['d', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2247);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2248);
					match s {
						[] => true,
						['c', 'h', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4399);
							s.is_empty()
						}
						['m', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 612);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['h', 'u', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['m', 'e', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2250);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word suoe.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	s.is_empty()
}

/// Words ending -pe.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['e', 'm', 'e', 'r', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2254);
					match s {
						[] => true,
						['t', 'o', 'l', s @ ..] => {
							super::push(out, 4401);
							match s {
								[] => true,
								['i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5730);
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
			super::push(out, 618);
			match s {
				[] => true,
				['o', 'p', 'h', 'o', 'r', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qe.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['u', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['d', 'i', 'c', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2258);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2259);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 624);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 625);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4402);
							match s {
								[] => true,
								['t', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5731);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2261);
					s.is_empty()
				}
				['s', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2262);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 626);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2264);
					s.is_empty()
				}
				['t', 'y', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['c', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2266);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2267);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2269);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2270);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 631);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2272);
					match s {
						[] => true,
						['r', 'i', 'o', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'x', '\u{1}', s @ ..] => {
			super::push(out, 632);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 633);
			s.is_empty()
		}
		['t', 'o', 'v', 'e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 634);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 637);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2274);
					s.is_empty()
				}
				['i', 't', 'r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'u', 'c', 'h', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 640);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 641);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2276);
					match s {
						[] => true,
						['p', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4405);
							s.is_empty()
						}
						['t', 'i', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4406);
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

/// Words ending -ve.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['e', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4407);
							s.is_empty()
						}
						['g', 'e', 'l', 'i', 'z', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2278);
					s.is_empty()
				}
				['s', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['n', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xe.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2282);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4409);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2283);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['a', 'n', 'd', 'e', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2288);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['a', 'm', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2290);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4412);
							s.is_empty()
						}
						['r', 'i', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2291);
					match s {
						[] => true,
						['e', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4415);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4416);
							match s {
								[] => true,
								['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5732);
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
			super::push(out, 649);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2293);
					match s {
						[] => true,
						['r', 'a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 650);
			match s {
				[] => true,
				['a', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2298);
					match s {
						[] => true,
						['i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2299);
					match s {
						[] => true,
						['t', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 652);
			match s {
				[] => true,
				['d', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['l', 'e', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				['r', 'c', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['a', 'n', 's', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4420);
							match s {
								[] => true,
								['i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4421);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4423);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4424);
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5734);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4425);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4426);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5736);
									s.is_empty()
								}
								['s', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2309);
					match s {
						[] => true,
						['o', 'b', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2310);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4428);
							s.is_empty()
						}
						['r', 'g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['u', 'i', 's', 'i', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2313);
					match s {
						[] => true,
						['i', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2314);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4431);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5738);
									s.is_empty()
								}
								['r', 'p', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5739);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4433);
							s.is_empty()
						}
						['d', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4434);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2316);
					match s {
						[] => true,
						['n', 's', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4436);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2317);
					match s {
						[] => true,
						['n', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2318);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4439);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5740);
									s.is_empty()
								}
								['v', 'a', 'g', 'a', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4440);
							match s {
								[] => true,
								['m', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5742);
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
			super::push(out, 657);
			match s {
				[] => true,
				['d', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lef.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 673);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2338);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2339);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2340);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 4463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'n', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2341);
			s.is_empty()
		}
		['t', 'r', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2342);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -emef.
pub(crate) fn n2403(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 4464);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 4465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nef.
pub(crate) fn n687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2344);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2345);
			match s {
				[] => true,
				['r', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2346);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oef.
pub(crate) fn n688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	match s {
		[] => true,
		['d', 'a', 't', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4467);
					s.is_empty()
				}
				['o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4468);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ref.
pub(crate) fn n689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2349);
			match s {
				[] => true,
				['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2350);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 4471);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sef.
pub(crate) fn n690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 678);
	match s {
		[] => true,
		['c', 'e', 'n', 'n', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2351);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4473);
					match s {
						[] => true,
						['n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5747);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 5748);
							match s {
								[] => true,
								['i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 6174);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4474);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5749);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 5750);
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

/// Words ending -tef.
pub(crate) fn n691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	match s {
		[] => true,
		['u', 't', 'i', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uef.
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 680);
	match s {
		[] => true,
		['d', 'a', s @ ..] => {
			super::push(out, 2353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -if.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['b', 'u', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 681);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2354);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2357);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4475);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4476);
							match s {
								[] => true,
								['i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4477);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				['u', 'c', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					super::push(out, 2360);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4480);
							s.is_empty()
						}
						['c', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4481);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4482);
							s.is_empty()
						}
						['x', '\u{1}', s @ ..] => {
							super::push(out, 4483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'r', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2364);
					s.is_empty()
				}
				['s', 't', 'a', 'l', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['m', 'a', 'm', s @ ..] => {
					super::push(out, 2366);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 689);
			s.is_empty()
		}
		['t', 'i', 'l', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 690);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lf.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				['c', 'c', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4485);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4486);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4488);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'n', 'e', 'l', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2369);
					s.is_empty()
				}
				['s', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4490);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['m', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				['t', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2372);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'c', 't', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 693);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2374);
					match s {
						[] => true,
						['c', 'i', 'n', 'a', 'u', 'c', 'i', 'n', 'i', 'h', 'i', 'l', 'i', 'p', 'i', 'l', 'i', 'f', 'i', 'c', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2377);
					s.is_empty()
				}
				['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2378);
					s.is_empty()
				}
				['i', 'd', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2379);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2380);
					s.is_empty()
				}
				['t', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2381);
					s.is_empty()
				}
				['v', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2382);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -of.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2383);
					match s {
						[] => true,
						['c', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4493);
							s.is_empty()
						}
						['l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4494);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4495);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6176);
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6177);
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
					super::push(out, 2384);
					match s {
						[] => true,
						['l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 697);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 698);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 699);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2385);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 700);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2387);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2388);
					match s {
						[] => true,
						['n', 's', 'e', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2389);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4498);
							match s {
								[] => true,
								['g', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5752);
									s.is_empty()
								}
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5753);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5754);
									match s {
										[] => true,
										['i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4499);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5755);
									s.is_empty()
								}
								['d', 'a', 'm', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4500);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2390);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2391);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 701);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rf.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['c', 't', s @ ..] => {
					super::push(out, 2393);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2394);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2395);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2396);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2397);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4503);
							s.is_empty()
						}
						['i', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4504);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2398);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2401);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 705);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2403);
					s.is_empty()
				}
				['g', 'a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				['i', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2405);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2406);
					s.is_empty()
				}
				['n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2407);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['s', 'a', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4507);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5757);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2408);
					s.is_empty()
				}
				['x', '\u{1}', s @ ..] => {
					super::push(out, 2409);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uf.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['c', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 707);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['a', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2410);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							super::push(out, 4508);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2413);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2414);
					match s {
						[] => true,
						['g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2417);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2418);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 4510);
							match s {
								[] => true,
								['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2419);
					match s {
						[] => true,
						['r', 'a', 't', 'i', 'c', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2423);
					match s {
						[] => true,
						['t', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2424);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2425);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2427);
					s.is_empty()
				}
				['u', 'n', 'c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 713);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2429);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2430);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 714);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2432);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4513);
							s.is_empty()
						}
						['t', 'r', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 4514);
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

/// Words ending -rg.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4528);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4530);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2463);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4532);
							s.is_empty()
						}
						['i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4533);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2464);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4535);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4536);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4537);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['h', 'e', 'o', 'c', 'r', 'a', 't', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2467);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 745);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 746);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2469);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4540);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4541);
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

/// Words ending -ug.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['r', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 749);
			s.is_empty()
		}
		['t', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yg.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 751);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 752);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['g', 'i', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -prih.
pub(crate) fn n2580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2505);
	s.is_empty()
}

/// Words ending -srih.
pub(crate) fn n2581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2506);
	s.is_empty()
}

/// Words ending -urih.
pub(crate) fn n2582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2507);
	s.is_empty()
}

/// Words ending -sih.
pub(crate) fn n790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	s.is_empty()
}

/// Words ending -oh.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2508);
					match s {
						[] => true,
						['r', 'o', 'm', 'a', 's', 't', 'i', 'g', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2510);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							super::push(out, 4556);
							match s {
								[] => true,
								['f', 'i', 'c', 'a', 'b', 'i', 'l', 'i', 't', 'u', 'd', 'i', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5771);
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
			super::push(out, 781);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2511);
					match s {
						[] => true,
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2512);
					s.is_empty()
				}
				['o', 'l', 'o', 'g', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2513);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2514);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2515);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 4558);
							match s {
								[] => true,
								['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5772);
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
			super::push(out, 782);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2516);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					super::push(out, 2517);
					match s {
						[] => true,
						['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4559);
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

/// Words ending -uh.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2518);
					match s {
						[] => true,
						['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4560);
							s.is_empty()
						}
						['t', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4562);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['a', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2520);
					s.is_empty()
				}
				['b', 'a', 'n', 'd', 'r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2521);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yh.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['r', 'o', 'p', s @ ..] => {
					super::push(out, 2522);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4564);
							s.is_empty()
						}
						['i', 's', 'm', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 785);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 786);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2523);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['i', 't', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 5773);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5774);
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

/// Words ending -matimi.
pub(crate) fn n4751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4578);
	match s {
		[] => true,
		['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmi.
pub(crate) fn n829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 814);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2546);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2547);
			match s {
				[] => true,
				['n', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2548);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2549);
			match s {
				[] => true,
				['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4581);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2550);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4583);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5780);
							s.is_empty()
						}
						['i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4584);
					match s {
						[] => true,
						['a', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5782);
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

/// Words ending -pmi.
pub(crate) fn n830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 815);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2551);
			match s {
				[] => true,
				['c', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2552);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4586);
					match s {
						[] => true,
						['i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2553);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2554);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4587);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4588);
					match s {
						[] => true,
						['s', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2555);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2556);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4589);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4590);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5786);
							s.is_empty()
						}
						['v', 'i', 's', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2557);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4592);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4593);
					s.is_empty()
				}
				['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4594);
					s.is_empty()
				}
				['r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word mumi.
pub(crate) fn n831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 816);
	s.is_empty()
}

/// Words ending -ni.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2558);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2559);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 4596);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'e', 'c', 'i', 'l', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 818);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2560);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2561);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4598);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5788);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2562);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4600);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4601);
							match s {
								[] => true,
								['u', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4602);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 6183);
											s.is_empty()
										}
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6184);
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
				['l', s @ ..] => {
					super::push(out, 2563);
					match s {
						[] => true,
						['e', 'm', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4603);
							s.is_empty()
						}
						['i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4604);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2564);
					match s {
						[] => true,
						['g', 'i', 't', 'a', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4606);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4607);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4608);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', 's', 'u', 'r', 'a', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 6185);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 6186);
											match s {
												[] => true,
												['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
													super::push(out, 6270);
													s.is_empty()
												}
												['d', s @ ..] => {
													super::push(out, 6271);
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
						['n', s @ ..] => {
							super::push(out, 4609);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5791);
									match s {
										[] => true,
										['i', 'd', 'e', 'r', 'a', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2565);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4611);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5792);
									s.is_empty()
								}
								['p', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2566);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4613);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4614);
							s.is_empty()
						}
						['s', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2567);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5794);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 5795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['b', 'i', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4616);
							s.is_empty()
						}
						['p', 'e', 'n', 'd', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2569);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4618);
							s.is_empty()
						}
						['g', 'e', 'r', 'i', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4619);
							s.is_empty()
						}
						['l', 'i', 'g', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4620);
							s.is_empty()
						}
						['v', 'i', 'd', 'u', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2570);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2571);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', 'a', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5796);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5797);
									s.is_empty()
								}
								['u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4622);
							s.is_empty()
						}
						['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4623);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4624);
							s.is_empty()
						}
						['t', 'i', 'a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['b', 'r', 'i', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2572);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				['r', 'u', 'd', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2576);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5799);
									s.is_empty()
								}
								['u', 'n', 'd', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'i', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4626);
							s.is_empty()
						}
						['r', s @ ..] => super::t1::n4803(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n2655(s, out),
				['l', s @ ..] => super::t1::n2656(s, out),
				['o', s @ ..] => super::t1::n2657(s, out),
				['r', s @ ..] => super::t1::n2658(s, out),
				['u', s @ ..] => super::t1::n2659(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n838(s, out),
		['h', s @ ..] => super::t1::n839(s, out),
		['i', s @ ..] => super::t1::n840(s, out),
		['l', s @ ..] => super::t1::n841(s, out),
		['m', s @ ..] => super::t1::n842(s, out),
		['n', s @ ..] => super::t1::n843(s, out),
		['o', s @ ..] => super::t1::n844(s, out),
		['p', s @ ..] => super::t1::n845(s, out),
		['q', s @ ..] => super::t1::n846(s, out),
		['r', s @ ..] => super::t1::n847(s, out),
		['s', s @ ..] => super::t1::n848(s, out),
		['t', s @ ..] => super::t1::n849(s, out),
		['u', s @ ..] => super::t1::n850(s, out),
		['v', s @ ..] => super::t1::n851(s, out),
		_ => false,
	}
}

/// Words ending -oi.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 837);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['u', 'n', s @ ..] => {
					super::push(out, 2612);
					match s {
						[] => true,
						['d', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 839);
			s.is_empty()
		}
		['g', 'u', 'r', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 840);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pi.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	s.is_empty()
}

/// Words ending -ri.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['c', 'u', 'n', 'd', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 843);
			s.is_empty()
		}
		['o', 'n', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 844);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2614);
					match s {
						[] => true,
						['g', 'u', 'l', 'a', 'r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2615);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4665);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4666);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4667);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2616);
					match s {
						[] => true,
						['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -si.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['u', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 848);
			s.is_empty()
		}
		['l', 'a', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 849);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['m', 'e', 't', 'r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2619);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'e', 'l', 'i', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 850);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ti.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	s.is_empty()
}

/// Words ending -ui.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2621);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['i', 'c', 'a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2624);
					match s {
						[] => true,
						['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2625);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5814);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'a', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2626);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'u', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 856);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 857);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2628);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				['o', 'n', 'i', 'c', 'o', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2631);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'c', 'o', 'n', 's', 'u', 'l', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2632);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2633);
					match s {
						[] => true,
						['s', 'c', 'o', 'n', 's', 'u', 'l', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2634);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2635);
					s.is_empty()
				}
				['i', 'u', 'r', 'a', 'n', 'd', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2636);
					s.is_empty()
				}
				['q', 'u', 'i', 'a', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2637);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2638);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2640);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2641);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -etal.
pub(crate) fn n2756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2676);
	match s {
		[] => true,
		['r', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4683);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word soryhtal.
pub(crate) fn n2757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2677);
	s.is_empty()
}

/// Words ending -ital.
pub(crate) fn n2758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2678);
	match s {
		[] => true,
		['c', 'l', 'a', 'v', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4684);
			s.is_empty()
		}
		['f', 'u', 'n', 'd', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4685);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4686);
			match s {
				[] => true,
				['i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 4687);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4688);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5824);
					s.is_empty()
				}
				['u', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5825);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otal.
pub(crate) fn n2759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2679);
	s.is_empty()
}

/// Words ending -rtal.
pub(crate) fn n2760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2680);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4689);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4690);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'n', 'c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utal.
pub(crate) fn n2761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2681);
	s.is_empty()
}

/// Words ending -ual.
pub(crate) fn n892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 876);
	s.is_empty()
}

/// Words ending -val.
pub(crate) fn n893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 877);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2682);
			match s {
				[] => true,
				['t', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xal.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 878);
	match s {
		[] => true,
		['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -el.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 879);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5827);
									match s {
										[] => true,
										['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6192);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5828);
									match s {
										[] => true,
										['n', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6193);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 't', 'e', 'r', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5829);
									s.is_empty()
								}
								['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4693);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4694);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 't', 'h', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 881);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2686);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2687);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4696);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4697);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4698);
							s.is_empty()
						}
						['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4700);
							s.is_empty()
						}
						['m', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				['m', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2689);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2690);
					s.is_empty()
				}
				['o', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2691);
					s.is_empty()
				}
				['u', 'r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2693);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2694);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4702);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2695);
					match s {
						[] => true,
						['c', 'i', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 884);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 885);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 887);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 888);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2697);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4705);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2699);
					match s {
						[] => true,
						['s', 't', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4707);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5831);
									s.is_empty()
								}
								['o', 'n', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5832);
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
		['x', s @ ..] => {
			super::push(out, 890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -il.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['a', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 891);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2700);
					match s {
						[] => true,
						['c', 'u', 'n', 'c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4708);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4709);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4710);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6194);
											s.is_empty()
										}
										['t', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6195);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4711);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4712);
							match s {
								[] => true,
								['o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2701);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4713);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6196);
											s.is_empty()
										}
										['u', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6197);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4714);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4715);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5834);
									match s {
										[] => true,
										['l', 'i', 's', 'm', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6198);
											s.is_empty()
										}
										['t', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6199);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5835);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2702);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4716);
							match s {
								[] => true,
								['i', 'n', 'o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4717);
							match s {
								[] => true,
								['i', 'n', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2703);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5839);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 5840);
									match s {
										[] => true,
										['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6200);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5841);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['o', 'l', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 6272);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 6273);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2705);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2706);
					match s {
						[] => true,
						['e', 'n', 'o', 'l', 'o', 'g', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2707);
					match s {
						[] => true,
						['a', 't', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4720);
							s.is_empty()
						}
						['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4721);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2709);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4723);
							s.is_empty()
						}
						['t', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 895);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2711);
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2712);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2713);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2714);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4726);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 4727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 897);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2717);
					s.is_empty()
				}
				['c', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2718);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2719);
					match s {
						[] => true,
						['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2720);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2721);
					match s {
						[] => true,
						['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4729);
							s.is_empty()
						}
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2722);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 898);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 899);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4731);
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 900);
			match s {
				[] => true,
				['i', 'c', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n917(s, out),
		['v', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['e', 'r', 'm', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ol.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2732);
					match s {
						[] => true,
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4737);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2733);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2734);
					match s {
						[] => true,
						['p', 'l', 'e', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4739);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4740);
							match s {
								[] => true,
								['t', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4741);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 5846);
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
			super::push(out, 906);
			match s {
				[] => true,
				['i', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 907);
			s.is_empty()
		}
		['l', 'i', 'g', 'o', '\u{1}', s @ ..] => {
			super::push(out, 908);
			s.is_empty()
		}
		['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 909);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['g', 'e', 'v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4742);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2738);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 913);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ul.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['b', 'i', 'd', 'o', '\u{1}', s @ ..] => {
			super::push(out, 914);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2739);
					match s {
						[] => true,
						['n', 'i', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2740);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2741);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2742);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2743);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2744);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4747);
							match s {
								[] => true,
								['r', 'a', 't', 'i', 'u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4748);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 916);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 917);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2745);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2746);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2747);
					s.is_empty()
				}
				['i', 'n', 'a', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2748);
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['t', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2751);
					match s {
						[] => true,
						['n', 'a', 'r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4750);
							s.is_empty()
						}
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2753);
					match s {
						[] => true,
						['c', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2754);
					s.is_empty()
				}
				['t', 'r', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4753);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4754);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2755);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2757);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yl.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 925);
			match s {
				[] => true,
				['i', 'm', 'a', 'c', 'h', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siram.
pub(crate) fn n4959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4776);
	s.is_empty()
}

/// Words ending -atiram.
pub(crate) fn n4960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4777);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5851);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ropram.
pub(crate) fn n2876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2788);
	s.is_empty()
}

/// Words ending -sram.
pub(crate) fn n2877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2789);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4778);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sam.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['c', 'a', s @ ..] => {
			super::push(out, 2790);
			match s {
				[] => true,
				['r', 'p', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2791);
			s.is_empty()
		}
		['u', 'c', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2792);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tam.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 937);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2793);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2794);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4781);
					s.is_empty()
				}
				['o', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4782);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2795);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5852);
							s.is_empty()
						}
						['s', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2796);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2797);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4784);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 6201);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6202);
									match s {
										[] => true,
										['l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6274);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4785);
					match s {
						[] => true,
						['n', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2798);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2799);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4786);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4787);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word allixam.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 938);
	s.is_empty()
}

/// Words ending -zam.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 939);
	s.is_empty()
}

/// Words ending -em.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 940);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 941);
			match s {
				[] => true,
				['o', 'n', 'i', 't', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4788);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 6203);
											match s {
												[] => true,
												['e', 'n', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n6579(s, out),
														['t', s @ ..] => super::t1::n6580(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 6204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'i', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4789);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4790);
							match s {
								[] => true,
								['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2801);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 943);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 944);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2802);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2803);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2804);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2805);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4793);
							s.is_empty()
						}
						['o', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2806);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2807);
					match s {
						[] => true,
						['d', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4795);
							s.is_empty()
						}
						['s', 'm', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 4796);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4797);
							match s {
								[] => true,
								['r', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 5857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2809);
					match s {
						[] => true,
						['r', 'a', 'n', 'a', s @ ..] => {
							super::push(out, 4798);
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'c', 'y', 'l', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2810);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['a', 'r', 'c', 'h', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2812);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2813);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4799);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5859);
									match s {
										[] => true,
										['i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6205);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4800);
							match s {
								[] => true,
								['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2814);
					s.is_empty()
				}
				['o', 'g', 'e', 'n', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2815);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 947);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2818);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4801);
							match s {
								[] => true,
								['d', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2819);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2820);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2821);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4802);
							s.is_empty()
						}
						['s', 'm', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 4803);
							s.is_empty()
						}
						['t', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 948);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 949);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4805);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4806);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4807);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4808);
							match s {
								[] => true,
								['a', 'r', 's', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2824);
					match s {
						[] => true,
						['o', 'r', 'o', 'l', 'o', 'g', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2825);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2826);
					match s {
						[] => true,
						['p', 'i', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2827);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 950);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -im.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 951);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['e', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2828);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'd', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4812);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4813);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4815);
							s.is_empty()
						}
						['i', 'a', 'r', 'd', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2830);
					s.is_empty()
				}
				['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2831);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2832);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2833);
					match s {
						[] => true,
						['f', 'l', 'e', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4817);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 4818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2834);
					match s {
						[] => true,
						['t', 'i', 'l', 'o', 'q', 'u', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 956);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2835);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5863);
									s.is_empty()
								}
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'i', 'l', 'l', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['n', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				['s', 'a', s @ ..] => {
					super::push(out, 2838);
					match s {
						[] => true,
						['t', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4820);
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
				['i', 'g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2839);
					s.is_empty()
				}
				['u', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nm.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['e', 'm', 'o', 's', 'y', 'n', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -om.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2841);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2842);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4821);
							match s {
								[] => true,
								['u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2843);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4823);
							match s {
								[] => true,
								['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4824);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4825);
							match s {
								[] => true,
								['i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5868);
									s.is_empty()
								}
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2844);
					match s {
						[] => true,
						['i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2846);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4828);
							s.is_empty()
						}
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 965);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2847);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2848);
					match s {
						[] => true,
						['d', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4830);
							s.is_empty()
						}
						['t', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2849);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2850);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2851);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2853);
					match s {
						[] => true,
						['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4832);
							s.is_empty()
						}
						['r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2854);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2855);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2856);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2857);
					match s {
						[] => true,
						['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2858);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2859);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2860);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4835);
							match s {
								[] => true,
								['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2861);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'q', 'u', 'e', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -um.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2863);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2864);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4838);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2865);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2866);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2867);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2869);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2870);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4840);
							s.is_empty()
						}
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4841);
							s.is_empty()
						}
						['t', 'i', 'u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2872);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4843);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5871);
									s.is_empty()
								}
								['u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', '\u{1}', s @ ..] => {
							super::push(out, 4844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2873);
					match s {
						[] => true,
						['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 4845);
							s.is_empty()
						}
						['l', 'e', 'g', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2874);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2875);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2877);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2878);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4847);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4848);
							s.is_empty()
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2879);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4850);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4851);
							match s {
								[] => true,
								['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', 't', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2880);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2881);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4852);
							s.is_empty()
						}
						['e', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4853);
							s.is_empty()
						}
						['r', 'i', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'm', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2883);
					match s {
						[] => true,
						['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2884);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2885);
					match s {
						[] => true,
						['n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2886);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2887);
					match s {
						[] => true,
						['n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4857);
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

/// Words ending -ym.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['i', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2888);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 978);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -on.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['e', 'l', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1016);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2924);
					s.is_empty()
				}
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2925);
					s.is_empty()
				}
				['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2926);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'n', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1017);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2928);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2929);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4866);
							s.is_empty()
						}
						['s', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1019);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2930);
					s.is_empty()
				}
				['e', 'o', 'l', 'a', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2931);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4868);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'm', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1020);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2932);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2933);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2934);
					s.is_empty()
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2936);
					match s {
						[] => true,
						['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2937);
					s.is_empty()
				}
				['i', 'c', 'i', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bun.
pub(crate) fn n1043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1025);
	match s {
		[] => true,
		['e', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2939);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gun.
pub(crate) fn n1045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1027);
	match s {
		[] => true,
		['a', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2941);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lun.
pub(crate) fn n1046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1028);
	s.is_empty()
}

/// Words ending -mun.
pub(crate) fn n1047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1029);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4872);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nun.
pub(crate) fn n1048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1030);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2943);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4874);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2944);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2945);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pun.
pub(crate) fn n1049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1031);
	s.is_empty()
}

/// Words ending -run.
pub(crate) fn n1050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1032);
	s.is_empty()
}

/// Words ending -tun.
pub(crate) fn n1051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1033);
	match s {
		[] => true,
		['r', 'i', 'm', s @ ..] => {
			super::push(out, 2946);
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word xun.
pub(crate) fn n1052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1034);
	s.is_empty()
}

/// Words ending -yn.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	s.is_empty()
}

/// Words ending -aelo.
pub(crate) fn n3073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2975);
	match s {
		[] => true,
		['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4892);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word satisoelo.
pub(crate) fn n3075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2977);
	s.is_empty()
}

/// The word mutelo.
pub(crate) fn n3076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2978);
	s.is_empty()
}

/// Words ending -flo.
pub(crate) fn n1074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1055);
	s.is_empty()
}

/// Words ending -ilo.
pub(crate) fn n1075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1056);
	match s {
		[] => true,
		['p', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2979);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2980);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2981);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 4893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word allo.
pub(crate) fn n1076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1057);
	s.is_empty()
}

/// The word muitavlo.
pub(crate) fn n1077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1058);
	s.is_empty()
}

/// Words ending -mo.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['a', 's', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1059);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1060);
			s.is_empty()
		}
		['i', 's', 's', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['r', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['c', 'r', 'o', 't', 'a', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2983);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2984);
					s.is_empty()
				}
				['n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oo.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	s.is_empty()
}

/// Words ending -po.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4894);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4895);
							match s {
								[] => true,
								['s', 't', 'i', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1068);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2987);
					s.is_empty()
				}
				['l', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2988);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['i', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2991);
					s.is_empty()
				}
				['n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2992);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2993);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1074);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1075);
			match s {
				[] => true,
				['i', 'n', 'a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4898);
							match s {
								[] => true,
								['i', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1076);
			match s {
				[] => true,
				['a', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1077);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1078);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2995);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2996);
					match s {
						[] => true,
						['h', 'a', 'l', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2997);
					s.is_empty()
				}
				['f', 'i', 'c', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2998);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2999);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4901);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4902);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1079);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3001);
					match s {
						[] => true,
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1080);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1082);
			s.is_empty()
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1083);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1085);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1087);
			s.is_empty()
		}
		['o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1088);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3006);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['s', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5884);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3007);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3008);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['h', 'o', 'n', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1091);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3009);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3010);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['d', 'u', 'c', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3012);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3013);
					match s {
						[] => true,
						['i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xo.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	s.is_empty()
}

/// The word alocidulap.
pub(crate) fn n6152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5892);
	s.is_empty()
}

/// Words ending -mulap.
pub(crate) fn n5104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4915);
	s.is_empty()
}

/// Words ending -sulap.
pub(crate) fn n5105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4916);
	s.is_empty()
}

/// Words ending -map.
pub(crate) fn n1120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1101);
	s.is_empty()
}

/// Words ending -nap.
pub(crate) fn n1121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1102);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3035);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4917);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4918);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4919);
					s.is_empty()
				}
				['x', '\u{1}', s @ ..] => {
					super::push(out, 4920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3036);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3037);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3038);
			match s {
				[] => true,
				['g', 'y', 'r', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4921);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4922);
					s.is_empty()
				}
				['t', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3039);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3040);
			s.is_empty()
		}
		['s', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3041);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3042);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3043);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pap.
pub(crate) fn n1122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1103);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3044);
			match s {
				[] => true,
				['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4924);
					s.is_empty()
				}
				['v', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3045);
			match s {
				[] => true,
				['l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rap.
pub(crate) fn n1123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3047);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4927);
					match s {
						[] => true,
						['l', 'e', 'l', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', 'i', 'p', 'e', 'd', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6213);
									s.is_empty()
								}
								['o', 'g', 'r', 'a', 'm', 'm', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4928);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4929);
					match s {
						[] => true,
						['i', 't', 'a', 's', 't', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3048);
			match s {
				[] => true,
				['l', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4931);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'c', 'i', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3049);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3050);
			match s {
				[] => true,
				['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3051);
			match s {
				[] => true,
				['n', 'y', s @ ..] => {
					super::push(out, 4934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3052);
			s.is_empty()
		}
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sap.
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1105);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3054);
			match s {
				[] => true,
				['h', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4935);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3055);
			match s {
				[] => true,
				['a', 'p', 'o', 'r', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3056);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 4938);
					s.is_empty()
				}
				['e', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4939);
					s.is_empty()
				}
				['i', 'n', s @ ..] => {
					super::push(out, 4940);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						['p', 'h', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tap.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1106);
	match s {
		[] => true,
		['r', 'o', s @ ..] => {
			super::push(out, 3057);
			match s {
				[] => true,
				['c', 'i', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4942);
					s.is_empty()
				}
				['n', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uap.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1107);
	s.is_empty()
}

/// Words ending -vap.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1108);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3058);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4944);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 4945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xap.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1109);
	match s {
		[] => true,
		['i', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ep.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3062);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3063);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3064);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1111);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3065);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5896);
									s.is_empty()
								}
								['o', 'g', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5897);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4948);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5898);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6215);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 6216);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5899);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 5900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5901);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4949);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5903);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3067);
					s.is_empty()
				}
				['u', 'c', s @ ..] => {
					super::push(out, 3068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1112);
			s.is_empty()
		}
		['i', 'o', 'r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1113);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3069);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 3070);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3071);
					match s {
						[] => true,
						['c', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3072);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3073);
					s.is_empty()
				}
				['v', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3076);
					match s {
						[] => true,
						['l', 'o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 4953);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3077);
					match s {
						[] => true,
						['n', 's', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3078);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3079);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4956);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3080);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4958);
							match s {
								[] => true,
								['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4959);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5906);
									s.is_empty()
								}
								['r', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 5907);
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
						['m', 'b', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4960);
							s.is_empty()
						}
						['r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1116);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1117);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4962);
							s.is_empty()
						}
						['m', 'b', 'u', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4964);
							match s {
								[] => true,
								['l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4966);
							s.is_empty()
						}
						['m', 'p', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3085);
					match s {
						[] => true,
						['i', 'n', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4970);
							s.is_empty()
						}
						['u', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 3088);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4972);
							s.is_empty()
						}
						['o', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3090);
					s.is_empty()
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1118);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3092);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3093);
					match s {
						[] => true,
						['l', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'i', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3094);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3095);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hp.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3096);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['a', 'n', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4975);
							s.is_empty()
						}
						['i', 's', 'a', 'e', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4976);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3098);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4978);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5909);
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
		['e', 'l', 'e', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1121);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3099);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3100);
					match s {
						[] => true,
						['o', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1123);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['c', 'a', 'e', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3101);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['n', 'i', 'c', 'o', 'p', 't', 'e', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				['s', 'p', 'h', 'o', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3104);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3105);
					match s {
						[] => true,
						['o', 'm', 'a', 'c', 'h', 'i', 'n', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1125);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1126);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['c', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3106);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4982);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4984);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4985);
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

/// Words ending -ip.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['e', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3109);
					s.is_empty()
				}
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3110);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['t', 'a', 't', 'i', 'c', 'u', 'l', 't', 'r', 'i', 'x', '\u{1}', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3114);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4986);
							s.is_empty()
						}
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4987);
							s.is_empty()
						}
						['s', 't', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1132);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4989);
							s.is_empty()
						}
						['s', 't', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['a', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', 'e', 'r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				['s', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'i', s @ ..] => {
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3128);
					match s {
						[] => true,
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1134);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				['s', 'i', 'n', 't', 'u', 'n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 3133);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5910);
									s.is_empty()
								}
								['i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5911);
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
		['t', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				['u', 'i', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lp.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3136);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3138);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4994);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5912);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4995);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6217);
											s.is_empty()
										}
										['u', 'l', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4996);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4997);
							match s {
								[] => true,
								['s', 'p', 'h', 'a', 'e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5914);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4998);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4999);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3140);
					match s {
						[] => true,
						['o', 'n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['b', 's', '\u{1}', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['x', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1137);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1138);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				['x', 'e', 'n', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3149);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5917);
									s.is_empty()
								}
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 5001);
							s.is_empty()
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3150);
					match s {
						[] => true,
						['a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'q', 'u', 'a', 'm', 'p', 'e', 'r', 'f', 'e', 'c', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				['t', 'o', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -np.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['i', 'g', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -op.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1143);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				['s', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3158);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5004);
							match s {
								[] => true,
								['s', 't', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5919);
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
		['g', 'o', 'n', 'i', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1144);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['a', 'r', 'i', 'z', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3159);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3160);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3161);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5006);
							s.is_empty()
						}
						['m', 'e', 'n', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3162);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							super::push(out, 5008);
							match s {
								[] => true,
								['i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5920);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3164);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5009);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5921);
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
		['m', s @ ..] => {
			super::push(out, 1146);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3165);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3166);
					match s {
						[] => true,
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1147);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['u', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5011);
							s.is_empty()
						}
						['e', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3173);
					match s {
						[] => true,
						['a', 't', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3176);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5014);
							match s {
								[] => true,
								['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5015);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1151);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', 'o', 'g', 'e', 't', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5017);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5019);
							s.is_empty()
						}
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5020);
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

/// Words ending -rp.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1152);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3178);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5021);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5022);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['l', 'e', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
											super::push(out, 6219);
											s.is_empty()
										}
										['v', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
											super::push(out, 6220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5023);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5024);
							match s {
								[] => true,
								['u', 'd', 'i', 'c', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5025);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5026);
							match s {
								[] => true,
								['r', 'd', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5027);
							match s {
								[] => true,
								['o', 'g', 'a', 't', 'i', 'v', 'a', 't', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5028);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5029);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5926);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5927);
									match s {
										[] => true,
										['r', 'i', 'o', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6221);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5030);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5928);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3179);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3180);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5031);
							s.is_empty()
						}
						['s', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3181);
					match s {
						[] => true,
						['i', 'n', 'c', 'o', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5033);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3182);
					s.is_empty()
				}
				['x', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', 'i', 'e', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5930);
									s.is_empty()
								}
								['t', 'i', 'v', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3185);
					s.is_empty()
				}
				['n', 's', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3186);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3187);
					s.is_empty()
				}
				['v', 'a', 'r', 'i', 'c', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1154);
			match s {
				[] => true,
				['a', 'p', 'i', 's', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3189);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3190);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5035);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3191);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3192);
					match s {
						[] => true,
						['r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3193);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n1177(s, out),
		['u', s @ ..] => {
			super::push(out, 1156);
			match s {
				[] => true,
				['i', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3213);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5075);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1157);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sp.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1158);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['p', 'h', 'i', 's', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3215);
					s.is_empty()
				}
				['t', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3216);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1160);
			match s {
				[] => true,
				['l', 'o', 't', 'h', 'r', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1161);
			match s {
				[] => true,
				['l', 'e', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 3219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1162);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3220);
					s.is_empty()
				}
				['l', 'l', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tp.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['o', 'c', 'h', 'o', 't', 'r', 'o', 'p', 'h', 'e', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1163);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -up.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1165);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1166);
			match s {
				[] => true,
				['i', 'c', 'i', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1167);
			match s {
				[] => true,
				['r', 'i', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5077);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1168);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3225);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5079);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									super::push(out, 5970);
									match s {
										[] => true,
										['r', 'i', s @ ..] => {
											super::push(out, 6229);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 6279);
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													super::push(out, 6280);
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
						['o', '\u{1}', s @ ..] => {
							super::push(out, 5080);
							s.is_empty()
						}
						['u', 'n', 'c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5971);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5972);
									match s {
										[] => true,
										['r', 'i', 'x', '\u{1}', s @ ..] => {
											super::push(out, 6230);
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
		['l', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3227);
					match s {
						[] => true,
						['n', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1170);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1171);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1173);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3231);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				['p', 'u', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5084);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 5085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1175);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3237);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3238);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3239);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yp.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1176);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1177);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1178);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1179);
			match s {
				[] => true,
				['a', 'm', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3240);
					s.is_empty()
				}
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['h', 'o', 'n', 'i', 's', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3242);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word suiragirdauq.
pub(crate) fn n6566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6281);
	s.is_empty()
}

/// Words ending -er.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				['d', 'u', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3271);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5104);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1197);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3272);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3273);
					match s {
						[] => true,
						['n', 's', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5106);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3274);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3275);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3276);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3277);
					match s {
						[] => true,
						['u', 'd', 'e', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3278);
					match s {
						[] => true,
						['o', 'r', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3279);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5111);
							s.is_empty()
						}
						['s', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3280);
					match s {
						[] => true,
						['c', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3281);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3282);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3284);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3285);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3286);
					match s {
						[] => true,
						['c', 'i', 'l', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5114);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3287);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3288);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5117);
							s.is_empty()
						}
						['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3289);
					match s {
						[] => true,
						['l', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', 'e', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3290);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3291);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5120);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5121);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5122);
							match s {
								[] => true,
								['u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3292);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3293);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3294);
					s.is_empty()
				}
				['y', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1202);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3296);
					match s {
						[] => true,
						['t', 'i', 'v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3297);
					match s {
						[] => true,
						['g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5124);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 5125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3298);
					s.is_empty()
				}
				['u', 'c', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1203);
			match s {
				[] => true,
				['a', 'n', 's', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3300);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3301);
					match s {
						[] => true,
						['x', '\u{1}', s @ ..] => {
							super::push(out, 5126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3302);
					match s {
						[] => true,
						['g', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5127);
							s.is_empty()
						}
						['n', 'i', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3303);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3304);
					match s {
						[] => true,
						['n', 'e', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5129);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['a', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3305);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3306);
					match s {
						[] => true,
						['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5132);
							s.is_empty()
						}
						['v', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3308);
					match s {
						[] => true,
						['n', 'c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1205);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3309);
					match s {
						[] => true,
						['g', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5135);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3310);
					match s {
						[] => true,
						['n', 's', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5137);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3311);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3312);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5139);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5140);
							match s {
								[] => true,
								['b', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5978);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 5979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3314);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3315);
					match s {
						[] => true,
						['r', 'g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1206);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1207);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3316);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3317);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5142);
							s.is_empty()
						}
						['p', 'i', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n3430(s, out),
				['p', s @ ..] => super::t1::n3431(s, out),
				['t', s @ ..] => super::t1::n3432(s, out),
				['u', s @ ..] => {
					super::push(out, 3321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n1230(s, out),
		['u', s @ ..] => super::t1::n1231(s, out),
		['v', s @ ..] => super::t1::n1232(s, out),
		_ => false,
	}
}

/// Words ending -sor.
pub(crate) fn n1253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1231);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3340);
			match s {
				[] => true,
				['l', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5161);
					s.is_empty()
				}
				['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'r', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3341);
			s.is_empty()
		}
		['t', 'r', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3342);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tor.
pub(crate) fn n1254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1232);
	match s {
		[] => true,
		['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3343);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1233);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3345);
					match s {
						[] => true,
						['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5164);
							s.is_empty()
						}
						['g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 't', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1234);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1235);
			match s {
				[] => true,
				['e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 3347);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5166);
							s.is_empty()
						}
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1236);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1238);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3349);
					s.is_empty()
				}
				['e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 3350);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3351);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3352);
					match s {
						[] => true,
						['o', 't', 'i', 'n', 'e', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3353);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1239);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3355);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3356);
					match s {
						[] => true,
						['c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'i', 'c', 'o', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1240);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3358);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3359);
					s.is_empty()
				}
				['s', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3360);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1242);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5171);
							s.is_empty()
						}
						['b', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3363);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cs.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1255);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3390);
					s.is_empty()
				}
				['c', 'h', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3391);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3392);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5993);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', s @ ..] => {
					super::push(out, 3393);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5196);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', 'o', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1256);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3395);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3396);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3397);
					match s {
						[] => true,
						['t', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1257);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5200);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5201);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6235);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 6236);
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
			super::push(out, 1258);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3398);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3399);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3400);
					s.is_empty()
				}
				['p', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3401);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3402);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5202);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3403);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5204);
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1259);
			match s {
				[] => true,
				['e', 'r', 'o', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1260);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3405);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3406);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3407);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5207);
							s.is_empty()
						}
						['i', 'l', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5208);
							s.is_empty()
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5210);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5211);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3408);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3409);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3410);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3411);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5214);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3412);
					match s {
						[] => true,
						['f', 'u', 'l', 'a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3413);
					match s {
						[] => true,
						['p', 'u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5997);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3414);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3415);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3416);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5218);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'l', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5219);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5220);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -es.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1264);
			match s {
				[] => true,
				['a', 'c', 'e', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['a', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3418);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3419);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['p', 'i', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6000);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3420);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3421);
					match s {
						[] => true,
						['e', 't', 'a', s @ ..] => {
							super::push(out, 5222);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3422);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3423);
					match s {
						[] => true,
						['b', 'i', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5223);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5224);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5225);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5226);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6237);
											s.is_empty()
										}
										['t', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 6238);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5227);
							match s {
								[] => true,
								['u', 'l', 'e', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6003);
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
			super::push(out, 1266);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3424);
					match s {
						[] => true,
						['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3425);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5229);
							s.is_empty()
						}
						['t', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3426);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5231);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 5232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'e', 'l', 'd', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1267);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3427);
					s.is_empty()
				}
				['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3428);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1269);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3430);
					match s {
						[] => true,
						['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5233);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3431);
					match s {
						[] => true,
						['b', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5235);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3433);
					match s {
						[] => true,
						['l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5237);
							s.is_empty()
						}
						['n', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3434);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5239);
							match s {
								[] => true,
								['i', 'n', s @ ..] => {
									super::push(out, 6004);
									match s {
										[] => true,
										['t', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6239);
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
				['p', 'i', 't', 'e', 'r', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3435);
					s.is_empty()
				}
				['u', 'n', 'c', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1271);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3437);
					match s {
						[] => true,
						['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5240);
							s.is_empty()
						}
						['p', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5241);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 6005);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6006);
									match s {
										[] => true,
										['s', 'c', 'o', 'n', 's', 'u', 'l', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6240);
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
					super::push(out, 3438);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3439);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 5242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3440);
					match s {
						[] => true,
						['a', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3441);
					match s {
						[] => true,
						['i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3442);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 6007);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3443);
					s.is_empty()
				}
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3444);
					s.is_empty()
				}
				['l', 'a', 's', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3445);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3446);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3447);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5245);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5246);
							match s {
								[] => true,
								['c', 'a', 'e', 'm', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5247);
							match s {
								[] => true,
								['a', 'g', 'e', 'n', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5248);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['e', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3449);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3450);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5250);
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5251);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => super::t1::n5464(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n3571(s, out),
				['p', s @ ..] => super::t1::n3572(s, out),
				['r', s @ ..] => super::t1::n3573(s, out),
				['v', s @ ..] => super::t1::n3574(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n1298(s, out),
		['t', s @ ..] => super::t1::n1299(s, out),
		['v', s @ ..] => super::t1::n1300(s, out),
		['x', s @ ..] => super::t1::n1301(s, out),
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['a', 'g', 'o', 'n', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 1279);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5254);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1281);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5256);
							s.is_empty()
						}
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3465);
					s.is_empty()
				}
				['e', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3466);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3467);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6011);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6241);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1282);
			match s {
				[] => true,
				['e', 'r', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1283);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5258);
							match s {
								[] => true,
								['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1285);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3471);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5260);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5261);
							s.is_empty()
						}
						['o', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5262);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3472);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1286);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3474);
					match s {
						[] => true,
						['p', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3475);
					match s {
						[] => true,
						['e', 'r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3476);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3477);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3478);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 5266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1287);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1288);
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3479);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5267);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3480);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 't', 'r', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1289);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3482);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3483);
					match s {
						[] => true,
						['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5269);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 6014);
									s.is_empty()
								}
								['r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 6015);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -os.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['b', 'r', 'i', s @ ..] => {
			match s {
				[] => true,
				['e', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3486);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3488);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3489);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5270);
							match s {
								[] => true,
								['l', 'i', s @ ..] => {
									match s {
										[] => true,
										['s', 'm', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6242);
											s.is_empty()
										}
										['t', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 6243);
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
					super::push(out, 3490);
					match s {
						[] => true,
						['r', 'd', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3492);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5272);
							s.is_empty()
						}
						['g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5273);
							s.is_empty()
						}
						['m', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6016);
									s.is_empty()
								}
								['t', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5274);
							s.is_empty()
						}
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5275);
							s.is_empty()
						}
						['t', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3493);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5277);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5278);
							s.is_empty()
						}
						['n', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3494);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5280);
							match s {
								[] => true,
								['a', 'r', 'i', 'e', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 6018);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5281);
							s.is_empty()
						}
						['t', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5282);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5283);
							s.is_empty()
						}
						['v', 'e', 'r', 't', 'i', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3495);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3496);
					s.is_empty()
				}
				['s', 't', 'i', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3497);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1298);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1299);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3499);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3500);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3501);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 5285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3503);
					match s {
						[] => true,
						['r', 'o', 'n', 'i', 's', 't', 'e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1301);
			match s {
				[] => true,
				['a', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3505);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3506);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3507);
					s.is_empty()
				}
				['e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 3508);
					s.is_empty()
				}
				['i', 't', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3509);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3510);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6020);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3511);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'u', 's', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ps.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1303);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3513);
					match s {
						[] => true,
						['i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 5287);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5288);
							match s {
								[] => true,
								['n', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3514);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1304);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3516);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3517);
					match s {
						[] => true,
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3518);
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					super::push(out, 3519);
					match s {
						[] => true,
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'r', 'a', 'e', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1306);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3521);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5291);
							match s {
								[] => true,
								['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3522);
					match s {
						[] => true,
						['e', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5292);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5293);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3523);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3524);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6024);
									s.is_empty()
								}
								['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6025);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6026);
									match s {
										[] => true,
										['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6244);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'i', 't', 'u', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3525);
					s.is_empty()
				}
				['t', 'h', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1307);
			match s {
				[] => true,
				['e', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3528);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3529);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3530);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5296);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5297);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5298);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'n', 'e', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1309);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['m', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3532);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3533);
					s.is_empty()
				}
				['t', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3534);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iuqs.
pub(crate) fn n3659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3535);
	s.is_empty()
}

/// Words ending -ts.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3536);
					match s {
						[] => true,
						['n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3537);
					s.is_empty()
				}
				['t', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3539);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5304);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3540);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5306);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 6245);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6282);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 6246);
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
			super::push(out, 1313);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3541);
					s.is_empty()
				}
				['l', 'l', s @ ..] => {
					super::push(out, 3542);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3543);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3544);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5308);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5309);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5310);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5312);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1314);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3546);
					match s {
						[] => true,
						['c', 'h', 'e', 'i', 'o', 'l', 'o', 'g', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3548);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5315);
							match s {
								[] => true,
								['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3549);
					match s {
						[] => true,
						['n', 'u', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3550);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3551);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3552);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5318);
							s.is_empty()
						}
						['t', 'h', s @ ..] => {
							super::push(out, 5319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1316);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3553);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5320);
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

/// Words ending -us.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['e', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3554);
					s.is_empty()
				}
				['l', 'i', 't', 'e', 'r', 'n', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3555);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3556);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1319);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3558);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5321);
							s.is_empty()
						}
						['l', 'a', 'r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3559);
					match s {
						[] => true,
						['o', 'n', 's', 'c', 'i', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3560);
					match s {
						[] => true,
						['i', 'v', 'i', 's', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5324);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3561);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5326);
							s.is_empty()
						}
						['x', '\u{1}', s @ ..] => {
							super::push(out, 5327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3562);
					match s {
						[] => true,
						['l', 'e', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3563);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 6030);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6031);
									match s {
										[] => true,
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6247);
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
					super::push(out, 3564);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3565);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5329);
							match s {
								[] => true,
								['l', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3566);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5330);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3567);
					match s {
						[] => true,
						['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5332);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5333);
							match s {
								[] => true,
								['c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1320);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3569);
					s.is_empty()
				}
				['i', 'n', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3570);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3571);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1321);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3573);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1322);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3575);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3576);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5337);
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5338);
							match s {
								[] => true,
								['a', 't', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5339);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5340);
							match s {
								[] => true,
								['s', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1324);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['c', 'i', 'd', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1326);
			match s {
				[] => true,
				['t', 'a', 'n', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3580);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3581);
					s.is_empty()
				}
				['p', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1328);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3583);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'o', 'g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6037);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6038);
									match s {
										[] => true,
										['e', 'm', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
											super::push(out, 6248);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 6249);
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
				['p', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5341);
							match s {
								[] => true,
								['l', 'a', 't', 'r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 6039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5342);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6040);
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5343);
							match s {
								[] => true,
								['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 's', 'c', 'r', 'i', 'p', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1329);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3586);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3587);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3589);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5344);
							s.is_empty()
						}
						['i', 'r', s @ ..] => {
							super::push(out, 5345);
							match s {
								[] => true,
								['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3590);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5346);
							s.is_empty()
						}
						['i', 'n', 'a', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1331);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ys.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1332);
			match s {
				[] => true,
				['o', 'm', 'o', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1333);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1334);
			match s {
				[] => true,
				['a', 'e', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3593);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1335);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['p', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3596);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5349);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['i', 'n', s @ ..] => {
					super::push(out, 3597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'y', 'g', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1337);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -canet.
pub(crate) fn n5586(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 6049);
			s.is_empty()
		}
		['u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 6050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enet.
pub(crate) fn n3749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3622);
	match s {
		[] => true,
		['s', 'm', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 5367);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -snet.
pub(crate) fn n3750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3623);
	s.is_empty()
}

/// Words ending -tnet.
pub(crate) fn n3751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3624);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6051);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6052);
					match s {
						[] => true,
						['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6252);
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

/// Words ending -pet.
pub(crate) fn n1379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['a', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ret.
pub(crate) fn n1380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 3626);
			match s {
				[] => true,
				['e', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5368);
					s.is_empty()
				}
				['u', 'n', 'c', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'n', 'c', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -set.
pub(crate) fn n1381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1356);
	match s {
		[] => true,
		['t', 'a', s @ ..] => {
			super::push(out, 3628);
			match s {
				[] => true,
				['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5370);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tet.
pub(crate) fn n1382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1357);
	match s {
		[] => true,
		['h', 'e', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3629);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3630);
			match s {
				[] => true,
				['a', 'p', 'h', 'a', 'r', 'm', 'a', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5372);
					s.is_empty()
				}
				['i', 't', 'u', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'i', 'g', 'o', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xet.
pub(crate) fn n1383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1358);
	s.is_empty()
}

/// Words ending -ht.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1359);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3633);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3634);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5375);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5376);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6053);
									match s {
										[] => true,
										['m', 'a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6283);
													s.is_empty()
												}
												['t', 'i', 'o', 'n', '\u{1}', s @ ..] => {
													super::push(out, 6284);
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
							super::push(out, 5377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3635);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5378);
							match s {
								[] => true,
								['a', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3636);
					match s {
						[] => true,
						['a', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3637);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3638);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5381);
							match s {
								[] => true,
								['c', 'i', 'c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6055);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'n', 'g', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5382);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['a', 's', 'c', 'i', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3639);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3640);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 5384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['l', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3641);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3642);
					s.is_empty()
				}
				['r', 'i', 'b', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3643);
					s.is_empty()
				}
				['s', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['m', 'i', s @ ..] => {
					super::push(out, 3645);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5385);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6253);
											s.is_empty()
										}
										['t', 'e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6254);
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

/// Words ending -it.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['i', 'c', 'i', s @ ..] => {
					super::push(out, 3646);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6056);
									s.is_empty()
								}
								['i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6057);
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
		['g', s @ ..] => {
			super::push(out, 1367);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 3647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3648);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3649);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3650);
					match s {
						[] => true,
						['i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3651);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1370);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1371);
			match s {
				[] => true,
				['o', 'c', 'i', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1372);
			match s {
				[] => true,
				['a', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3654);
					s.is_empty()
				}
				['h', 'y', 'm', 'a', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3655);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3656);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1373);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sisemt.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	s.is_empty()
}

/// Words ending -ot.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['f', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1374);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1375);
			match s {
				[] => true,
				['a', 't', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', s @ ..] => {
			super::push(out, 1376);
			match s {
				[] => true,
				['r', 'a', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1377);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3660);
					s.is_empty()
				}
				['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1378);
			match s {
				[] => true,
				['e', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3662);
					s.is_empty()
				}
				['g', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3663);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3664);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3665);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3666);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1379);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 3668);
					match s {
						[] => true,
						['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1382);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3669);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3670);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5389);
							match s {
								[] => true,
								['e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6058);
									s.is_empty()
								}
								['i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5390);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 6255);
											s.is_empty()
										}
										['t', 'o', s @ ..] => {
											super::push(out, 6256);
											match s {
												[] => true,
												['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
													super::push(out, 6285);
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
				['d', s @ ..] => {
					super::push(out, 3671);
					match s {
						[] => true,
						['i', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5391);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3672);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5393);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5394);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5395);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5396);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 6060);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3673);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3674);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5397);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3675);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 5399);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				['u', 'm', 'a', 't', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1383);
			match s {
				[] => true,
				['d', 'e', 'c', 'e', 'n', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3678);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3679);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3680);
					match s {
						[] => true,
						['p', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5401);
							s.is_empty()
						}
						['s', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5402);
							s.is_empty()
						}
						['v', 'i', 'r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1384);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3681);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3682);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5404);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 6062);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6286);
													s.is_empty()
												}
												['t', 'i', 'o', '\u{1}', s @ ..] => {
													super::push(out, 6287);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6257);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6063);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 6288);
													s.is_empty()
												}
												['t', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 6289);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'c', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3683);
					match s {
						[] => true,
						['a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5405);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5406);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5407);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5408);
							match s {
								[] => true,
								['i', 'n', 'i', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6259);
											s.is_empty()
										}
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6260);
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
						['e', s @ ..] => {
							super::push(out, 5409);
							s.is_empty()
						}
						['u', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3684);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5411);
							s.is_empty()
						}
						['r', 'a', 'r', 'c', 'h', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3686);
					match s {
						[] => true,
						['o', 'n', 'o', 'm', 'e', 't', 'r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3687);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3688);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3689);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5414);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6065);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6066);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t1::n3820(s, out),
				['u', s @ ..] => {
					super::push(out, 3691);
					s.is_empty()
				}
				['v', s @ ..] => super::t1::n3822(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n1410(s, out),
		['u', s @ ..] => super::t1::n1411(s, out),
		['y', s @ ..] => super::t1::n1412(s, out),
		_ => false,
	}
}

/// Words ending -ut.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1388);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3697);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3698);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1389);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1390);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1391);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1392);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['b', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5421);
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
				['c', s @ ..] => {
					super::push(out, 3701);
					match s {
						[] => true,
						['u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1393);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3703);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3704);
					match s {
						[] => true,
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yt.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3706);
					match s {
						[] => true,
						['n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3707);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3709);
					s.is_empty()
				}
				['o', 't', 'a', 'r', 'i', 'c', 'h', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 3710);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rev.
pub(crate) fn n1473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1444);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3769);
			match s {
				[] => true,
				['x', '\u{1}', s @ ..] => {
					super::push(out, 5452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3770);
			match s {
				[] => true,
				['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3771);
			match s {
				[] => true,
				['c', 'u', 'n', 'd', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5454);
					s.is_empty()
				}
				['d', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5455);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3772);
			match s {
				[] => true,
				['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3773);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3774);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3775);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3776);
			match s {
				[] => true,
				['i', 'g', s @ ..] => {
					super::push(out, 5458);
					match s {
						[] => true,
						['i', 'n', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3777);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 5459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3778);
			match s {
				[] => true,
				['e', 'c', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5460);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sev.
pub(crate) fn n1474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1445);
	match s {
		[] => true,
		['a', 'n', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3779);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3780);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6074);
							s.is_empty()
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3781);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3782);
			match s {
				[] => true,
				['i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tev.
pub(crate) fn n1475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1446);
	match s {
		[] => true,
		['e', 'r', 'i', s @ ..] => {
			super::push(out, 3783);
			match s {
				[] => true,
				['n', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xev.
pub(crate) fn n1476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1447);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3784);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3785);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iv.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3786);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1449);
			match s {
				[] => true,
				['e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 3788);
					s.is_empty()
				}
				['i', 'x', '\u{1}', s @ ..] => {
					super::push(out, 3789);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3790);
					match s {
						[] => true,
						['a', 't', 'i', s @ ..] => {
							super::push(out, 5463);
							match s {
								[] => true,
								['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'r', 'n', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3792);
					match s {
						[] => true,
						['n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5464);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3793);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5466);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3794);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5468);
							match s {
								[] => true,
								['i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 6077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3795);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5470);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5471);
							match s {
								[] => true,
								['r', 'i', 'a', s @ ..] => {
									match s {
										[] => true,
										['l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 6261);
											s.is_empty()
										}
										['t', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6262);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5472);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5473);
							match s {
								[] => true,
								['m', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6078);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['u', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1452);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5474);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5475);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3799);
					s.is_empty()
				}
				['l', 'u', s @ ..] => {
					super::push(out, 3800);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1454);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1455);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3801);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6079);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3802);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5479);
							s.is_empty()
						}
						['l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5480);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5481);
							s.is_empty()
						}
						['u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3803);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5483);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 6081);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 6290);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n6583(s, out),
														['t', s @ ..] => super::t1::n6584(s, out),
														_ => false,
													}
												}
												['o', 'l', 'a', '\u{1}', s @ ..] => {
													super::push(out, 6291);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['x', '\u{1}', s @ ..] => {
									super::push(out, 6082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3804);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3805);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3806);
					match s {
						[] => true,
						['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1456);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1457);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3808);
					s.is_empty()
				}
				['i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3810);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3811);
					match s {
						[] => true,
						['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3813);
					s.is_empty()
				}
				['c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3814);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3815);
					match s {
						[] => true,
						['g', 'o', 't', 'h', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3816);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3817);
					match s {
						[] => true,
						['a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5488);
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1460);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3818);
					match s {
						[] => true,
						['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n3954(s, out),
				['i', s @ ..] => super::t1::n3955(s, out),
				['o', 'r', '\u{1}', s @ ..] => super::t1::n3956(s, out),
				['r', 'i', s @ ..] => super::t1::n3957(s, out),
				['t', s @ ..] => super::t1::n3958(s, out),
				['u', s @ ..] => super::t1::n3959(s, out),
				_ => false,
			}
		}
		['v', s @ ..] => super::t1::n1490(s, out),
		_ => false,
	}
}

/// Words ending -ov.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3826);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5500);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5501);
							match s {
								[] => true,
								['i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 6084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1463);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1464);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3828);
					match s {
						[] => true,
						['t', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3829);
					match s {
						[] => true,
						['a', 'r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3830);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3831);
					match s {
						[] => true,
						['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5504);
							s.is_empty()
						}
						['c', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5505);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5506);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1465);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3832);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3833);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5509);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 6085);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6086);
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
		['t', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uv.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['l', 'g', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5510);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5511);
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

/// Words ending -amoc.
pub(crate) fn n2021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1972);
	match s {
		[] => true,
		['c', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bmoc.
pub(crate) fn n2022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1973);
	match s {
		[] => true,
		['i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 4181);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -emoc.
pub(crate) fn n2023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1974);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5597);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5598);
					match s {
						[] => true,
						['i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -imoc.
pub(crate) fn n2024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1975);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 4183);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4184);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5599);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5600);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mmoc.
pub(crate) fn n2025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1976);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4185);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4186);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5601);
					s.is_empty()
				}
				['t', 'a', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4187);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5603);
					match s {
						[] => true,
						['i', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4188);
			match s {
				[] => true,
				['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4189);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 6131);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 6267);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6268);
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

/// Words ending -omoc.
pub(crate) fn n2026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1977);
	s.is_empty()
}

/// Words ending -pmoc.
pub(crate) fn n2027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1978);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4190);
			match s {
				[] => true,
				['r', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4191);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5607);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6132);
							s.is_empty()
						}
						['s', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4192);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5608);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4193);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6134);
							s.is_empty()
						}
						['t', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6135);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 6136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4194);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5610);
					s.is_empty()
				}
				['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4195);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5612);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5613);
					match s {
						[] => true,
						['m', 'i', 's', 's', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4196);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4197);
			match s {
				[] => true,
				['n', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5614);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -noc.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1979);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4198);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 5616);
							match s {
								[] => true,
								['t', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4199);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 5617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4200);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5618);
							s.is_empty()
						}
						['u', 's', 'i', 'u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4201);
					match s {
						[] => true,
						['p', 'i', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1981);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4202);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5621);
							s.is_empty()
						}
						['n', 's', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5622);
							s.is_empty()
						}
						['s', 'c', 'e', 'n', 's', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4203);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5624);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4204);
					match s {
						[] => true,
						['n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1982);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 4206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1983);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4207);
					match s {
						[] => true,
						['b', 'u', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4208);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4209);
					match s {
						[] => true,
						['d', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5628);
							s.is_empty()
						}
						['r', 'm', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4210);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5630);
							match s {
								[] => true,
								['t', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5631);
							match s {
								[] => true,
								['c', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6140);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4211);
					match s {
						[] => true,
						['e', 'd', 'e', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5632);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4212);
					match s {
						[] => true,
						['a', 't', 'e', 'r', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4213);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5635);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1984);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4214);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4215);
					match s {
						[] => true,
						['u', 't', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4216);
					match s {
						[] => true,
						['a', 't', 'u', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5638);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1985);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4217);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4218);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5640);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 5641);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5642);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6141);
									s.is_empty()
								}
								['x', '\u{1}', s @ ..] => {
									super::push(out, 6142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 't', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5643);
							s.is_empty()
						}
						['x', '\u{1}', s @ ..] => {
							super::push(out, 5644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1986);
			match s {
				[] => true,
				['i', 'b', 'e', 'r', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4220);
					s.is_empty()
				}
				['u', 'c', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1987);
			s.is_empty()
		}
		['o', 'p', 'e', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1988);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1989);
			match s {
				[] => true,
				['u', 'n', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1990);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4223);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4224);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4225);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5645);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5646);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5647);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4226);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5649);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5650);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6143);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4227);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4228);
					match s {
						[] => true,
						['e', 'l', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4229);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5653);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5654);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 6145);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'd', 'e', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1992);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4231);
					match s {
						[] => true,
						['m', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => super::t1::n5898(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n4384(s, out),
				['i', s @ ..] => super::t1::n4385(s, out),
				['r', s @ ..] => super::t1::n4386(s, out),
				['u', s @ ..] => super::t1::n4387(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n2042(s, out),
		['v', s @ ..] => super::t2::n2043(s, out),
		_ => false,
	}
}

/// Words ending -ooc.
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1995);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -poc.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['a', 'd', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1997);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1998);
			s.is_empty()
		}
		['h', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1999);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4241);
					s.is_empty()
				}
				['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4242);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2000);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2001);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -qoc.
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 478);
	s.is_empty()
}

/// Words ending -roc.
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 479);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2002);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2003);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2004);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2005);
			match s {
				[] => true,
				['u', 'c', 'o', 'p', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['l', 'l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5666);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6153);
									s.is_empty()
								}
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6154);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2007);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4245);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4246);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5670);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 2009);
			s.is_empty()
		}
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2010);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2011);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4247);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4248);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4249);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -soc.
pub(crate) fn n491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 480);
	match s {
		[] => true,
		['m', 'o', 'g', s @ ..] => {
			super::push(out, 2012);
			match s {
				[] => true,
				['r', 'a', 'p', 'h', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4251);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -toc.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 481);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 4252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2015);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4254);
					s.is_empty()
				}
				['r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2016);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -voc.
pub(crate) fn n493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 482);
	match s {
		[] => true,
		['i', 'n', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4257);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4258);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xoc.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 483);
	s.is_empty()
}

/// The word nohcitsid.
pub(crate) fn n5966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5720);
	s.is_empty()
}

/// Words ending -rtsid.
pub(crate) fn n4535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4372);
	s.is_empty()
}

/// Words ending -tid.
pub(crate) fn n558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uid.
pub(crate) fn n559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 548);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 2171);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2172);
			s.is_empty()
		}
		['t', 'u', 'r', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2173);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vid.
pub(crate) fn n560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 549);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2174);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4373);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4374);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5721);
							s.is_empty()
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -refni.
pub(crate) fn n4803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4627);
	s.is_empty()
}

/// Words ending -ifni.
pub(crate) fn n2655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2577);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 4628);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4629);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lfni.
pub(crate) fn n2656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2578);
	s.is_empty()
}

/// Words ending -ofni.
pub(crate) fn n2657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2579);
	match s {
		[] => true,
		['r', 'm', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 4630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rfni.
pub(crate) fn n2658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2580);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4631);
			s.is_empty()
		}
		['i', 'g', 'i', 'd', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 4632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ufni.
pub(crate) fn n2659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2581);
	s.is_empty()
}

/// Words ending -gni.
pub(crate) fn n838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 823);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2582);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			super::push(out, 2583);
			match s {
				[] => true,
				['t', 'i', 't', 'u', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4633);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hni.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 824);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2584);
			s.is_empty()
		}
		['u', 'm', 'a', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ini.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2586);
			match s {
				[] => true,
				['i', 'c', 'i', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', 'u', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2587);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2588);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2589);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4635);
					match s {
						[] => true,
						['i', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5801);
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

/// Words ending -lni.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 826);
	match s {
		[] => true,
		['u', 'm', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2590);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mni.
pub(crate) fn n842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 827);
	match s {
		[] => true,
		['a', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2591);
			s.is_empty()
		}
		['o', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2592);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4636);
					s.is_empty()
				}
				['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4637);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nni.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 828);
	s.is_empty()
}

/// Words ending -oni.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 829);
	match s {
		[] => true,
		['r', 'd', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pni.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 830);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2594);
			match s {
				[] => true,
				['d', 'i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4638);
					s.is_empty()
				}
				['t', 'i', 'g', 'i', 'n', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 'd', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2595);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2596);
			match s {
				[] => true,
				['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4640);
					s.is_empty()
				}
				['r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4641);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qni.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 831);
	match s {
		[] => true,
		['u', 'i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4642);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4643);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4644);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4645);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rni.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 832);
	match s {
		[] => true,
		['i', 't', s @ ..] => {
			super::push(out, 2597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sni.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 833);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2598);
			match s {
				[] => true,
				['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2599);
			match s {
				[] => true,
				['e', 'n', 's', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4647);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2600);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 5802);
							s.is_empty()
						}
						['u', 'r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2601);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2602);
			match s {
				[] => true,
				['l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4649);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2603);
			match s {
				[] => true,
				['i', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2604);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4652);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4653);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5804);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5805);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4654);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6188);
									s.is_empty()
								}
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6189);
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
			super::push(out, 2605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tni.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 834);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2606);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4655);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5807);
							s.is_empty()
						}
						['t', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4656);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5809);
							match s {
								[] => true,
								['m', 'p', 't', 'r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 6190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5810);
							match s {
								[] => true,
								['e', 'n', 's', 't', 'r', 'u', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6191);
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
			super::push(out, 2607);
			match s {
				[] => true,
				['o', 'd', s @ ..] => {
					super::push(out, 4657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uni.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 835);
	match s {
		[] => true,
		['t', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2608);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vni.
pub(crate) fn n851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 836);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2609);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4658);
					match s {
						[] => true,
						['e', 'c', 'u', 'n', 'd', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2610);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4659);
					s.is_empty()
				}
				['s', 'i', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4660);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4661);
					match s {
						[] => true,
						['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['l', 'u', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -til.
pub(crate) fn n917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 901);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2725);
			s.is_empty()
		}
		['e', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2726);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2727);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2728);
			match s {
				[] => true,
				['c', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4733);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 't', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 2729);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2730);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					super::push(out, 4735);
					match s {
						[] => true,
						['i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4736);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word nemacidem.
pub(crate) fn n6579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6294);
	s.is_empty()
}

/// Words ending -tnemacidem.
pub(crate) fn n6580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6295);
	s.is_empty()
}

/// Words ending -orp.
pub(crate) fn n1177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1155);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3195);
			match s {
				[] => true,
				['g', 'o', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5038);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3196);
			match s {
				[] => true,
				['a', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5041);
					match s {
						[] => true,
						['c', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5932);
							s.is_empty()
						}
						['n', 't', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5042);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5934);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5935);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6222);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', 'c', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5043);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5044);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5045);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 's', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6224);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6225);
									match s {
										[] => true,
										['u', 'l', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6296);
													s.is_empty()
												}
												['a', 't', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 6297);
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
				['r', s @ ..] => {
					super::push(out, 5046);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5937);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5047);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5939);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6226);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3198);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3199);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 3200);
			match s {
				[] => true,
				['a', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5048);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5049);
					match s {
						[] => true,
						['c', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5941);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5050);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5051);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3201);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5053);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5054);
					match s {
						[] => true,
						['o', 's', 't', 'i', 'c', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5944);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3202);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3203);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3204);
			match s {
				[] => true,
				['e', 't', 'a', 'r', 'i', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3205);
			match s {
				[] => true,
				['e', 't', 'h', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5056);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5057);
					s.is_empty()
				}
				['n', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5058);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3206);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5060);
					match s {
						[] => true,
						['p', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5061);
					match s {
						[] => true,
						['e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5062);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5948);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'e', 'm', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3207);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3208);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5063);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5950);
							match s {
								[] => true,
								['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 6228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', 's', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5952);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5064);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5065);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5954);
							s.is_empty()
						}
						['r', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5955);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5957);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 5067);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3209);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5069);
					match s {
						[] => true,
						['p', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5960);
							s.is_empty()
						}
						['e', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5070);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5962);
							match s {
								[] => true,
								['y', 't', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6275);
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => super::t2::n6561(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'm', 'e', 't', 'r', 'u', 'm', '\u{1}', s @ ..] => super::t2::n5271(s, out),
				['o', s @ ..] => super::t2::n5272(s, out),
				['p', 'e', 'r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => super::t2::n5273(s, out),
				['t', s @ ..] => super::t2::n5274(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t2::n3318(s, out),
		['v', s @ ..] => super::t2::n3319(s, out),
		['x', s @ ..] => super::t2::n3320(s, out),
		_ => false,
	}
}

/// Words ending -oser.
pub(crate) fn n3430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3318);
	match s {
		[] => true,
		['l', 'u', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 5144);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pser.
pub(crate) fn n3431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3319);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5146);
			s.is_empty()
		}
		['i', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5147);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5148);
			match s {
				[] => true,
				['n', 's', 'a', 'b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'b', 'l', 'i', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tser.
pub(crate) fn n3432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3320);
	match s {
		[] => true,
		['i', 'n', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n1230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1208);
	match s {
		[] => true,
		['a', 'e', '\u{1}', s @ ..] => {
			super::push(out, 3322);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5151);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3323);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5153);
					match s {
						[] => true,
						['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5154);
					s.is_empty()
				}
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5982);
							s.is_empty()
						}
						['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3324);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5155);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5156);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uer.
pub(crate) fn n1231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1209);
	s.is_empty()
}

/// Words ending -ver.
pub(crate) fn n1232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1210);
	match s {
		[] => true,
		['e', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 3325);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sires.
pub(crate) fn n5464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5252);
	s.is_empty()
}

/// Words ending -mres.
pub(crate) fn n3571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3451);
	match s {
		[] => true,
		['o', 'c', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5253);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pres.
pub(crate) fn n3572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3452);
	s.is_empty()
}

/// Words ending -rres.
pub(crate) fn n3573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3453);
	s.is_empty()
}

/// Words ending -vres.
pub(crate) fn n3574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3454);
	s.is_empty()
}

/// Words ending -ses.
pub(crate) fn n1298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1275);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3455);
			s.is_empty()
		}
		['e', 'l', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 3456);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 3457);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3458);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tes.
pub(crate) fn n1299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1276);
	match s {
		[] => true,
		['u', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ves.
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1277);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3461);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xes.
pub(crate) fn n1301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1278);
	match s {
		[] => true,
		['u', 'a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 3463);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tirt.
pub(crate) fn n3820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3690);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5415);
			s.is_empty()
		}
		['i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5416);
			s.is_empty()
		}
		['o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5417);
			s.is_empty()
		}
		['u', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -virt.
pub(crate) fn n3822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3692);
	s.is_empty()
}

/// Words ending -ort.
pub(crate) fn n1410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1385);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3693);
			s.is_empty()
		}
		['g', 'l', 'o', 'd', 'y', 't', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 3694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urt.
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1386);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3695);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5419);
					match s {
						[] => true,
						['t', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6067);
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

/// Words ending -yrt.
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	match s {
		[] => true,
		['g', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 3696);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word aimedniv.
pub(crate) fn n6583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6298);
	s.is_empty()
}

/// Words ending -taimedniv.
pub(crate) fn n6584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6299);
	s.is_empty()
}

/// Words ending -etiv.
pub(crate) fn n3954(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5491);
			s.is_empty()
		}
		['x', '\u{1}', s @ ..] => {
			super::push(out, 5492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itiv.
pub(crate) fn n3955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3819);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5493);
			match s {
				[] => true,
				['i', 't', 'i', 'g', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 5494);
			s.is_empty()
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5495);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word rotiv.
pub(crate) fn n3956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3820);
	s.is_empty()
}

/// Words ending -irtiv.
pub(crate) fn n3957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3821);
	match s {
		[] => true,
		['o', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttiv.
pub(crate) fn n3958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3822);
	s.is_empty()
}

/// Words ending -utiv.
pub(crate) fn n3959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3823);
	match s {
		[] => true,
		['l', 'a', 'm', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5497);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5498);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 5499);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viv.
pub(crate) fn n1490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1461);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3824);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3825);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word oitanimatnoc.
pub(crate) fn n5898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5656);
	s.is_empty()
}

/// Words ending -etnoc.
pub(crate) fn n4384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4232);
	s.is_empty()
}

/// Words ending -itnoc.
pub(crate) fn n4385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4233);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 5657);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5658);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5659);
			s.is_empty()
		}
		['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtnoc.
pub(crate) fn n4386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4234);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5661);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6147);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6148);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6149);
					s.is_empty()
				}
				['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5662);
			s.is_empty()
		}
		['o', 'v', 'e', 'r', 's', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utnoc.
pub(crate) fn n4387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4235);
	s.is_empty()
}

/// Words ending -unoc.
pub(crate) fn n2042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1993);
	match s {
		[] => true,
		['b', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4236);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 4237);
			s.is_empty()
		}
		_ => false,
	}
}

