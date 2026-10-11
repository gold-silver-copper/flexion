use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => super::t0::n1(s, out),
		['b', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 60);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 61);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 62);
					match s {
						[] => true,
						['o', 'l', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 63);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', '\u{1}', s @ ..] => {
					super::push(out, 64);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['d', 'n', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 560);
							s.is_empty()
						}
						['l', 'c', 'o', s @ ..] => {
							super::push(out, 561);
							match s {
								[] => true,
								['e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1759);
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
		['c', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 66);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 562);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 563);
							s.is_empty()
						}
						['m', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 564);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 565);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 566);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 567);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1760);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 568);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 569);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 67);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 68);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 69);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 573);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 574);
							match s {
								[] => true,
								['í', 's', 'a', 'c', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1762);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 't', '\u{1}', s @ ..] => {
					super::push(out, 70);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 71);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 575);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 72);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 73);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 74);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'u', '\u{1}', s @ ..] => {
					super::push(out, 75);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 76);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 579);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1763);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2832);
											match s {
												[] => true,
												['i', 't', 'n', s @ ..] => {
													super::push(out, 3295);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 2833);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 3296);
													match s {
														[] => true,
														['e', s @ ..] => super::t0::n3635(s, out),
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
						['e', 'h', 'n', 'i', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 580);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 581);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 582);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1764);
									s.is_empty()
								}
								['k', 'c', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 1765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 583);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 1766);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', 'w', '\u{1}', s @ ..] => {
					super::push(out, 77);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 78);
					s.is_empty()
				}
				['d', 'p', '\u{1}', s @ ..] => {
					super::push(out, 79);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 80);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 584);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 585);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'c', '\u{1}', s @ ..] => {
					super::push(out, 81);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 82);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 587);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'h', 't', 'r', 'a', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 588);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 589);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n84(s, out),
				['m', s @ ..] => super::t0::n85(s, out),
				['n', s @ ..] => super::t0::n86(s, out),
				['o', s @ ..] => super::t0::n87(s, out),
				['r', s @ ..] => super::t0::n88(s, out),
				['s', s @ ..] => super::t0::n89(s, out),
				['u', s @ ..] => super::t0::n90(s, out),
				['v', 'd', '\u{1}', s @ ..] => super::t0::n91(s, out),
				['ú', s @ ..] => super::t0::n92(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n5(s, out),
		['f', s @ ..] => super::t0::n6(s, out),
		['g', s @ ..] => super::t0::n7(s, out),
		['h', s @ ..] => super::t0::n8(s, out),
		['i', s @ ..] => super::t0::n9(s, out),
		['j', s @ ..] => super::t0::n10(s, out),
		['k', s @ ..] => super::t0::n11(s, out),
		['l', s @ ..] => super::t0::n12(s, out),
		['m', s @ ..] => super::t0::n13(s, out),
		['n', s @ ..] => super::t0::n14(s, out),
		['o', s @ ..] => super::t0::n15(s, out),
		['p', s @ ..] => super::t0::n16(s, out),
		['q', s @ ..] => super::t0::n17(s, out),
		['r', s @ ..] => super::t0::n18(s, out),
		['s', s @ ..] => super::t0::n19(s, out),
		['t', s @ ..] => super::t0::n20(s, out),
		['u', s @ ..] => super::t0::n21(s, out),
		['v', s @ ..] => super::t0::n22(s, out),
		['w', s @ ..] => super::t0::n23(s, out),
		['x', s @ ..] => super::t0::n24(s, out),
		['y', s @ ..] => super::t0::n25(s, out),
		['z', s @ ..] => super::t0::n26(s, out),
		['á', s @ ..] => super::t0::n27(s, out),
		['é', s @ ..] => super::t0::n28(s, out),
		['í', s @ ..] => super::t0::n29(s, out),
		['ó', s @ ..] => super::t0::n30(s, out),
		['ú', s @ ..] => super::t0::n31(s, out),
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				['i', 'r', 'c', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						['o', 'b', 'o', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
						}
						['u', 'z', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1378);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'i', 'h', 'a', s @ ..] => {
					super::push(out, 396);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 398);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1381);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1382);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						['s', 'e', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 399);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1386);
							s.is_empty()
						}
						['m', 'l', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1388);
							match s {
								[] => true,
								['l', 'a', 'c', s @ ..] => {
									super::push(out, 2581);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2582);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 2583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 401);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 1390);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1395);
							match s {
								[] => true,
								['r', 'r', s @ ..] => {
									super::push(out, 2584);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1396);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 2585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 404);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1398);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1400);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 2587);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 405);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1402);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2588);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 2589);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2590);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2591);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2592);
									match s {
										[] => true,
										['o', 'c', s @ ..] => {
											super::push(out, 3197);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2593);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3198);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2594);
									match s {
										[] => true,
										['e', 'j', s @ ..] => {
											super::push(out, 3199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', s @ ..] => {
					super::push(out, 406);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 407);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 408);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 1406);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2595);
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
		['d', s @ ..] => super::t0::n35(s, out),
		['e', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 417);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'í', s @ ..] => {
					super::push(out, 419);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 420);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 422);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 423);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', s @ ..] => {
					super::push(out, 425);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 426);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t0::n38(s, out),
		['h', s @ ..] => super::t0::n39(s, out),
		['i', s @ ..] => super::t0::n40(s, out),
		['j', s @ ..] => super::t0::n41(s, out),
		['k', s @ ..] => super::t0::n42(s, out),
		['l', s @ ..] => super::t0::n43(s, out),
		['m', s @ ..] => super::t0::n44(s, out),
		['n', s @ ..] => super::t0::n45(s, out),
		['o', s @ ..] => {
			super::push(out, 46);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t0::n47(s, out),
		['r', s @ ..] => super::t0::n48(s, out),
		['s', s @ ..] => super::t0::n49(s, out),
		['t', s @ ..] => super::t0::n50(s, out),
		['u', s @ ..] => super::t0::n51(s, out),
		['v', s @ ..] => super::t0::n52(s, out),
		['w', s @ ..] => super::t0::n53(s, out),
		['y', s @ ..] => super::t0::n54(s, out),
		['z', s @ ..] => super::t0::n55(s, out),
		['ç', s @ ..] => super::t0::n56(s, out),
		['í', s @ ..] => super::t0::n57(s, out),
		['ñ', s @ ..] => super::t0::n58(s, out),
		['ú', s @ ..] => super::t0::n59(s, out),
		_ => false,
	}
}

/// Words ending -ealidad.
pub(crate) fn n3635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	match s {
		[] => true,
		['n', 'i', 'l', 'i', s @ ..] => {
			super::push(out, 3556);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ld.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	s.is_empty()
}

/// Words ending -md.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 591);
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 592);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nd.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 593);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 595);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -od.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 596);
			s.is_empty()
		}
		['o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rd.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['d', 'n', 'á', 't', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				['h', 'y', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sd.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['l', '\u{1}', s @ ..] => {
			super::push(out, 600);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 602);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 604);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dvd.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	s.is_empty()
}

/// Words ending -úd.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['r', 'á', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1775);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1776);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 2834);
									s.is_empty()
								}
								['í', s @ ..] => {
									super::push(out, 2835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 1780);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1781);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2836);
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
			super::push(out, 95);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1782);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1783);
							s.is_empty()
						}
						['í', s @ ..] => {
							super::push(out, 1784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1786);
							match s {
								[] => true,
								['á', s @ ..] => {
									super::push(out, 2837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1787);
							match s {
								[] => true,
								['á', s @ ..] => {
									super::push(out, 2838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1788);
							match s {
								[] => true,
								['c', 'o', s @ ..] => {
									super::push(out, 2839);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 3297);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 3298);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 3489);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 3299);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3300);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 3301);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3302);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 3303);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', s @ ..] => {
									super::push(out, 2840);
									s.is_empty()
								}
								['z', 'i', s @ ..] => {
									super::push(out, 2841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1789);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1790);
							match s {
								[] => true,
								['á', s @ ..] => {
									super::push(out, 2843);
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
						['a', s @ ..] => {
							super::push(out, 1791);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1795);
							match s {
								[] => true,
								['v', 'i', s @ ..] => {
									super::push(out, 2844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['e', 'j', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							super::push(out, 1799);
							match s {
								[] => true,
								['s', 'á', s @ ..] => {
									super::push(out, 2845);
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
			super::push(out, 97);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 625);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2846);
									s.is_empty()
								}
								['r', s @ ..] => super::t0::n2955(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['y', s @ ..] => super::t0::n1868(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n99(s, out),
		['i', s @ ..] => super::t0::n100(s, out),
		['j', s @ ..] => super::t0::n101(s, out),
		['k', s @ ..] => super::t0::n102(s, out),
		['l', s @ ..] => super::t0::n103(s, out),
		['m', s @ ..] => super::t0::n104(s, out),
		['n', s @ ..] => super::t0::n105(s, out),
		['o', s @ ..] => super::t0::n106(s, out),
		['p', s @ ..] => super::t0::n107(s, out),
		['r', s @ ..] => super::t0::n108(s, out),
		['s', s @ ..] => super::t0::n109(s, out),
		['t', s @ ..] => super::t0::n110(s, out),
		['u', s @ ..] => super::t0::n111(s, out),
		['v', s @ ..] => super::t0::n112(s, out),
		['w', '\u{1}', s @ ..] => super::t0::n113(s, out),
		['y', s @ ..] => super::t0::n114(s, out),
		['é', s @ ..] => super::t0::n115(s, out),
		['í', s @ ..] => super::t0::n116(s, out),
		['ñ', s @ ..] => super::t0::n117(s, out),
		['ü', 'g', 'i', s @ ..] => super::t0::n118(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['d', 'p', '\u{1}', s @ ..] => {
			super::push(out, 118);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 119);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 120);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 704);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'r', 'i', '\u{1}', s @ ..] => {
			super::push(out, 124);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 708);
					s.is_empty()
				}
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
			super::push(out, 127);
			s.is_empty()
		}
		['c', 'e', '\u{1}', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1916);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2928);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['a', 'c', '\u{1}', s @ ..] => {
													super::push(out, 3536);
													s.is_empty()
												}
												['á', 'c', '\u{1}', s @ ..] => {
													super::push(out, 3537);
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
				['o', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'm', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['s', 'e', '\u{1}', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 132);
			match s {
				[] => true,
				['r', 'm', 'u', '\u{1}', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['e', 's', 'i', 'o', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 1918);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						['r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['l', 'm', 'o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1922);
							s.is_empty()
						}
						['v', 'é', 'r', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1923);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['u', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['n', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1926);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2929);
									s.is_empty()
								}
								['u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 2931);
									s.is_empty()
								}
								['w', 't', 'r', 'a', 'm', 's', '\u{1}', s @ ..] => {
									super::push(out, 2932);
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
			super::push(out, 134);
			match s {
				[] => true,
				['i', 'e', 'l', 's', 'b', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1927);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
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
			super::push(out, 137);
			match s {
				[] => true,
				['g', 'h', 's', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 1929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				['m', 'o', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['b', 'n', 'a', 'i', 'x', '\u{1}', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['y', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['o', 'r', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', s @ ..] => {
							super::push(out, 1930);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'g', 'n', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					super::push(out, 747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1933);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1934);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 753);
					match s {
						[] => true,
						['l', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 754);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['z', 'r', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 760);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1939);
							match s {
								[] => true,
								['n', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'a', 'd', 'n', 'e', 'l', s @ ..] => {
							super::push(out, 1940);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1941);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 2934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1942);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1945);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 2935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['i', 'f', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1948);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1950);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 1951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', 'i', 'u', 'r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['a', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['á', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['l', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -j.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['d', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['s', 'm', '\u{1}', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['h', 'c', 'a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				['y', '\u{1}', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['í', 'n', 'd', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1953);
							s.is_empty()
						}
						['í', s @ ..] => {
							super::push(out, 1954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						['o', 'v', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1959);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 1961);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1963);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1964);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2938);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2939);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2940);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 3389);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', s @ ..] => {
							super::push(out, 1965);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1967);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2942);
									match s {
										[] => true,
										['l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3390);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2943);
									s.is_empty()
								}
								['é', 'l', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1969);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'r', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1971);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1972);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1973);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2946);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2947);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 2948);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2949);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3391);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1974);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2951);
									s.is_empty()
								}
								['i', s @ ..] => super::t0::n3071(s, out),
								['m', s @ ..] => super::t0::n3072(s, out),
								['r', s @ ..] => super::t0::n3073(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n2054(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t0::n831(s, out),
				['r', s @ ..] => super::t0::n832(s, out),
				['s', s @ ..] => super::t0::n833(s, out),
				['t', s @ ..] => super::t0::n834(s, out),
				['u', s @ ..] => super::t0::n835(s, out),
				['v', s @ ..] => super::t0::n836(s, out),
				['z', s @ ..] => super::t0::n837(s, out),
				['ñ', s @ ..] => super::t0::n838(s, out),
				_ => false,
			}
		}
		['d', 'd', '\u{1}', s @ ..] => super::t0::n175(s, out),
		['e', s @ ..] => super::t0::n176(s, out),
		['i', s @ ..] => super::t0::n177(s, out),
		['l', s @ ..] => super::t0::n178(s, out),
		['m', '\u{1}', s @ ..] => super::t0::n179(s, out),
		['o', s @ ..] => super::t0::n180(s, out),
		['q', '\u{1}', s @ ..] => super::t0::n181(s, out),
		['r', 's', '\u{1}', s @ ..] => super::t0::n182(s, out),
		['s', 'd', 'a', '\u{1}', s @ ..] => super::t0::n183(s, out),
		['t', 'e', 'm', 'o', 'l', 'a', 'p', 'a', 'p', '\u{1}', s @ ..] => super::t0::n184(s, out),
		['u', s @ ..] => super::t0::n185(s, out),
		['w', 'o', 'b', '\u{1}', s @ ..] => super::t0::n186(s, out),
		['í', s @ ..] => super::t0::n187(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 849);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 850);
					s.is_empty()
				}
				['f', 's', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 853);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2044);
							s.is_empty()
						}
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						['m', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'k', 'l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 856);
					match s {
						[] => true,
						['g', 'á', 'l', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['n', 'e', 'p', 'a', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 857);
					s.is_empty()
				}
				['t', 'í', '\u{1}', s @ ..] => {
					super::push(out, 858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'p', '\u{1}', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		['g', 'o', '\u{1}', s @ ..] => {
			super::push(out, 190);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['f', 'a', 'r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['n', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 193);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['c', 'o', 't', 'n', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 860);
					s.is_empty()
				}
				['l', 'a', 'l', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 861);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 862);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['t', 'u', '\u{1}', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 864);
					match s {
						[] => true,
						['i', 'u', 'q', 's', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				['l', 'u', 'c', 'í', 'r', 'r', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 866);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 867);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2051);
							match s {
								[] => true,
								['ú', 's', '\u{1}', s @ ..] => {
									super::push(out, 2969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 868);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 869);
					match s {
						[] => true,
						['t', 's', 'p', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 870);
					match s {
						[] => true,
						['n', 'é', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						['í', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 2970);
									s.is_empty()
								}
								['á', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 2971);
									s.is_empty()
								}
								_ => false,
							}
						}
						['á', 'm', 'i', 't', 'l', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						['ó', 't', 'c', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2056);
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
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['a', 'n', 'u', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 874);
					match s {
						[] => true,
						['i', 'x', 'e', 't', 'i', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['a', 'l', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2060);
							match s {
								[] => true,
								['l', 'o', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 2972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2062);
							match s {
								[] => true,
								['l', 's', '\u{1}', s @ ..] => {
									super::push(out, 2973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						['u', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 879);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2066);
							match s {
								[] => true,
								['r', 'o', 'c', 'é', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'l', 't', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2068);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2069);
							match s {
								[] => true,
								['h', 'c', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 2976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'l', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2070);
							s.is_empty()
						}
						['l', 'u', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2071);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2072);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 2977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2073);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2978);
									s.is_empty()
								}
								['e', 'b', 'ó', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2074);
							match s {
								[] => true,
								['e', 'u', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 884);
					match s {
						[] => true,
						['e', 'm', 'ú', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				['t', 'i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 886);
					s.is_empty()
				}
				['u', 'y', '\u{1}', s @ ..] => {
					super::push(out, 887);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2078);
							match s {
								[] => true,
								['o', 'o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2079);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2982);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2080);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2081);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2984);
									match s {
										[] => true,
										['v', 'a', 'l', 'a', 'v', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 3399);
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
				['h', 'c', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 893);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						['t', 'u', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['i', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				['m', s @ ..] => super::t0::n934(s, out),
				['p', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['m', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['a', 'l', 'c', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2096);
							match s {
								[] => true,
								['s', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 899);
					match s {
						[] => true,
						['e', 't', 'a', 'c', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						['i', 'm', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						['l', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						['s', 'e', 't', 'a', 'c', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['u', 'l', 'g', '\u{1}', s @ ..] => super::t0::n2182(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t0::n939(s, out),
				['v', s @ ..] => super::t0::n940(s, out),
				_ => false,
			}
		}
		['g', '\u{1}', s @ ..] => super::t0::n202(s, out),
		['i', s @ ..] => super::t0::n203(s, out),
		['n', s @ ..] => super::t0::n204(s, out),
		['o', s @ ..] => super::t0::n205(s, out),
		['p', 'd', '\u{1}', s @ ..] => super::t0::n206(s, out),
		['r', s @ ..] => super::t0::n207(s, out),
		['u', s @ ..] => super::t0::n208(s, out),
		['w', s @ ..] => super::t0::n209(s, out),
		['y', 'a', 'z', '\u{1}', s @ ..] => super::t0::n210(s, out),
		['á', s @ ..] => super::t0::n211(s, out),
		['é', s @ ..] => super::t0::n212(s, out),
		['í', s @ ..] => super::t0::n213(s, out),
		['ó', s @ ..] => super::t0::n214(s, out),
		['ú', s @ ..] => super::t0::n215(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 991);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['i', 'j', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				['e', 'n', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2202);
							s.is_empty()
						}
						['d', 'é', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2205);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 2206);
							s.is_empty()
						}
						['s', 'é', s @ ..] => {
							super::push(out, 2207);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2208);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3035);
									s.is_empty()
								}
								['á', s @ ..] => {
									super::push(out, 3036);
									match s {
										[] => true,
										['m', 'a', s @ ..] => {
											super::push(out, 3416);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['í', s @ ..] => {
									super::push(out, 3037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 2210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1002);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2211);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2212);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1003);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2214);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3040);
									s.is_empty()
								}
								['ó', 'l', s @ ..] => {
									super::push(out, 3041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1004);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2215);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3042);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 3418);
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
					super::push(out, 1005);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2216);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3043);
									match s {
										[] => true,
										['l', 'e', 'm', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3419);
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
				['u', 'c', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				['l', 'ú', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				['r', 'ó', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['r', 'g', 'ó', 'l', s @ ..] => {
							super::push(out, 2219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2221);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2222);
							match s {
								[] => true,
								['á', s @ ..] => {
									super::push(out, 3044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1018);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							super::push(out, 2224);
							match s {
								[] => true,
								['a', 'j', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3045);
									s.is_empty()
								}
								['o', 'j', 'i', s @ ..] => {
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 3420);
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
											super::push(out, 3421);
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
					super::push(out, 1020);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1022);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 2226);
							match s {
								[] => true,
								['n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 3046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['a', s @ ..] => super::t0::n2309(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n1064(s, out),
				['r', s @ ..] => super::t0::n1065(s, out),
				['t', s @ ..] => super::t0::n1066(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t0::n226(s, out),
		['k', s @ ..] => super::t0::n227(s, out),
		['l', s @ ..] => super::t0::n228(s, out),
		['m', s @ ..] => super::t0::n229(s, out),
		['n', s @ ..] => super::t0::n230(s, out),
		['p', s @ ..] => super::t0::n231(s, out),
		['r', s @ ..] => super::t0::n232(s, out),
		['s', s @ ..] => super::t0::n233(s, out),
		['t', s @ ..] => super::t0::n234(s, out),
		['u', s @ ..] => super::t0::n235(s, out),
		['v', s @ ..] => super::t0::n236(s, out),
		['y', s @ ..] => super::t0::n237(s, out),
		['z', s @ ..] => super::t0::n238(s, out),
		['í', 't', s @ ..] => super::t0::n239(s, out),
		['ñ', 'e', s @ ..] => super::t0::n240(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 239);
			s.is_empty()
		}
		['c', 'c', '\u{1}', s @ ..] => {
			super::push(out, 240);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['l', 's', '\u{1}', s @ ..] => {
					super::push(out, 1073);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 247);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -q.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	s.is_empty()
}

/// Words ending -r.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['k', 'c', 'a', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 2284);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2286);
							s.is_empty()
						}
						['ú', s @ ..] => {
							super::push(out, 2287);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 3071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1081);
					match s {
						[] => true,
						['d', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['t', 'o', 'b', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2290);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2291);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3072);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 3073);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1088);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				['q', 's', 'é', 'w', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2293);
							match s {
								[] => true,
								['n', 'g', 'á', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2294);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2295);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2296);
							match s {
								[] => true,
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 3076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2297);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2298);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3077);
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
		['c', 'p', '\u{1}', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		['e', s @ ..] => super::t0::n253(s, out),
		['g', 'p', '\u{1}', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1115);
					match s {
						[] => true,
						['e', 'v', 'u', s @ ..] => {
							super::push(out, 2385);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1117);
					match s {
						[] => true,
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
		['l', 'b', 'm', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 254);
			s.is_empty()
		}
		['n', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 255);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 3149);
									match s {
										[] => true,
										['c', 'n', 'i', s @ ..] => {
											super::push(out, 3442);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									super::push(out, 3150);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3443);
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
				['h', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['n', 'ú', 'y', '\u{1}', s @ ..] => {
							super::push(out, 2388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2389);
							s.is_empty()
						}
						['i', 'é', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2391);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2392);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', s @ ..] => {
							super::push(out, 2393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							super::push(out, 2395);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2396);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 3152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['d', 'n', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 2398);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2399);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', 'j', '\u{1}', s @ ..] => {
											super::push(out, 3444);
											s.is_empty()
										}
										['á', s @ ..] => {
											super::push(out, 3445);
											match s {
												[] => true,
												['j', '\u{1}', s @ ..] => {
													super::push(out, 3549);
													s.is_empty()
												}
												['y', '\u{1}', s @ ..] => {
													super::push(out, 3550);
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
				['y', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		['u', s @ ..] => super::t0::n260(s, out),
		['v', 'u', '\u{1}', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		['y', 'g', '\u{1}', s @ ..] => {
			super::push(out, 260);
			s.is_empty()
		}
		['í', 'h', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2405);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2406);
							match s {
								[] => true,
								['u', 'g', 'a', 'm', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'b', s @ ..] => {
							super::push(out, 2407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2408);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2409);
							match s {
								[] => true,
								['n', 'ó', 'i', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'r', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				['i', 'b', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2411);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2412);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1146);
					match s {
						[] => true,
						['e', 'u', s @ ..] => {
							super::push(out, 2414);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3158);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3159);
									match s {
										[] => true,
										['p', 'o', 't', 'n', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 3446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'b', s @ ..] => {
							super::push(out, 2415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1147);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1148);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2417);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2418);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2420);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2421);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2422);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3161);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2424);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2426);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3163);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2427);
							match s {
								[] => true,
								['e', 'h', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2428);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				['v', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1154);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ñ', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'i', 'o', 'r', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1159);
					match s {
						[] => true,
						['c', 'o', 'c', 'a', 'd', s @ ..] => {
							super::push(out, 2433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1160);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1162);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 2437);
							match s {
								[] => true,
								['c', 'a', 't', s @ ..] => {
									super::push(out, 3166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1164);
					match s {
						[] => true,
						['o', 'p', 'a', 'x', '\u{1}', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						['b', 'm', 'o', s @ ..] => {
							super::push(out, 2442);
							s.is_empty()
						}
						['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t0::n1212(s, out),
				['u', 'q', 'r', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n266(s, out),
		['m', s @ ..] => super::t0::n267(s, out),
		['n', s @ ..] => super::t0::n268(s, out),
		['o', s @ ..] => super::t0::n269(s, out),
		['p', s @ ..] => {
			super::push(out, 268);
			s.is_empty()
		}
		['r', s @ ..] => super::t0::n271(s, out),
		['s', s @ ..] => super::t0::n272(s, out),
		['t', s @ ..] => super::t0::n273(s, out),
		['u', s @ ..] => super::t0::n274(s, out),
		['á', s @ ..] => super::t0::n275(s, out),
		['é', s @ ..] => super::t0::n276(s, out),
		['í', s @ ..] => super::t0::n277(s, out),
		['ó', s @ ..] => super::t0::n278(s, out),
		['ú', s @ ..] => super::t0::n279(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				['d', 'i', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', '\u{1}', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				['f', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1238);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2511);
							s.is_empty()
						}
						['á', s @ ..] => {
							super::push(out, 2512);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1241);
					match s {
						[] => true,
						['q', 'i', s @ ..] => {
							super::push(out, 2514);
							match s {
								[] => true,
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3188);
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
		['h', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['n', 'i', 'l', 't', '\u{1}', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2516);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 3189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['é', 'c', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1248);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['r', 'p', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1251);
					match s {
						[] => true,
						['í', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1254);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['g', 'n', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 1256);
					s.is_empty()
				}
				['n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['j', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2522);
							s.is_empty()
						}
						['t', 'u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['i', 'r', 'e', 'j', 'd', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 301);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1267);
					match s {
						[] => true,
						['n', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 305);
			s.is_empty()
		}
		['ñ', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -v.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['c', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 309);
			s.is_empty()
		}
		['m', 'j', '\u{1}', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1269);
					match s {
						[] => true,
						['i', 'n', 'h', 's', 'a', 'l', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['t', 'i', '\u{1}', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -w.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['s', 'a', 'k', 'c', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1271);
					s.is_empty()
				}
				['t', 'c', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1273);
					s.is_empty()
				}
				['c', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1274);
					s.is_empty()
				}
				['r', 'ó', 't', 'o', 'l', 'a', 'f', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1275);
					s.is_empty()
				}
				['y', '\u{1}', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1278);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1279);
					match s {
						[] => true,
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						['i', 'r', 'u', 't', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['t', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3190);
									s.is_empty()
								}
								['i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1283);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 't', 's', 'e', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 324);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -y.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1287);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2534);
							s.is_empty()
						}
						['i', 'r', 'i', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2535);
							s.is_empty()
						}
						['r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2536);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1290);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1291);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						['n', 'o', 'd', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1292);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1293);
					match s {
						[] => true,
						['b', 'm', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2541);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2542);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1294);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['b', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1298);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1300);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1301);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2543);
							match s {
								[] => true,
								['o', 'j', '\u{1}', s @ ..] => {
									super::push(out, 3194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1302);
					match s {
						[] => true,
						['d', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2545);
							s.is_empty()
						}
						['l', 'o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1303);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2547);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2548);
							match s {
								[] => true,
								['u', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1304);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1305);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2551);
							match s {
								[] => true,
								['é', s @ ..] => {
									match s {
										[] => true,
										['j', '\u{1}', s @ ..] => {
											super::push(out, 3456);
											s.is_empty()
										}
										['y', '\u{1}', s @ ..] => {
											super::push(out, 3457);
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
					super::push(out, 1306);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 2552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'g', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 331);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2553);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3196);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 3458);
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
				['c', s @ ..] => {
					super::push(out, 1310);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1312);
					match s {
						[] => true,
						['v', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1313);
					match s {
						[] => true,
						['d', 'a', 't', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2555);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2556);
							match s {
								[] => true,
								['a', 'e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3459);
											s.is_empty()
										}
										['r', 'u', 'c', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 3460);
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
		['u', s @ ..] => {
			super::push(out, 338);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1314);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1316);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1317);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1319);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2559);
							s.is_empty()
						}
						['ó', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['e', 'h', 'c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2562);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1324);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 2564);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 2565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['á', 'r', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1327);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1328);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1329);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1330);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2566);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['q', '\u{1}', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 1332);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1333);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1334);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1335);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 343);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 344);
			s.is_empty()
		}
		['í', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -á.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['c', 'a', s @ ..] => {
			super::push(out, 346);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 348);
			s.is_empty()
		}
		['h', 'c', 'a', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2570);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['a', 'r', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 1338);
					s.is_empty()
				}
				['r', 'o', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['a', 'c', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 355);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['p', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['z', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['t', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ñ', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -é.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 363);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1350);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 364);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1352);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'e', s @ ..] => {
			super::push(out, 366);
			s.is_empty()
		}
		['r', 'a', 'c', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -í.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', s @ ..] => {
					super::push(out, 1356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					super::push(out, 1357);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'u', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 372);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1359);
					s.is_empty()
				}
				['u', 'b', s @ ..] => {
					super::push(out, 1360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1361);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 374);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 375);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							super::push(out, 2573);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', s @ ..] => {
					super::push(out, 1363);
					s.is_empty()
				}
				['t', 'o', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1364);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1365);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2575);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'u', s @ ..] => {
			super::push(out, 377);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 1366);
					s.is_empty()
				}
				['i', 'p', s @ ..] => {
					super::push(out, 1367);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1369);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1370);
					s.is_empty()
				}
				['q', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2577);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 2578);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		['ñ', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ó.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ú.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['d', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 389);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['o', 'v', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', s @ ..] => {
			super::push(out, 391);
			s.is_empty()
		}
		['r', 'u', 'c', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 393);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -da.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1407);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', s @ ..] => {
					super::push(out, 1408);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'ó', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1410);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['í', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 2602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2605);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3200);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3461);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3202);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3463);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3464);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3465);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 3466);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3467);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 3203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'n', s @ ..] => {
							super::push(out, 2606);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ú', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						['i', 't', s @ ..] => {
							super::push(out, 2609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						['á', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2613);
							match s {
								[] => true,
								['á', s @ ..] => {
									super::push(out, 3205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ú', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 2615);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2616);
							s.is_empty()
						}
						['á', s @ ..] => {
							super::push(out, 2617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 412);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2618);
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
				['c', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'b', s @ ..] => {
			super::push(out, 415);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ga.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['n', 'o', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2625);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1451);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1452);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['s', 'e', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2627);
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
					super::push(out, 1456);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['f', 's', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1461);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2628);
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

/// Words ending -ha.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1462);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1463);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2630);
							s.is_empty()
						}
						['i', 'h', s @ ..] => {
							super::push(out, 2631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1464);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1465);
					s.is_empty()
				}
				['u', 'c', s @ ..] => {
					super::push(out, 1466);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 2632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ia.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							super::push(out, 2633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1468);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2634);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3206);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3468);
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
			super::push(out, 435);
			match s {
				[] => true,
				['e', 'm', 'i', s @ ..] => {
					super::push(out, 1469);
					s.is_empty()
				}
				['r', 'a', 'u', s @ ..] => {
					super::push(out, 1470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 437);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['p', 'm', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 441);
			s.is_empty()
		}
		['t', 's', 'e', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ja.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 446);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['a', 'r', 'a', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['l', 'f', 'a', 'c', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1481);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ka.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		['i', 'r', s @ ..] => {
			super::push(out, 451);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1483);
					s.is_empty()
				}
				['d', 'n', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1485);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				['s', 'e', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['u', 'h', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', 'n', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				['r', 'b', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1496);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2636);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				['m', 'i', s @ ..] => {
					super::push(out, 1501);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['a', 'v', s @ ..] => {
					super::push(out, 1503);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1504);
					match s {
						[] => true,
						['r', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 1505);
					match s {
						[] => true,
						['c', 'e', 'b', s @ ..] => {
							super::push(out, 2640);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2641);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', 'a', s @ ..] => {
							super::push(out, 2642);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2643);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3208);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 'c', s @ ..] => {
							super::push(out, 2644);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2645);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3209);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2646);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2647);
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
							super::push(out, 2648);
							match s {
								[] => true,
								['t', 'a', 'p', 'o', s @ ..] => {
									super::push(out, 3212);
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
			super::push(out, 457);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2649);
							s.is_empty()
						}
						['í', s @ ..] => {
							super::push(out, 2650);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1510);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1512);
					match s {
						[] => true,
						['á', s @ ..] => {
							super::push(out, 2652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'ú', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1514);
					s.is_empty()
				}
				['p', 'á', s @ ..] => {
					super::push(out, 1515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1520);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1522);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2653);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2654);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2655);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1526);
					s.is_empty()
				}
				['d', 'a', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1529);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1530);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2657);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'f', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2659);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							super::push(out, 2660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['s', 'i', s @ ..] => {
							super::push(out, 2661);
							match s {
								[] => true,
								['s', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ñ', s @ ..] => {
					super::push(out, 1534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 462);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1536);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1539);
					match s {
						[] => true,
						['q', 'n', s @ ..] => {
							super::push(out, 2663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['c', 'r', 'a', 's', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				['l', 'a', s @ ..] => {
					super::push(out, 1546);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2665);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2666);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 2667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1550);
					match s {
						[] => true,
						['f', 'a', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						['n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1555);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1556);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1557);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1559);
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1561);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1563);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1564);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 2672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1565);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1566);
					match s {
						[] => true,
						['g', 'l', s @ ..] => {
							super::push(out, 2673);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', s @ ..] => {
					super::push(out, 1567);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 1568);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				['ñ', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1572);
					match s {
						[] => true,
						['í', s @ ..] => {
							super::push(out, 2675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 473);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1575);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 2676);
							s.is_empty()
						}
						['i', 'm', 'a', s @ ..] => {
							super::push(out, 2677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'e', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['m', 'l', s @ ..] => {
							super::push(out, 2678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 'n', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				['m', 'r', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				['x', 'o', 't', 'o', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1583);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1585);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['a', 'f', 'a', s @ ..] => {
					super::push(out, 1591);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['í', 'e', 'l', s @ ..] => {
			super::push(out, 478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['m', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'u', 'y', '\u{1}', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1602);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['r', 'r', s @ ..] => {
					super::push(out, 1604);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1605);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 2685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1606);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1607);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 1608);
					s.is_empty()
				}
				['u', 'c', s @ ..] => {
					super::push(out, 1609);
					s.is_empty()
				}
				['z', 'a', s @ ..] => {
					super::push(out, 1610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['a', 'c', 'a', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 1612);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['d', 'a', 'r', s @ ..] => {
					super::push(out, 1614);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1615);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'o', s @ ..] => {
					super::push(out, 1616);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', s @ ..] => {
					super::push(out, 1618);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1619);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 2689);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2690);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 2691);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1620);
					match s {
						[] => true,
						['a', 'l', 'a', s @ ..] => {
							super::push(out, 2693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 2694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 493);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1622);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['p', 'm', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 496);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2695);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2697);
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
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1627);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2698);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2699);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2700);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2701);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2702);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 2703);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2704);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2705);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2706);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2707);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2708);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3215);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2709);
							s.is_empty()
						}
						['z', 't', s @ ..] => {
							super::push(out, 2710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1629);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 499);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['m', 'ó', s @ ..] => {
							super::push(out, 2712);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1634);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1636);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1638);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1639);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1640);
					match s {
						[] => true,
						['l', 'u', 'c', 'i', 't', s @ ..] => {
							super::push(out, 2718);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							super::push(out, 2719);
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

/// Words ending -sa.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1641);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2720);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 1642);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1644);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 507);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1648);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1650);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1651);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1652);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2723);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1656);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2725);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 2726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2731);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1660);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				['ñ', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1664);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1665);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1667);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1668);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1670);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2734);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2735);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2736);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2737);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2738);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2739);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1672);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2741);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2743);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2744);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 3221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1673);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2746);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2747);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1675);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2749);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1676);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2750);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 2751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 512);
			s.is_empty()
		}
		['i', s @ ..] => super::t0::n533(s, out),
		['l', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2792);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2793);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3249);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 2795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', s @ ..] => {
					super::push(out, 1697);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2796);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3250);
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
			super::push(out, 516);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1701);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2797);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2798);
							match s {
								[] => true,
								['a', 'c', s @ ..] => {
									super::push(out, 3251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				['l', 'a', s @ ..] => {
					super::push(out, 1703);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				['r', s @ ..] => super::t0::n1767(s, out),
				['t', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				['u', s @ ..] => super::t0::n1769(s, out),
				['y', s @ ..] => super::t0::n1770(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n537(s, out),
		['s', s @ ..] => super::t0::n538(s, out),
		['t', s @ ..] => super::t0::n539(s, out),
		['u', s @ ..] => super::t0::n540(s, out),
		['x', s @ ..] => super::t0::n541(s, out),
		['í', s @ ..] => super::t0::n542(s, out),
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', 'a', s @ ..] => {
							super::push(out, 2823);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2824);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				['h', 'z', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1733);
					match s {
						[] => true,
						['a', 'w', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 's', s @ ..] => {
			super::push(out, 528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wa.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 529);
			s.is_empty()
		}
		['i', 'l', 'i', 'k', '\u{1}', s @ ..] => {
			super::push(out, 530);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ya.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1735);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 532);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1739);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 535);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 536);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 537);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ça.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	s.is_empty()
}

/// Words ending -ía.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 539);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 540);
			s.is_empty()
		}
		['g', 'o', s @ ..] => {
			super::push(out, 541);
			s.is_empty()
		}
		['l', 'a', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 543);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 544);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1746);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1748);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 2830);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2831);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 3294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'ñ', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 547);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1751);
					s.is_empty()
				}
				['q', 'r', 'a', 'n', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1753);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ña.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1755);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -úa.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 554);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 555);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ringe.
pub(crate) fn n2955(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3304);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 3305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yuge.
pub(crate) fn n1868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1800);
	s.is_empty()
}

/// Words ending -he.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', 'a', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2848);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'l', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1802);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1803);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 1804);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1805);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 2851);
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

/// Words ending -ie.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 628);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 629);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 2852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 630);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 631);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 632);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 634);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -je.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1811);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 637);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ke.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['a', 't', 'i', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 1813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1814);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2854);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2856);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['b', 'e', 'd', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1816);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1818);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'i', 'h', s @ ..] => {
			super::push(out, 643);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 644);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1820);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2858);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2859);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'u', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 646);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1823);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2862);
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
				['i', 't', s @ ..] => {
					super::push(out, 1824);
					s.is_empty()
				}
				['m', 'i', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 647);
			s.is_empty()
		}
		['u', 'o', s @ ..] => {
			super::push(out, 648);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			super::push(out, 649);
			s.is_empty()
		}
		['e', 'r', s @ ..] => {
			super::push(out, 650);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1826);
					s.is_empty()
				}
				['n', 'á', s @ ..] => {
					super::push(out, 1827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', s @ ..] => {
			super::push(out, 652);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['a', 'd', s @ ..] => {
					super::push(out, 1828);
					s.is_empty()
				}
				['o', 'f', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2863);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 2864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 654);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 655);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ne.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 657);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1830);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1831);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1832);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 659);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oe.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	s.is_empty()
}

/// Words ending -pe.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1835);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2866);
							s.is_empty()
						}
						['í', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 662);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1837);
					s.is_empty()
				}
				['l', 'a', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1839);
					s.is_empty()
				}
				['í', s @ ..] => {
					super::push(out, 1840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1841);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1842);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1843);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1845);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3309);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3310);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', s @ ..] => {
							super::push(out, 2869);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2870);
							match s {
								[] => true,
								['h', 'o', 'c', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2871);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3313);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2872);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2873);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1848);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1849);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1851);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 668);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 669);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 670);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 671);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 673);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 1854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1855);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1856);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1857);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1858);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2878);
							match s {
								[] => true,
								['o', 's', s @ ..] => {
									super::push(out, 3315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2879);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', s @ ..] => {
							super::push(out, 2880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1864);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 2881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 682);
			s.is_empty()
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 683);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1865);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1866);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1867);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1869);
					s.is_empty()
				}
				['n', 'g', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['r', 'i', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				['j', 'e', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1877);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2882);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1879);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 687);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1881);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				['u', 's', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n1958(s, out),
				['e', s @ ..] => super::t0::n1959(s, out),
				['i', s @ ..] => {
					super::push(out, 1886);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1887);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 690);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1890);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1891);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 2906);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n721(s, out),
		['s', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1898);
					s.is_empty()
				}
				['e', s @ ..] => super::t0::n1975(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t0::n723(s, out),
		['y', s @ ..] => super::t0::n724(s, out),
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 695);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1902);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1905);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2916);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2918);
							match s {
								[] => true,
								['o', 'c', 'o', 'x', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1906);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1907);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2921);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2922);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2923);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1908);
					s.is_empty()
				}
				['o', 'z', s @ ..] => {
					super::push(out, 1909);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				['s', 'o', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'c', 'n', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1913);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1914);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 700);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word we.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	s.is_empty()
}

/// Words ending -ye.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 701);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ée.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	s.is_empty()
}

/// Words ending -íe.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	s.is_empty()
}

/// Words ending -ñe.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 702);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -igüe.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	s.is_empty()
}

/// Words ending -ional.
pub(crate) fn n3071(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 3538);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3539);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3392);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3541);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -monal.
pub(crate) fn n3072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2952);
	s.is_empty()
}

/// Words ending -ronal.
pub(crate) fn n3073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2953);
	s.is_empty()
}

/// Words ending -rnal.
pub(crate) fn n2054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1975);
	s.is_empty()
}

/// Words ending -pal.
pub(crate) fn n831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 793);
	match s {
		[] => true,
		['i', 'c', s @ ..] => {
			super::push(out, 1976);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ral.
pub(crate) fn n832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 794);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1977);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1979);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2955);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', s @ ..] => {
					super::push(out, 2956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1980);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1981);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1982);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2957);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sal.
pub(crate) fn n833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 795);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1983);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1984);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1985);
			s.is_empty()
		}
		['o', 'r', s @ ..] => {
			super::push(out, 1986);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1987);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 2959);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tal.
pub(crate) fn n834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 796);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1988);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1989);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['p', 'a', s @ ..] => {
					super::push(out, 2960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1992);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3394);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3395);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1993);
			s.is_empty()
		}
		['r', 'o', s @ ..] => {
			super::push(out, 1994);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1995);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2962);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ual.
pub(crate) fn n835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 797);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1996);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1997);
			s.is_empty()
		}
		['g', 'a', s @ ..] => {
			super::push(out, 1998);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1999);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2000);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 2001);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -val.
pub(crate) fn n836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 798);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2002);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2003);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2004);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2964);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zal.
pub(crate) fn n837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 799);
	s.is_empty()
}

/// Words ending -ñal.
pub(crate) fn n838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 800);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2005);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ddl.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	s.is_empty()
}

/// Words ending -el.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2007);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2008);
					s.is_empty()
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 803);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['a', 'l', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2011);
					s.is_empty()
				}
				['á', 'l', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 806);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2014);
					s.is_empty()
				}
				['n', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2015);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 808);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2018);
					s.is_empty()
				}
				['m', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2019);
					s.is_empty()
				}
				['í', s @ ..] => {
					super::push(out, 2020);
					match s {
						[] => true,
						['u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['o', 'r', s @ ..] => {
					super::push(out, 2021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['p', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 811);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['é', 'i', 'd', 'o', 'r', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['k', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 814);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -il.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['m', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2026);
					s.is_empty()
				}
				['r', 't', 'a', 'r', 't', 'l', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2027);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2967);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['c', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3397);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3398);
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
		['b', 'i', s @ ..] => {
			super::push(out, 818);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 819);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 820);
			s.is_empty()
		}
		['g', 'ú', s @ ..] => {
			super::push(out, 821);
			s.is_empty()
		}
		['m', 'a', 't', s @ ..] => {
			super::push(out, 822);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 823);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 2028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				['á', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ll.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['b', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 829);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ml.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	s.is_empty()
}

/// Words ending -ol.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2033);
					s.is_empty()
				}
				['e', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				['i', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 2035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['v', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 835);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2039);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 838);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['p', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 2041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 841);
			s.is_empty()
		}
		['t', 'i', 'l', 'y', 'x', '\u{1}', s @ ..] => {
			super::push(out, 842);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 843);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ql.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	s.is_empty()
}

/// The word srl.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	s.is_empty()
}

/// The word adsl.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	s.is_empty()
}

/// The word papalometl.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	s.is_empty()
}

/// Words ending -ul.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 844);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2042);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 846);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word bowl.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	s.is_empty()
}

/// Words ending -íl.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['a', 'r', 'r', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['v', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -men.
pub(crate) fn n934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 896);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2085);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2086);
			match s {
				[] => true,
				['x', s @ ..] => {
					super::push(out, 2985);
					match s {
						[] => true,
						['e', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2087);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2088);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3401);
							s.is_empty()
						}
						['i', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2089);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2986);
					match s {
						[] => true,
						['é', 'p', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2987);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2090);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2091);
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2092);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 2990);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2991);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2992);
					match s {
						[] => true,
						['e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['â', 'x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2093);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gluten.
pub(crate) fn n2182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2101);
	s.is_empty()
}

/// Words ending -uen.
pub(crate) fn n939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 901);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 2102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ven.
pub(crate) fn n940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 902);
	match s {
		[] => true,
		['e', 'k', 'a', 'e', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2103);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2104);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gn.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	s.is_empty()
}

/// Words ending -in.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 903);
			match s {
				[] => true,
				['l', 'd', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 906);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 907);
			match s {
				[] => true,
				['n', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2107);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2108);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2109);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				['p', 'm', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['e', 'r', 'e', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 2112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', 't', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 910);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['e', 't', 'á', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 913);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['s', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 2116);
					match s {
						[] => true,
						['n', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', 'l', 'e', 'k', '\u{1}', s @ ..] => {
			super::push(out, 916);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 917);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 'c', 't', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nn.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['y', '\u{1}', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -on.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['a', 'x', '\u{1}', s @ ..] => {
					super::push(out, 2119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2120);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2121);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2998);
							s.is_empty()
						}
						['t', 'o', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 2123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'n', 'i', 'l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 924);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 925);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 926);
			match s {
				[] => true,
				['h', 's', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2126);
					s.is_empty()
				}
				['m', 'o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2127);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2128);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2129);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2131);
					match s {
						[] => true,
						['f', 'é', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2132);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3003);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['s', 'a', 'n', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 2134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				['g', 'i', 'v', 'u', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 930);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['a', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				['e', 't', 'n', 'é', 'u', 'q', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3005);
							s.is_empty()
						}
						['m', 'í', 'x', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2140);
					match s {
						[] => true,
						['e', 'n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 932);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2141);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2143);
					match s {
						[] => true,
						['b', 'r', 'é', 'p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2144);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2145);
					match s {
						[] => true,
						['d', 'n', 'í', 's', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 3405);
									s.is_empty()
								}
								['i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2146);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3011);
							match s {
								[] => true,
								['p', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'e', 'l', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 934);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 935);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dpn.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	s.is_empty()
}

/// Words ending -rn.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 937);
			match s {
				[] => true,
				['f', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -un.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['t', 'n', 'u', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['á', 't', 'n', 'u', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wn.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 940);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word zayn.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	s.is_empty()
}

/// Words ending -án.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 941);
			match s {
				[] => true,
				['i', 'l', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 942);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 943);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 944);
			match s {
				[] => true,
				['s', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 945);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['i', 'h', s @ ..] => {
					super::push(out, 2154);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 3012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 947);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 948);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 949);
			match s {
				[] => true,
				['i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2155);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -én.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 951);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['q', 'i', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2158);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ín.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['a', 's', '\u{1}', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 956);
			match s {
				[] => true,
				['c', 'n', s @ ..] => {
					super::push(out, 2160);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 957);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2164);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 2165);
					match s {
						[] => true,
						['o', 'r', 'e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['q', 'n', 'e', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 965);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2167);
					match s {
						[] => true,
						['u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3015);
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

/// Words ending -ón.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 968);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 969);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2170);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3017);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3408);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3543);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 3564);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3409);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3410);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3544);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 3565);
													match s {
														[] => true,
														['u', s @ ..] => super::t0::n3718(s, out),
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
						['c', 'e', 'l', s @ ..] => {
							super::push(out, 3018);
							match s {
								[] => true,
								['e', 's', s @ ..] => {
									super::push(out, 3411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3019);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3412);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2173);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', s @ ..] => {
					super::push(out, 2174);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2175);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2176);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2177);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3023);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3024);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2179);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2180);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2181);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3028);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3413);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3414);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 3545);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'b', s @ ..] => {
									super::push(out, 3415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['l', 'i', 'm', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 977);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 978);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 979);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2187);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3030);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2188);
					s.is_empty()
				}
				['i', 'p', s @ ..] => {
					super::push(out, 2189);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2191);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 983);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 984);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 985);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', s @ ..] => {
					super::push(out, 2195);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2198);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2199);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ñ', s @ ..] => {
			super::push(out, 988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ún.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	s.is_empty()
}

/// Words ending -adio.
pub(crate) fn n2309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2227);
	s.is_empty()
}

/// Words ending -lio.
pub(crate) fn n1064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1025);
	match s {
		[] => true,
		['o', 'p', s @ ..] => {
			super::push(out, 2228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rio.
pub(crate) fn n1065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1026);
	s.is_empty()
}

/// Words ending -tio.
pub(crate) fn n1066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1027);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jo.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1028);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ko.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1030);
			s.is_empty()
		}
		['i', 'r', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3047);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3422);
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
			super::push(out, 1036);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1039);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3048);
							match s {
								[] => true,
								['i', 'd', s @ ..] => {
									super::push(out, 3423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'r', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						['t', 'e', s @ ..] => {
							super::push(out, 3050);
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 3424);
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

/// Words ending -no.
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 229);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3052);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2238);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2239);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2240);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2242);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1043);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2244);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2245);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	match s {
		[] => true,
		['a', 'h', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1048);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['c', 'r', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['d', 'o', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2250);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', 'i', 't', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2252);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3060);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3425);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3061);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3062);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2253);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', s @ ..] => {
					super::push(out, 2254);
					s.is_empty()
				}
				['t', 'e', 'h', s @ ..] => {
					super::push(out, 2255);
					s.is_empty()
				}
				['ñ', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1050);
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 232);
	match s {
		[] => true,
		['i', 't', s @ ..] => {
			super::push(out, 1053);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2258);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2259);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 233);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					super::push(out, 2261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1056);
			match s {
				[] => true,
				['l', 'l', s @ ..] => {
					super::push(out, 2263);
					s.is_empty()
				}
				['n', 'a', s @ ..] => {
					super::push(out, 2264);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', s @ ..] => {
					super::push(out, 2265);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2266);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 2267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3067);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3068);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['l', 'l', s @ ..] => {
											super::push(out, 3546);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3547);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'u', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2268);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2270);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2271);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2272);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2273);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2274);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1060);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 2276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1061);
			match s {
				[] => true,
				['e', 'u', 'h', s @ ..] => {
					super::push(out, 2277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['e', 'r', 'o', 'p', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1064);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	s.is_empty()
}

/// Words ending -vo.
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yo.
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1066);
			s.is_empty()
		}
		['i', 't', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1068);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tío.
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	s.is_empty()
}

/// Words ending -eño.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		['u', 'g', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 2282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -er.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2299);
					s.is_empty()
				}
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2302);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				['ú', s @ ..] => {
					super::push(out, 2304);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['a', 'l', 'e', 'e', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3080);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3081);
							match s {
								[] => true,
								['u', 'l', 'f', 'n', 'i', 'r', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['á', s @ ..] => {
							super::push(out, 3082);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3084);
							match s {
								[] => true,
								['l', 'r', 'e', 'e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2309);
					match s {
						[] => true,
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 3085);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 3086);
							s.is_empty()
						}
						['é', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2310);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3088);
							match s {
								[] => true,
								['i', 'f', 'r', 'e', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3089);
							s.is_empty()
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3090);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 3091);
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
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3431);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3432);
									match s {
										[] => true,
										['r', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3548);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['é', 't', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3092);
							s.is_empty()
						}
						['í', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['r', 'a', 'o', 'b', 'w', 'o', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				['í', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1097);
			match s {
				[] => true,
				['f', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['n', 'á', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3095);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 3433);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3096);
							s.is_empty()
						}
						['á', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3097);
							s.is_empty()
						}
						['í', s @ ..] => {
							super::push(out, 3098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1100);
			match s {
				[] => true,
				['b', 'g', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2320);
					s.is_empty()
				}
				['h', 't', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2321);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2323);
					match s {
						[] => true,
						['e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'n', 'o', 'f', 'f', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2324);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3100);
							s.is_empty()
						}
						['s', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['s', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2329);
					match s {
						[] => true,
						['e', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 3103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'ú', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2331);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				['s', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2333);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 2334);
					match s {
						[] => true,
						['p', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['a', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2335);
					s.is_empty()
				}
				['f', 'f', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2336);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3106);
							s.is_empty()
						}
						['e', 'w', 't', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3107);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3108);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3109);
							s.is_empty()
						}
						['á', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2337);
					match s {
						[] => true,
						['e', 's', 't', 's', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3111);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3434);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'o', 'c', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1104);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2339);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2340);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2341);
					match s {
						[] => true,
						['o', 'b', 'y', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n1148(s, out),
		['p', s @ ..] => super::t0::n1149(s, out),
		['r', s @ ..] => {
			super::push(out, 1107);
			s.is_empty()
		}
		['s', s @ ..] => super::t0::n1151(s, out),
		['t', s @ ..] => super::t1::n1152(s, out),
		['u', s @ ..] => super::t1::n1153(s, out),
		['v', s @ ..] => super::t1::n1154(s, out),
		['x', s @ ..] => super::t1::n1155(s, out),
		['y', s @ ..] => super::t1::n1156(s, out),
		['z', s @ ..] => super::t1::n1157(s, out),
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1130);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1131);
			s.is_empty()
		}
		['c', 'i', 'n', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1132);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1133);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1134);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'u', 'q', 'i', s @ ..] => {
			match s {
				[] => true,
				['d', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2401);
					s.is_empty()
				}
				['n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1135);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['t', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1137);
			match s {
				[] => true,
				['a', 'u', 'q', 'e', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				['á', s @ ..] => {
					match s {
						[] => true,
						['m', 'i', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3154);
							s.is_empty()
						}
						['u', 'q', 'e', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3155);
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

/// Words ending -tes.
pub(crate) fn n1212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1166);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2446);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2447);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2448);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2449);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2450);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2451);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1171);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1173);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3170);
							s.is_empty()
						}
						['n', 'u', 'r', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1175);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2455);
					s.is_empty()
				}
				['o', 'p', 'ó', 'r', s @ ..] => {
					super::push(out, 2456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1177);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1178);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1179);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2457);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2459);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3172);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3447);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3173);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 3174);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3448);
									match s {
										[] => true,
										['l', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3551);
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
					super::push(out, 2460);
					match s {
						[] => true,
						['t', 'n', s @ ..] => {
							super::push(out, 3175);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 3449);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2461);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3176);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3450);
									s.is_empty()
								}
								['á', s @ ..] => {
									super::push(out, 3451);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3552);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 3553);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 3554);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ó', s @ ..] => {
									super::push(out, 3452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1181);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2463);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3177);
							match s {
								[] => true,
								['e', 'l', 'p', 's', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2464);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1182);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2467);
					s.is_empty()
				}
				['q', 'a', s @ ..] => {
					super::push(out, 2468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['á', 'l', 'u', 'r', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ms.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			super::push(out, 1184);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ns.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['a', 'e', s @ ..] => {
			super::push(out, 1185);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -os.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1186);
			match s {
				[] => true,
				['a', 'm', 'o', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2469);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2470);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3179);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2472);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2474);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1188);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2475);
					match s {
						[] => true,
						['u', 'f', 'a', 'g', s @ ..] => {
							super::push(out, 3182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1189);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1190);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2476);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1191);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['e', 'u', s @ ..] => {
					super::push(out, 2478);
					match s {
						[] => true,
						['j', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1193);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1194);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2480);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2481);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['a', 'b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', s @ ..] => {
					super::push(out, 2483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2484);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2485);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2486);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3454);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'm', s @ ..] => {
					super::push(out, 2487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1197);
			match s {
				[] => true,
				['a', 't', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ñ', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rs.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	s.is_empty()
}

/// Words ending -ss.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1201);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1202);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', '\u{1}', s @ ..] => {
			super::push(out, 1203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ts.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2496);
					s.is_empty()
				}
				['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1206);
			match s {
				[] => true,
				['o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1207);
			s.is_empty()
		}
		['l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1208);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1209);
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 1210);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1211);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ás.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['n', 'a', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1213);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1214);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1215);
			match s {
				[] => true,
				['b', 'a', 'r', 'e', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2499);
					s.is_empty()
				}
				['f', 'a', 's', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2500);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'p', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -és.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1217);
			match s {
				[] => true,
				['e', 'b', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1218);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['p', 'a', 'c', 's', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1220);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1222);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2505);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', 'a', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'i', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1223);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ís.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1226);
			match s {
				[] => true,
				['f', 's', 'i', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1227);
			s.is_empty()
		}
		['s', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ós.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	s.is_empty()
}

/// Words ending -ús.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1229);
			match s {
				[] => true,
				['i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['o', 'r', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'a', 'p', 'u', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1230);
			s.is_empty()
		}
		['l', 'e', 'p', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1231);
			s.is_empty()
		}
		['y', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ita.
pub(crate) fn n533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 513);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1679);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2752);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1681);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3225);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2754);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3227);
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
					super::push(out, 2755);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							super::push(out, 3228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2756);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2757);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2758);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2760);
					s.is_empty()
				}
				['o', 't', 'a', s @ ..] => {
					super::push(out, 2761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1683);
			s.is_empty()
		}
		['h', 'c', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1685);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1686);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1687);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1688);
			match s {
				[] => true,
				['a', 'i', s @ ..] => {
					super::push(out, 2763);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2764);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2765);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					super::push(out, 2766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1689);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3231);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3232);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3469);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3233);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3471);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 3472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2767);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', s @ ..] => {
					super::push(out, 2768);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2769);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2770);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2771);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1690);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2773);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3237);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2774);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3239);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3240);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 3241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1691);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 2775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2776);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3242);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2777);
					s.is_empty()
				}
				['d', 'n', s @ ..] => {
					super::push(out, 2778);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2779);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 3244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'b', s @ ..] => {
					super::push(out, 2780);
					s.is_empty()
				}
				['u', 't', s @ ..] => {
					super::push(out, 2781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2782);
					s.is_empty()
				}
				['i', 'f', s @ ..] => {
					super::push(out, 2783);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2784);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1692);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2786);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2788);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3246);
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
				['g', 'n', s @ ..] => {
					super::push(out, 2789);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 2790);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2791);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1693);
			s.is_empty()
		}
		['ñ', s @ ..] => {
			super::push(out, 1694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rota.
pub(crate) fn n1767(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2799);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uota.
pub(crate) fn n1769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1706);
	s.is_empty()
}

/// Words ending -yota.
pub(crate) fn n1770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1707);
	s.is_empty()
}

/// Words ending -rta.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 517);
	s.is_empty()
}

/// Words ending -sta.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 518);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1708);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2801);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2802);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2803);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2804);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3252);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1709);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2805);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1710);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 2807);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2808);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2809);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 3254);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 3255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2810);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3256);
							s.is_empty()
						}
						['r', 'u', s @ ..] => {
							super::push(out, 3257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'o', s @ ..] => {
					super::push(out, 2811);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2812);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3259);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3473);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 3474);
									s.is_empty()
								}
								['i', 'r', s @ ..] => {
									super::push(out, 3475);
									s.is_empty()
								}
								['r', 't', s @ ..] => {
									super::push(out, 3476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 3260);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', s @ ..] => {
							super::push(out, 3261);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3262);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2813);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3264);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3265);
							s.is_empty()
						}
						['o', 'n', s @ ..] => {
							super::push(out, 3266);
							match s {
								[] => true,
								['o', 'c', 'e', 'o', s @ ..] => {
									super::push(out, 3479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2814);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3267);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3268);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'u', s @ ..] => {
							super::push(out, 3269);
							s.is_empty()
						}
						['o', 'i', 'c', 'c', 'e', s @ ..] => {
							super::push(out, 3270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2815);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3271);
							s.is_empty()
						}
						['l', 'o', s @ ..] => {
							super::push(out, 3272);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3273);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3274);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									super::push(out, 3482);
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
						['a', 'm', s @ ..] => {
							super::push(out, 3275);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3276);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3277);
							s.is_empty()
						}
						['o', 'y', s @ ..] => {
							super::push(out, 3278);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 3279);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3281);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									super::push(out, 3483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'n', s @ ..] => {
							super::push(out, 3282);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3283);
							s.is_empty()
						}
						['n', 'e', 'd', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 3285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2817);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3286);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['q', 'n', s @ ..] => {
							super::push(out, 3288);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3484);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3485);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 3555);
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
					super::push(out, 2818);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3289);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3290);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3291);
							match s {
								[] => true,
								['t', 'e', s @ ..] => {
									super::push(out, 3487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ñ', 'a', s @ ..] => {
					super::push(out, 2819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1711);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1712);
			s.is_empty()
		}
		['í', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['o', 'a', s @ ..] => {
					super::push(out, 2820);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tta.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 519);
	s.is_empty()
}

/// Words ending -uta.
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1714);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2821);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1715);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1716);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1717);
			s.is_empty()
		}
		['l', 'o', s @ ..] => {
			super::push(out, 1718);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1719);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1720);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xta.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 521);
	s.is_empty()
}

/// Words ending -íta.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'd', s @ ..] => {
			super::push(out, 1722);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1723);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ante.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1884);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2883);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3319);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3320);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3557);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'v', s @ ..] => {
					super::push(out, 3321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2884);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3322);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'u', s @ ..] => {
			super::push(out, 2885);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2886);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3324);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3491);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2887);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3326);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					super::push(out, 3327);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3328);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3329);
					match s {
						[] => true,
						['a', 'v', s @ ..] => {
							super::push(out, 3492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2888);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2889);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3331);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3332);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3333);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3334);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3335);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3336);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3496);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3497);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3498);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3499);
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
						['i', s @ ..] => {
							super::push(out, 3500);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3337);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2890);
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					super::push(out, 3338);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3339);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3503);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3504);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3505);
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
				['a', 'g', s @ ..] => {
					super::push(out, 3340);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3341);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3342);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3343);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3344);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3507);
							s.is_empty()
						}
						['p', 's', 'a', s @ ..] => {
							super::push(out, 3508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3345);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							super::push(out, 3509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'b', 'r', s @ ..] => {
					super::push(out, 3346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 3347);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'g', s @ ..] => {
					super::push(out, 3348);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3349);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', s @ ..] => {
					super::push(out, 3350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2891);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3351);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3352);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3353);
					s.is_empty()
				}
				['i', 'c', s @ ..] => {
					super::push(out, 3354);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3355);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3512);
							s.is_empty()
						}
						['e', 's', s @ ..] => {
							super::push(out, 3513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'v', s @ ..] => {
					super::push(out, 3356);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 3357);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3358);
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 3514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', 'l', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3359);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'g', s @ ..] => {
			super::push(out, 2892);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2893);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2894);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3362);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3516);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ñ', s @ ..] => {
			super::push(out, 2895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ente.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1885);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2896);
			match s {
				[] => true,
				['s', 'e', 'l', s @ ..] => {
					super::push(out, 3363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2897);
			match s {
				[] => true,
				['e', 'c', 'e', s @ ..] => {
					super::push(out, 3364);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3518);
							s.is_empty()
						}
						['s', 'e', 'r', 'p', 'e', s @ ..] => {
							super::push(out, 3519);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 3520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2898);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3365);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3367);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3368);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3561);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 3562);
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
				['b', s @ ..] => {
					super::push(out, 3369);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3522);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3370);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 3524);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							super::push(out, 3525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3526);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3527);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 3566);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3567);
											match s {
												[] => true,
												['e', 'd', 'o', s @ ..] => {
													super::push(out, 3569);
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
				['m', s @ ..] => {
					super::push(out, 3371);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3372);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3528);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3529);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3530);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3373);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3532);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2899);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2900);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3376);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3377);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3378);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							super::push(out, 3534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2901);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					super::push(out, 3379);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3381);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3382);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3383);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 2902);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2903);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3385);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rte.
pub(crate) fn n721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2908);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2910);
					s.is_empty()
				}
				['u', 'f', s @ ..] => {
					super::push(out, 2911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1897);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2912);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2913);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -este.
pub(crate) fn n1975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1899);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2914);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tte.
pub(crate) fn n723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yte.
pub(crate) fn n724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	s.is_empty()
}

/// Words ending -unización.
pub(crate) fn n3718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3568);
	s.is_empty()
}

/// Words ending -ner.
pub(crate) fn n1148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1105);
	match s {
		[] => true,
		['e', 't', 's', 'á', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2342);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2343);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2344);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3114);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 't', 's', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2345);
			s.is_empty()
		}
		['r', 'ó', s @ ..] => {
			super::push(out, 2346);
			s.is_empty()
		}
		['á', s @ ..] => {
			super::push(out, 2347);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 't', '\u{1}', s @ ..] => {
			super::push(out, 2348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n1149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1106);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2349);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['á', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2350);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['í', s @ ..] => {
			super::push(out, 2351);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ú', s @ ..] => {
			super::push(out, 2352);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n1151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2353);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', 'n', 'a', 'h', 'c', 'r', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3124);
					match s {
						[] => true,
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2354);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2355);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3125);
					s.is_empty()
				}
				['é', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['á', 'l', s @ ..] => {
			super::push(out, 2356);
			s.is_empty()
		}
		['ó', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2357);
			s.is_empty()
		}
		_ => false,
	}
}

