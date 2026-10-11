use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => super::t0::n1(s, out),
		['\u{12}', s @ ..] => super::t0::n2(s, out),
		['\u{13}', s @ ..] => super::t0::n3(s, out),
		_ => false,
	}
}

/// Words ending - (1 syllables).
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 168);
							s.is_empty()
						}
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 169);
							s.is_empty()
						}
						['r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'e', '\u{1}', s @ ..] => {
					super::push(out, 54);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 171);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 173);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['c', 'i', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 55);
					s.is_empty()
				}
				['i', 'h', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 175);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'n', '\u{1}', s @ ..] => {
					super::push(out, 56);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 57);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 177);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 178);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 553);
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 179);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 180);
							s.is_empty()
						}
						['o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 181);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 182);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 58);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 185);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 186);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 60);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 556);
									s.is_empty()
								}
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1281);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 1282);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 187);
							s.is_empty()
						}
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 188);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 557);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 61);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 559);
									s.is_empty()
								}
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 189);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 562);
									s.is_empty()
								}
								['l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 564);
									s.is_empty()
								}
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 565);
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
						['m', '\u{1}', s @ ..] => {
							super::push(out, 190);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 566);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1283);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 567);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 191);
							s.is_empty()
						}
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 192);
							s.is_empty()
						}
						['i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 570);
									s.is_empty()
								}
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 572);
									s.is_empty()
								}
								['r', 'h', 's', '\u{1}', s @ ..] => super::t0::n1005(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t0::n321(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n7(s, out),
		['f', s @ ..] => super::t0::n8(s, out),
		['g', s @ ..] => super::t0::n9(s, out),
		['h', s @ ..] => super::t0::n10(s, out),
		['k', s @ ..] => super::t0::n11(s, out),
		['l', s @ ..] => super::t0::n12(s, out),
		['m', s @ ..] => super::t0::n13(s, out),
		['n', s @ ..] => super::t0::n14(s, out),
		['p', s @ ..] => super::t0::n15(s, out),
		['r', s @ ..] => super::t0::n16(s, out),
		['s', s @ ..] => super::t0::n17(s, out),
		['t', s @ ..] => super::t0::n18(s, out),
		['u', '\u{1}', s @ ..] => super::t0::n19(s, out),
		['w', s @ ..] => super::t0::n20(s, out),
		['x', 'a', 'l', '\u{1}', s @ ..] => super::t0::n21(s, out),
		['y', s @ ..] => super::t0::n22(s, out),
		_ => false,
	}
}

/// Words ending - (2 syllables).
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 23);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 325);
							s.is_empty()
						}
						['p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 24);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 94);
					match s {
						[] => true,
						['e', 'r', 'h', s @ ..] => {
							super::push(out, 327);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1326);
											s.is_empty()
										}
										['p', 'u', '\u{1}', s @ ..] => {
											super::push(out, 1327);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 328);
							s.is_empty()
						}
						['s', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 95);
					match s {
						[] => true,
						['g', 'a', s @ ..] => {
							super::push(out, 330);
							s.is_empty()
						}
						['k', 'c', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 331);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 332);
							match s {
								[] => true,
								['m', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 727);
									s.is_empty()
								}
								['r', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 728);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'r', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 96);
					match s {
						[] => true,
						['b', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 334);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1328);
											s.is_empty()
										}
										['l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1329);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'l', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 730);
									s.is_empty()
								}
								['r', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 731);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 335);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 336);
							match s {
								[] => true,
								['a', 'u', 'q', 's', '\u{1}', s @ ..] => {
									super::push(out, 732);
									s.is_empty()
								}
								['l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 733);
									s.is_empty()
								}
								['o', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', 't', '\u{1}', s @ ..] => {
									super::push(out, 735);
									s.is_empty()
								}
								['u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 337);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['o', 'o', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 737);
									s.is_empty()
								}
								['u', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 738);
									s.is_empty()
								}
								['w', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 739);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 740);
									s.is_empty()
								}
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 741);
									s.is_empty()
								}
								['r', 'o', s @ ..] => {
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 1330);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 1331);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 97);
					match s {
						[] => true,
						['a', 'b', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 339);
							s.is_empty()
						}
						['e', 'i', 'f', s @ ..] => {
							match s {
								[] => true,
								['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 742);
									s.is_empty()
								}
								['p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 743);
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
						['i', s @ ..] => {
							super::push(out, 340);
							match s {
								[] => true,
								['k', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 744);
									s.is_empty()
								}
								['w', 'p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'o', 'g', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 98);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 99);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 746);
									s.is_empty()
								}
								['w', 'r', 'o', s @ ..] => {
									super::push(out, 747);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 1332);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n27(s, out),
		['f', s @ ..] => {
			super::push(out, 27);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							super::push(out, 388);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1376);
											s.is_empty()
										}
										['i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 1377);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n30(s, out),
		['i', s @ ..] => {
			super::push(out, 30);
			match s {
				[] => true,
				['n', 'n', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 31);
			match s {
				[] => true,
				['c', 'a', 'l', 'b', 'r', 'o', 'f', '\u{1}', s @ ..] => super::t0::n189(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n33(s, out),
		['m', s @ ..] => super::t0::n34(s, out),
		['n', s @ ..] => super::t0::n35(s, out),
		['o', s @ ..] => super::t0::n36(s, out),
		['r', s @ ..] => super::t0::n37(s, out),
		['s', s @ ..] => super::t0::n38(s, out),
		['t', s @ ..] => super::t0::n39(s, out),
		['w', s @ ..] => super::t0::n40(s, out),
		['y', s @ ..] => super::t0::n41(s, out),
		_ => false,
	}
}

/// Words ending - (3 syllables).
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 41);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 42);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 143);
					match s {
						[] => true,
						['c', 'n', 'a', 'v', s @ ..] => {
							super::push(out, 484);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 485);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'g', 'g', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 486);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 487);
							match s {
								[] => true,
								['s', 'e', 'c', s @ ..] => {
									super::push(out, 1175);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 1993);
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
		['e', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['d', 'i', 'w', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 144);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 145);
					match s {
						[] => true,
						['n', 'a', 'r', s @ ..] => {
							match s {
								[] => true,
								['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1176);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 1177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 's', 'e', 't', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 146);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 147);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 488);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 489);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1178);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1994);
											match s {
												[] => true,
												['u', '\u{1}', s @ ..] => {
													super::push(out, 2331);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 1995);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 2332);
													match s {
														[] => true,
														['o', 'f', 'm', 'o', 'c', '\u{1}', s @ ..] => super::t0::n3297(s, out),
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
									super::push(out, 1179);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1996);
											match s {
												[] => true,
												['r', s @ ..] => {
													match s {
														[] => true,
														['e', 't', '\u{1}', s @ ..] => super::t0::n3298(s, out),
														['o', 'h', '\u{1}', s @ ..] => super::t0::n3299(s, out),
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
									super::push(out, 1180);
									match s {
										[] => true,
										['n', 'g', 'i', '\u{1}', s @ ..] => {
											super::push(out, 1997);
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
								['g', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1998);
											s.is_empty()
										}
										['r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 1999);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 1181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 490);
							match s {
								[] => true,
								['b', 'u', 's', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 1182);
									s.is_empty()
								}
								['n', 'e', 'g', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 1183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t0::n250(s, out),
				['n', s @ ..] => super::t0::n251(s, out),
				['p', s @ ..] => {
					super::push(out, 150);
					match s {
						[] => true,
						['i', 'r', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 496);
							s.is_empty()
						}
						['o', 'l', s @ ..] => super::t0::n898(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n253(s, out),
				['s', s @ ..] => super::t0::n254(s, out),
				['t', s @ ..] => super::t0::n255(s, out),
				['u', 'q', s @ ..] => super::t0::n256(s, out),
				['v', s @ ..] => {
					super::push(out, 153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t0::n45(s, out),
		['i', 'n', 'i', 'l', 'o', 's', 's', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 45);
			s.is_empty()
		}
		['k', s @ ..] => super::t0::n47(s, out),
		['l', s @ ..] => super::t0::n48(s, out),
		['m', s @ ..] => super::t0::n49(s, out),
		['n', s @ ..] => super::t0::n50(s, out),
		['r', s @ ..] => super::t0::n51(s, out),
		['s', s @ ..] => super::t0::n52(s, out),
		['t', s @ ..] => super::t0::n53(s, out),
		['y', s @ ..] => super::t0::n54(s, out),
		_ => false,
	}
}

/// The word shrewd (1 syllables).
pub(crate) fn n1005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	s.is_empty()
}

/// Words ending -owd (1 syllables).
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', '\u{1}', s @ ..] => {
			super::push(out, 574);
			s.is_empty()
		}
		['r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e (1 syllables).
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', 's', '\u{1}', s @ ..] => {
					super::push(out, 195);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 576);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 577);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'u', 'a', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 62);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 197);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'h', '\u{1}', s @ ..] => {
			super::push(out, 63);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -f (1 syllables).
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 199);
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 200);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 579);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 580);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 201);
					s.is_empty()
				}
				['i', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 202);
					s.is_empty()
				}
				['o', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 203);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 582);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 583);
							s.is_empty()
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 584);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1284);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 1285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 65);
			s.is_empty()
		}
		['r', 'a', 'w', 'd', '\u{1}', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -g (1 syllables).
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 67);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 204);
					s.is_empty()
				}
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 206);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 207);
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
						['l', '\u{1}', s @ ..] => {
							super::push(out, 586);
							s.is_empty()
						}
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 588);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 590);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1286);
									s.is_empty()
								}
								['y', '\u{1}', s @ ..] => {
									super::push(out, 1287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 1288);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 1289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 592);
							s.is_empty()
						}
						['o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 68);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 594);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 's', '\u{1}', s @ ..] => {
					super::push(out, 208);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -h (1 syllables).
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 209);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 596);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 598);
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 599);
							s.is_empty()
						}
						['u', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 210);
					s.is_empty()
				}
				['s', 't', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 211);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 601);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 's', '\u{1}', s @ ..] => {
									super::push(out, 1290);
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 603);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'o', s @ ..] => {
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 605);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 606);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 607);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1292);
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 609);
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 213);
					match s {
						[] => true,
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 612);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 614);
							s.is_empty()
						}
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 616);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 617);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 618);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 619);
							s.is_empty()
						}
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 620);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'w', 's', '\u{1}', s @ ..] => {
					super::push(out, 214);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 622);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 623);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', 's', '\u{1}', s @ ..] => {
									super::push(out, 1294);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 'w', 's', '\u{1}', s @ ..] => {
					super::push(out, 215);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -k (1 syllables).
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 624);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 216);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 217);
					s.is_empty()
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 218);
					s.is_empty()
				}
				['y', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 626);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 627);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 628);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1296);
									s.is_empty()
								}
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 1297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 630);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1298);
									s.is_empty()
								}
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 1299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 220);
					s.is_empty()
				}
				['u', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 222);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 224);
					s.is_empty()
				}
				['w', 'k', '\u{1}', s @ ..] => {
					super::push(out, 225);
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
						['j', '\u{1}', s @ ..] => {
							super::push(out, 633);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 634);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 636);
							s.is_empty()
						}
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 637);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 639);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 227);
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					super::push(out, 228);
					s.is_empty()
				}
				['o', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 230);
					s.is_empty()
				}
				['i', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 231);
					s.is_empty()
				}
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 233);
					s.is_empty()
				}
				['n', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 234);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'a', '\u{1}', s @ ..] => {
			super::push(out, 70);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l (1 syllables).
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 236);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 641);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'o', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 643);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'f', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 645);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 238);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 239);
					s.is_empty()
				}
				['y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 241);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 647);
							s.is_empty()
						}
						['r', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 648);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1300);
									s.is_empty()
								}
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 244);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 649);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 650);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 651);
							s.is_empty()
						}
						['u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'a', '\u{1}', s @ ..] => {
			super::push(out, 71);
			s.is_empty()
		}
		['u', 'o', '\u{1}', s @ ..] => {
			super::push(out, 72);
			s.is_empty()
		}
		['w', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 73);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -m (1 syllables).
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 246);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 74);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 248);
					s.is_empty()
				}
				['l', 's', '\u{1}', s @ ..] => {
					super::push(out, 249);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 653);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 654);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 75);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 656);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 252);
					s.is_empty()
				}
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 658);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 659);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 660);
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

/// Words ending -n (1 syllables).
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1302);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 254);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 257);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 663);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 664);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 665);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 666);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', '\u{1}', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'o', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 76);
			s.is_empty()
		}
		['w', 'o', s @ ..] => {
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 262);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 263);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -p (1 syllables).
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 264);
					s.is_empty()
				}
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 266);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 268);
					s.is_empty()
				}
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 669);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 671);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['s', 'i', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -r (1 syllables).
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 673);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 674);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 675);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1304);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 676);
							s.is_empty()
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'e', s @ ..] => {
			match s {
				[] => true,
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 272);
					s.is_empty()
				}
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 273);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 274);
					s.is_empty()
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 275);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 79);
			s.is_empty()
		}
		['o', 'o', s @ ..] => {
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 277);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 279);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -s (1 syllables).
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 80);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 679);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 680);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1306);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1307);
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
						['b', '\u{1}', s @ ..] => {
							super::push(out, 681);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1308);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 1309);
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

/// Words ending -t (1 syllables).
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 682);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 683);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 281);
					s.is_empty()
				}
				['h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 282);
					s.is_empty()
				}
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 283);
					s.is_empty()
				}
				['u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 81);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1310);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 685);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 1312);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1313);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 285);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 286);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 687);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 689);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'w', 's', '\u{1}', s @ ..] => {
					super::push(out, 288);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 691);
							s.is_empty()
						}
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 692);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['g', 'i', s @ ..] => {
					super::push(out, 290);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1314);
									s.is_empty()
								}
								['k', 's', '\u{1}', s @ ..] => {
									super::push(out, 1315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1316);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 1317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 694);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1318);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1319);
									s.is_empty()
								}
								['p', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 695);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 696);
							s.is_empty()
						}
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 292);
					s.is_empty()
				}
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 83);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1321);
									s.is_empty()
								}
								['u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 1322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 296);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 699);
							s.is_empty()
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		['p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 85);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 701);
							s.is_empty()
						}
						['m', 's', '\u{1}', s @ ..] => {
							super::push(out, 702);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 703);
							s.is_empty()
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 705);
							s.is_empty()
						}
						['m', 's', '\u{1}', s @ ..] => {
							super::push(out, 706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 707);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 299);
					s.is_empty()
				}
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 709);
							s.is_empty()
						}
						['u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'o', 'h', 'w', '\u{1}', s @ ..] => {
			super::push(out, 86);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 711);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 303);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word u (1 syllables).
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	s.is_empty()
}

/// Words ending -w (1 syllables).
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 87);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 305);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 713);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 714);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word lax (1 syllables).
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	s.is_empty()
}

/// Words ending -y (1 syllables).
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 311);
					s.is_empty()
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 312);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 716);
							s.is_empty()
						}
						['q', '\u{1}', s @ ..] => {
							super::push(out, 717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 718);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 313);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 314);
					s.is_empty()
				}
				['h', 'g', '\u{1}', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 720);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 721);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 722);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 1323);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 2087);
											s.is_empty()
										}
										['f', '\u{1}', s @ ..] => {
											super::push(out, 2088);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1324);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['a', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 725);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 's', '\u{1}', s @ ..] => {
			super::push(out, 90);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 91);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'a', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e (2 syllables).
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['b', 'e', '\u{1}', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 748);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 750);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1333);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 751);
							s.is_empty()
						}
						['e', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 753);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 754);
							s.is_empty()
						}
						['r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 755);
							s.is_empty()
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
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 's', '\u{1}', s @ ..] => {
							super::push(out, 756);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				['n', 'o', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1335);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'e', 'r', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				['y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				['i', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 353);
					s.is_empty()
				}
				['n', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 354);
					s.is_empty()
				}
				['r', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 355);
					s.is_empty()
				}
				['u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', 'u', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 357);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 761);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1337);
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1338);
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
			match s {
				[] => true,
				['b', 'b', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 358);
					s.is_empty()
				}
				['d', 'd', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 763);
							s.is_empty()
						}
						['u', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['s', 'l', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 765);
							s.is_empty()
						}
						['t', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 767);
							s.is_empty()
						}
						['l', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 768);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 1339);
									s.is_empty()
								}
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1340);
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
						['e', 'e', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 769);
							s.is_empty()
						}
						['o', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 770);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1341);
									s.is_empty()
								}
								['u', 'w', '\u{1}', s @ ..] => {
									super::push(out, 1342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						['t', 'h', 'g', 'i', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'm', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 1343);
									s.is_empty()
								}
								['j', '\u{1}', s @ ..] => {
									super::push(out, 1344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'n', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 360);
					s.is_empty()
				}
				['p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2089);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 2090);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1345);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => super::t0::n2113(s, out),
								_ => false,
							}
						}
						['r', 'e', 'm', '\u{1}', s @ ..] => super::t0::n1237(s, out),
						['y', 'm', '\u{1}', s @ ..] => super::t0::n1238(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t0::n570(s, out),
				['t', s @ ..] => super::t0::n571(s, out),
				['v', 'v', 'a', 'h', 'c', '\u{1}', s @ ..] => super::t0::n572(s, out),
				['z', 'o', 'c', '\u{1}', s @ ..] => super::t0::n573(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t0::n172(s, out),
		['l', s @ ..] => super::t0::n173(s, out),
		['m', s @ ..] => super::t0::n174(s, out),
		['n', s @ ..] => super::t0::n175(s, out),
		['p', s @ ..] => super::t0::n176(s, out),
		['r', s @ ..] => super::t0::n177(s, out),
		['s', s @ ..] => super::t0::n178(s, out),
		['t', s @ ..] => super::t0::n179(s, out),
		['u', s @ ..] => super::t0::n180(s, out),
		['v', s @ ..] => super::t0::n181(s, out),
		['x', s @ ..] => super::t0::n182(s, out),
		['y', 'r', 'e', 'v', '\u{1}', s @ ..] => super::t0::n183(s, out),
		_ => false,
	}
}

/// Words ending -h (2 syllables).
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['g', 'u', 'o', 'r', 'o', 'h', 't', '\u{1}', s @ ..] => {
			super::push(out, 104);
			s.is_empty()
		}
		['s', 'i', s @ ..] => {
			match s {
				[] => true,
				['g', 'g', 'u', 'l', 's', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['v', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				['w', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', 'o', 'c', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word forblack (2 syllables).
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	s.is_empty()
}

/// Words ending -l (2 syllables).
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['m', 'r', 'o', 'n', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['r', 't', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['t', 'y', 'l', '\u{1}', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				['v', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['q', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'e', '\u{1}', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['i', 'h', s @ ..] => {
					super::push(out, 399);
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 845);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'f', s @ ..] => {
					match s {
						[] => true,
						['h', 't', 'i', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 848);
							s.is_empty()
						}
						['o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'o', 'c', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 110);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						['h', 't', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1378);
									s.is_empty()
								}
								['r', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1380);
									s.is_empty()
								}
								['i', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1381);
									s.is_empty()
								}
								['r', 'u', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'a', '\u{1}', s @ ..] => {
							super::push(out, 852);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1383);
									s.is_empty()
								}
								['o', 'j', '\u{1}', s @ ..] => {
									super::push(out, 1384);
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

/// Words ending -m (2 syllables).
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['a', 'e', 'r', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 854);
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
				['s', 's', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				['x', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'i', 'f', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -n (2 syllables).
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['l', 'c', s @ ..] => {
							super::push(out, 855);
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 1385);
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
			super::push(out, 115);
			match s {
				[] => true,
				['d', 'd', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 404);
					s.is_empty()
				}
				['h', 't', 'a', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 405);
					s.is_empty()
				}
				['l', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 406);
					s.is_empty()
				}
				['p', 'p', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 407);
					s.is_empty()
				}
				['r', 'r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 408);
					s.is_empty()
				}
				['t', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 409);
					s.is_empty()
				}
				['v', 'e', '\u{1}', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 'n', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 116);
			s.is_empty()
		}
		['i', 'h', 'c', 't', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 117);
			s.is_empty()
		}
		['m', 'e', 'l', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 118);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['f', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1386);
									s.is_empty()
								}
								['h', 'g', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'l', 'r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 857);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 858);
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

/// Words ending -o (2 syllables).
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['r', 'o', 'h', 't', '\u{1}', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -r (2 syllables).
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', 'e', 'l', 'c', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 123);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 413);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1388);
									s.is_empty()
								}
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				['h', 't', s @ ..] => {
					super::push(out, 415);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 1390);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'l', 'a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				['n', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 417);
					s.is_empty()
				}
				['p', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 863);
							s.is_empty()
						}
						['i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 866);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1392);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 1393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 418);
					s.is_empty()
				}
				['w', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'a', 'f', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 124);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -s (2 syllables).
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['e', 't', 'u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['i', 'a', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 868);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'e', 'l', 'k', 'c', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 127);
			s.is_empty()
		}
		['u', 'o', 'l', 'a', 'e', 'j', '\u{1}', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -t (2 syllables).
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 420);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 870);
							s.is_empty()
						}
						['j', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 't', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['i', 'u', 'q', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 422);
					s.is_empty()
				}
				['l', 'o', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 423);
					s.is_empty()
				}
				['s', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'g', 'i', 't', 'p', 'u', '\u{1}', s @ ..] => {
			super::push(out, 130);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['f', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 425);
					s.is_empty()
				}
				['o', 'r', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'n', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 2093);
											s.is_empty()
										}
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2094);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 2095);
											s.is_empty()
										}
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2096);
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
								['i', 's', '\u{1}', s @ ..] => {
									super::push(out, 1394);
									s.is_empty()
								}
								['o', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'r', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 872);
							s.is_empty()
						}
						['s', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 's', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 874);
							s.is_empty()
						}
						['e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 875);
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
				['a', s @ ..] => {
					super::push(out, 427);
					match s {
						[] => true,
						['f', 'd', s @ ..] => {
							match s {
								[] => true,
								['a', 'e', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 1396);
									s.is_empty()
								}
								['e', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 1397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', 'n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', 'r', 'a', 'e', '\u{1}', s @ ..] => {
					super::push(out, 428);
					s.is_empty()
				}
				['u', 'g', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', 'v', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -w (2 syllables).
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1398);
									s.is_empty()
								}
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 1399);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1401);
									s.is_empty()
								}
								['y', '\u{1}', s @ ..] => {
									super::push(out, 1402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'r', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -y (2 syllables).
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 878);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'l', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1404);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1406);
									s.is_empty()
								}
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1408);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1409);
									s.is_empty()
								}
								['y', '\u{1}', s @ ..] => {
									super::push(out, 1410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1411);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1412);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 2097);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2098);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				['i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 432);
					s.is_empty()
				}
				['l', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1414);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1416);
									s.is_empty()
								}
								['y', '\u{1}', s @ ..] => {
									super::push(out, 1417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1418);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 1419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 881);
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1420);
									s.is_empty()
								}
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1421);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1422);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 434);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 884);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 886);
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
				['c', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						['o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 888);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1424);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1425);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 436);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 890);
							s.is_empty()
						}
						['e', 'i', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 891);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1426);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 1427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1428);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 1429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n226(s, out),
		['e', s @ ..] => super::t0::n227(s, out),
		['f', s @ ..] => super::t0::n228(s, out),
		['g', s @ ..] => super::t0::n229(s, out),
		['h', s @ ..] => super::t0::n230(s, out),
		['k', s @ ..] => super::t0::n231(s, out),
		['l', s @ ..] => super::t0::n232(s, out),
		['m', s @ ..] => super::t0::n233(s, out),
		['n', s @ ..] => super::t0::n234(s, out),
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', 'y', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['c', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				['j', 'r', 'o', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t0::n236(s, out),
		['r', s @ ..] => super::t0::n237(s, out),
		['s', s @ ..] => super::t0::n238(s, out),
		['t', s @ ..] => super::t0::n239(s, out),
		['v', s @ ..] => super::t0::n240(s, out),
		['w', s @ ..] => super::t0::n241(s, out),
		['x', s @ ..] => super::t0::n242(s, out),
		['z', s @ ..] => super::t0::n243(s, out),
		_ => false,
	}
}

/// The word comfortable (3 syllables).
pub(crate) fn n3297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2393);
	s.is_empty()
}

/// The word terrible (3 syllables).
pub(crate) fn n3298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2394);
	s.is_empty()
}

/// The word horrible (3 syllables).
pub(crate) fn n3299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2395);
	s.is_empty()
}

/// Words ending -me (3 syllables).
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['e', 'r', s @ ..] => {
			match s {
				[] => true,
				['p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['t', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', 'b', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['l', 'e', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2333);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2334);
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
						['d', 'a', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2000);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'h', s @ ..] => {
									super::push(out, 2335);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2396);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['o', 'h', 'w', '\u{1}', s @ ..] => {
									super::push(out, 2337);
									s.is_empty()
								}
								['u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2001);
							s.is_empty()
						}
						['s', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2002);
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

/// Words ending -ne (3 syllables).
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['b', 'r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['d', 'n', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['h', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				['m', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'c', 's', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 493);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 494);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['g', 'g', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2004);
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

/// Words ending -lope (3 syllables).
pub(crate) fn n898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2005);
					s.is_empty()
				}
				['p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -re (3 syllables).
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['a', 'e', 'v', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['t', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['v', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'h', 's', 'p', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2007);
							s.is_empty()
						}
						['s', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['s', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -se (3 syllables).
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'w', 'n', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2009);
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2010);
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
				['r', 't', 's', 'b', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2011);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te (3 syllables).
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['r', 'c', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2013);
							s.is_empty()
						}
						['s', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'l', s @ ..] => {
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2015);
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2016);
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
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				['n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['t', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -que (3 syllables).
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['l', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['t', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', 't', 'o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -g (3 syllables).
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 506);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 1208);
							match s {
								[] => true,
								['a', 'e', 'r', 'b', 'w', 'a', 'l', s @ ..] => {
									super::push(out, 2017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1210);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2018);
									match s {
										[] => true,
										['u', 'g', 's', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2339);
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

/// Words ending -k (3 syllables).
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['i', 'd', 'l', 'a', 'z', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l (3 syllables).
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'a', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2019);
									s.is_empty()
								}
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 'y', '\u{1}', s @ ..] => {
									super::push(out, 2021);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2340);
											s.is_empty()
										}
										['n', 'e', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2341);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'd', 'n', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1211);
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

/// Words ending -m (3 syllables).
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['w', 'e', 'k', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1212);
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

/// Words ending -n (3 syllables).
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	s.is_empty()
}

/// Words ending -r (3 syllables).
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['v', 'i', 'r', 'p', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 1213);
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

/// Words ending -s (3 syllables).
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['u', 'o', 'i', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 509);
					match s {
						[] => true,
						['i', 'b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1214);
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

/// Words ending -t (3 syllables).
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['e', 'r', 'c', 's', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['h', 'g', 'i', 'r', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['r', 'o', 'n', 'g', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1215);
							s.is_empty()
						}
						['s', 'a', 'e', 'l', 'p', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1216);
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

/// Words ending -y (3 syllables).
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['b', 'b', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 's', 'k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				['e', 'w', 'b', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['o', 'r', 'h', 't', 't', 'r', 'a', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', 't', 's', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['e', 'g', 'g', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['i', 't', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1217);
							s.is_empty()
						}
						['o', 'm', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['h', 'l', 'o', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1219);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1220);
							match s {
								[] => true,
								['s', 'u', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 2342);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 2343);
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
		['e', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2344);
											s.is_empty()
										}
										['r', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 2345);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 2022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', s @ ..] => {
					super::push(out, 518);
					match s {
						[] => true,
						['a', 'v', s @ ..] => {
							match s {
								[] => true,
								['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2023);
									s.is_empty()
								}
								['p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', 'l', 'd', 'a', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['o', 't', 'a', 'm', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				['t', 'a', 'l', 'o', 'c', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2025);
									s.is_empty()
								}
								['r', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1223);
							s.is_empty()
						}
						['i', 'd', 'd', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'b', 'e', 'c', 'i', '\u{1}', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'e', 'p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['s', 'i', 'b', 'b', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', 'e', 'h', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						['r', 'o', 'w', s @ ..] => {
							match s {
								[] => true,
								['a', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2027);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['c', 'i', 'o', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2346);
											s.is_empty()
										}
										['g', 'a', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 2347);
											s.is_empty()
										}
										['k', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 2348);
											s.is_empty()
										}
										['m', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2349);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['i', 'a', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 2397);
													s.is_empty()
												}
												['r', 'u', 'c', '\u{1}', s @ ..] => {
													super::push(out, 2398);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 2350);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'o', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2028);
									s.is_empty()
								}
								['h', 'c', 'e', 'e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2029);
									s.is_empty()
								}
								['k', 'n', s @ ..] => {
									match s {
										[] => true,
										['a', 'h', 't', '\u{1}', s @ ..] => {
											super::push(out, 2351);
											s.is_empty()
										}
										['i', 'r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2352);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2030);
									s.is_empty()
								}
								['r', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2031);
									s.is_empty()
								}
								['s', 'w', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 2032);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['e', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 2353);
											s.is_empty()
										}
										['h', 'g', 'u', 'o', 'h', 't', '\u{1}', s @ ..] => {
											super::push(out, 2354);
											s.is_empty()
										}
										['s', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2355);
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
		['k', 'c', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 't', 'o', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1226);
							s.is_empty()
						}
						['m', 'm', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						['n', s @ ..] => super::t0::n1968(s, out),
						['p', 't', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						['t', 's', 'p', 'a', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'n', 'u', '\u{1}', s @ ..] => super::t0::n939(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n275(s, out),
		['m', 'o', s @ ..] => super::t0::n276(s, out),
		['n', s @ ..] => super::t0::n277(s, out),
		['p', s @ ..] => super::t0::n278(s, out),
		['r', s @ ..] => super::t0::n279(s, out),
		['s', s @ ..] => super::t0::n280(s, out),
		['t', s @ ..] => super::t0::n281(s, out),
		['w', 'o', s @ ..] => super::t0::n282(s, out),
		['x', 'e', 's', 'n', 'u', '\u{1}', s @ ..] => super::t0::n283(s, out),
		['z', 'z', 'a', 'z', s @ ..] => super::t0::n284(s, out),
		_ => false,
	}
}

/// The word verie (2 syllables).
pub(crate) fn n2113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1346);
	s.is_empty()
}

/// The word merrie (2 syllables).
pub(crate) fn n1237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 774);
	s.is_empty()
}

/// The word myrie (2 syllables).
pub(crate) fn n1238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 775);
	s.is_empty()
}

/// Words ending -sie (2 syllables).
pub(crate) fn n570(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 776);
			s.is_empty()
		}
		['t', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 777);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tie (2 syllables).
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['s', 'y', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		['t', 'i', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1347);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word chavvie (2 syllables).
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	s.is_empty()
}

/// The word cozie (2 syllables).
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	s.is_empty()
}

/// Words ending -ke (2 syllables).
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 364);
			s.is_empty()
		}
		['y', 'l', '\u{1}', s @ ..] => {
			super::push(out, 365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -le (2 syllables).
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						['u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'd', 'i', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		['i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 368);
			s.is_empty()
		}
		['k', 'c', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1354);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', 'h', 'w', '\u{1}', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['w', 's', '\u{1}', s @ ..] => {
					super::push(out, 791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1358);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['r', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['e', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 1360);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1361);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1362);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1363);
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

/// Words ending -me (2 syllables).
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 799);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 370);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ne (2 syllables).
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1364);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['h', 't', '\u{1}', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pe (2 syllables).
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 372);
			s.is_empty()
		}
		['o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 373);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -re (2 syllables).
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				['u', 'q', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1366);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', '\u{1}', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'm', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 374);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'a', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 375);
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 377);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -se (2 syllables).
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		['n', 'e', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 827);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2091);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te (2 syllables).
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1370);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				['w', 'h', '\u{1}', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 'v', 's', '\u{1}', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['s', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 839);
					s.is_empty()
				}
				['u', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ue (2 syllables).
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		['q', 's', 'u', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ve (2 syllables).
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'o', 'm', 's', '\u{1}', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['u', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xe (2 syllables).
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word verye (2 syllables).
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	s.is_empty()
}

/// Words ending -dy (2 syllables).
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1430);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1431);
							s.is_empty()
						}
						['r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1433);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2099);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1435);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1436);
							s.is_empty()
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1437);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1440);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1441);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1443);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1444);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1445);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2101);
									s.is_empty()
								}
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2102);
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
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1446);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'e', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['s', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['e', 'i', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1449);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1450);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 1451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['r', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1452);
							s.is_empty()
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1454);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1455);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2103);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2104);
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
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'o', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1459);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						['o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1468);
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
				['e', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				['o', 'r', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ey (2 syllables).
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2105);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2106);
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
						['\u{1}', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', 'm', 's', '\u{1}', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2107);
									s.is_empty()
								}
								['l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 2108);
									s.is_empty()
								}
								['m', 's', '\u{1}', s @ ..] => {
									super::push(out, 2109);
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2111);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				['u', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1482);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1484);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1486);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1487);
							s.is_empty()
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1488);
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				['y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				['b', 'b', 'i', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 1490);
							s.is_empty()
						}
						['r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'f', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['g', 'g', 'i', 'u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				['i', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				['p', 'p', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				['y', 'h', s @ ..] => {
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1497);
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
				['a', 'o', 'l', 's', '\u{1}', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2113);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2114);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						['r', s @ ..] => super::t0::n2282(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n1458(s, out),
				['o', s @ ..] => super::t0::n1459(s, out),
				['y', s @ ..] => super::t0::n1460(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n715(s, out),
		['s', s @ ..] => super::t0::n716(s, out),
		['t', s @ ..] => super::t0::n717(s, out),
		['u', s @ ..] => super::t0::n718(s, out),
		['v', s @ ..] => super::t0::n719(s, out),
		['z', 'e', 'e', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fy (2 syllables).
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'i', s @ ..] => {
			match s {
				[] => true,
				['h', 't', '\u{1}', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1525);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2122);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2124);
									s.is_empty()
								}
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2125);
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
		['i', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				['e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1527);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 947);
					s.is_empty()
				}
				['w', 'd', '\u{1}', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gy (2 syllables).
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2126);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1533);
							s.is_empty()
						}
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1535);
							s.is_empty()
						}
						['l', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1537);
							s.is_empty()
						}
						['q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1538);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 950);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1544);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						['r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						['r', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1555);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1556);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1559);
							s.is_empty()
						}
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1560);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2128);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2130);
									s.is_empty()
								}
								['r', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1561);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1562);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 2133);
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
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 1563);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2134);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2135);
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
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1564);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1566);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hy (2 syllables).
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 2136);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2368);
											s.is_empty()
										}
										['b', '\u{1}', s @ ..] => {
											super::push(out, 2369);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 2137);
									s.is_empty()
								}
								['u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 2138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2139);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2140);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2370);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2371);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2142);
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2143);
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
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1569);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1571);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2144);
									s.is_empty()
								}
								['l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2145);
									s.is_empty()
								}
								['r', 'w', '\u{1}', s @ ..] => {
									super::push(out, 2146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2147);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 2372);
											s.is_empty()
										}
										['g', '\u{1}', s @ ..] => {
											super::push(out, 2373);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 2148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2149);
									s.is_empty()
								}
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 953);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2151);
									s.is_empty()
								}
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2152);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1572);
							s.is_empty()
						}
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1573);
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
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2154);
									s.is_empty()
								}
								['l', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2155);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 2156);
									s.is_empty()
								}
								['u', 'q', 's', '\u{1}', s @ ..] => {
									super::push(out, 2157);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2158);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 2159);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2161);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2162);
									s.is_empty()
								}
								['r', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2163);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2164);
									s.is_empty()
								}
								['u', 'q', 's', '\u{1}', s @ ..] => {
									super::push(out, 2165);
									s.is_empty()
								}
								['w', 's', '\u{1}', s @ ..] => {
									super::push(out, 2166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2167);
									s.is_empty()
								}
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => super::t0::n2373(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t0::n1517(s, out),
				_ => false,
			}
		}
		['g', 'u', s @ ..] => super::t0::n735(s, out),
		['p', s @ ..] => super::t0::n736(s, out),
		['r', 'r', 'y', 'm', '\u{1}', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		['s', s @ ..] => super::t0::n738(s, out),
		['t', s @ ..] => super::t0::n739(s, out),
		_ => false,
	}
}

/// Words ending -ky (2 syllables).
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1596);
							s.is_empty()
						}
						['r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['o', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 967);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1599);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1603);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						['r', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						['t', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				['n', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1611);
							s.is_empty()
						}
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1612);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1613);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1614);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', 'h', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1616);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'e', s @ ..] => {
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'a', 'l', 'g', '\u{1}', s @ ..] => {
			super::push(out, 446);
			s.is_empty()
		}
		['k', 'e', 'r', s @ ..] => {
			match s {
				[] => true,
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 974);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1618);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1620);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1621);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1622);
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1623);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2188);
									s.is_empty()
								}
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1625);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1626);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2190);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1627);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1628);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1630);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1631);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1632);
							s.is_empty()
						}
						['r', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1636);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1637);
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
				['c', '\u{1}', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2192);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', '\u{1}', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1641);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n1572(s, out),
				['t', 'o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => super::t0::n1574(s, out),
				_ => false,
			}
		}
		['u', 'e', 'y', '\u{1}', s @ ..] => super::t0::n750(s, out),
		['w', 'a', s @ ..] => super::t0::n751(s, out),
		_ => false,
	}
}

/// Words ending -ly (2 syllables).
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 's', '\u{1}', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2194);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 2195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 2196);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 2378);
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											super::push(out, 2379);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 2197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2198);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2380);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 2381);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 2199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2200);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 2201);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 2202);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2203);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 991);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2205);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 2206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1652);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2207);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 2208);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2382);
											s.is_empty()
										}
										['c', '\u{1}', s @ ..] => {
											super::push(out, 2383);
											s.is_empty()
										}
										['g', '\u{1}', s @ ..] => {
											super::push(out, 2384);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1653);
							s.is_empty()
						}
						['r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1654);
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
				['a', 'e', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				['s', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2210);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2211);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 2212);
									s.is_empty()
								}
								['w', 't', '\u{1}', s @ ..] => {
									super::push(out, 2213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2214);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				['i', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						['r', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n1595(s, out),
				['o', s @ ..] => super::t0::n1596(s, out),
				['r', s @ ..] => super::t0::n1597(s, out),
				['u', s @ ..] => super::t0::n1598(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n756(s, out),
		['f', s @ ..] => super::t0::n757(s, out),
		['g', s @ ..] => super::t0::n758(s, out),
		['h', s @ ..] => super::t0::n759(s, out),
		['i', s @ ..] => super::t0::n760(s, out),
		['k', s @ ..] => super::t0::n761(s, out),
		['l', s @ ..] => super::t0::n762(s, out),
		['m', s @ ..] => super::t0::n763(s, out),
		['n', s @ ..] => super::t0::n764(s, out),
		['o', s @ ..] => super::t0::n765(s, out),
		['p', s @ ..] => super::t0::n766(s, out),
		['r', s @ ..] => super::t0::n767(s, out),
		['s', s @ ..] => super::t0::n768(s, out),
		['t', s @ ..] => super::t0::n769(s, out),
		['u', 'r', '\u{1}', s @ ..] => super::t0::n770(s, out),
		['w', s @ ..] => super::t0::n771(s, out),
		['z', 'z', 'i', 'r', s @ ..] => super::t0::n772(s, out),
		_ => false,
	}
}

/// Words ending -my (2 syllables).
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 449);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				['n', 'i', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 1027);
					s.is_empty()
				}
				['y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 1029);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1744);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1746);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1030);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1751);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 1753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1754);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1756);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1757);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2280);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2282);
									s.is_empty()
								}
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2283);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2284);
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
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1031);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1032);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2285);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['m', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1759);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1035);
					s.is_empty()
				}
				['o', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1036);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'l', 's', '\u{1}', s @ ..] => {
			super::push(out, 451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ny (2 syllables).
pub(crate) fn n234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1038);
					s.is_empty()
				}
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1039);
					s.is_empty()
				}
				['i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1040);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1042);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1043);
					s.is_empty()
				}
				['y', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', 'r', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1762);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				['w', 't', '\u{1}', s @ ..] => {
					super::push(out, 1046);
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
						['k', '\u{1}', s @ ..] => {
							super::push(out, 1765);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2287);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 2288);
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
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1766);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1767);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1769);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						['w', 't', '\u{1}', s @ ..] => {
							super::push(out, 1772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						['r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1774);
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
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1048);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1775);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1779);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1781);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'i', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 453);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1050);
					s.is_empty()
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1051);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1783);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1784);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1785);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', '\u{1}', s @ ..] => {
			super::push(out, 454);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -py (2 syllables).
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1056);
					s.is_empty()
				}
				['r', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', 'w', 's', '\u{1}', s @ ..] => {
			super::push(out, 458);
			s.is_empty()
		}
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1058);
					s.is_empty()
				}
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1787);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1788);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1789);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 1790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1791);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2289);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2290);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1803);
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
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2291);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 2292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2293);
									s.is_empty()
								}
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2295);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'h', s @ ..] => {
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2297);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1811);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1812);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1815);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'w', 't', '\u{1}', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'i', s @ ..] => {
			match s {
				[] => true,
				['h', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ry (2 syllables).
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2299);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1826);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				['n', 's', '\u{1}', s @ ..] => {
					super::push(out, 1067);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				['u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 461);
			s.is_empty()
		}
		['c', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 462);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1071);
					s.is_empty()
				}
				['h', 'g', 'u', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				['l', 'u', 'o', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 1073);
					s.is_empty()
				}
				['n', 'a', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1074);
					s.is_empty()
				}
				['w', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1830);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1834);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1077);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				['u', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'o', '\u{1}', s @ ..] => {
			super::push(out, 463);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1082);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1848);
							s.is_empty()
						}
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1851);
							s.is_empty()
						}
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						['m', 's', '\u{1}', s @ ..] => {
							super::push(out, 1853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['m', 's', '\u{1}', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1855);
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
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 1856);
							s.is_empty()
						}
						['u', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				['t', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', 'a', '\u{1}', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sy (2 syllables).
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1091);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'i', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['h', 't', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 466);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1094);
					s.is_empty()
				}
				['u', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1867);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				['o', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['r', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 467);
			s.is_empty()
		}
		['m', 'i', s @ ..] => {
			match s {
				[] => true,
				['l', 's', '\u{1}', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1869);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1871);
							s.is_empty()
						}
						['w', 't', '\u{1}', s @ ..] => {
							super::push(out, 1872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1873);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['m', 'i', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1875);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1878);
							s.is_empty()
						}
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						['o', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1881);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1882);
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
				['a', 'm', 'x', '\u{1}', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', 'n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1885);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1886);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1888);
							s.is_empty()
						}
						['o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1889);
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
				['a', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				['f', 'a', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1109);
					s.is_empty()
				}
				['i', 't', '\u{1}', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['n', 'e', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				['r', 'o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'o', s @ ..] => {
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 468);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ty (2 syllables).
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1891);
							s.is_empty()
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1893);
							s.is_empty()
						}
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1894);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1895);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1896);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2301);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2302);
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
		['b', 'u', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1898);
							s.is_empty()
						}
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1899);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2303);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1900);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1902);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1903);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 's', '\u{1}', s @ ..] => {
									super::push(out, 2305);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2306);
									s.is_empty()
								}
								['l', 's', '\u{1}', s @ ..] => {
									super::push(out, 2307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2308);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 2309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		['k', 'n', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 471);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1124);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1905);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1907);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1908);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1909);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1911);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1912);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1913);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1914);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 1915);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1916);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1918);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1920);
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2310);
									s.is_empty()
								}
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 1922);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2312);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2313);
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
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2314);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2315);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1923);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2317);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 1924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'o', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t0::n847(s, out),
		['r', s @ ..] => super::t0::n848(s, out),
		['s', s @ ..] => super::t0::n849(s, out),
		['t', s @ ..] => super::t0::n850(s, out),
		['u', s @ ..] => super::t0::n851(s, out),
		_ => false,
	}
}

/// Words ending -vy (2 syllables).
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1132);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 473);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1139);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1140);
					s.is_empty()
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wy (2 syllables).
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				['h', 't', '\u{1}', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1145);
					s.is_empty()
				}
				['y', '\u{1}', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 't', '\u{1}', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'e', '\u{1}', s @ ..] => {
			super::push(out, 475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xy (2 syllables).
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', '\u{1}', s @ ..] => {
			super::push(out, 476);
			s.is_empty()
		}
		['i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 477);
			s.is_empty()
		}
		['n', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 478);
			s.is_empty()
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1157);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zy (2 syllables).
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				['e', 'u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 481);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				['e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['o', 'm', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['o', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						['m', 'h', s @ ..] => {
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2329);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1985);
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
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1171);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['a', 'z', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				['e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1992);
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

/// Words ending -nicky (3 syllables).
pub(crate) fn n1968(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2033);
			s.is_empty()
		}
		['n', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2034);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word unlucky (3 syllables).
pub(crate) fn n939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	s.is_empty()
}

/// Words ending -ly (3 syllables).
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['l', 'r', 'o', 'w', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['n', 'e', 'i', 'r', 'f', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1231);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['t', 's', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						['w', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				['d', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				['f', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				['k', 'i', 'l', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2037);
							s.is_empty()
						}
						['i', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2356);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2399);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 2400);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'o', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				['s', 'a', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2040);
							s.is_empty()
						}
						['i', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						['o', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2359);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2360);
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
		['h', 't', 'r', 'a', 'e', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 527);
			s.is_empty()
		}
		['i', 'o', 'f', 'n', 'i', 't', '\u{1}', s @ ..] => {
			super::push(out, 528);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 's', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				['e', 'r', 'r', 'i', 'u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'e', 'e', 's', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 529);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 't', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2044);
							s.is_empty()
						}
						['o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						['o', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', 'g', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				['o', 'e', 'g', 'd', 'u', 'm', 'r', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'h', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 530);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'g', 'g', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['o', 'b', 'h', 'g', 'i', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omy (3 syllables).
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'r', 'h', 's', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 532);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				['s', 'o', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ny (3 syllables).
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 'w', 'o', 'l', 'l', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['k', 'c', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', 'd', 'a', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 533);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1249);
					s.is_empty()
				}
				['i', 'k', 's', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				['u', 'f', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['g', 'r', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				['i', 'n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1253);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['a', 'n', 'n', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						['r', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'l', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -py (3 syllables).
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 's', 's', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 534);
			s.is_empty()
		}
		['p', 'a', 'h', s @ ..] => {
			match s {
				[] => true,
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1256);
					s.is_empty()
				}
				['p', 'a', 'l', 's', '\u{1}', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				['y', 's', '\u{1}', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ry (3 syllables).
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'w', s @ ..] => {
					match s {
						[] => true,
						['d', 'l', 'r', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						['r', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				['n', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1261);
					s.is_empty()
				}
				['s', 's', 'e', 'c', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1262);
					s.is_empty()
				}
				['w', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['a', 'r', 't', 'l', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['b', 'b', s @ ..] => {
					match s {
						[] => true,
						['o', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', 's', '\u{1}', s @ ..] => {
									super::push(out, 2361);
									s.is_empty()
								}
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2056);
							s.is_empty()
						}
						['i', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'm', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2058);
							match s {
								[] => true,
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 2363);
									s.is_empty()
								}
								['l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2060);
							s.is_empty()
						}
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'p', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						['i', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						['n', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2065);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['j', '\u{1}', s @ ..] => {
											super::push(out, 2401);
											s.is_empty()
										}
										['k', 's', '\u{1}', s @ ..] => {
											super::push(out, 2402);
											s.is_empty()
										}
										['l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 2403);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 2365);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 2404);
											s.is_empty()
										}
										['l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2405);
											s.is_empty()
										}
										['t', 's', '\u{1}', s @ ..] => {
											super::push(out, 2406);
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
				['v', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2066);
							s.is_empty()
						}
						['l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2068);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2069);
							s.is_empty()
						}
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1265);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					match s {
						[] => true,
						['e', 'u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2072);
							s.is_empty()
						}
						['p', 's', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2073);
							s.is_empty()
						}
						['w', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['n', 'u', 'o', 'c', 'n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', 'v', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sy (3 syllables).
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'e', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 542);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				['t', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'o', 'd', 't', 'u', 'o', '\u{1}', s @ ..] => {
			super::push(out, 543);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						['m', 't', 's', 'i', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1269);
					s.is_empty()
				}
				['u', 'f', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'r', 'a', 'w', 'g', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ty (3 syllables).
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'l', 'o', 'c', 'o', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 545);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', 'd', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1271);
					s.is_empty()
				}
				['h', 'c', 't', 'o', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1272);
					s.is_empty()
				}
				['k', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', 'h', 't', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1273);
					s.is_empty()
				}
				['o', 'l', 'p', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					match s {
						[] => true,
						['k', 'c', 'i', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						['r', 'e', 'e', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'l', 'p', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1275);
					s.is_empty()
				}
				['p', 'p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'm', 'e', 's', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 546);
			s.is_empty()
		}
		['o', 'r', 'r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 547);
			s.is_empty()
		}
		['r', 'i', 'd', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 548);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 't', 'i', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						['r', 'u', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', 'h', 't', 'd', 'o', 'o', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['k', 'c', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2083);
							s.is_empty()
						}
						['r', 'p', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', 's', 'l', 'l', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				['o', 'p', 'k', 'c', 'a', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1279);
					s.is_empty()
				}
				['u', 'n', 'o', 'c', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -owy (3 syllables).
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'a', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 549);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'h', 's', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						['e', 'y', '\u{1}', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2366);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 2367);
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

/// The word unsexy (3 syllables).
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	s.is_empty()
}

/// Words ending -zazzy (3 syllables).
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 551);
			s.is_empty()
		}
		['z', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rapey (2 syllables).
pub(crate) fn n2282(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 2115);
			s.is_empty()
		}
		['d', '\u{1}', s @ ..] => {
			super::push(out, 2116);
			s.is_empty()
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 2117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ipey (2 syllables).
pub(crate) fn n1458(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 's', '\u{1}', s @ ..] => {
			super::push(out, 1504);
			s.is_empty()
		}
		['r', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -opey (2 syllables).
pub(crate) fn n1459(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 1505);
			s.is_empty()
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1506);
			s.is_empty()
		}
		['r', '\u{1}', s @ ..] => {
			super::push(out, 1507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ypey (2 syllables).
pub(crate) fn n1460(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', '\u{1}', s @ ..] => {
			super::push(out, 1508);
			s.is_empty()
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 1509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rey (2 syllables).
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 927);
			s.is_empty()
		}
		['o', 'h', 'w', '\u{1}', s @ ..] => {
			super::push(out, 928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sey (2 syllables).
pub(crate) fn n716(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'e', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 929);
			s.is_empty()
		}
		['i', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 930);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1510);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1511);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 931);
			s.is_empty()
		}
		['u', 'o', s @ ..] => {
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'o', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tey (2 syllables).
pub(crate) fn n717(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 's', '\u{1}', s @ ..] => {
					super::push(out, 1515);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 2121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 933);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uey (2 syllables).
pub(crate) fn n718(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['n', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			match s {
				[] => true,
				['i', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				['r', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vey (2 syllables).
pub(crate) fn n719(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 934);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utchy (2 syllables).
pub(crate) fn n2373(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 's', '\u{1}', s @ ..] => {
			super::push(out, 2169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uchy (2 syllables).
pub(crate) fn n1517(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', '\u{1}', s @ ..] => {
			super::push(out, 1574);
			s.is_empty()
		}
		['y', '\u{1}', s @ ..] => {
			super::push(out, 1575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ughy (2 syllables).
pub(crate) fn n735(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				['l', 's', '\u{1}', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', '\u{1}', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -phy (2 syllables).
pub(crate) fn n736(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'y', 's', '\u{1}', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['m', 'y', 'l', '\u{1}', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -shy (2 syllables).
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2172);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				['w', 's', '\u{1}', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 957);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2176);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 2177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', '\u{1}', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		['k', 'r', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		['n', 'u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2178);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2376);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 2180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'h', '\u{1}', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -thy (2 syllables).
pub(crate) fn n739(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1586);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2181);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2182);
							s.is_empty()
						}
						['r', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				['w', 's', '\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1591);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2184);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 2185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'o', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['n', 'y', 's', '\u{1}', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		['o', 'l', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		['r', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 964);
			s.is_empty()
		}
		['s', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 's', '\u{1}', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2186);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 2187);
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

/// Words ending -isky (2 syllables).
pub(crate) fn n1572(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['k', '\u{1}', s @ ..] => {
			super::push(out, 1643);
			s.is_empty()
		}
		['l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1644);
			s.is_empty()
		}
		['r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1645);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word busky (2 syllables).
pub(crate) fn n1574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 984);
	s.is_empty()
}

/// The word yeuky (2 syllables).
pub(crate) fn n750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	s.is_empty()
}

/// Words ending -awky (2 syllables).
pub(crate) fn n751(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 985);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 986);
			s.is_empty()
		}
		['h', '\u{1}', s @ ..] => {
			super::push(out, 987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ndly (2 syllables).
pub(crate) fn n1595(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				['i', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odly (2 syllables).
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', '\u{1}', s @ ..] => {
			super::push(out, 1660);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdly (2 syllables).
pub(crate) fn n1597(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1661);
			s.is_empty()
		}
		['o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udly (2 syllables).
pub(crate) fn n1598(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1663);
			s.is_empty()
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 1664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ely (2 syllables).
pub(crate) fn n756(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1665);
					s.is_empty()
				}
				['h', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 1667);
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1668);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 997);
			s.is_empty()
		}
		['u', 'o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fly (2 syllables).
pub(crate) fn n757(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 999);
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1671);
					s.is_empty()
				}
				['i', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 1672);
					s.is_empty()
				}
				['u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gly (2 syllables).
pub(crate) fn n758(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2385);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 2222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						['r', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						['u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 2226);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 2227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 2228);
							s.is_empty()
						}
						['n', 's', '\u{1}', s @ ..] => {
							super::push(out, 2229);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2230);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2231);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 2232);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2233);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2235);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 2236);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 2237);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 2239);
							s.is_empty()
						}
						['r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'o', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hly (2 syllables).
pub(crate) fn n759(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', 'r', 'u', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1001);
			s.is_empty()
		}
		['s', 'e', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2387);
									s.is_empty()
								}
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2242);
							s.is_empty()
						}
						['o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2243);
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

/// Words ending -ily (2 syllables).
pub(crate) fn n760(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', 's', '\u{1}', s @ ..] => {
					super::push(out, 1680);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', '\u{1}', s @ ..] => {
			super::push(out, 1003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kly (2 syllables).
pub(crate) fn n761(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1004);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1686);
					s.is_empty()
				}
				['e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2245);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2247);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2248);
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
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2389);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 2390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2249);
							s.is_empty()
						}
						['w', 't', '\u{1}', s @ ..] => {
							super::push(out, 2250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2251);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 2252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1005);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2253);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lly (2 syllables).
pub(crate) fn n762(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				['u', 'q', 's', '\u{1}', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1692);
					s.is_empty()
				}
				['m', 's', '\u{1}', s @ ..] => {
					super::push(out, 1693);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1694);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				['y', '\u{1}', s @ ..] => {
					super::push(out, 1696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2255);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 1697);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1698);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2257);
							s.is_empty()
						}
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1703);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				['o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mly (2 syllables).
pub(crate) fn n763(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 1007);
			s.is_empty()
		}
		['i', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1008);
			s.is_empty()
		}
		['o', 'o', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1009);
			s.is_empty()
		}
		['u', 'r', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nly (2 syllables).
pub(crate) fn n764(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 1010);
			s.is_empty()
		}
		['i', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1011);
			s.is_empty()
		}
		['o', 's', '\u{1}', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oly (2 syllables).
pub(crate) fn n765(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', '\u{1}', s @ ..] => {
			super::push(out, 1014);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ply (2 syllables).
pub(crate) fn n766(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		['l', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1016);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2261);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 2263);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rly (2 syllables).
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2265);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['h', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1018);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1719);
					s.is_empty()
				}
				['n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sly (2 syllables).
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1019);
			s.is_empty()
		}
		['i', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tly (2 syllables).
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['h', 'g', 'i', s @ ..] => {
			match s {
				[] => true,
				['n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				['r', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1725);
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
							super::push(out, 2271);
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['u', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2273);
							s.is_empty()
						}
						['h', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 2275);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2391);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2392);
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
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2276);
							s.is_empty()
						}
						['h', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2277);
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
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ruly (2 syllables).
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	s.is_empty()
}

/// Words ending -wly (2 syllables).
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2278);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1735);
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rizzly (2 syllables).
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 1022);
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1023);
			s.is_empty()
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pty (2 syllables).
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 1128);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 2319);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2320);
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

/// Words ending -rty (2 syllables).
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				['w', 's', '\u{1}', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1129);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1928);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1932);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sty (2 syllables).
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1933);
					s.is_empty()
				}
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1935);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1936);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1937);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1938);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1939);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'n', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1130);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1942);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1943);
					s.is_empty()
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 'h', 't', '\u{1}', s @ ..] => {
			super::push(out, 1131);
			s.is_empty()
		}
		['u', 'o', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tty (2 syllables).
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1951);
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1952);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				['w', 't', '\u{1}', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1957);
					s.is_empty()
				}
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1961);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				['r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uty (2 syllables).
pub(crate) fn n851(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1968);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1971);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

