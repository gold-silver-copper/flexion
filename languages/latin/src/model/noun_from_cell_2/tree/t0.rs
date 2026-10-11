use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', 'n', 'n', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['a', 'b', 'o', 'h', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n3(s, out),
		['h', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['t', 'e', 'n', 'a', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 13);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n5(s, out),
		['s', s @ ..] => super::t0::n6(s, out),
		['t', 'e', 'b', 'i', 'l', 'i', 'l', 'u', 't', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 7);
			s.is_empty()
		}
		['æ', 't', 'a', 'h', 'c', 'a', 'm', 'æ', 'h', '\u{1}', s @ ..] => {
			super::push(out, 8);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n10(s, out),
		['l', s @ ..] => {
			super::push(out, 11);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 58);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 353);
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

/// Words ending -i.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 60);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 61);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 62);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 63);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 64);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 356);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1080);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 357);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1083);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 359);
							s.is_empty()
						}
						['o', 'c', s @ ..] => {
							super::push(out, 360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 66);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 67);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 362);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1085);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 363);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 364);
							match s {
								[] => true,
								['i', 'b', s @ ..] => {
									super::push(out, 1087);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 365);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 366);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 367);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1089);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1090);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1091);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1092);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 368);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 369);
							match s {
								[] => true,
								['s', 'o', s @ ..] => {
									super::push(out, 1094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 68);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 371);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									super::push(out, 1095);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 1096);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 1869);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 372);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1097);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 69);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							super::push(out, 373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 70);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 374);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 375);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 71);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 377);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1099);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1870);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 1100);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1101);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 378);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 379);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 72);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 380);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 381);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 382);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 73);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n16(s, out),
		['e', s @ ..] => super::t0::n17(s, out),
		['f', s @ ..] => {
			super::push(out, 18);
			s.is_empty()
		}
		['g', s @ ..] => super::t0::n19(s, out),
		['h', s @ ..] => super::t0::n20(s, out),
		['i', s @ ..] => super::t0::n21(s, out),
		['l', s @ ..] => super::t0::n22(s, out),
		['m', s @ ..] => super::t0::n23(s, out),
		['n', s @ ..] => super::t0::n24(s, out),
		['o', s @ ..] => {
			super::push(out, 25);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 155);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 156);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 157);
					s.is_empty()
				}
				['o', 'c', s @ ..] => {
					super::push(out, 158);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 159);
					s.is_empty()
				}
				['r', s @ ..] => super::t0::n163(s, out),
				['u', s @ ..] => {
					super::push(out, 161);
					s.is_empty()
				}
				['y', s @ ..] => super::t0::n165(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n27(s, out),
		['s', s @ ..] => super::t0::n28(s, out),
		['t', s @ ..] => super::t0::n29(s, out),
		['u', s @ ..] => super::t0::n30(s, out),
		['v', s @ ..] => super::t0::n31(s, out),
		['x', s @ ..] => super::t0::n32(s, out),
		['y', s @ ..] => super::t0::n33(s, out),
		['z', s @ ..] => super::t0::n34(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 213);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 214);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 775);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1416);
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
		['i', s @ ..] => super::t0::n36(s, out),
		['o', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 232);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['g', 'i', 'n', 'p', '\u{1}', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 235);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 'e', 'm', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 238);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 887);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1732);
									match s {
										[] => true,
										['u', 'a', 'h', 't', '\u{1}', s @ ..] => {
											super::push(out, 2336);
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
				['y', s @ ..] => {
					super::push(out, 239);
					match s {
						[] => true,
						['h', 'c', 'a', 'r', 'b', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 888);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 889);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 890);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 1733);
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
			super::push(out, 38);
			match s {
				[] => true,
				['b', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 240);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 241);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 891);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 1734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 892);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1735);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 893);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'e', '\u{1}', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 243);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 244);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 895);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 896);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 897);
							s.is_empty()
						}
						['r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 245);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 899);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 900);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1737);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 1738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 901);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 902);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 246);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1740);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 247);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 903);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1742);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2338);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 904);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 905);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 906);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1743);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2339);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1744);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 907);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 1745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 248);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ae.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 39);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 249);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 40);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 251);
					match s {
						[] => true,
						['s', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 909);
							s.is_empty()
						}
						['z', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 252);
					s.is_empty()
				}
				['e', 'h', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 254);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 911);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 912);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1746);
									match s {
										[] => true,
										['u', 'p', 'i', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2340);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 913);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1747);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 2341);
											match s {
												[] => true,
												['t', 'a', 'i', '\u{1}', s @ ..] => {
													super::push(out, 2567);
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
							super::push(out, 914);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1748);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 915);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 916);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1749);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 2342);
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
					super::push(out, 255);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 256);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 41);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 258);
					match s {
						[] => true,
						['i', 'p', 'e', 'l', 'c', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 260);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 918);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1750);
									match s {
										[] => true,
										['s', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2343);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1751);
									match s {
										[] => true,
										['r', 't', s @ ..] => {
											super::push(out, 2344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 919);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 920);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 921);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 922);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'b', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 262);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 263);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 42);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 266);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 267);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 268);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					super::push(out, 269);
					s.is_empty()
				}
				['e', 'l', s @ ..] => {
					super::push(out, 270);
					s.is_empty()
				}
				['i', 'r', 't', s @ ..] => {
					super::push(out, 271);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 272);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 273);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 274);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 275);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 44);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 276);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 932);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 933);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1754);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2345);
											match s {
												[] => true,
												['l', 'i', 'h', 'c', '\u{1}', s @ ..] => super::t0::n2628(s, out),
												['r', 't', 'a', 'p', '\u{1}', s @ ..] => super::t0::n2629(s, out),
												['s', s @ ..] => super::t0::n2630(s, out),
												_ => false,
											}
										}
										['l', 'y', 'h', 'p', '\u{1}', s @ ..] => super::t0::n2403(s, out),
										['r', s @ ..] => super::t0::n2404(s, out),
										['t', s @ ..] => super::t0::n2405(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t0::n283(s, out),
				['r', s @ ..] => super::t0::n284(s, out),
				['t', s @ ..] => super::t0::n285(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n45(s, out),
		['l', s @ ..] => super::t0::n46(s, out),
		['m', s @ ..] => super::t0::n47(s, out),
		['n', s @ ..] => super::t0::n48(s, out),
		['o', s @ ..] => super::t0::n49(s, out),
		['p', s @ ..] => super::t0::n50(s, out),
		['r', s @ ..] => super::t0::n51(s, out),
		['s', s @ ..] => super::t0::n52(s, out),
		['t', s @ ..] => super::t0::n53(s, out),
		['u', s @ ..] => super::t0::n54(s, out),
		['v', s @ ..] => super::t0::n55(s, out),
		['x', s @ ..] => super::t0::n56(s, out),
		['z', s @ ..] => super::t0::n57(s, out),
		_ => false,
	}
}

/// Words ending -di.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 74);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 385);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 387);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 390);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1104);
							match s {
								[] => true,
								['i', 's', 'i', 'r', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 392);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 393);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1105);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1106);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1107);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1108);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 80);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ei.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 399);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 401);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 404);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['a', 'n', 'i', 'r', s @ ..] => {
					super::push(out, 406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 407);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 408);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 85);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 411);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 412);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1112);
							s.is_empty()
						}
						['o', 'h', 'p', 'l', 'e', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 414);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1114);
							match s {
								[] => true,
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 1873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 415);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 417);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1116);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'i', s @ ..] => {
									super::push(out, 1874);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 2372);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'm', s @ ..] => {
									super::push(out, 1875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1876);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1877);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1878);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2373);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 420);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 421);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1118);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1119);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 422);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 90);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['t', 'c', s @ ..] => {
							super::push(out, 1121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 425);
					s.is_empty()
				}
				['l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 426);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 428);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['o', 'h', s @ ..] => {
					super::push(out, 430);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 431);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 94);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['a', 'm', 'm', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 434);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gi.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 437);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 439);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 440);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 99);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 100);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 442);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hi.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 445);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 446);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 447);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 449);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1128);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 450);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 451);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 452);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 454);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 457);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1131);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1132);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ii.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['r', 'e', 'v', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 463);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1135);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1136);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1137);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 465);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1139);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1881);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', s @ ..] => {
					super::push(out, 467);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 469);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1141);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 471);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							super::push(out, 1143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1144);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1145);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1146);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 472);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1148);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1149);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 473);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1151);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'i', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 477);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 481);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1154);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 482);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 1156);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1883);
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
			super::push(out, 114);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1157);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1161);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1162);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1164);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 488);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 492);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1167);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1169);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1170);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1172);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1884);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1885);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n120(s, out),
		['s', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					super::push(out, 509);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t0::n122(s, out),
		['u', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => super::t0::n124(s, out),
		['z', s @ ..] => super::t0::n125(s, out),
		_ => false,
	}
}

/// Words ending -li.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1205);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 1924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'g', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1207);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1208);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1926);
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
		['b', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1210);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['b', 'u', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				['p', 'i', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1211);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1212);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1213);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1929);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1931);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1932);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1933);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', s @ ..] => {
							super::push(out, 1214);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1215);
							s.is_empty()
						}
						['p', 'a', s @ ..] => {
							super::push(out, 1216);
							s.is_empty()
						}
						['s', 'a', s @ ..] => {
							super::push(out, 1217);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1935);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1936);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 540);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1218);
							match s {
								[] => true,
								['i', 'p', s @ ..] => {
									super::push(out, 1938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1219);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1220);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1221);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1222);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1941);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'a', s @ ..] => {
							super::push(out, 1223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 541);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 543);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 544);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1230);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1231);
							match s {
								[] => true,
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n561(s, out),
				['m', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t0::n133(s, out),
		['u', s @ ..] => super::t0::n134(s, out),
		['y', s @ ..] => super::t0::n135(s, out),
		_ => false,
	}
}

/// Words ending -mi.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 134);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 138);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1273);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1276);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 1277);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1279);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1280);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1971);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 1281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ni.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1282);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1289);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1295);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 604);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['c', 'y', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1304);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1305);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1307);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1308);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1309);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1313);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 616);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1316);
							match s {
								[] => true,
								['m', 's', 'i', 'r', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 1317);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1318);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1319);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1323);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1325);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 625);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 626);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n152(s, out),
		['o', s @ ..] => super::t0::n153(s, out),
		['p', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['r', s @ ..] => super::t0::n155(s, out),
		['u', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n659(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t0::n157(s, out),
		_ => false,
	}
}

/// Words ending -rpi.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 642);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ypi.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['t', 'o', s @ ..] => {
			super::push(out, 644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ri.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				['m', 'm', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 650);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 654);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['e', 'a', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1343);
							s.is_empty()
						}
						['u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 664);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 667);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1348);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1349);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1351);
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
				['b', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['p', 'a', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 679);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1355);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1980);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 694);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1358);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1982);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1983);
									s.is_empty()
								}
								['l', 'p', s @ ..] => {
									super::push(out, 1984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 1985);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 2415);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 1986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1359);
							s.is_empty()
						}
						['o', 'l', s @ ..] => {
							super::push(out, 1360);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1362);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 698);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -si.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 178);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1364);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1366);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1368);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ti.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1370);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1372);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1990);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1991);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1373);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 1992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1376);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1993);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2416);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1994);
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
							super::push(out, 1377);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 721);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1379);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				['t', 'r', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 725);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 1996);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 2417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1997);
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
						['l', s @ ..] => {
							super::push(out, 1382);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 728);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 729);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1387);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1390);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1999);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 2418);
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
			super::push(out, 187);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['u', 's', 'n', 'o', 'c', 's', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 745);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1397);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2000);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1398);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['l', s @ ..] => super::t0::n1438(s, out),
						['m', s @ ..] => super::t0::n1439(s, out),
						['p', s @ ..] => super::t0::n1440(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n771(s, out),
				['i', s @ ..] => super::t0::n772(s, out),
				['u', s @ ..] => super::t0::n773(s, out),
				['y', s @ ..] => super::t0::n774(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t0::n199(s, out),
		['u', s @ ..] => super::t0::n200(s, out),
		['y', s @ ..] => super::t0::n201(s, out),
		_ => false,
	}
}

/// Words ending -ui.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		['r', 't', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1409);
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

/// Words ending -vi.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1412);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1414);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xi.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yi.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zi.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['s', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				['y', 'l', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1420);
							match s {
								[] => true,
								['n', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1421);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2004);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 2419);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 2005);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 2420);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1422);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1423);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2007);
									match s {
										[] => true,
										['c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2421);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1424);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1426);
							match s {
								[] => true,
								['r', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1427);
							s.is_empty()
						}
						['m', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1429);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1430);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 2009);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2010);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2422);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 2423);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 2424);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2011);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1431);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2425);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 2426);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1432);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2013);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2014);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2015);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2016);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1433);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2018);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1434);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2020);
									match s {
										[] => true,
										['o', 'h', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2427);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2021);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 2428);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1435);
							match s {
								[] => true,
								['s', s @ ..] => super::t0::n2074(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n1473(s, out),
						['t', s @ ..] => super::t0::n1474(s, out),
						['v', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n807(s, out),
				['n', s @ ..] => super::t0::n808(s, out),
				['o', s @ ..] => super::t0::n809(s, out),
				['r', s @ ..] => super::t0::n810(s, out),
				['s', s @ ..] => super::t0::n811(s, out),
				['u', s @ ..] => super::t0::n812(s, out),
				['y', s @ ..] => super::t0::n813(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n223(s, out),
		['g', s @ ..] => super::t0::n224(s, out),
		['h', s @ ..] => super::t0::n225(s, out),
		['l', s @ ..] => super::t0::n226(s, out),
		['m', s @ ..] => super::t0::n227(s, out),
		['n', s @ ..] => super::t0::n228(s, out),
		['o', s @ ..] => super::t0::n229(s, out),
		['p', s @ ..] => super::t0::n230(s, out),
		['r', s @ ..] => super::t0::n231(s, out),
		['s', s @ ..] => super::t0::n232(s, out),
		['t', s @ ..] => super::t0::n233(s, out),
		['u', s @ ..] => super::t0::n234(s, out),
		['v', s @ ..] => super::t0::n235(s, out),
		['x', s @ ..] => super::t0::n236(s, out),
		['y', 'h', 'p', 'i', 'd', '\u{1}', s @ ..] => super::t0::n237(s, out),
		_ => false,
	}
}

/// The word chiliarchae.
pub(crate) fn n2628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2568);
	s.is_empty()
}

/// The word patriarchae.
pub(crate) fn n2629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2569);
	s.is_empty()
}

/// Words ending -siarchae.
pub(crate) fn n2630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2570);
	s.is_empty()
}

/// The word phylarchae.
pub(crate) fn n2403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2346);
	s.is_empty()
}

/// Words ending -rarchae.
pub(crate) fn n2404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2347);
	match s {
		[] => true,
		['e', 'i', 'h', '\u{1}', s @ ..] => {
			super::push(out, 2571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tarchae.
pub(crate) fn n2405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2348);
	match s {
		[] => true,
		['o', 'm', 'o', 'n', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2572);
			s.is_empty()
		}
		['s', 'y', 'x', '\u{1}', s @ ..] => {
			super::push(out, 2573);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -phae.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	s.is_empty()
}

/// Words ending -rhae.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	s.is_empty()
}

/// Words ending -thae.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['y', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 934);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iae.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				['s', 'a', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1756);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['o', 'c', 'i', s @ ..] => {
							super::push(out, 1758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 949);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2350);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2351);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2352);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['p', 'a', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 954);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 957);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				['i', 'c', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1762);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1765);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1767);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							super::push(out, 1769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 965);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1772);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 966);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1774);
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
					super::push(out, 967);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1775);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1776);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 2356);
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
			super::push(out, 292);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1779);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1780);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2358);
									match s {
										[] => true,
										['i', 'v', 'e', s @ ..] => {
											super::push(out, 2574);
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
					super::push(out, 975);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lae.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					super::push(out, 978);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1783);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2359);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['t', 'p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 986);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1787);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1788);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1789);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2362);
									match s {
										[] => true,
										['r', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 2575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mae.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 301);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 303);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 304);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 305);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['c', 'e', 's', 'r', 'e', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nae.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 993);
					match s {
						[] => true,
						['l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', 'n', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1003);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1798);
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

/// Words ending -oae.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pae.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		['u', 'r', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rae.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['a', 'r', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['o', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				['h', 'p', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 325);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['l', 'o', 'l', 'o', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', s @ ..] => {
					super::push(out, 1018);
					match s {
						[] => true,
						['o', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['m', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sae.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 328);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 331);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tae.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1024);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1026);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1027);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['t', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1029);
					match s {
						[] => true,
						['i', 'p', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1030);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1031);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1810);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2363);
									match s {
										[] => true,
										['m', 'o', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 2576);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1032);
					match s {
						[] => true,
						['s', 'o', 'g', 'y', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1035);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1036);
					match s {
						[] => true,
						['a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 'o', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1037);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1038);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1039);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1040);
					match s {
						[] => true,
						['c', 'o', 'n', 'e', 'h', 'r', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1041);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1042);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1820);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2365);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2577);
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
				['z', s @ ..] => {
					super::push(out, 1043);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1044);
					match s {
						[] => true,
						['a', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1045);
					match s {
						[] => true,
						['r', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1046);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1047);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1048);
					match s {
						[] => true,
						['c', 's', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1826);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						['y', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 1830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1051);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['o', 's', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						['e', 'd', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1834);
							s.is_empty()
						}
						['h', 't', 'a', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1836);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1056);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1059);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1060);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						['l', 'e', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => super::t0::n1890(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n1093(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n347(s, out),
		['s', s @ ..] => super::t0::n348(s, out),
		['t', s @ ..] => super::t0::n349(s, out),
		['u', s @ ..] => super::t0::n350(s, out),
		['x', s @ ..] => super::t0::n351(s, out),
		['y', s @ ..] => super::t0::n352(s, out),
		_ => false,
	}
}

/// Words ending -uae.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['s', 'b', s @ ..] => {
							super::push(out, 1868);
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

/// Words ending -vae.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 346);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xae.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 348);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 350);
			s.is_empty()
		}
		['r', 'e', 'x', '\u{1}', s @ ..] => {
			super::push(out, 351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zae.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	s.is_empty()
}

/// Words ending -rii.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1886);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1888);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1891);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1180);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2374);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1893);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1894);
							s.is_empty()
						}
						['t', 'e', s @ ..] => {
							super::push(out, 1895);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2376);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1898);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1899);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2379);
									s.is_empty()
								}
								['c', 'i', s @ ..] => {
									super::push(out, 2380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1182);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1900);
							s.is_empty()
						}
						['r', 'i', s @ ..] => {
							super::push(out, 1901);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1902);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1903);
							s.is_empty()
						}
						['i', 't', s @ ..] => {
							super::push(out, 1904);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1905);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1906);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2382);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 1907);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', 'u', 'n', s @ ..] => {
									super::push(out, 2384);
									s.is_empty()
								}
								['n', 'i', s @ ..] => {
									super::push(out, 2385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1908);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 1909);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1185);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1911);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 1912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1913);
							s.is_empty()
						}
						['n', 'e', 'm', 'a', s @ ..] => {
							super::push(out, 1914);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2386);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1187);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1915);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1916);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1917);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1920);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 506);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 507);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				['g', 'u', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1198);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tii.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['a', 'v', s @ ..] => {
			super::push(out, 512);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 513);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vii.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1204);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zii.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	s.is_empty()
}

/// Words ending -ioli.
pub(crate) fn n561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['c', 'a', s @ ..] => {
			super::push(out, 1233);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1235);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1236);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pli.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 550);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 551);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uli.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 1948);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1949);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1951);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1958);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1959);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2390);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2391);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1960);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1961);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2393);
									s.is_empty()
								}
								['l', 'o', s @ ..] => {
									super::push(out, 2394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2395);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2396);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2397);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2579);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 2398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1962);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2399);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2400);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1963);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', 'o', s @ ..] => {
									super::push(out, 2402);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 2403);
									match s {
										[] => true,
										['v', s @ ..] => {
											super::push(out, 2580);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'e', s @ ..] => {
									super::push(out, 2404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2405);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2406);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2407);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2408);
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
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2409);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 2581);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', s @ ..] => {
									super::push(out, 2410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1965);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1967);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2412);
									s.is_empty()
								}
								['p', 'e', s @ ..] => {
									super::push(out, 2413);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2414);
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
			super::push(out, 556);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				['e', 'l', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 557);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 559);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1254);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1256);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1257);
					match s {
						[] => true,
						['c', 's', 'i', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 561);
			s.is_empty()
		}
		['t', s @ ..] => super::t0::n578(s, out),
		['v', s @ ..] => {
			super::push(out, 563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yli.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 564);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'p', s @ ..] => {
			super::push(out, 566);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1269);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nni.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1328);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oni.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 631);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 632);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 633);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 634);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'o', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1334);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rni.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 637);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1338);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -auni.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 640);
	s.is_empty()
}

/// Words ending -yni.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lasti.
pub(crate) fn n1438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1401);
	s.is_empty()
}

/// Words ending -masti.
pub(crate) fn n1439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1402);
	s.is_empty()
}

/// Words ending -pasti.
pub(crate) fn n1440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1403);
	s.is_empty()
}

/// Words ending -esti.
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 750);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -isti.
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 751);
	match s {
		[] => true,
		['r', 'h', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usti.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 752);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ysti.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 753);
	s.is_empty()
}

/// Words ending -tti.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uti.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 755);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 756);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 757);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 758);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yti.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 760);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -spicis.
pub(crate) fn n2074(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 't', 'x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2429);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ricis.
pub(crate) fn n1473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1436);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2022);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2023);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 2431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ticis.
pub(crate) fn n1474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1437);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2026);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lcis.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 786);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1439);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ncis.
pub(crate) fn n808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 787);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1440);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ocis.
pub(crate) fn n809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 788);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1442);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcis.
pub(crate) fn n810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 789);
	s.is_empty()
}

/// Words ending -scis.
pub(crate) fn n811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 790);
	s.is_empty()
}

/// Words ending -ucis.
pub(crate) fn n812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 791);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1443);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1444);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ycis.
pub(crate) fn n813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 792);
	s.is_empty()
}

/// Words ending -dis.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1447);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['o', 'd', s @ ..] => {
							super::push(out, 2033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['i', 's', 's', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1453);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2036);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1456);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2039);
							s.is_empty()
						}
						['o', 'c', 'n', 'o', 'c', 'a', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1457);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1461);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1462);
					match s {
						[] => true,
						['l', 'y', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1463);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1464);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1465);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2044);
							match s {
								[] => true,
								['o', 'g', 'y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						['s', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1466);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1467);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1468);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['a', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1470);
					s.is_empty()
				}
				['o', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1471);
					s.is_empty()
				}
				['u', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 797);
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1480);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gis.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1481);
					s.is_empty()
				}
				['r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1483);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -his.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1487);
					match s {
						[] => true,
						['h', 'r', 'o', 'd', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1491);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'y', 'c', '\u{1}', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lis.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1492);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2057);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2433);
									s.is_empty()
								}
								['s', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2434);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 2435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2058);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1494);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', 'i', 's', 'y', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2062);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'u', 's', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						['r', 'o', 'm', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2065);
							match s {
								[] => true,
								['c', 'o', 'n', 'i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2438);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2582);
											s.is_empty()
										}
										['s', 's', 'e', '\u{1}', s @ ..] => {
											super::push(out, 2583);
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
					super::push(out, 1498);
					match s {
						[] => true,
						['r', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 2066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1499);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2068);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2069);
							match s {
								[] => true,
								['d', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2439);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2070);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2441);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 2584);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2071);
							s.is_empty()
						}
						['p', 'y', '\u{1}', s @ ..] => {
							super::push(out, 2072);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2073);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1500);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						['o', 'p', 'r', 'o', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2076);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 2077);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 2442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						['n', 'o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						['s', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1502);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2083);
							match s {
								[] => true,
								['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						['o', 'd', 'r', 'e', 'c', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						['s', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 2089);
							match s {
								[] => true,
								['n', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 2444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1504);
					match s {
						[] => true,
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 808);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1505);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2093);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2445);
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
			super::push(out, 809);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1507);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1509);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2097);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2098);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1511);
					s.is_empty()
				}
				['t', s @ ..] => super::t0::n1550(s, out),
				['u', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				['v', s @ ..] => super::t0::n1552(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n831(s, out),
		['o', s @ ..] => super::t0::n832(s, out),
		['u', s @ ..] => super::t0::n833(s, out),
		_ => false,
	}
}

/// Words ending -mis.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 813);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1526);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				['y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', '\u{1}', s @ ..] => {
			super::push(out, 816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nis.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['o', 'g', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 1533);
					s.is_empty()
				}
				['t', 'y', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1536);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1537);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1540);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1541);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2108);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2109);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2110);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2453);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2585);
											match s {
												[] => true,
												['u', 's', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 2619);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 2586);
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
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2111);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2113);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1544);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2114);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2455);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 2587);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2115);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2116);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2117);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2118);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2456);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1546);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1548);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1549);
					match s {
						[] => true,
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2122);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1550);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1551);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t0::n1590(s, out),
				['e', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['l', s @ ..] => super::t0::n2183(s, out),
						_ => false,
					}
				}
				['g', s @ ..] => super::t0::n1592(s, out),
				['h', s @ ..] => super::t0::n1593(s, out),
				['i', s @ ..] => super::t0::n1594(s, out),
				['l', s @ ..] => super::t0::n1595(s, out),
				['m', s @ ..] => super::t0::n1596(s, out),
				['n', s @ ..] => super::t0::n1597(s, out),
				['p', s @ ..] => super::t0::n1598(s, out),
				['r', s @ ..] => super::t0::n1599(s, out),
				['s', s @ ..] => super::t0::n1600(s, out),
				['t', s @ ..] => super::t0::n1601(s, out),
				['u', s @ ..] => super::t0::n1602(s, out),
				['v', s @ ..] => super::t0::n1603(s, out),
				['x', s @ ..] => super::t0::n1604(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n845(s, out),
		['u', s @ ..] => super::t0::n846(s, out),
		_ => false,
	}
}

/// Words ending -ois.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pis.
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1569);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2156);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2158);
							s.is_empty()
						}
						['s', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1574);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 830);
			s.is_empty()
		}
		['m', 'a', 'l', 'i', 'm', 'z', '\u{1}', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ris.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['m', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2162);
							s.is_empty()
						}
						['u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2164);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2165);
							s.is_empty()
						}
						['u', 'g', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				['h', 'c', 'c', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1583);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2167);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2169);
							s.is_empty()
						}
						['e', 't', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2171);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2492);
									s.is_empty()
								}
								['i', 'g', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2172);
							match s {
								[] => true,
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2173);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2495);
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 2601);
											s.is_empty()
										}
										['i', 'r', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2602);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 2496);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2603);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2497);
									match s {
										[] => true,
										['o', 's', '\u{1}', s @ ..] => {
											super::push(out, 2604);
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
					super::push(out, 1585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2176);
							s.is_empty()
						}
						['e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 2177);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2180);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2182);
							s.is_empty()
						}
						['c', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2183);
							s.is_empty()
						}
						['i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 2185);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 2186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				['u', 'l', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2187);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 2498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1595);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2189);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2190);
							match s {
								[] => true,
								['e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1597);
					match s {
						[] => true,
						['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2192);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 2500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2193);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2195);
							s.is_empty()
						}
						['o', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2196);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2197);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2502);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t0::n1641(s, out),
				['v', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				['x', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1606);
					match s {
						[] => true,
						['t', s @ ..] => super::t0::n2257(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n863(s, out),
		['o', s @ ..] => super::t0::n864(s, out),
		['p', 'e', 'v', '\u{1}', s @ ..] => super::t0::n865(s, out),
		['r', s @ ..] => super::t0::n866(s, out),
		['t', s @ ..] => super::t0::n867(s, out),
		['u', s @ ..] => super::t0::n868(s, out),
		['y', s @ ..] => super::t0::n869(s, out),
		_ => false,
	}
}

/// Words ending -sis.
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1645);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2245);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1651);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2247);
							s.is_empty()
						}
						['s', 'a', 't', 'a', 'c', 'o', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['e', 'o', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1654);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2251);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1655);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2253);
							s.is_empty()
						}
						['e', 'g', 'o', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1656);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 2609);
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2610);
											s.is_empty()
										}
										['n', 'y', 's', '\u{1}', s @ ..] => {
											super::push(out, 2611);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2255);
							match s {
								[] => true,
								['h', 'p', 'a', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2519);
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
			super::push(out, 851);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 853);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1660);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				['h', 'p', 'r', 'o', 'm', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				['l', 'u', 'c', 'r', 'e', 'b', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2256);
							s.is_empty()
						}
						['s', 'e', 'd', 'n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 2257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1664);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'o', 'h', 'c', 'n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						['e', 'l', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2520);
									s.is_empty()
								}
								['o', 'i', 'r', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'a', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'u', 'a', 'c', 'o', 'l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['i', 'l', 'c', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				['o', 'c', 'u', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1668);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['c', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2261);
							s.is_empty()
						}
						['n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'e', 't', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 856);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tis.
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1673);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2264);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2523);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2524);
									match s {
										[] => true,
										['b', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2526);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2527);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2266);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2529);
									s.is_empty()
								}
								['t', 'p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2267);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2268);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2269);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 2531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2272);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2273);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1675);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2275);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1677);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2277);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2533);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2534);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2613);
											match s {
												[] => true,
												['v', s @ ..] => {
													super::push(out, 2624);
													match s {
														[] => true,
														['a', 't', 'a', 'p', '\u{1}', s @ ..] => super::t0::n2691(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'u', s @ ..] => {
									super::push(out, 2535);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2536);
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 2625);
													match s {
														[] => true,
														['t', s @ ..] => super::t0::n2692(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 2626);
													match s {
														[] => true,
														['i', s @ ..] => super::t0::n2693(s, out),
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
									super::push(out, 2537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1681);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1682);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2280);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2281);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 2539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				['o', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1685);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2283);
							s.is_empty()
						}
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2284);
							s.is_empty()
						}
						['p', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2285);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				['g', s @ ..] => super::t0::n1730(s, out),
				['h', 'p', 'e', 'm', '\u{1}', s @ ..] => super::t0::n1731(s, out),
				['l', s @ ..] => super::t0::n1732(s, out),
				['m', s @ ..] => super::t0::n1733(s, out),
				['p', s @ ..] => super::t0::n1734(s, out),
				['r', s @ ..] => super::t0::n1735(s, out),
				['s', '\u{1}', s @ ..] => super::t0::n1736(s, out),
				['t', s @ ..] => super::t0::n1737(s, out),
				['v', s @ ..] => super::t0::n1738(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n883(s, out),
		['n', s @ ..] => super::t0::n884(s, out),
		['o', s @ ..] => super::t0::n885(s, out),
		['p', s @ ..] => super::t0::n886(s, out),
		['r', s @ ..] => super::t0::n887(s, out),
		['s', s @ ..] => super::t0::n888(s, out),
		['t', 'u', 'b', '\u{1}', s @ ..] => super::t0::n889(s, out),
		['u', s @ ..] => super::t0::n890(s, out),
		_ => false,
	}
}

/// Words ending -uis.
pub(crate) fn n234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2333);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vis.
pub(crate) fn n235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 229);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 873);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2565);
									s.is_empty()
								}
								['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1722);
					match s {
						[] => true,
						['b', 'i', s @ ..] => {
							super::push(out, 2335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				['l', 'c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['t', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 877);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 878);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xis.
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 879);
			match s {
				[] => true,
				['l', 'l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				['n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['t', 'n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'd', 'o', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word diphyis.
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	s.is_empty()
}

/// The word baptae.
pub(crate) fn n1890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1848);
	s.is_empty()
}

/// Words ending -iptae.
pub(crate) fn n1093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1066);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1849);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtae.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1067);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1068);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stae.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1851);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['s', 'u', 'h', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1853);
					s.is_empty()
				}
				['t', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1070);
			match s {
				[] => true,
				['a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1855);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['p', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1858);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2369);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1859);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1860);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1861);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						['y', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1863);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1864);
					s.is_empty()
				}
				['n', 'g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1866);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttae.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	s.is_empty()
}

/// Words ending -utae.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1075);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xtae.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 343);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytae.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['d', 'o', 'l', 'g', 'o', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 1077);
			s.is_empty()
		}
		['l', 'o', s @ ..] => {
			super::push(out, 1078);
			match s {
				[] => true,
				['c', 'o', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tuli.
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1260);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1261);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1262);
			s.is_empty()
		}
		['i', 'p', s @ ..] => {
			super::push(out, 1263);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1264);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1265);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1266);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tilis.
pub(crate) fn n1550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1512);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2099);
			match s {
				[] => true,
				['e', 'l', 'l', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vilis.
pub(crate) fn n1552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1514);
	s.is_empty()
}

/// Words ending -llis.
pub(crate) fn n831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 810);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1515);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 2101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2102);
					match s {
						[] => true,
						['o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2104);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olis.
pub(crate) fn n832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 811);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1518);
			s.is_empty()
		}
		['h', 'o', 'c', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1519);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1520);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2105);
					match s {
						[] => true,
						['c', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						['r', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 1521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulis.
pub(crate) fn n833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 812);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1522);
			s.is_empty()
		}
		['b', 'i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 1523);
			s.is_empty()
		}
		['d', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1524);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -donis.
pub(crate) fn n1590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1552);
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			super::push(out, 2126);
			s.is_empty()
		}
		['n', 'a', s @ ..] => {
			super::push(out, 2127);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2128);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -leonis.
pub(crate) fn n2183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2130);
	match s {
		[] => true,
		['u', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gonis.
pub(crate) fn n1592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1554);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -honis.
pub(crate) fn n1593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1555);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 2132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ionis.
pub(crate) fn n1594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1556);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2133);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['i', 'd', s @ ..] => {
					super::push(out, 2459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2135);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2136);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2137);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2138);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2463);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2464);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2620);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2139);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2140);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2468);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2469);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 2472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2141);
			match s {
				[] => true,
				['o', 'm', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2142);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2476);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2477);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2590);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2478);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 2479);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2480);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2592);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2622);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2628);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2481);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2595);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2143);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2483);
					s.is_empty()
				}
				['i', 't', s @ ..] => {
					super::push(out, 2484);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2486);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2487);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2599);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2623);
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
			super::push(out, 2144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lonis.
pub(crate) fn n1595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1557);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -monis.
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1558);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nonis.
pub(crate) fn n1597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1559);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ponis.
pub(crate) fn n1598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1560);
	s.is_empty()
}

/// Words ending -ronis.
pub(crate) fn n1599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1561);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2148);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sonis.
pub(crate) fn n1600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1562);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tonis.
pub(crate) fn n1601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1563);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2151);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2152);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2489);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uonis.
pub(crate) fn n1602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1564);
	s.is_empty()
}

/// Words ending -vonis.
pub(crate) fn n1603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1565);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xonis.
pub(crate) fn n1604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1566);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnis.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 824);
	match s {
		[] => true,
		['o', 'c', 'i', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 1567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unis.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	match s {
		[] => true,
		['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teris.
pub(crate) fn n1641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1603);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2198);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2199);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2200);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2201);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2202);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2203);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2506);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2605);
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

/// Words ending -tigris.
pub(crate) fn n2257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2204);
	s.is_empty()
}

/// Words ending -iris.
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1607);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 1608);
			s.is_empty()
		}
		['d', 'd', 'a', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1609);
			s.is_empty()
		}
		['l', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1610);
			s.is_empty()
		}
		['m', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1611);
			s.is_empty()
		}
		['x', 'i', 'l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1612);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oris.
pub(crate) fn n864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2206);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1616);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1617);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1618);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2213);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2215);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2217);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2219);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2223);
					match s {
						[] => true,
						['o', 'c', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1623);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2224);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1624);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2226);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2512);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 2513);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2514);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2608);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1625);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 1626);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word vepris.
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 844);
	s.is_empty()
}

/// Words ending -rris.
pub(crate) fn n866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1627);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1628);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tris.
pub(crate) fn n867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 846);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1629);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1630);
			match s {
				[] => true,
				['e', 'v', 'i', s @ ..] => {
					match s {
						[] => true,
						['b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						['r', 'g', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2235);
					match s {
						[] => true,
						['g', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2236);
					s.is_empty()
				}
				['u', 'l', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 1632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uris.
pub(crate) fn n868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 847);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 1633);
			s.is_empty()
		}
		['b', '\u{1}', s @ ..] => {
			super::push(out, 1634);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1635);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2238);
					s.is_empty()
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1636);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1637);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1638);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2240);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1640);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1641);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2242);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yris.
pub(crate) fn n869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 848);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1643);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1644);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word patavinitatis.
pub(crate) fn n2691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2629);
	s.is_empty()
}

/// Words ending -tiositatis.
pub(crate) fn n2692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2630);
	s.is_empty()
}

/// Words ending -inositatis.
pub(crate) fn n2693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2631);
	s.is_empty()
}

/// Words ending -gitis.
pub(crate) fn n1730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1689);
	match s {
		[] => true,
		['n', 'i', 'r', 'y', 's', '\u{1}', s @ ..] => {
			super::push(out, 2287);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2288);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word mephitis.
pub(crate) fn n1731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1690);
	s.is_empty()
}

/// Words ending -litis.
pub(crate) fn n1732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1691);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2289);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2290);
			s.is_empty()
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 2291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mitis.
pub(crate) fn n1733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1692);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2292);
			s.is_empty()
		}
		['d', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2293);
			s.is_empty()
		}
		['g', 'a', 'r', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2294);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2295);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2296);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pitis.
pub(crate) fn n1734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1693);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2297);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2540);
					match s {
						[] => true,
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ritis.
pub(crate) fn n1735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1694);
	match s {
		[] => true,
		['u', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2299);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sitis.
pub(crate) fn n1736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1695);
	s.is_empty()
}

/// Words ending -titis.
pub(crate) fn n1737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1696);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vitis.
pub(crate) fn n1738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1697);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltis.
pub(crate) fn n883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 862);
	s.is_empty()
}

/// Words ending -ntis.
pub(crate) fn n884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1698);
			match s {
				[] => true,
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2303);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2304);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2615);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['i', 'm', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2544);
							match s {
								[] => true,
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2617);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'u', 'c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2545);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['b', 'm', 'u', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2311);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2547);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2618);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2627);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 2632);
													s.is_empty()
												}
												['o', '\u{1}', s @ ..] => {
													super::push(out, 2633);
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
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2548);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2312);
					match s {
						[] => true,
						['r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2313);
					match s {
						[] => true,
						['c', 'i', 'f', 'f', 'ë', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2551);
							s.is_empty()
						}
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2552);
							s.is_empty()
						}
						['v', 'r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2317);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1700);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2319);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2320);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2323);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2325);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2559);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2560);
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

/// Words ending -otis.
pub(crate) fn n885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 864);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['i', 'p', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1702);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					super::push(out, 2328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ptis.
pub(crate) fn n886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 865);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1704);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtis.
pub(crate) fn n887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 866);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1706);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 's', '\u{1}', s @ ..] => {
			super::push(out, 1707);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stis.
pub(crate) fn n888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 867);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1708);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1709);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1710);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1711);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word buttis.
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 868);
	s.is_empty()
}

/// Words ending -utis.
pub(crate) fn n890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 869);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1712);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 1713);
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1714);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1715);
			s.is_empty()
		}
		_ => false,
	}
}

