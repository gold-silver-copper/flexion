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
					super::push(out, 45);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 370);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1751);
									match s {
										[] => true,
										['l', 'u', 's', 'i', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4018);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 1752);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1753);
									match s {
										[] => true,
										['e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 4019);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 1754);
									match s {
										[] => true,
										['l', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5546);
													match s {
														[] => true,
														['r', 'i', 's', '\u{1}', s @ ..] => super::t0::n6376(s, out),
														_ => false,
													}
												}
												['u', s @ ..] => {
													super::push(out, 5547);
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
							super::push(out, 371);
							match s {
								[] => true,
								['a', 'g', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 372);
							match s {
								[] => true,
								['a', s @ ..] => super::t0::n1795(s, out),
								['i', 's', 't', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => super::t0::n1796(s, out),
								['l', s @ ..] => super::t0::n1797(s, out),
								_ => false,
							}
						}
						['p', s @ ..] => super::t0::n376(s, out),
						['r', s @ ..] => super::t0::n377(s, out),
						['s', 'i', s @ ..] => super::t0::n378(s, out),
						['t', s @ ..] => super::t0::n379(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n46(s, out),
				['i', s @ ..] => super::t0::n47(s, out),
				['l', s @ ..] => super::t0::n48(s, out),
				['o', s @ ..] => super::t0::n49(s, out),
				['r', s @ ..] => super::t0::n50(s, out),
				['u', s @ ..] => super::t0::n51(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n3(s, out),
		['d', s @ ..] => super::t0::n4(s, out),
		['e', s @ ..] => super::t0::n5(s, out),
		['f', s @ ..] => super::t0::n6(s, out),
		['g', s @ ..] => super::t0::n7(s, out),
		['h', s @ ..] => super::t0::n8(s, out),
		['i', s @ ..] => super::t0::n9(s, out),
		['k', s @ ..] => super::t0::n10(s, out),
		['l', s @ ..] => super::t0::n11(s, out),
		['m', s @ ..] => super::t0::n12(s, out),
		['n', s @ ..] => super::t0::n13(s, out),
		['o', s @ ..] => super::t0::n14(s, out),
		['p', s @ ..] => super::t0::n15(s, out),
		['q', s @ ..] => super::t0::n16(s, out),
		['r', s @ ..] => super::t0::n17(s, out),
		['s', s @ ..] => super::t0::n18(s, out),
		['t', s @ ..] => super::t0::n19(s, out),
		['u', s @ ..] => super::t0::n20(s, out),
		['v', s @ ..] => super::t0::n21(s, out),
		['w', s @ ..] => super::t0::n22(s, out),
		['x', s @ ..] => super::t0::n23(s, out),
		['y', s @ ..] => super::t0::n24(s, out),
		['z', s @ ..] => super::t0::n25(s, out),
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 224);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1495);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3849);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 225);
					match s {
						[] => true,
						['a', 't', 'i', 's', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 226);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 227);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 228);
					match s {
						[] => true,
						['u', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 229);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1501);
							match s {
								[] => true,
								['g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', 't', 'e', 'r', 't', 'e', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 230);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 231);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 232);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1503);
							match s {
								[] => true,
								['a', 's', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3852);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1504);
							match s {
								[] => true,
								['i', 'g', 'i', 'n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 3854);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['e', 'n', 'u', 'n', 't', 'i', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 1506);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1507);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1508);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3856);
									s.is_empty()
								}
								['r', 'd', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 3857);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 235);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['c', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1512);
							match s {
								[] => true,
								['e', 'm', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 3859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 237);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1515);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3860);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 5515);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 6089);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5516);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 3861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1519);
							match s {
								[] => true,
								['e', 's', 'c', 'i', 'a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1520);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3863);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 3864);
									s.is_empty()
								}
								['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3865);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 238);
					match s {
						[] => true,
						['d', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1521);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1522);
							s.is_empty()
						}
						['t', s @ ..] => super::t0::n1554(s, out),
						_ => false,
					}
				}
				['h', s @ ..] => super::t0::n240(s, out),
				['i', s @ ..] => super::t0::n241(s, out),
				['l', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 241);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['q', 'u', 'i', 's', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => super::t0::n244(s, out),
				['r', s @ ..] => super::t0::n245(s, out),
				['t', s @ ..] => super::t0::n246(s, out),
				['u', s @ ..] => super::t0::n247(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n28(s, out),
		['e', s @ ..] => super::t0::n29(s, out),
		['f', s @ ..] => super::t0::n30(s, out),
		['g', s @ ..] => super::t0::n31(s, out),
		['h', 'e', 'n', 'u', 'm', '\u{1}', s @ ..] => super::t0::n32(s, out),
		['l', s @ ..] => super::t0::n33(s, out),
		['m', s @ ..] => super::t0::n34(s, out),
		['n', s @ ..] => super::t0::n35(s, out),
		['p', s @ ..] => super::t0::n36(s, out),
		['q', s @ ..] => super::t0::n37(s, out),
		['r', s @ ..] => super::t0::n38(s, out),
		['s', s @ ..] => super::t0::n39(s, out),
		['t', s @ ..] => super::t0::n40(s, out),
		['u', s @ ..] => super::t0::n41(s, out),
		['v', s @ ..] => super::t0::n42(s, out),
		['x', s @ ..] => super::t0::n43(s, out),
		['z', s @ ..] => super::t0::n44(s, out),
		_ => false,
	}
}

/// The word siralucab.
pub(crate) fn n6376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6104);
	s.is_empty()
}

/// Words ending -alab.
pub(crate) fn n1795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1756);
	match s {
		[] => true,
		['e', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4020);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4021);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4022);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word muiratsilab.
pub(crate) fn n1796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1757);
	s.is_empty()
}

/// Words ending -llab.
pub(crate) fn n1797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1758);
	s.is_empty()
}

/// Words ending -pab.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	s.is_empty()
}

/// Words ending -rab.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1759);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['n', 'u', 'l', 'a', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4023);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4024);
					s.is_empty()
				}
				['t', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4025);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1761);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4027);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -isab.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1762);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1763);
			s.is_empty()
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tab.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	s.is_empty()
}

/// Words ending -eb.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['t', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4028);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 377);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['h', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4030);
							s.is_empty()
						}
						['o', 'n', 'a', 'r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4031);
							s.is_empty()
						}
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['i', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', '\u{1}', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ib.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['o', 'n', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['u', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'i', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				['o', 'c', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				['u', 'b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['d', 'i', 'v', 'e', 'r', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['a', 'l', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 391);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 393);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lb.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1784);
					match s {
						[] => true,
						['o', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['d', 'i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4034);
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

/// Words ending -ob.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['t', 'a', 'r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4035);
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
				['b', s @ ..] => {
					super::push(out, 1790);
					s.is_empty()
				}
				['i', 's', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 396);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['a', 'c', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1792);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 398);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1794);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['i', 'n', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rb.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4037);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4038);
							s.is_empty()
						}
						['h', 'i', s @ ..] => {
							match s {
								[] => true,
								['o', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5549);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4039);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 402);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1798);
					match s {
						[] => true,
						['i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4041);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['a', 'r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1799);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1800);
					s.is_empty()
				}
				['g', 'a', 'r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4043);
							s.is_empty()
						}
						['i', 't', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4044);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1803);
					s.is_empty()
				}
				['t', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1804);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ub.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['i', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1806);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1807);
					s.is_empty()
				}
				['u', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4046);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4047);
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
				['a', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1809);
					match s {
						[] => true,
						['i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1811);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 406);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 408);
			s.is_empty()
		}
		['m', 'a', 's', 't', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 409);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 410);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 411);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1813);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1814);
					s.is_empty()
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1816);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4049);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4050);
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
				['a', 'n', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['e', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1818);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1820);
					s.is_empty()
				}
				['y', 'r', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 413);
			s.is_empty()
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
			super::push(out, 52);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 415);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['u', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 416);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1825);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5551);
											s.is_empty()
										}
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5552);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 417);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1826);
							match s {
								[] => true,
								['a', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1828);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4055);
									match s {
										[] => true,
										['t', 'o', s @ ..] => {
											super::push(out, 5553);
											match s {
												[] => true,
												['r', '\u{1}', s @ ..] => {
													super::push(out, 6105);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 4056);
									match s {
										[] => true,
										['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5554);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4057);
									match s {
										[] => true,
										['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5555);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1829);
							match s {
								[] => true,
								['g', s @ ..] => super::t0::n4207(s, out),
								['p', s @ ..] => {
									super::push(out, 4060);
									match s {
										[] => true,
										['h', 'a', 't', 'u', 's', '\u{1}', s @ ..] => super::t0::n5792(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => super::t0::n1873(s, out),
						['o', s @ ..] => super::t0::n1874(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t0::n430(s, out),
				['n', s @ ..] => super::t0::n431(s, out),
				['p', s @ ..] => super::t0::n432(s, out),
				['r', s @ ..] => super::t0::n433(s, out),
				['s', s @ ..] => super::t0::n434(s, out),
				['t', s @ ..] => super::t0::n435(s, out),
				['u', s @ ..] => super::t0::n436(s, out),
				['v', s @ ..] => super::t0::n437(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n53(s, out),
		['h', s @ ..] => super::t0::n54(s, out),
		['i', s @ ..] => super::t0::n55(s, out),
		['l', s @ ..] => super::t0::n56(s, out),
		['n', s @ ..] => super::t0::n57(s, out),
		['o', s @ ..] => super::t0::n58(s, out),
		['r', s @ ..] => super::t0::n59(s, out),
		['u', s @ ..] => super::t0::n60(s, out),
		['y', s @ ..] => super::t0::n61(s, out),
		_ => false,
	}
}

/// Words ending -d.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['d', 'u', 'c', 'h', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2073);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				['r', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 514);
					match s {
						[] => true,
						['h', 'i', 'a', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2076);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4291);
									match s {
										[] => true,
										['o', 'g', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5681);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2077);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4293);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4294);
									match s {
										[] => true,
										['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5682);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 4295);
									s.is_empty()
								}
								['r', s @ ..] => super::t0::n4453(s, out),
								['s', s @ ..] => {
									super::push(out, 4296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t0::n2131(s, out),
						['l', s @ ..] => super::t0::n2132(s, out),
						['o', s @ ..] => super::t0::n2133(s, out),
						['r', s @ ..] => super::t0::n2134(s, out),
						['u', s @ ..] => super::t0::n2135(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['f', s @ ..] => super::t0::n530(s, out),
				['g', s @ ..] => super::t0::n531(s, out),
				['h', s @ ..] => super::t0::n532(s, out),
				['i', s @ ..] => super::t0::n533(s, out),
				['l', s @ ..] => super::t0::n534(s, out),
				['m', s @ ..] => super::t0::n535(s, out),
				['n', s @ ..] => super::t0::n536(s, out),
				['p', s @ ..] => super::t0::n537(s, out),
				['r', s @ ..] => super::t0::n538(s, out),
				['s', s @ ..] => super::t0::n539(s, out),
				['t', s @ ..] => super::t0::n540(s, out),
				['u', s @ ..] => super::t0::n541(s, out),
				['v', s @ ..] => super::t0::n542(s, out),
				['x', s @ ..] => super::t0::n543(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n64(s, out),
		['o', s @ ..] => super::t1::n65(s, out),
		['r', s @ ..] => super::t1::n66(s, out),
		['u', s @ ..] => super::t1::n67(s, out),
		['y', s @ ..] => super::t1::n68(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['i', 'o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['l', 'l', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['l', 'e', 's', 'i', 'o', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['i', 'p', s @ ..] => {
							super::push(out, 2206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['a', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				['e', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2207);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2208);
							match s {
								[] => true,
								['i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4389);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4390);
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
		['f', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2210);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2211);
							match s {
								[] => true,
								['a', 'g', 'i', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4391);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'm', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2212);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['e', 's', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['l', 'e', 'c', 'o', 'p', 'a', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['c', 't', 'a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 's', 't', 'e', 'i', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['u', 'l', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['c', 'a', 't', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2215);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						['q', 'u', 'e', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2217);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2218);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4392);
									s.is_empty()
								}
								['i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2219);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['i', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
													super::push(out, 6171);
													s.is_empty()
												}
												['o', 'n', 'i', 'c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 6172);
													s.is_empty()
												}
												['u', 'm', '\u{1}', s @ ..] => {
													super::push(out, 6173);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5727);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2221);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4394);
									match s {
										[] => true,
										['n', 't', 'i', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5728);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4395);
									match s {
										[] => true,
										['d', 'a', 'r', 'i', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 5729);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2222);
							match s {
								[] => true,
								['o', 's', 'y', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						['v', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['x', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['i', 'p', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2227);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2228);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2230);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2231);
							match s {
								[] => true,
								['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2232);
							s.is_empty()
						}
						['t', 'i', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n613(s, out),
				['o', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['p', s @ ..] => super::t1::n615(s, out),
				['u', s @ ..] => super::t1::n616(s, out),
				['y', 's', '\u{1}', s @ ..] => super::t1::n617(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n77(s, out),
		['o', 'u', 's', '\u{1}', s @ ..] => super::t1::n78(s, out),
		['p', s @ ..] => super::t1::n79(s, out),
		['q', s @ ..] => super::t1::n80(s, out),
		['r', s @ ..] => super::t1::n81(s, out),
		['s', s @ ..] => super::t1::n82(s, out),
		['t', s @ ..] => super::t1::n83(s, out),
		['u', s @ ..] => super::t1::n84(s, out),
		['v', s @ ..] => super::t1::n85(s, out),
		['x', s @ ..] => super::t1::n86(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2320);
							match s {
								[] => true,
								['t', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4441);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2322);
							match s {
								[] => true,
								['i', 'c', 'a', 't', s @ ..] => {
									super::push(out, 4442);
									match s {
										[] => true,
										['r', 'i', 'x', '\u{1}', s @ ..] => {
											super::push(out, 5743);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2323);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5744);
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5745);
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
					super::push(out, 659);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2324);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4443);
									s.is_empty()
								}
								['n', 'd', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 4445);
									s.is_empty()
								}
								['x', '\u{1}', s @ ..] => {
									super::push(out, 4446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4447);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['t', 'a', 's', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2329);
							match s {
								[] => true,
								['i', 'm', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2330);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2331);
							match s {
								[] => true,
								['n', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2333);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2334);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4451);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2335);
							match s {
								[] => true,
								['g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2336);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4454);
									s.is_empty()
								}
								['i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4455);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 4456);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4457);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2337);
							match s {
								[] => true,
								['n', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4459);
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
			super::push(out, 88);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4460);
									s.is_empty()
								}
								['u', 'a', s @ ..] => {
									super::push(out, 4461);
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5746);
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
					super::push(out, 672);
					s.is_empty()
				}
				['l', s @ ..] => super::t1::n685(s, out),
				['m', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['e', s @ ..] => super::t1::n2403(s, out),
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n687(s, out),
				['o', s @ ..] => super::t1::n688(s, out),
				['r', s @ ..] => super::t1::n689(s, out),
				['s', s @ ..] => super::t1::n690(s, out),
				['t', s @ ..] => super::t1::n691(s, out),
				['u', s @ ..] => super::t1::n692(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n89(s, out),
		['l', s @ ..] => super::t1::n90(s, out),
		['o', s @ ..] => super::t1::n91(s, out),
		['r', s @ ..] => super::t1::n92(s, out),
		['u', s @ ..] => super::t1::n93(s, out),
		_ => false,
	}
}

/// Words ending -g.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['t', 'a', 'n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2437);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4515);
									match s {
										[] => true,
										['i', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5761);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 5762);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'l', 'a', 'e', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2439);
							match s {
								[] => true,
								['i', 'n', s @ ..] => {
									super::push(out, 4516);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6179);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 6180);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5763);
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
					super::push(out, 718);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['v', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 725);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2442);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4517);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5764);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 5765);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2445);
							match s {
								[] => true,
								['i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2446);
							match s {
								[] => true,
								['a', 'n', 'i', s @ ..] => {
									super::push(out, 4520);
									match s {
										[] => true,
										['t', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 5766);
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
					super::push(out, 728);
					s.is_empty()
				}
				['t', 'h', 'y', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['g', 'e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['r', 'e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2447);
							s.is_empty()
						}
						['v', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2449);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2452);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2454);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2455);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2456);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4521);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4522);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4523);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['d', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2457);
							s.is_empty()
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2458);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2459);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4524);
									s.is_empty()
								}
								['t', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4526);
									s.is_empty()
								}
								['h', 'i', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['m', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				['e', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['i', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'y', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n100(s, out),
		['u', s @ ..] => super::t1::n101(s, out),
		['y', s @ ..] => super::t1::n102(s, out),
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 754);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2472);
							match s {
								[] => true,
								['n', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2473);
							match s {
								[] => true,
								['t', 'a', 't', 'i', s @ ..] => {
									super::push(out, 4543);
									match s {
										[] => true,
										['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5767);
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
					super::push(out, 755);
					match s {
						[] => true,
						['r', 'e', 'd', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['i', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2475);
							s.is_empty()
						}
						['l', 'u', 'c', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2476);
							s.is_empty()
						}
						['u', 'c', 'i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2478);
							match s {
								[] => true,
								['d', 'r', 'y', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'o', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2479);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 758);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2481);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6181);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 6182);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5768);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2483);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				['d', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2486);
							match s {
								[] => true,
								['o', 't', 'r', 'o', 'p', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4545);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2488);
							match s {
								[] => true,
								['e', 't', 'i', 'o', 'm', 'a', 's', 't', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 4547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2489);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2491);
							s.is_empty()
						}
						['t', 'e', 'r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2493);
							match s {
								[] => true,
								['a', 't', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2494);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4549);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4551);
									s.is_empty()
								}
								['n', 'a', 'c', 'e', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4552);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2495);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2496);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2497);
							match s {
								[] => true,
								['d', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 770);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2498);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5769);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 5770);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2499);
							match s {
								[] => true,
								['s', 'c', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 772);
					match s {
						[] => true,
						['r', 'a', 'c', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2501);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['s', 'i', 'd', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2504);
							s.is_empty()
						}
						['p', s @ ..] => super::t1::n2580(s, out),
						['s', s @ ..] => super::t1::n2581(s, out),
						['u', s @ ..] => super::t1::n2582(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t1::n790(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n106(s, out),
		['u', s @ ..] => super::t1::n107(s, out),
		['y', s @ ..] => super::t1::n108(s, out),
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						['u', 'l', 'a', s @ ..] => {
							super::push(out, 2525);
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4566);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4568);
									s.is_empty()
								}
								['t', 'r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 4569);
									s.is_empty()
								}
								['z', 'a', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4570);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'h', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['i', 'o', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				['r', 'i', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 799);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 804);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2533);
							s.is_empty()
						}
						['n', 'e', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['u', 'n', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['t', 'a', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2536);
							s.is_empty()
						}
						['u', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2538);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4571);
									match s {
										[] => true,
										['a', 'b', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5775);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 810);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2540);
							match s {
								[] => true,
								['p', 's', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2541);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4573);
									s.is_empty()
								}
								['g', 'i', 't', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2542);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2543);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4575);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', 'o', '\u{1}', s @ ..] => {
											super::push(out, 5776);
											s.is_empty()
										}
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 5777);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 5778);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'i', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 4576);
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
			super::push(out, 116);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 811);
					match s {
						[] => true,
						['g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2545);
							match s {
								[] => true,
								['c', 'i', 'l', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => super::t1::n4751(s, out),
								['t', s @ ..] => {
									super::push(out, 4579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n829(s, out),
				['p', s @ ..] => super::t1::n830(s, out),
				['u', 'm', '\u{1}', s @ ..] => super::t1::n831(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n117(s, out),
		['o', s @ ..] => super::t1::n118(s, out),
		['p', s @ ..] => super::t1::n119(s, out),
		['r', s @ ..] => super::t1::n120(s, out),
		['s', s @ ..] => super::t1::n121(s, out),
		['t', s @ ..] => super::t1::n122(s, out),
		['u', s @ ..] => super::t1::n123(s, out),
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	s.is_empty()
}

/// Words ending -l.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 862);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2642);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2643);
							match s {
								[] => true,
								['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4673);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2644);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2645);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									super::push(out, 4675);
									match s {
										[] => true,
										['t', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2646);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 863);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2647);
							match s {
								[] => true,
								['r', 'a', 'm', s @ ..] => {
									super::push(out, 4677);
									match s {
										[] => true,
										['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5817);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'n', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2648);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2649);
							match s {
								[] => true,
								['u', 'c', 'a', s @ ..] => {
									super::push(out, 4678);
									match s {
										[] => true,
										['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2650);
							match s {
								[] => true,
								['n', 'a', 'r', s @ ..] => {
									super::push(out, 4679);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5819);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 5820);
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
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 864);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 866);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 867);
					match s {
						[] => true,
						['i', 's', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 868);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2652);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2653);
							s.is_empty()
						}
						['i', 'n', s @ ..] => {
							super::push(out, 2654);
							s.is_empty()
						}
						['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2655);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2656);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 869);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2658);
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2659);
							s.is_empty()
						}
						['e', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2660);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2661);
							s.is_empty()
						}
						['i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2662);
							s.is_empty()
						}
						['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2663);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2664);
							s.is_empty()
						}
						['u', 'g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', 'h', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2666);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2667);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						['d', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2669);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2670);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5821);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 5822);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'd', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2671);
							s.is_empty()
						}
						['v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 874);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 2673);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2674);
							match s {
								[] => true,
								['i', 'v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4682);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['e', s @ ..] => super::t1::n2756(s, out),
						['h', 'y', 'r', 'o', 's', '\u{1}', s @ ..] => super::t1::n2757(s, out),
						['i', s @ ..] => super::t1::n2758(s, out),
						['o', s @ ..] => super::t1::n2759(s, out),
						['r', s @ ..] => super::t1::n2760(s, out),
						['u', s @ ..] => super::t1::n2761(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n892(s, out),
				['v', s @ ..] => super::t1::n893(s, out),
				['x', s @ ..] => super::t1::n894(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n125(s, out),
		['i', s @ ..] => super::t1::n126(s, out),
		['o', s @ ..] => super::t1::n127(s, out),
		['u', s @ ..] => super::t1::n128(s, out),
		['y', s @ ..] => super::t1::n129(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2760);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2761);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4756);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2762);
							match s {
								[] => true,
								['a', 'e', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4758);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2764);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2765);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['f', 'o', 'r', 's', '\u{1}', s @ ..] => {
					super::push(out, 928);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 929);
					match s {
						[] => true,
						['a', 'l', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2767);
							s.is_empty()
						}
						['n', 'e', 't', 'o', 's', 'c', 'o', 'p', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2769);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5848);
											s.is_empty()
										}
										['i', 't', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 5849);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4760);
									s.is_empty()
								}
								['g', 'r', 'a', 'n', 'a', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2771);
							match s {
								[] => true,
								['d', 'i', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2772);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2773);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2774);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2775);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 4763);
									s.is_empty()
								}
								['n', 'd', 'r', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2776);
							match s {
								[] => true,
								['t', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2777);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2778);
							match s {
								[] => true,
								['u', 'c', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2779);
							match s {
								[] => true,
								['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4767);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2780);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2781);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4769);
									match s {
										[] => true,
										['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5850);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2782);
							match s {
								[] => true,
								['u', 'e', 't', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2783);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2784);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4771);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4772);
									s.is_empty()
								}
								['p', 'r', 'e', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2785);
							match s {
								[] => true,
								['i', 'p', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2786);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2787);
							match s {
								[] => true,
								['n', 'e', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4775);
									s.is_empty()
								}
								['s', s @ ..] => super::t1::n4959(s, out),
								['t', 'a', s @ ..] => super::t1::n4960(s, out),
								_ => false,
							}
						}
						['p', 'o', 'r', '\u{1}', s @ ..] => super::t1::n2876(s, out),
						['s', s @ ..] => super::t1::n2877(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t1::n952(s, out),
				['t', s @ ..] => super::t1::n953(s, out),
				['x', 'i', 'l', 'l', 'a', '\u{1}', s @ ..] => super::t1::n954(s, out),
				['z', s @ ..] => super::t1::n955(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n131(s, out),
		['i', s @ ..] => super::t1::n132(s, out),
		['n', s @ ..] => super::t1::n133(s, out),
		['o', s @ ..] => super::t1::n134(s, out),
		['u', s @ ..] => super::t1::n135(s, out),
		['y', s @ ..] => super::t1::n136(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['o', 't', 'e', 'c', 'h', 'n', 'o', 'l', 'o', 'g', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2891);
							s.is_empty()
						}
						['i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2894);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2895);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2896);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2897);
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5874);
											match s {
												[] => true,
												['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
													super::push(out, 6206);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 5875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['a', 'm', 'o', 'n', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2899);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2900);
							s.is_empty()
						}
						['s', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 2903);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2904);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4858);
									match s {
										[] => true,
										['i', 'd', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5876);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 4859);
									s.is_empty()
								}
								['v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2905);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2906);
							match s {
								[] => true,
								['r', 'a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 986);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2907);
							match s {
								[] => true,
								['e', 'g', 'i', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['z', 'i', 's', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['a', 'n', 'i', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2908);
							match s {
								[] => true,
								['t', 'r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 4863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 993);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 994);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 997);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2914);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['e', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2916);
							s.is_empty()
						}
						['i', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1000);
					match s {
						[] => true,
						['t', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1002);
					match s {
						[] => true,
						['r', 'o', 's', 'c', 'e', 'l', 'e', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2920);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				['s', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2922);
							match s {
								[] => true,
								['d', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n140(s, out),
		['u', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['b', s @ ..] => super::t1::n1043(s, out),
				['c', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				['g', s @ ..] => super::t1::n1045(s, out),
				['l', s @ ..] => super::t1::n1046(s, out),
				['m', s @ ..] => super::t1::n1047(s, out),
				['n', s @ ..] => super::t1::n1048(s, out),
				['p', s @ ..] => super::t1::n1049(s, out),
				['r', s @ ..] => super::t1::n1050(s, out),
				['t', s @ ..] => super::t1::n1051(s, out),
				['x', '\u{1}', s @ ..] => super::t1::n1052(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t1::n142(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1035);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1036);
					match s {
						[] => true,
						['e', 'n', 's', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2948);
							match s {
								[] => true,
								['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4877);
									s.is_empty()
								}
								['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1037);
					match s {
						[] => true,
						['d', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2949);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2950);
							s.is_empty()
						}
						['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						['x', '\u{1}', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1038);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2953);
							s.is_empty()
						}
						['u', 'r', 'g', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1039);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2955);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2956);
							match s {
								[] => true,
								['q', 'u', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4879);
									s.is_empty()
								}
								['v', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1040);
					match s {
						[] => true,
						['u', 'b', 'i', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1041);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1042);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1043);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2959);
							match s {
								[] => true,
								['e', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4881);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4882);
									match s {
										[] => true,
										['r', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5877);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2960);
							match s {
								[] => true,
								['t', 'r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 4883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2961);
							s.is_empty()
						}
						['o', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2962);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2963);
							match s {
								[] => true,
								['a', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1044);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2964);
							match s {
								[] => true,
								['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4885);
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
			super::push(out, 144);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1046);
					match s {
						[] => true,
						['a', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1048);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2966);
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4886);
									s.is_empty()
								}
								['v', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', 'u', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2967);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2968);
							match s {
								[] => true,
								['p', 'h', 'o', 'r', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4888);
									s.is_empty()
								}
								['v', 'i', 'r', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4889);
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
			super::push(out, 145);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['n', 'o', 'l', 'y', 't', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2969);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1051);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4890);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4891);
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5878);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 5879);
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
			super::push(out, 146);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2971);
							s.is_empty()
						}
						['u', 'm', 'e', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2973);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5880);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 5881);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 6207);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 6208);
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
							super::push(out, 2974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1053);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1054);
					match s {
						[] => true,
						['a', s @ ..] => super::t1::n3073(s, out),
						['n', s @ ..] => {
							super::push(out, 2976);
							s.is_empty()
						}
						['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => super::t1::n3075(s, out),
						['t', 'u', 'm', '\u{1}', s @ ..] => super::t1::n3076(s, out),
						_ => false,
					}
				}
				['f', s @ ..] => super::t1::n1074(s, out),
				['i', s @ ..] => super::t1::n1075(s, out),
				['l', 'a', '\u{1}', s @ ..] => super::t1::n1076(s, out),
				['v', 'a', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => super::t1::n1077(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t1::n149(s, out),
		['n', s @ ..] => super::t1::n150(s, out),
		['o', s @ ..] => super::t1::n151(s, out),
		['p', s @ ..] => super::t1::n152(s, out),
		['r', s @ ..] => super::t1::n153(s, out),
		['s', s @ ..] => super::t1::n154(s, out),
		['t', s @ ..] => super::t1::n155(s, out),
		['v', s @ ..] => super::t1::n156(s, out),
		['x', s @ ..] => super::t1::n157(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['i', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 3015);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3018);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 4906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3020);
							match s {
								[] => true,
								['i', 'c', s @ ..] => {
									super::push(out, 4907);
									match s {
										[] => true,
										['a', 't', s @ ..] => {
											match s {
												[] => true,
												['i', 'o', '\u{1}', s @ ..] => {
													super::push(out, 6209);
													s.is_empty()
												}
												['o', 'r', '\u{1}', s @ ..] => {
													super::push(out, 6210);
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
							super::push(out, 3021);
							match s {
								[] => true,
								['i', 'n', 's', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3022);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5886);
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5887);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3023);
							match s {
								[] => true,
								['n', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 4909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3024);
							s.is_empty()
						}
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3025);
							s.is_empty()
						}
						['r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3026);
							s.is_empty()
						}
						['u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3028);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4910);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4911);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4912);
									match s {
										[] => true,
										['h', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5888);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 5889);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 6211);
													s.is_empty()
												}
												['o', '\u{1}', s @ ..] => {
													super::push(out, 6212);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 5890);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3029);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3030);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3031);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3032);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3033);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									super::push(out, 4913);
									match s {
										[] => true,
										['e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 5891);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3034);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4914);
									match s {
										[] => true,
										['i', 'c', 'o', 'l', 'a', '\u{1}', s @ ..] => super::t1::n6152(s, out),
										_ => false,
									}
								}
								['m', s @ ..] => super::t1::n5104(s, out),
								['s', s @ ..] => super::t1::n5105(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n1120(s, out),
				['n', s @ ..] => super::t1::n1121(s, out),
				['p', s @ ..] => super::t1::n1122(s, out),
				['r', s @ ..] => super::t1::n1123(s, out),
				['s', s @ ..] => super::t1::n1124(s, out),
				['t', s @ ..] => super::t1::n1125(s, out),
				['u', s @ ..] => super::t1::n1126(s, out),
				['v', s @ ..] => super::t1::n1127(s, out),
				['x', s @ ..] => super::t1::n1128(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n159(s, out),
		['h', s @ ..] => super::t1::n160(s, out),
		['i', s @ ..] => super::t1::n161(s, out),
		['l', s @ ..] => super::t1::n162(s, out),
		['n', s @ ..] => super::t1::n163(s, out),
		['o', s @ ..] => super::t1::n164(s, out),
		['r', s @ ..] => super::t1::n165(s, out),
		['s', s @ ..] => super::t1::n166(s, out),
		['t', s @ ..] => super::t1::n167(s, out),
		['u', s @ ..] => super::t1::n168(s, out),
		['y', s @ ..] => super::t1::n169(s, out),
		_ => false,
	}
}

/// Words ending -q.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3243);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5973);
											match s {
												[] => true,
												['g', 'e', 's', 'i', 'm', 'a', '\u{1}', s @ ..] => {
													super::push(out, 6231);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 6232);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 5974);
											match s {
												[] => true,
												['g', 'a', s @ ..] => {
													super::push(out, 6233);
													match s {
														[] => true,
														['r', 'i', 'u', 's', '\u{1}', s @ ..] => super::t1::n6566(s, out),
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
							super::push(out, 3244);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3245);
							match s {
								[] => true,
								['e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5087);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5088);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3246);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3247);
							match s {
								[] => true,
								['c', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5090);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['d', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3248);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3249);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5092);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3250);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3251);
							match s {
								[] => true,
								['q', 'u', 'a', 'g', 'e', 's', 'i', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3252);
							match s {
								[] => true,
								['i', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5095);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'q', 'u', 'i', 'l', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3253);
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
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1185);
					match s {
						[] => true,
						['a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3254);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3255);
							match s {
								[] => true,
								['m', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5097);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5098);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5099);
									s.is_empty()
								}
								['x', '\u{1}', s @ ..] => {
									super::push(out, 5100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['c', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						['g', 'i', 'f', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3259);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5101);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3262);
							s.is_empty()
						}
						['i', 's', 't', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3263);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3264);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['e', 'f', 'a', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3266);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1193);
					match s {
						[] => true,
						['o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['i', 'o', s @ ..] => {
							super::push(out, 3269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5102);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 5103);
									s.is_empty()
								}
								['n', 'a', 'l', s @ ..] => {
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 5975);
											s.is_empty()
										}
										['i', 's', 't', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5976);
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
					super::push(out, 1195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n172(s, out),
		['h', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3326);
							s.is_empty()
						}
						['u', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3327);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 3328);
							match s {
								[] => true,
								['i', 's', '\u{1}', s @ ..] => {
									super::push(out, 5157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['n', 'o', 'c', 'e', 'r', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 3329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3331);
							s.is_empty()
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'i', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				['n', 'o', 'c', 'e', 'r', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1221);
					match s {
						[] => true,
						['a', 't', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1224);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3334);
							match s {
								[] => true,
								['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['a', 't', 'r', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 3335);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 3336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1226);
					match s {
						[] => true,
						['o', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1228);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5159);
									s.is_empty()
								}
								['t', 'i', 'u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1229);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3338);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5984);
											s.is_empty()
										}
										['i', 't', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 5985);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 5986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'h', 'e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['s', s @ ..] => super::t1::n1253(s, out),
				['t', s @ ..] => super::t1::n1254(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n176(s, out),
		['y', 'x', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3364);
							s.is_empty()
						}
						['o', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3365);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3366);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5173);
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5987);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['p', 'i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3368);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5175);
									s.is_empty()
								}
								['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3369);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5177);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 5988);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3371);
							match s {
								[] => true,
								['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5179);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 5180);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5181);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5989);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 5990);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3372);
							match s {
								[] => true,
								['l', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5182);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 5183);
									match s {
										[] => true,
										['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
											super::push(out, 5991);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3373);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3374);
							match s {
								[] => true,
								['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3375);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3376);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5185);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5186);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5187);
									match s {
										[] => true,
										['a', 't', 'o', s @ ..] => {
											super::push(out, 5992);
											match s {
												[] => true,
												['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
													super::push(out, 6234);
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
							super::push(out, 3377);
							match s {
								[] => true,
								['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['b', 'u', 'c', 'i', s @ ..] => {
							super::push(out, 3378);
							match s {
								[] => true,
								['s', 't', 'r', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3380);
							match s {
								[] => true,
								['t', 'r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 5190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 3381);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3382);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3383);
							s.is_empty()
						}
						['i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3384);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3385);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1250);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3387);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1251);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3388);
							match s {
								[] => true,
								['s', 'o', 'p', 'h', 'o', 'r', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 5192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1253);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3389);
							match s {
								[] => true,
								['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5193);
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
		['c', s @ ..] => super::t1::n179(s, out),
		['e', s @ ..] => super::t1::n180(s, out),
		['i', s @ ..] => super::t1::n181(s, out),
		['m', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1291);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1292);
					s.is_empty()
				}
				['i', 'l', 'a', 'x', '\u{1}', s @ ..] => {
					super::push(out, 1293);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1294);
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n183(s, out),
		['p', s @ ..] => super::t1::n184(s, out),
		['q', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'i', 't', 'u', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5300);
									s.is_empty()
								}
								['m', 'i', 'g', 'e', 'r', 'i', '\u{1}', s @ ..] => {
									super::push(out, 5301);
									s.is_empty()
								}
								['t', 'i', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t1::n3659(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n186(s, out),
		['u', s @ ..] => super::t1::n187(s, out),
		['y', s @ ..] => super::t1::n188(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3598);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5351);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5352);
									match s {
										[] => true,
										['n', 'a', 'c', s @ ..] => {
											super::push(out, 6043);
											match s {
												[] => true,
												['u', 'l', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 6251);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3599);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3600);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5354);
									match s {
										[] => true,
										['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6044);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1339);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3601);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', 's', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5356);
									s.is_empty()
								}
								['s', 's', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3603);
							s.is_empty()
						}
						['e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3604);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3605);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3606);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3607);
							s.is_empty()
						}
						['u', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				['p', 'r', 'o', 'b', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1344);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1346);
					s.is_empty()
				}
				['x', 'i', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['r', 'a', 'e', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1349);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3610);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1350);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3612);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5358);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6045);
											s.is_empty()
										}
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6046);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 5359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1351);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3613);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 5360);
									match s {
										[] => true,
										['i', 's', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6047);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3614);
							match s {
								[] => true,
								['c', 'a', 'r', 'd', 'i', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 5361);
									s.is_empty()
								}
								['r', 'r', 'h', 'i', 'z', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 5362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3615);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1352);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5363);
									s.is_empty()
								}
								['t', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 3617);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3618);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5365);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5366);
									match s {
										[] => true,
										['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6048);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'l', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1353);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3620);
							match s {
								[] => true,
								['c', s @ ..] => super::t1::n5586(s, out),
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3621);
							s.is_empty()
						}
						['e', s @ ..] => super::t1::n3749(s, out),
						['s', s @ ..] => super::t1::n3750(s, out),
						['t', s @ ..] => super::t1::n3751(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t1::n1379(s, out),
				['r', s @ ..] => super::t1::n1380(s, out),
				['s', s @ ..] => super::t1::n1381(s, out),
				['t', s @ ..] => super::t1::n1382(s, out),
				['x', s @ ..] => super::t1::n1383(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t1::n191(s, out),
		['i', s @ ..] => super::t1::n192(s, out),
		['m', 'e', 's', 'i', 's', '\u{1}', s @ ..] => super::t1::n193(s, out),
		['o', s @ ..] => super::t1::n194(s, out),
		['r', s @ ..] => super::t1::n195(s, out),
		['u', s @ ..] => super::t1::n196(s, out),
		['y', s @ ..] => super::t1::n197(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1394);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1395);
					match s {
						[] => true,
						['c', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3711);
							s.is_empty()
						}
						['q', 'u', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['v', 'o', 'r', 'n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1397);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1398);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', 'n', 'o', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5426);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 5427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1399);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1401);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3714);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3715);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6068);
											s.is_empty()
										}
										['t', 'i', 'c', 'o', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6069);
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
					super::push(out, 1402);
					s.is_empty()
				}
				['i', 'd', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1403);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['i', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3716);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3717);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3719);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3720);
							s.is_empty()
						}
						['u', 'l', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5428);
									s.is_empty()
								}
								['t', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1406);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1407);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1408);
					match s {
						[] => true,
						['v', 'i', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'i', 'l', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 202);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1410);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1411);
					match s {
						[] => true,
						['a', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3723);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3724);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3726);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3727);
							match s {
								[] => true,
								['a', 't', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['i', 'c', 'a', 'r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3730);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3732);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3733);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5431);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 5432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['n', 's', 'i', 'l', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3737);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3738);
							match s {
								[] => true,
								['c', 'i', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5433);
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
			super::push(out, 206);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['o', 'r', 'i', 'c', 'i', 'd', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -v.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['a', 't', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3739);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3740);
							match s {
								[] => true,
								['v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1429);
					match s {
						[] => true,
						['r', 'a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3742);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1431);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3744);
							match s {
								[] => true,
								['n', 't', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5435);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5436);
									match s {
										[] => true,
										['u', 'd', 'i', 'n', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6070);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1432);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3746);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['o', 'r', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3748);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3750);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3751);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 3752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3753);
							s.is_empty()
						}
						['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3754);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 5437);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3755);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5439);
									s.is_empty()
								}
								['l', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['o', 'r', 'd', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3757);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3759);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5441);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 5442);
									match s {
										[] => true,
										['e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 6071);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3760);
							match s {
								[] => true,
								['t', 'e', 'r', 'n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3761);
							s.is_empty()
						}
						['o', 'c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3763);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5444);
									s.is_empty()
								}
								['b', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5445);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 5446);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5447);
									match s {
										[] => true,
										['r', 'i', 'x', '\u{1}', s @ ..] => {
											super::push(out, 6072);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3764);
							match s {
								[] => true,
								['i', 't', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3765);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 5449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3766);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3767);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3768);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5450);
									s.is_empty()
								}
								['s', 't', s @ ..] => {
									super::push(out, 5451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['r', s @ ..] => super::t1::n1473(s, out),
				['s', s @ ..] => super::t1::n1474(s, out),
				['t', s @ ..] => super::t1::n1475(s, out),
				['x', s @ ..] => super::t1::n1476(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n211(s, out),
		['o', s @ ..] => super::t1::n212(s, out),
		['u', s @ ..] => super::t1::n213(s, out),
		_ => false,
	}
}

/// Words ending -w.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['i', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['y', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1469);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1470);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3834);
							s.is_empty()
						}
						['x', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -y.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	s.is_empty()
}

/// Words ending -z.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['o', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				['b', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				['d', 'o', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1477);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3837);
							match s {
								[] => true,
								['t', 'r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 5512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				['p', 'h', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'u', s @ ..] => {
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5513);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 3840);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1479);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1481);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1483);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3843);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 6087);
											s.is_empty()
										}
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6088);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 3844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['e', 'g', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				['i', 'l', 'a', 'x', '\u{1}', s @ ..] => {
					super::push(out, 1485);
					s.is_empty()
				}
				['y', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['m', 'o', 't', 'e', 'g', 'a', 'n', 'i', 't', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1489);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3846);
							s.is_empty()
						}
						['s', 'p', 'o', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				['r', 'a', 'n', 'i', 's', 'c', 'e', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1494);
					match s {
						[] => true,
						['h', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3848);
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

/// Words ending -teca.
pub(crate) fn n1554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1523);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3867);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5517);
					s.is_empty()
				}
				['r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 's', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hca.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1524);
			s.is_empty()
		}
		['e', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ica.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	match s {
		[] => true,
		['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1526);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1527);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5519);
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

/// The word oitisiuqca.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	s.is_empty()
}

/// Words ending -rca.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['e', 'd', 'u', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1528);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['d', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3870);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1530);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3872);
					s.is_empty()
				}
				['m', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3873);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tca.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1531);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', '\u{1}', s @ ..] => {
			super::push(out, 1532);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3875);
					s.is_empty()
				}
				['v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1534);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3877);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3878);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uca.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1535);
			s.is_empty()
		}
		['c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1536);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1537);
			match s {
				[] => true,
				['e', 'o', 'n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1538);
			s.is_empty()
		}
		['p', 'u', 'n', 'c', 't', 'u', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1539);
			s.is_empty()
		}
		['s', 't', 'i', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1540);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -da.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['u', 'n', 'e', 'p', 'h', 'r', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 3880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['l', 'a', 'm', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				['m', 'p', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['i', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1552);
					match s {
						[] => true,
						['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3883);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3884);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6090);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 6091);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3885);
							match s {
								[] => true,
								['o', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6092);
											s.is_empty()
										}
										['i', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6093);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 5521);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5522);
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
			super::push(out, 254);
			match s {
				[] => true,
				['a', 'p', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1555);
					match s {
						[] => true,
						['v', 'a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3887);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3888);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 6094);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 6095);
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
					super::push(out, 1556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1559);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3889);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['a', 'r', 'i', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1562);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 3891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1563);
					match s {
						[] => true,
						['i', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3893);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3894);
							match s {
								[] => true,
								['s', 'c', 'e', 'n', 't', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1565);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1566);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1567);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5524);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5525);
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

/// Words ending -ea.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1568);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3897);
							match s {
								[] => true,
								['i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1570);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3898);
							s.is_empty()
						}
						['l', 'i', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 3899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3900);
							s.is_empty()
						}
						['o', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 264);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['e', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3902);
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5527);
									s.is_empty()
								}
								['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5528);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3904);
							s.is_empty()
						}
						['m', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3905);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['t', 'i', 'v', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1581);
					match s {
						[] => true,
						['t', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['i', 't', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fa.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'i', 'c', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ga.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 274);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 276);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['m', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['a', 'n', 'o', 'm', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['a', 't', 'i', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['o', 's', 't', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word muneha.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	s.is_empty()
}

/// Words ending -la.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3914);
							s.is_empty()
						}
						['r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['h', 'e', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['m', 'i', 's', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3916);
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
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1603);
					match s {
						[] => true,
						['t', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1606);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['a', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1608);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1609);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1610);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3918);
							s.is_empty()
						}
						['t', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1612);
					match s {
						[] => true,
						['i', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3920);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1613);
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
					super::push(out, 1614);
					match s {
						[] => true,
						['n', 't', 'o', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 3922);
							s.is_empty()
						}
						['t', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1615);
					match s {
						[] => true,
						['c', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3924);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3925);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 3926);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6096);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 6097);
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
					super::push(out, 1616);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['d', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1620);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							super::push(out, 3928);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5530);
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
			super::push(out, 290);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['i', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3930);
							s.is_empty()
						}
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3931);
							s.is_empty()
						}
						['i', 'n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3932);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1624);
					match s {
						[] => true,
						['i', 'a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['p', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1629);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['a', 'n', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3936);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3937);
							s.is_empty()
						}
						['y', 'l', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 3938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3939);
							s.is_empty()
						}
						['i', 'u', s @ ..] => {
							super::push(out, 3940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'o', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				['e', 's', 't', 'r', 'i', 'x', '\u{1}', s @ ..] => {
					super::push(out, 1635);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1636);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3942);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3943);
							match s {
								[] => true,
								['a', 't', 'i', s @ ..] => {
									super::push(out, 5531);
									match s {
										[] => true,
										['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6098);
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
			super::push(out, 295);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1638);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1640);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3944);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5532);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3945);
							s.is_empty()
						}
						['u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['o', 'n', 'i', 'a', 'c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['e', 's', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1645);
					s.is_empty()
				}
				['l', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				['m', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1648);
					s.is_empty()
				}
				['t', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 301);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['n', 'o', 's', 't', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 304);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['a', 'b', 'a', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['l', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 3948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'h', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'r', 'a', 'c', 't', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 307);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['e', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1656);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3949);
							match s {
								[] => true,
								['l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5534);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1661);
					match s {
						[] => true,
						['l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3953);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'm', 'a', 'l', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1663);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3955);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3956);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5535);
									match s {
										[] => true,
										['p', 'o', 'c', 'e', 'n', 't', 'r', 'i', 's', 'm', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6099);
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
					super::push(out, 1664);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3957);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 3958);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'g', 'o', 'g', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5536);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5537);
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
			super::push(out, 315);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['i', 'e', 't', 'u', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['l', 'i', 'o', 't', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3959);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3960);
							s.is_empty()
						}
						['s', 't', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['u', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['h', 'e', 'c', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1672);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5538);
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6100);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 6101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1673);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				['u', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['u', 'm', 'e', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['e', 'n', 'o', 'd', 'y', 't', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qa.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5540);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3963);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 6102);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 6103);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['b', 's', '\u{1}', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				['m', 'a', 'i', 's', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1685);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3965);
							s.is_empty()
						}
						['t', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 327);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['t', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1689);
					match s {
						[] => true,
						['i', 'e', 'l', 'e', 'c', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3970);
							s.is_empty()
						}
						['p', 'a', 'g', 'i', 't', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'e', 'n', 'n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1693);
					s.is_empty()
				}
				['t', 'a', 'l', 'o', 'g', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1696);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1697);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3972);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3973);
							s.is_empty()
						}
						['t', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1701);
					match s {
						[] => true,
						['u', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1702);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							super::push(out, 3976);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3977);
							s.is_empty()
						}
						['t', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1703);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3979);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1706);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3981);
							s.is_empty()
						}
						['r', 'i', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5542);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5543);
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
			super::push(out, 338);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['i', 'n', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3982);
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
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1709);
					match s {
						[] => true,
						['l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3983);
							s.is_empty()
						}
						['n', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['e', 'p', 'i', 'a', 'd', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1713);
					match s {
						[] => true,
						['i', 'p', 't', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3986);
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
				['i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['r', 'c', 'h', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', '\u{1}', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 343);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['r', 'n', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1720);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3989);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1722);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3991);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3992);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5544);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1724);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3994);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1725);
					match s {
						[] => true,
						['r', 'i', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1727);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3997);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3998);
							s.is_empty()
						}
						['t', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 348);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				['l', 'e', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['a', 't', 'e', 'r', 't', 'e', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 351);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['e', 'b', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4000);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['c', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4002);
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

/// Words ending -ua.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['t', 'o', s @ ..] => {
					super::push(out, 1738);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 4003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['c', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', s @ ..] => {
					match s {
						[] => true,
						['i', 'u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4005);
							s.is_empty()
						}
						['o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		['m', 'a', 't', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4007);
							s.is_empty()
						}
						['t', 'u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1741);
					match s {
						[] => true,
						['p', 'i', 'g', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['h', 'o', s @ ..] => {
					super::push(out, 1743);
					match s {
						[] => true,
						['r', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1744);
					s.is_empty()
				}
				['r', 'i', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4012);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1745);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4014);
							s.is_empty()
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1747);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['c', 'a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xa.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', 'm', 'e', 'n', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['o', 't', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gilac.
pub(crate) fn n4207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4059);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5556);
			match s {
				[] => true,
				['t', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 5557);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sutahpilac.
pub(crate) fn n5792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5558);
	s.is_empty()
}

/// Words ending -llac.
pub(crate) fn n1873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1830);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4061);
			s.is_empty()
		}
		['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 4062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olac.
pub(crate) fn n1874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1831);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mac.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 4064);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1833);
			match s {
				[] => true,
				['n', 'a', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1834);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4067);
					match s {
						[] => true,
						['n', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 4068);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nac.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1836);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4069);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6107);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6108);
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
			super::push(out, 1837);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 4070);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6109);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6110);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 6292);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 6293);
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
		['i', s @ ..] => {
			super::push(out, 1839);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4071);
					match s {
						[] => true,
						['a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5560);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4072);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1840);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1841);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pac.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1843);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4074);
					s.is_empty()
				}
				['n', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1844);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4076);
					match s {
						[] => true,
						['l', 'a', 'm', s @ ..] => {
							super::push(out, 5562);
							match s {
								[] => true,
								['e', 'n', 't', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 1845);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1846);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1847);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1848);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4078);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4079);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									super::push(out, 6112);
									s.is_empty()
								}
								['i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 6113);
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
			super::push(out, 1849);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rac.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1850);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4080);
					match s {
						[] => true,
						['i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4081);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5564);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5565);
							match s {
								[] => true,
								['i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1852);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1853);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1854);
			match s {
				[] => true,
				['n', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4085);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4086);
					s.is_empty()
				}
				['t', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1856);
			match s {
				[] => true,
				['a', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1857);
			match s {
				[] => true,
				['t', 'i', 'd', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 4089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1858);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1859);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sac.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1860);
			match s {
				[] => true,
				['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4091);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4092);
					s.is_empty()
				}
				['o', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4093);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1862);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4095);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4096);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5566);
							s.is_empty()
						}
						['t', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 5567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4098);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tac.
pub(crate) fn n435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1866);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4099);
					s.is_empty()
				}
				['g', 'o', 'r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4100);
					s.is_empty()
				}
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4101);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4102);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6115);
									s.is_empty()
								}
								['t', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1868);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1869);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4104);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6117);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 6118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1870);
			match s {
				[] => true,
				['b', 'l', 'e', 'p', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4106);
					s.is_empty()
				}
				['n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1871);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uac.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1872);
			match s {
				[] => true,
				['o', 'n', 'a', s @ ..] => {
					super::push(out, 4108);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5569);
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

/// Words ending -vac.
pub(crate) fn n437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['o', 's', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ec.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 427);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['a', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4109);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4110);
							s.is_empty()
						}
						['t', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1878);
					match s {
						[] => true,
						['e', 's', 'i', 'm', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1880);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4113);
							s.is_empty()
						}
						['e', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1881);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4115);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4116);
							s.is_empty()
						}
						['t', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4118);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1883);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4120);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1884);
					s.is_empty()
				}
				['o', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1886);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1887);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1890);
					match s {
						[] => true,
						['c', 'e', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4122);
							s.is_empty()
						}
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'a', '\u{1}', s @ ..] => {
			super::push(out, 435);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hc.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1894);
					match s {
						[] => true,
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4124);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1895);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4126);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5570);
									match s {
										[] => true,
										['t', 'e', 'r', 'i', 's', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6119);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 5571);
									match s {
										[] => true,
										['r', 'i', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 6120);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4128);
							match s {
								[] => true,
								['c', 'e', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5572);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5573);
									match s {
										[] => true,
										['o', 'n', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6263);
													s.is_empty()
												}
												['i', 'a', '\u{1}', s @ ..] => {
													super::push(out, 6264);
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
							super::push(out, 4129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				['n', 'i', 's', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1897);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1899);
					match s {
						[] => true,
						['u', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', 'a', 'r', 'c', 'h', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1900);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				['t', 'i', 'n', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1904);
					match s {
						[] => true,
						['r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1906);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4132);
							match s {
								[] => true,
								['s', 't', 'e', 'r', 'o', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'a', 'm', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4134);
							s.is_empty()
						}
						['e', 'p', 'i', 's', 'c', 'o', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4135);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1907);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1908);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5575);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5576);
									match s {
										[] => true,
										['o', 'g', 'r', 'a', 'm', 'm', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6121);
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
					super::push(out, 1909);
					match s {
						[] => true,
						['m', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4137);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1910);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5577);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 6122);
											s.is_empty()
										}
										['g', 'a', 's', 't', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6123);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 6124);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 6125);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6126);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5578);
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
			super::push(out, 442);
			match s {
				[] => true,
				['m', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				['r', 'o', 'g', 'r', 'y', 'l', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ic.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				['o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1915);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4139);
							s.is_empty()
						}
						['t', 'r', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 4140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1918);
					match s {
						[] => true,
						['t', 'i', 'c', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['a', 'l', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1919);
					s.is_empty()
				}
				['e', 'l', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1920);
					s.is_empty()
				}
				['l', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 446);
			s.is_empty()
		}
		['l', 'o', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['u', 's', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1925);
					match s {
						[] => true,
						['i', 'n', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4142);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1926);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4144);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1932);
					match s {
						[] => true,
						['i', 'n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4147);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 5580);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4148);
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

/// Words ending -lc.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['c', 'e', 'n', 'd', 'i', 'x', '\u{1}', s @ ..] => {
					super::push(out, 1933);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1934);
					match s {
						[] => true,
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 4149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'g', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1935);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1936);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4150);
							s.is_empty()
						}
						['n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1937);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1938);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1939);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5582);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				['n', 'o', 'd', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1942);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1943);
					s.is_empty()
				}
				['t', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['b', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				['d', 'i', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['m', 'a', 'x', '\u{1}', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				['n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1950);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1951);
					s.is_empty()
				}
				['v', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['s', 't', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1957);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nc.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['a', 's', 'o', 'n', 'a', 'e', '\u{1}', s @ ..] => {
			super::push(out, 461);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['c', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'd', 'a', 'x', '\u{1}', s @ ..] => {
			super::push(out, 463);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oc.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1960);
					match s {
						[] => true,
						['e', 'r', 'v', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4154);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4156);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1962);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4158);
							s.is_empty()
						}
						['s', 't', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1964);
					match s {
						[] => true,
						['e', 't', 'e', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['a', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4162);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4164);
							match s {
								[] => true,
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5584);
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
			super::push(out, 471);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1967);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4165);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4167);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5585);
									match s {
										[] => true,
										['i', 'v', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 6127);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4168);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5586);
									s.is_empty()
								}
								['g', 'i', s @ ..] => {
									super::push(out, 5587);
									match s {
										[] => true,
										['a', 't', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 6128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4169);
							match s {
								[] => true,
								['b', 'e', 'r', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5588);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4170);
							match s {
								[] => true,
								['c', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5589);
									s.is_empty()
								}
								['s', 't', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5590);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 5591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 4171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1969);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4172);
							match s {
								[] => true,
								['a', 't', 'h', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 4173);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4174);
							match s {
								[] => true,
								['a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5593);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5594);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6265);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 6266);
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
							super::push(out, 4175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1970);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4176);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'h', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1971);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4178);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'h', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n2021(s, out),
				['b', s @ ..] => super::t1::n2022(s, out),
				['e', s @ ..] => super::t1::n2023(s, out),
				['i', s @ ..] => super::t1::n2024(s, out),
				['m', s @ ..] => super::t1::n2025(s, out),
				['o', s @ ..] => super::t1::n2026(s, out),
				['p', s @ ..] => super::t1::n2027(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n486(s, out),
		['o', s @ ..] => super::t1::n487(s, out),
		['p', s @ ..] => super::t1::n488(s, out),
		['q', s @ ..] => super::t1::n489(s, out),
		['r', s @ ..] => super::t1::n490(s, out),
		['s', s @ ..] => super::t1::n491(s, out),
		['t', s @ ..] => super::t1::n492(s, out),
		['v', s @ ..] => super::t1::n493(s, out),
		['x', s @ ..] => super::t1::n494(s, out),
		_ => false,
	}
}

/// Words ending -rc.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['b', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2019);
					s.is_empty()
				}
				['c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2020);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['b', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4259);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2021);
					s.is_empty()
				}
				['p', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2024);
					match s {
						[] => true,
						['e', 'r', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4261);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2025);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', 'i', 'x', '\u{1}', s @ ..] => {
									super::push(out, 5672);
									s.is_empty()
								}
								['u', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2026);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2027);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2028);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4263);
							match s {
								[] => true,
								['t', 'o', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				['s', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2031);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4265);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 486);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2032);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2033);
					s.is_empty()
				}
				['n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2035);
					match s {
						[] => true,
						['p', 'i', 't', 'u', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4267);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['b', 'y', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4269);
							s.is_empty()
						}
						['i', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4270);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2038);
					match s {
						[] => true,
						['i', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5675);
									s.is_empty()
								}
								['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5676);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5677);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2039);
					match s {
						[] => true,
						['e', 'l', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4272);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2040);
					match s {
						[] => true,
						['e', 'n', 'i', 's', 'e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2041);
					match s {
						[] => true,
						['c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4275);
							s.is_empty()
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4276);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uc.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['d', 'o', '\u{1}', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2042);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2043);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2044);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 494);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2045);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2046);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2048);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4281);
							s.is_empty()
						}
						['n', 'g', 'e', 'r', 'i', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2050);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2051);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2052);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2053);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2054);
					match s {
						[] => true,
						['e', 'd', 'i', 'n', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2055);
					s.is_empty()
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2057);
					match s {
						[] => true,
						['l', 'i', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4284);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5678);
									s.is_empty()
								}
								['u', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2058);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2059);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4285);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4286);
							match s {
								[] => true,
								['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4287);
							s.is_empty()
						}
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2060);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				['t', 'i', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2062);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['t', 'o', 'd', 'i', s @ ..] => {
					super::push(out, 2064);
					match s {
						[] => true,
						['t', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['i', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yc.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		['d', 'o', 'n', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['i', 't', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2069);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 506);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2072);
					match s {
						[] => true,
						['o', 'n', 'i', 's', 'm', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		['z', 'i', 'c', 'e', 'n', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -reced.
pub(crate) fn n4453(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 5683);
			s.is_empty()
		}
		['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5684);
			s.is_empty()
		}
		['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iced.
pub(crate) fn n2131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2078);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 4297);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lced.
pub(crate) fn n2132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2079);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4299);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4300);
			match s {
				[] => true,
				['v', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5686);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oced.
pub(crate) fn n2133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2080);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4301);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rced.
pub(crate) fn n2134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2081);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5687);
					s.is_empty()
				}
				['s', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5688);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uced.
pub(crate) fn n2135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2082);
	match s {
		[] => true,
		['b', 'i', 't', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4303);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4304);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5690);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fed.
pub(crate) fn n530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 519);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2083);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2084);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4306);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4307);
					match s {
						[] => true,
						['e', 's', 't', 'r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5692);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5693);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6156);
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
			super::push(out, 2085);
			match s {
				[] => true,
				['n', 'i', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2086);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2087);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4309);
					s.is_empty()
				}
				['s', 's', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2088);
			s.is_empty()
		}
		['u', 'n', 'c', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ged.
pub(crate) fn n531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	s.is_empty()
}

/// Words ending -hed.
pub(crate) fn n532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 521);
	match s {
		[] => true,
		['i', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2090);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2091);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ied.
pub(crate) fn n533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 522);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2092);
			s.is_empty()
		}
		['d', 'e', 'c', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2093);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2094);
			s.is_empty()
		}
		['f', 'i', 'c', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2095);
			s.is_empty()
		}
		['p', 'a', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2096);
			s.is_empty()
		}
		['t', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2097);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2098);
			match s {
				[] => true,
				['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4311);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -led.
pub(crate) fn n534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 523);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2099);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['r', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 5694);
							s.is_empty()
						}
						['u', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2100);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4312);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6157);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4313);
					s.is_empty()
				}
				['n', 'i', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4314);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4315);
					match s {
						[] => true,
						['r', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 5696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2101);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4316);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4317);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4318);
					match s {
						[] => true,
						['a', 'm', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2102);
			s.is_empty()
		}
		['t', 'i', 'd', 'i', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2103);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2104);
			match s {
				[] => true,
				['b', 'r', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -med.
pub(crate) fn n535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 524);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2105);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2106);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2107);
			match s {
				[] => true,
				['c', 'r', 'a', 't', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4320);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4321);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4322);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ned.
pub(crate) fn n536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	match s {
		[] => true,
		['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2108);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2109);
			s.is_empty()
		}
		['i', 'g', 'r', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 2110);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2111);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4323);
					s.is_empty()
				}
				['t', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2112);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2113);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ped.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 526);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2115);
			s.is_empty()
		}
		['l', 'o', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 2116);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2117);
			match s {
				[] => true,
				['n', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 4325);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2118);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4327);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5698);
							s.is_empty()
						}
						['v', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4328);
					match s {
						[] => true,
						['c', 'a', 't', 'i', s @ ..] => {
							super::push(out, 5700);
							match s {
								[] => true,
								['u', 'n', 'c', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6159);
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

/// Words ending -red.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 527);
	match s {
		[] => true,
		['b', 'i', 't', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2119);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2120);
			match s {
				[] => true,
				['d', 'i', 'c', 'u', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4329);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4330);
					s.is_empty()
				}
				['v', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 't', 'i', 't', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 2121);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sed.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 528);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2123);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2124);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2125);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5701);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6160);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2126);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2127);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4333);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5703);
							s.is_empty()
						}
						['r', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4334);
					s.is_empty()
				}
				['u', 'm', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2128);
			match s {
				[] => true,
				['i', 'l', 'l', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ted.
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 529);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2129);
			match s {
				[] => true,
				['c', 't', 'i', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2130);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4338);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5705);
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

/// Words ending -ued.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 530);
	s.is_empty()
}

/// Words ending -ved.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 531);
	match s {
		[] => true,
		['a', 's', 't', 'a', 't', 'i', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2131);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2132);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2133);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4339);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4340);
					match s {
						[] => true,
						['a', 't', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5706);
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

/// Words ending -xed.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 532);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 4341);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4342);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4343);
					match s {
						[] => true,
						['a', 'l', 'i', 'o', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5707);
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

