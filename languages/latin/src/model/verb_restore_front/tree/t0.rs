use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['b', s @ ..] => super::t0::n23(s, out),
				['c', s @ ..] => super::t0::n24(s, out),
				['d', s @ ..] => super::t0::n25(s, out),
				['e', s @ ..] => {
					super::push(out, 26);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 202);
							match s {
								[] => true,
								['u', s @ ..] => super::t0::n832(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 27);
					s.is_empty()
				}
				['g', s @ ..] => super::t0::n28(s, out),
				['l', s @ ..] => super::t0::n29(s, out),
				['m', s @ ..] => super::t0::n30(s, out),
				['n', s @ ..] => super::t0::n31(s, out),
				['p', s @ ..] => super::t0::n32(s, out),
				['r', s @ ..] => super::t0::n33(s, out),
				['s', s @ ..] => super::t0::n34(s, out),
				['t', s @ ..] => super::t0::n35(s, out),
				['u', s @ ..] => super::t0::n36(s, out),
				['v', s @ ..] => super::t0::n37(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t0::n2(s, out),
		['c', s @ ..] => super::t0::n3(s, out),
		['d', s @ ..] => super::t0::n4(s, out),
		['e', s @ ..] => super::t0::n5(s, out),
		['f', s @ ..] => super::t0::n6(s, out),
		['g', s @ ..] => super::t0::n7(s, out),
		['h', s @ ..] => super::t0::n8(s, out),
		['i', s @ ..] => super::t0::n9(s, out),
		['l', s @ ..] => super::t0::n10(s, out),
		['m', s @ ..] => super::t0::n11(s, out),
		['n', s @ ..] => super::t0::n12(s, out),
		['o', s @ ..] => super::t0::n13(s, out),
		['p', s @ ..] => super::t0::n14(s, out),
		['q', s @ ..] => super::t0::n15(s, out),
		['r', s @ ..] => super::t0::n16(s, out),
		['s', s @ ..] => super::t0::n17(s, out),
		['t', s @ ..] => super::t0::n18(s, out),
		['u', s @ ..] => super::t0::n19(s, out),
		['v', s @ ..] => super::t0::n20(s, out),
		['w', s @ ..] => {
			super::push(out, 21);
			s.is_empty()
		}
		['z', s @ ..] => super::t0::n22(s, out),
		_ => false,
	}
}

/// Words ending -ba.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1354);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', 't', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1357);
							match s {
								[] => true,
								['t', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				['æ', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'd', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 174);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 1359);
							match s {
								[] => true,
								['a', 'c', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 177);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 747);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['æ', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['t', 'o', '\u{1}', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ca.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 751);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1360);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1694);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 752);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1696);
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
			super::push(out, 181);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 755);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 182);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -da.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 760);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1362);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 185);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1364);
							s.is_empty()
						}
						['n', 's', 'e', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 762);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 1366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 188);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['r', 'a', 't', s @ ..] => {
					super::push(out, 765);
					match s {
						[] => true,
						['u', 'l', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['r', 'e', 's', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1369);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1370);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 768);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['u', 'c', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['t', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 770);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['g', 'i', 't', s @ ..] => {
							super::push(out, 1378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1382);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1697);
									s.is_empty()
								}
								['r', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1698);
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
			super::push(out, 195);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1386);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['i', 'b', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['i', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'e', 'f', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['a', 'c', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['m', 'i', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urea.
pub(crate) fn n832(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1396);
			s.is_empty()
		}
		['s', 'c', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ga.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 794);
					match s {
						[] => true,
						['g', 'o', 'r', s @ ..] => {
							super::push(out, 1398);
							match s {
								[] => true,
								['i', 'z', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1399);
							match s {
								[] => true,
								['u', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['c', 't', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						['m', 'i', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['l', 'g', 'a', 'm', 'o', '\u{1}', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 799);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['v', 'i', 'g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['l', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						['m', 'e', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1405);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 1702);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1703);
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
			super::push(out, 215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 811);
					match s {
						[] => true,
						['r', 'e', 's', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							super::push(out, 1408);
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

/// Words ending -ra.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 220);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['a', 'c', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['u', 'm', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['m', 'a', 't', 'i', 'z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 821);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 822);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1411);
							match s {
								[] => true,
								['s', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1704);
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
			super::push(out, 229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['t', 'a', s @ ..] => {
			super::push(out, 230);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 823);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -b.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 38);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 232);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 233);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 39);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 236);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 238);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 239);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 40);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 240);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 41);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 42);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['b', 'u', 'l', 'o', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 243);
					match s {
						[] => true,
						['c', 'i', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -c.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 44);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 245);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 831);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1413);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 246);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 247);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 835);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 836);
							match s {
								[] => true,
								['n', 'i', 'z', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 248);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 249);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 838);
							match s {
								[] => true,
								['n', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 251);
					match s {
						[] => true,
						['e', 'c', s @ ..] => {
							super::push(out, 839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 252);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 840);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 253);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 842);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 843);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 844);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 845);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 254);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 46);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 47);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1417);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 850);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1419);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 852);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 257);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 259);
					match s {
						[] => true,
						['d', 'i', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 48);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 261);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 855);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1421);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 856);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 262);
					s.is_empty()
				}
				['g', s @ ..] => super::t0::n268(s, out),
				['h', s @ ..] => {
					super::push(out, 264);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 265);
					s.is_empty()
				}
				['l', s @ ..] => super::t0::n271(s, out),
				['m', s @ ..] => super::t0::n272(s, out),
				['n', s @ ..] => super::t0::n273(s, out),
				['o', s @ ..] => super::t0::n274(s, out),
				['p', s @ ..] => super::t0::n275(s, out),
				['q', s @ ..] => super::t0::n276(s, out),
				['r', s @ ..] => super::t0::n277(s, out),
				['u', s @ ..] => super::t0::n278(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n49(s, out),
		['u', s @ ..] => super::t0::n50(s, out),
		_ => false,
	}
}

/// Words ending -d.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 51);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 286);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 287);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 288);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 898);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1445);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', s @ ..] => {
							super::push(out, 899);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 900);
							s.is_empty()
						}
						['o', 'r', 'o', s @ ..] => {
							super::push(out, 901);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 902);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 1723);
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											super::push(out, 1724);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1725);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 1726);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 1447);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 291);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 903);
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 1449);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1727);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1728);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 904);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1450);
									match s {
										[] => true,
										['r', 'e', 's', s @ ..] => {
											super::push(out, 1729);
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
					super::push(out, 292);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 293);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 294);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 906);
							match s {
								[] => true,
								['i', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'd', s @ ..] => {
							super::push(out, 907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['i', 'b', s @ ..] => {
							super::push(out, 908);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 909);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1454);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 910);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1456);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1730);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1731);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 911);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 912);
							match s {
								[] => true,
								['l', 'i', 'o', 'r', s @ ..] => {
									super::push(out, 1457);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1458);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 297);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 913);
							match s {
								[] => true,
								['s', 'c', s @ ..] => {
									super::push(out, 1460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 914);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1461);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1462);
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
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 917);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1463);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 298);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 918);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 920);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t0::n966(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n306(s, out),
				['s', s @ ..] => super::t0::n307(s, out),
				['t', s @ ..] => super::t0::n308(s, out),
				['u', s @ ..] => super::t0::n309(s, out),
				['v', s @ ..] => super::t0::n310(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n53(s, out),
		['o', s @ ..] => super::t0::n54(s, out),
		['r', s @ ..] => super::t0::n55(s, out),
		['u', s @ ..] => super::t0::n56(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 57);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 58);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 943);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1476);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 944);
							match s {
								[] => true,
								['o', 'r', s @ ..] => {
									super::push(out, 1478);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 1796);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 1797);
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
		['g', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['e', 'o', s @ ..] => {
					super::push(out, 325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 945);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 327);
					match s {
						[] => true,
						['b', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 328);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 329);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 949);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1479);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 330);
					match s {
						[] => true,
						['u', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 331);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 332);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 952);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 333);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 334);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 955);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1481);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1482);
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
						['r', s @ ..] => {
							super::push(out, 956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 336);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 957);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 958);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 337);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 339);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1484);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 341);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 342);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 961);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 962);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 963);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 964);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 67);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 345);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1486);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 966);
							match s {
								[] => true,
								['t', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'e', s @ ..] => {
							super::push(out, 967);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', 'n', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1489);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1490);
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
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 353);
					match s {
						[] => true,
						['g', 'i', s @ ..] => {
							super::push(out, 968);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 969);
							match s {
								[] => true,
								['g', 'e', 'l', 'i', 'z', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t0::n1020(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n363(s, out),
				['i', s @ ..] => super::t0::n364(s, out),
				['o', s @ ..] => super::t0::n365(s, out),
				_ => false,
			}
		}
		['x', s @ ..] => super::t0::n72(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 988);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1503);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 369);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 370);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							super::push(out, 990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 371);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 991);
							s.is_empty()
						}
						['e', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 372);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 993);
							match s {
								[] => true,
								['o', 'r', s @ ..] => {
									super::push(out, 1505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 994);
							match s {
								[] => true,
								['e', 'f', s @ ..] => {
									super::push(out, 1506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 373);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 995);
							s.is_empty()
						}
						['i', 'c', 'i', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 996);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 998);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', '\u{1}', s @ ..] => {
					super::push(out, 378);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 379);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1000);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1001);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 380);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 381);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 382);
					s.is_empty()
				}
				['v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 384);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1003);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1004);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1507);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1005);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 385);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1007);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 387);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				['d', 'e', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 389);
					s.is_empty()
				}
				['m', 'e', 'n', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['r', 'm', 'o', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 394);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1012);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1013);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1509);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1510);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1016);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1017);
							s.is_empty()
						}
						['i', 't', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1018);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1019);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1020);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1021);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1512);
									s.is_empty()
								}
								['s', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1513);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1736);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 1737);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				['g', s @ ..] => super::t0::n410(s, out),
				['l', s @ ..] => super::t0::n411(s, out),
				['m', s @ ..] => super::t0::n412(s, out),
				['n', s @ ..] => super::t0::n413(s, out),
				['r', 'o', 'r', s @ ..] => super::t0::n414(s, out),
				['s', 't', s @ ..] => super::t0::n415(s, out),
				['t', s @ ..] => super::t0::n416(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -g.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 404);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 405);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 406);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1514);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1028);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1029);
							s.is_empty()
						}
						['t', 't', 'o', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 407);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1516);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1517);
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
			super::push(out, 81);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 408);
					s.is_empty()
				}
				['o', 's', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1031);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1032);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 83);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 411);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1034);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1035);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['l', 'u', s @ ..] => {
					super::push(out, 412);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 413);
					match s {
						[] => true,
						['d', 'i', 't', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1038);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 415);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['o', 'r', 'i', s @ ..] => {
							super::push(out, 1040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['m', 'e', 'o', s @ ..] => {
					super::push(out, 420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1041);
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
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 90);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 91);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 92);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1042);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1043);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1044);
							match s {
								[] => true,
								['c', 'e', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1738);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 1739);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 1518);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1519);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1520);
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
			super::push(out, 95);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 421);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 422);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 1045);
							match s {
								[] => true,
								['r', 'i', 'c', 'u', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 423);
					match s {
						[] => true,
						['r', 'o', 'm', s @ ..] => {
							super::push(out, 1046);
							match s {
								[] => true,
								['u', 't', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1522);
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
			super::push(out, 96);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1047);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1048);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1049);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1050);
							match s {
								[] => true,
								['c', 't', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1523);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 1524);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1525);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 425);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1052);
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 1527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1053);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1054);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 1528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1055);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 1529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 426);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							super::push(out, 1057);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1530);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 1531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 427);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1058);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 428);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1060);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1532);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1740);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1061);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 429);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 430);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1062);
							match s {
								[] => true,
								['r', 'e', 'd', 'i', 't', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 431);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							super::push(out, 1063);
							match s {
								[] => true,
								['i', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 432);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1064);
							match s {
								[] => true,
								['c', 'e', 's', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1536);
									s.is_empty()
								}
								['m', 'i', 'n', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1537);
									s.is_empty()
								}
								['s', 't', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				['n', s @ ..] => super::t0::n450(s, out),
				['o', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 436);
					s.is_empty()
				}
				['q', s @ ..] => super::t0::n453(s, out),
				['r', s @ ..] => super::t0::n454(s, out),
				['s', s @ ..] => super::t0::n455(s, out),
				['t', s @ ..] => super::t0::n456(s, out),
				['u', s @ ..] => super::t0::n457(s, out),
				['v', s @ ..] => super::t0::n458(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n99(s, out),
		['r', s @ ..] => super::t0::n100(s, out),
		['t', s @ ..] => super::t0::n101(s, out),
		['u', s @ ..] => super::t0::n102(s, out),
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['b', 'o', 'r', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1079);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 454);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1081);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1083);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 458);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1085);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1087);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1088);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 1555);
									match s {
										[] => true,
										['z', 'o', '\u{1}', s @ ..] => {
											super::push(out, 1750);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1089);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									super::push(out, 1556);
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
			super::push(out, 102);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['g', 'o', '\u{1}', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 463);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1090);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1091);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1558);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 1093);
							match s {
								[] => true,
								['c', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 1560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 469);
					match s {
						[] => true,
						['p', 'i', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 471);
					s.is_empty()
				}
				['q', 'u', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1095);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 1096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1097);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['b', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 475);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1099);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 477);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 1101);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 483);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 485);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 487);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1104);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1105);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1561);
									s.is_empty()
								}
								['s', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1106);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1563);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 1564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1107);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1108);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1109);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1110);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1565);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1566);
									s.is_empty()
								}
								['t', 'e', 'n', 'e', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 488);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 489);
					match s {
						[] => true,
						['u', 'r', 'e', s @ ..] => {
							super::push(out, 1111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 491);
					s.is_empty()
				}
				['n', 's', s @ ..] => {
					super::push(out, 492);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 494);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1112);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 1113);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1115);
							match s {
								[] => true,
								['s', 'c', s @ ..] => {
									super::push(out, 1568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1116);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1569);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1570);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1571);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 501);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1118);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 's', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				['r', 'i', 'g', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 507);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 509);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1120);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 511);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1122);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1123);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1124);
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

/// Words ending -n.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1125);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1126);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['g', 'o', s @ ..] => {
					super::push(out, 518);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1129);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1131);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				['n', 'c', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['o', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							super::push(out, 1133);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1135);
							s.is_empty()
						}
						['u', 'r', s @ ..] => {
							super::push(out, 1136);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1572);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['d', 'i', s @ ..] => {
							super::push(out, 1137);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 1574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 540);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1139);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 1575);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1141);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1576);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1142);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1578);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1143);
							match s {
								[] => true,
								['b', 'o', s @ ..] => {
									super::push(out, 1580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 117);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 545);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 119);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1146);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1148);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 551);
					s.is_empty()
				}
				['g', 'a', 'n', 'i', 'z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['v', 'i', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 560);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1150);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1581);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 562);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1152);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1153);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 564);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1155);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1583);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1156);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1157);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1159);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									super::push(out, 1585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						['i', 'm', s @ ..] => {
							super::push(out, 1161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1162);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1751);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 1752);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1164);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1166);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1587);
									match s {
										[] => true,
										['e', 'b', 'r', s @ ..] => {
											super::push(out, 1753);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1167);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1588);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 1754);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 1755);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1168);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 1590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1169);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1170);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1592);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 1756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 1172);
							match s {
								[] => true,
								['u', 'r', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1593);
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
			super::push(out, 129);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				['l', 'p', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1173);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 577);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1175);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1176);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							super::push(out, 1178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 581);
					match s {
						[] => true,
						['u', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['n', 'i', s @ ..] => super::t0::n1239(s, out),
						['t', s @ ..] => {
							super::push(out, 1180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n611(s, out),
				['n', s @ ..] => super::t0::n612(s, out),
				['r', 'c', s @ ..] => super::t0::n613(s, out),
				['s', s @ ..] => super::t0::n614(s, out),
				['t', s @ ..] => super::t0::n615(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n134(s, out),
		['u', s @ ..] => super::t0::n135(s, out),
		_ => false,
	}
}

/// Words ending -q.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1214);
							match s {
								[] => true,
								['i', 'f', 'i', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1215);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1216);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1217);
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

/// Words ending -r.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 601);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1626);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1764);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1765);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1627);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'f', 'i', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['i', 'o', s @ ..] => {
							super::push(out, 1219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1220);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1221);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1222);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1629);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1766);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 1767);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1223);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1630);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1226);
							match s {
								[] => true,
								['r', 't', s @ ..] => {
									super::push(out, 1632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1227);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1633);
									match s {
										[] => true,
										['i', 'l', 'l', 'o', '\u{1}', s @ ..] => {
											super::push(out, 1768);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1634);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1228);
							match s {
								[] => true,
								['i', 'g', s @ ..] => {
									super::push(out, 1635);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 1805);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 1806);
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
				['g', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1230);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1231);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1636);
									s.is_empty()
								}
								['r', 'g', 'i', 't', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1637);
									s.is_empty()
								}
								['s', 't', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1232);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1769);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 1770);
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
					super::push(out, 613);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1233);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1639);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t0::n1300(s, out),
						['o', s @ ..] => {
							super::push(out, 1235);
							match s {
								[] => true,
								['l', 'i', s @ ..] => super::t0::n1725(s, out),
								['n', 's', 't', 'r', 'o', '\u{1}', s @ ..] => super::t0::n1726(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => super::t0::n1302(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n645(s, out),
				['p', s @ ..] => super::t0::n646(s, out),
				['s', s @ ..] => super::t0::n647(s, out),
				['t', s @ ..] => super::t0::n648(s, out),
				['u', s @ ..] => super::t0::n649(s, out),
				['v', s @ ..] => super::t0::n650(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n139(s, out),
		['o', s @ ..] => super::t0::n140(s, out),
		['u', s @ ..] => super::t0::n141(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['g', 'i', 'o', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 642);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 643);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1657);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				['v', 'i', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1260);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						['t', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1263);
							match s {
								[] => true,
								['b', 'o', s @ ..] => {
									super::push(out, 1659);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1660);
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
			super::push(out, 141);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1661);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1662);
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
						['e', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 650);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 1271);
							match s {
								[] => true,
								['o', 'r', 'm', 'i', 't', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						['i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1276);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1277);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1279);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 1778);
											s.is_empty()
										}
										['n', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 1779);
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
						['i', s @ ..] => {
							super::push(out, 1280);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1282);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1285);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1664);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1665);
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
			super::push(out, 145);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1286);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						['p', 'e', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n149(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 1309);
							match s {
								[] => true,
								['n', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1671);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['g', 'o', '\u{1}', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['l', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 689);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['e', 's', 'a', 'u', 'r', 'i', 'z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1313);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1672);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1314);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 1674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1316);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1675);
									s.is_empty()
								}
								['q', s @ ..] => {
									super::push(out, 1676);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1677);
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
				['i', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['b', 'u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1678);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'o', 'r', s @ ..] => {
							super::push(out, 1317);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1318);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1319);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1680);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['s', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['c', 't', s @ ..] => {
					super::push(out, 704);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 708);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['n', 'o', 'r', s @ ..] => {
							super::push(out, 1323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -v.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['u', 'e', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1325);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1327);
							s.is_empty()
						}
						['i', 't', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['r', 'i', 'c', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1329);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1330);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1332);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1682);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1333);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1334);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['m', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1684);
									s.is_empty()
								}
								['n', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 725);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1335);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1686);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1336);
							match s {
								[] => true,
								['f', 'i', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1337);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1338);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1689);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1340);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1342);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1343);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'n', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						['t', 'i', 't', s @ ..] => {
							super::push(out, 1346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['i', 'p', s @ ..] => {
							super::push(out, 1347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1348);
							s.is_empty()
						}
						['d', 'e', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				['r', 'i', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1351);
							s.is_empty()
						}
						['u', 'l', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1690);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1691);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -goc.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 857);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 858);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 859);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -loc.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 860);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -moc.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1705);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1787);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'n', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['i', 'c', 'o', 'r', s @ ..] => {
							super::push(out, 1706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1789);
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
			super::push(out, 864);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -noc.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 866);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1428);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1790);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1707);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1708);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1709);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1430);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 868);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1432);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1711);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 1713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 872);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 873);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 874);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['s', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 876);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 877);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 879);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1715);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1792);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1716);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['b', 'e', s @ ..] => {
							super::push(out, 1718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1719);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							super::push(out, 1721);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1794);
									s.is_empty()
								}
								['t', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1795);
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
			super::push(out, 881);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['v', 'i', s @ ..] => {
							super::push(out, 1722);
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

/// Words ending -ooc.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	s.is_empty()
}

/// Words ending -poc.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	s.is_empty()
}

/// Words ending -qoc.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	s.is_empty()
}

/// Words ending -roc.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	s.is_empty()
}

/// Words ending -uoc.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	s.is_empty()
}

/// Words ending -rc.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['p', 'u', 'l', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 886);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 888);
					s.is_empty()
				}
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 889);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 891);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uc.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['l', 'o', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 283);
			s.is_empty()
		}
		['n', 'c', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sped.
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 921);
	s.is_empty()
}

/// Words ending -red.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	s.is_empty()
}

/// Words ending -sed.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1466);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 1732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1733);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1734);
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

/// Words ending -ted.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	s.is_empty()
}

/// Words ending -ued.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 924);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 925);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ved.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	s.is_empty()
}

/// Words ending -id.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 305);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 928);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['o', 's', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1471);
							match s {
								[] => true,
								['e', 'o', s @ ..] => {
									super::push(out, 1735);
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
			super::push(out, 310);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -od.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rd.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 321);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -save.
pub(crate) fn n1020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 970);
	s.is_empty()
}

/// Words ending -eve.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	s.is_empty()
}

/// Words ending -ive.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['l', 'e', 's', 'c', 'o', '\u{1}', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ove.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	s.is_empty()
}

/// Words ending -xe.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 974);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['t', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'c', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 363);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -guf.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	s.is_empty()
}

/// Words ending -luf.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	s.is_empty()
}

/// Words ending -muf.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	s.is_empty()
}

/// Words ending -nuf.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1023);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -roruf.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	s.is_empty()
}

/// Words ending -tsuf.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	s.is_empty()
}

/// Words ending -tuf.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1025);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nni.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1539);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1067);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qni.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	s.is_empty()
}

/// Words ending -rni.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	s.is_empty()
}

/// Words ending -sni.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1068);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 1741);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1798);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1799);
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
			super::push(out, 1069);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1070);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				['i', 's', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		['u', 'd', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tni.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1074);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1075);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1744);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1746);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1749);
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

/// Words ending -uni.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	s.is_empty()
}

/// Words ending -vni.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	s.is_empty()
}

/// Words ending -oi.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	s.is_empty()
}

/// Words ending -ri.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ti.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	s.is_empty()
}

/// Words ending -ui.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 446);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 447);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 448);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 't', 'i', 'f', 'i', 'c', 'o', '\u{1}', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 451);
			s.is_empty()
		}
		['x', 't', 'a', 'p', 'o', 'n', 'o', '\u{1}', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ineop.
pub(crate) fn n1239(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', '\u{1}', s @ ..] => {
			super::push(out, 1594);
			s.is_empty()
		}
		['t', 'e', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1595);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lop.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 583);
	s.is_empty()
}

/// Words ending -nop.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 584);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1181);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -crop.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 585);
	match s {
		[] => true,
		['e', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sop.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 586);
	s.is_empty()
}

/// Words ending -top.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 587);
	match s {
		[] => true,
		['e', 'v', 'o', 'l', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1184);
			s.is_empty()
		}
		['i', 'o', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1185);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rp.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1598);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1757);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 1800);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1599);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1600);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1759);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 1801);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1802);
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
					super::push(out, 1187);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['b', 'e', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['h', 'i', 'b', 'e', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 590);
			match s {
				[] => true,
				['n', 'c', 'i', 'p', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 591);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1193);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1803);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 1804);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1760);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'r', 'e', 'd', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1611);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['o', 'e', 'm', 'i', 'z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1613);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1761);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1762);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1614);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1615);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1616);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1617);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['c', 'u', s @ ..] => {
							super::push(out, 1619);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1620);
							s.is_empty()
						}
						['p', 'e', 'r', s @ ..] => {
							super::push(out, 1621);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'v', s @ ..] => {
							super::push(out, 1623);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 592);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -up.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 593);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 595);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 596);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['c', 't', 'u', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['g', 'o', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1210);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imer.
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1234);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1641);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ilomer.
pub(crate) fn n1725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1642);
	s.is_empty()
}

/// The word ortsnomer.
pub(crate) fn n1726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1643);
	s.is_empty()
}

/// Words ending -umer.
pub(crate) fn n1302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1236);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1644);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ner.
pub(crate) fn n645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 614);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1238);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1239);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1240);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1241);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1243);
			match s {
				[] => true,
				['o', 'm', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1244);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	s.is_empty()
}

/// Words ending -ter.
pub(crate) fn n648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 617);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1246);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1650);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1651);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1775);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1776);
							s.is_empty()
						}
						['v', 'e', 'n', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1777);
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

/// Words ending -uer.
pub(crate) fn n649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 618);
	match s {
		[] => true,
		['n', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1247);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ver.
pub(crate) fn n650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 619);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1248);
			s.is_empty()
		}
		['i', 'v', s @ ..] => {
			super::push(out, 1249);
			match s {
				[] => true,
				['e', 's', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1653);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ir.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 620);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 622);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 623);
			s.is_empty()
		}
		['t', 'o', '\u{1}', s @ ..] => {
			super::push(out, 624);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 626);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 628);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 631);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['g', 'i', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['i', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 632);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 633);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 634);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 640);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 671);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1666);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1780);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1293);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1782);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1783);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1294);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1669);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1785);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 673);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 674);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['s', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 676);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1298);
					s.is_empty()
				}
				['i', 'l', 'l', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1305);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1306);
					match s {
						[] => true,
						['a', 's', 'e', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 682);
			s.is_empty()
		}
		_ => false,
	}
}

