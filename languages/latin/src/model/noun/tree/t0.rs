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
				['a', 'b', 'o', 'h', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 39);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 40);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 41);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'u', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 42);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 250);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 251);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 252);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 253);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 254);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 43);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 44);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 257);
							match s {
								[] => true,
								['r', 'i', s @ ..] => {
									super::push(out, 854);
									match s {
										[] => true,
										['l', 'a', 'd', 'n', 'a', 'v', '\u{1}', s @ ..] => {
											super::push(out, 1846);
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
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 45);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 47);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 260);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 855);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 856);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 857);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 858);
									s.is_empty()
								}
								['t', 'y', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 261);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 860);
									s.is_empty()
								}
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 861);
									s.is_empty()
								}
								['t', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 262);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 263);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 863);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 1847);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 264);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 265);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 864);
									s.is_empty()
								}
								['t', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 49);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 50);
					match s {
						[] => true,
						['a', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 266);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 267);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'i', 's', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 268);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 269);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 270);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 271);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 272);
							s.is_empty()
						}
						['u', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 51);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 868);
									s.is_empty()
								}
								['s', 'u', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 274);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 275);
							s.is_empty()
						}
						['r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 54);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 278);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 279);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 872);
									s.is_empty()
								}
								['p', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 873);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 280);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['q', s @ ..] => super::t0::n285(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 56);
					match s {
						[] => true,
						['e', 'c', 's', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'h', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 57);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n6(s, out),
		['i', s @ ..] => super::t0::n7(s, out),
		['l', s @ ..] => super::t0::n8(s, out),
		['m', s @ ..] => super::t0::n9(s, out),
		['n', s @ ..] => super::t0::n10(s, out),
		['o', s @ ..] => super::t0::n11(s, out),
		['r', s @ ..] => super::t0::n12(s, out),
		['s', s @ ..] => super::t0::n13(s, out),
		['t', s @ ..] => super::t0::n14(s, out),
		['u', s @ ..] => super::t0::n15(s, out),
		['x', s @ ..] => super::t0::n16(s, out),
		['y', 'l', 'o', 'm', '\u{1}', s @ ..] => super::t0::n17(s, out),
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 128);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 19);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 130);
					match s {
						[] => true,
						['z', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 131);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 132);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 133);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 607);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 608);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1636);
									match s {
										[] => true,
										['u', 'p', 's', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 609);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 610);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 611);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1637);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 1638);
									s.is_empty()
								}
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1639);
									s.is_empty()
								}
								['i', 'p', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1640);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1641);
									s.is_empty()
								}
								['y', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1642);
									s.is_empty()
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
								['a', s @ ..] => {
									super::push(out, 1643);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 2690);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1644);
									match s {
										[] => true,
										['e', 'l', 'o', '\u{1}', s @ ..] => {
											super::push(out, 2691);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 1645);
									match s {
										[] => true,
										['y', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2692);
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
					super::push(out, 134);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 135);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 136);
					match s {
						[] => true,
						['u', 'n', 'i', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 20);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 138);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 139);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 140);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 615);
							match s {
								[] => true,
								['i', 'r', 't', 'a', s @ ..] => {
									super::push(out, 1646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 616);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 617);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1647);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 141);
					match s {
						[] => true,
						['u', 'c', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 142);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 143);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 21);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 145);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 147);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 621);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 1648);
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
						['o', s @ ..] => {
							super::push(out, 622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 149);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 150);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 623);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 151);
					match s {
						[] => true,
						['y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 22);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 23);
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					super::push(out, 153);
					s.is_empty()
				}
				['e', 'l', s @ ..] => {
					super::push(out, 154);
					s.is_empty()
				}
				['i', 'r', 't', s @ ..] => {
					super::push(out, 155);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 156);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 157);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 158);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 159);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 625);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 626);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 160);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 628);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 629);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 630);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1650);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2693);
											match s {
												[] => true,
												['r', 't', 'a', 'p', '\u{1}', s @ ..] => {
													super::push(out, 3191);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'o', 'm', 'o', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 2694);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 161);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n25(s, out),
		['l', s @ ..] => super::t0::n26(s, out),
		['m', s @ ..] => super::t0::n27(s, out),
		['n', s @ ..] => super::t0::n28(s, out),
		['o', s @ ..] => {
			super::push(out, 29);
			s.is_empty()
		}
		['p', s @ ..] => super::t0::n30(s, out),
		['r', s @ ..] => super::t0::n31(s, out),
		['s', s @ ..] => super::t0::n32(s, out),
		['t', s @ ..] => super::t0::n33(s, out),
		['u', s @ ..] => super::t0::n34(s, out),
		['v', s @ ..] => super::t0::n35(s, out),
		['x', s @ ..] => super::t0::n36(s, out),
		['y', s @ ..] => super::t0::n37(s, out),
		['z', s @ ..] => super::t0::n38(s, out),
		_ => false,
	}
}

/// Words ending -que.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['c', 'm', 'u', 'l', 'u', 't', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'u', 'r', 'e', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 876);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['t', 'e', 'n', 'a', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 58);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['o', 'g', 'a', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 60);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['d', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['e', 'm', 'o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 286);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 287);
					match s {
						[] => true,
						['u', 'n', 'n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 63);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['a', 'r', 'u', 'i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 292);
					match s {
						[] => true,
						['o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 293);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['h', 'o', 'c', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 298);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 880);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 881);
							match s {
								[] => true,
								['r', 'u', 'i', 's', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 882);
							match s {
								[] => true,
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 301);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 303);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 883);
							match s {
								[] => true,
								['r', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 884);
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 2773);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 2774);
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
								['i', s @ ..] => {
									super::push(out, 1853);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1854);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'i', 'r', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 886);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 887);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1857);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1858);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 2775);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 2776);
											match s {
												[] => true,
												['s', 'm', 'u', 'c', '\u{1}', s @ ..] => {
													super::push(out, 3222);
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
							super::push(out, 888);
							match s {
								[] => true,
								['o', 'c', 'o', s @ ..] => {
									super::push(out, 1859);
									s.is_empty()
								}
								['y', 'd', s @ ..] => {
									super::push(out, 1860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 889);
							match s {
								[] => true,
								['e', 'h', s @ ..] => {
									super::push(out, 1861);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1862);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2777);
											match s {
												[] => true,
												['i', 'b', 'i', 't', '\u{1}', s @ ..] => super::t0::n3280(s, out),
												_ => false,
											}
										}
										['t', 'c', s @ ..] => super::t0::n2819(s, out),
										_ => false,
									}
								}
								['o', 'm', s @ ..] => super::t0::n1890(s, out),
								['r', s @ ..] => super::t0::n1891(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n905(s, out),
						['t', s @ ..] => super::t0::n906(s, out),
						['u', s @ ..] => super::t0::n907(s, out),
						['v', s @ ..] => super::t0::n908(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n307(s, out),
				['m', s @ ..] => super::t0::n308(s, out),
				['n', s @ ..] => super::t0::n309(s, out),
				['p', s @ ..] => super::t0::n310(s, out),
				['r', s @ ..] => super::t0::n311(s, out),
				['s', s @ ..] => super::t0::n312(s, out),
				['t', s @ ..] => super::t0::n313(s, out),
				['u', s @ ..] => super::t0::n314(s, out),
				['v', s @ ..] => super::t0::n315(s, out),
				['x', s @ ..] => super::t0::n316(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
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
				['c', s @ ..] => {
					super::push(out, 314);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						['r', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 't', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				['h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 923);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 925);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1892);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 2791);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3228);
													s.is_empty()
												}
												['o', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 3229);
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
						['y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['a', 'c', 'a', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 927);
							s.is_empty()
						}
						['c', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'h', 'p', 'l', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 76);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['b', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 323);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 929);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1893);
									s.is_empty()
								}
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1894);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 1895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 324);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 932);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 933);
							match s {
								[] => true,
								['g', 'a', 'm', 'r', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1897);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
						}
						['n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 935);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 936);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1898);
									s.is_empty()
								}
								['n', 'a', 'r', 'e', 't', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 937);
							match s {
								[] => true,
								['h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'a', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 939);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 940);
							match s {
								[] => true,
								['r', 't', 's', 'e', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 327);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 941);
							match s {
								[] => true,
								['e', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1902);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1903);
									match s {
										[] => true,
										['t', s @ ..] => super::t0::n2833(s, out),
										_ => false,
									}
								}
								['r', 'a', '\u{1}', s @ ..] => super::t0::n1931(s, out),
								_ => false,
							}
						}
						['p', s @ ..] => super::t0::n957(s, out),
						['t', s @ ..] => super::t0::n958(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n331(s, out),
				['l', s @ ..] => super::t0::n332(s, out),
				['m', s @ ..] => super::t0::n333(s, out),
				['n', s @ ..] => super::t0::n334(s, out),
				['p', s @ ..] => super::t0::n335(s, out),
				['r', s @ ..] => super::t0::n336(s, out),
				['s', s @ ..] => super::t0::n337(s, out),
				['t', s @ ..] => super::t0::n338(s, out),
				['x', s @ ..] => super::t0::n339(s, out),
				['y', s @ ..] => super::t0::n340(s, out),
				['z', s @ ..] => super::t0::n341(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 339);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 344);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 345);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 980);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 981);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 982);
							s.is_empty()
						}
						['g', 'i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 983);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 984);
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 985);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 986);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 1932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 987);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 988);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 989);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				['l', 'e', 'i', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 990);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1934);
									s.is_empty()
								}
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 349);
					match s {
						[] => true,
						['e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 992);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 993);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 995);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 996);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 997);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1936);
									match s {
										[] => true,
										['u', 's', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2798);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1937);
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
			super::push(out, 81);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 351);
					match s {
						[] => true,
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 998);
							s.is_empty()
						}
						['l', 'h', 'p', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 999);
							s.is_empty()
						}
						['p', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1000);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 352);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1003);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1939);
									match s {
										[] => true,
										['e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 2799);
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
				['n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 353);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 355);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1007);
							s.is_empty()
						}
						['r', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 357);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 358);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 359);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => super::t0::n364(s, out),
				['h', s @ ..] => super::t0::n365(s, out),
				['l', s @ ..] => super::t0::n366(s, out),
				['n', s @ ..] => super::t0::n367(s, out),
				['p', s @ ..] => super::t0::n368(s, out),
				['r', s @ ..] => super::t0::n369(s, out),
				['s', s @ ..] => super::t0::n370(s, out),
				['t', s @ ..] => super::t0::n371(s, out),
				['x', s @ ..] => super::t0::n372(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n85(s, out),
		['m', s @ ..] => super::t0::n86(s, out),
		['n', s @ ..] => super::t0::n87(s, out),
		['p', s @ ..] => super::t0::n88(s, out),
		['r', s @ ..] => super::t0::n89(s, out),
		['s', s @ ..] => super::t0::n90(s, out),
		['t', s @ ..] => super::t0::n91(s, out),
		['u', s @ ..] => super::t0::n92(s, out),
		['v', s @ ..] => super::t0::n93(s, out),
		['x', s @ ..] => super::t0::n94(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['b', 'u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 391);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1045);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['h', 'c', 'c', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1047);
							s.is_empty()
						}
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 1049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1050);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1051);
							s.is_empty()
						}
						['l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1052);
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1053);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 399);
					match s {
						[] => true,
						['e', 'l', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 402);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1057);
							s.is_empty()
						}
						['b', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1058);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1059);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1060);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1958);
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1959);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1960);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1961);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2806);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 1962);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 1963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1061);
							match s {
								[] => true,
								['e', 't', 'p', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1964);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 1965);
									s.is_empty()
								}
								['y', '\u{1}', s @ ..] => {
									super::push(out, 1966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1062);
							match s {
								[] => true,
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1063);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1064);
							match s {
								[] => true,
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1065);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						['n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1067);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1068);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1969);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => super::t0::n408(s, out),
				['g', s @ ..] => super::t0::n409(s, out),
				['h', 't', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 406);
					s.is_empty()
				}
				['i', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 407);
					s.is_empty()
				}
				['l', 'i', 's', '\u{1}', s @ ..] => super::t0::n412(s, out),
				['m', 'o', 'v', '\u{1}', s @ ..] => super::t0::n413(s, out),
				['n', s @ ..] => super::t0::n414(s, out),
				['p', s @ ..] => super::t0::n415(s, out),
				['s', s @ ..] => super::t0::n416(s, out),
				['t', s @ ..] => super::t0::n417(s, out),
				['u', 'p', '\u{1}', s @ ..] => super::t0::n418(s, out),
				['v', s @ ..] => super::t0::n419(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n97(s, out),
		['o', s @ ..] => super::t0::n98(s, out),
		['u', s @ ..] => super::t0::n99(s, out),
		['y', s @ ..] => super::t0::n100(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 444);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 445);
					match s {
						[] => true,
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1132);
							s.is_empty()
						}
						['o', 'h', 'p', 'o', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1133);
							s.is_empty()
						}
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						['y', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 446);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1136);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 447);
					match s {
						[] => true,
						['n', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1138);
							s.is_empty()
						}
						['o', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1139);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1140);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2011);
									s.is_empty()
								}
								['c', 'n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2012);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2820);
											s.is_empty()
										}
										['b', '\u{1}', s @ ..] => {
											super::push(out, 2821);
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
					super::push(out, 448);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1141);
							s.is_empty()
						}
						['a', 'h', 'p', 'a', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1142);
							s.is_empty()
						}
						['e', 'l', 'p', 'm', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 1143);
							s.is_empty()
						}
						['i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 450);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1145);
							s.is_empty()
						}
						['p', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 451);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1147);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1148);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1149);
							match s {
								[] => true,
								['c', 's', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2014);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1150);
							match s {
								[] => true,
								['i', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1151);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2016);
									s.is_empty()
								}
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1152);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1153);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2018);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 2019);
									s.is_empty()
								}
								['y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1154);
							match s {
								[] => true,
								['y', 'n', 'o', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1155);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2022);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2822);
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
					super::push(out, 452);
					match s {
						[] => true,
						['c', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1156);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1157);
							match s {
								[] => true,
								['m', 'i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2023);
									s.is_empty()
								}
								['p', 'i', 's', 'y', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2025);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1160);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1161);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2028);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 2823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 't', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n458(s, out),
				['o', 'b', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['e', 'l', 'b', 'o', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1167);
							s.is_empty()
						}
						['m', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n461(s, out),
				['s', 'p', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				['t', s @ ..] => super::t0::n463(s, out),
				['v', s @ ..] => super::t0::n464(s, out),
				['x', 'e', 'h', '\u{1}', s @ ..] => super::t0::n465(s, out),
				['y', s @ ..] => super::t0::n466(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t0::n102(s, out),
		['e', s @ ..] => super::t0::n103(s, out),
		['i', s @ ..] => super::t0::n104(s, out),
		['l', 'u', 'p', '\u{1}', s @ ..] => super::t0::n105(s, out),
		['m', s @ ..] => super::t0::n106(s, out),
		['n', s @ ..] => super::t0::n107(s, out),
		['o', s @ ..] => super::t0::n108(s, out),
		['p', s @ ..] => super::t0::n109(s, out),
		['r', s @ ..] => super::t0::n110(s, out),
		['u', s @ ..] => super::t0::n111(s, out),
		['y', s @ ..] => super::t0::n112(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['c', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['c', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 115);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 116);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 562);
					match s {
						[] => true,
						['n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1581);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1582);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 564);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1584);
							s.is_empty()
						}
						['r', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 1585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1586);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1588);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						['t', 'r', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1592);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1593);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1594);
							match s {
								[] => true,
								['u', 'q', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1595);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2664);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 3186);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3343);
													s.is_empty()
												}
												['u', 's', '\u{1}', s @ ..] => {
													super::push(out, 3344);
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
							super::push(out, 1596);
							match s {
								[] => true,
								['i', 'r', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2665);
									s.is_empty()
								}
								['u', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 2666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1599);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1602);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1603);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 't', 'x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2669);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						['e', 'c', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'n', 'e', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1611);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2671);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1612);
							match s {
								[] => true,
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2672);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1613);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1614);
							match s {
								[] => true,
								['p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				['n', s @ ..] => super::t0::n586(s, out),
				['r', s @ ..] => super::t0::n587(s, out),
				['t', s @ ..] => super::t0::n588(s, out),
				['v', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['r', 'a', '\u{1}', s @ ..] => super::t0::n1646(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n122(s, out),
		['n', s @ ..] => super::t0::n123(s, out),
		['o', s @ ..] => super::t0::n124(s, out),
		['r', s @ ..] => super::t0::n125(s, out),
		['u', s @ ..] => super::t0::n126(s, out),
		['y', s @ ..] => super::t0::n127(s, out),
		_ => false,
	}
}

/// The word moly.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	s.is_empty()
}

/// Words ending -ia.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1651);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 2695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 636);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1652);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2696);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3192);
											s.is_empty()
										}
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3193);
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
					super::push(out, 637);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1653);
							s.is_empty()
						}
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 639);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1656);
							match s {
								[] => true,
								['i', 'd', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2698);
									s.is_empty()
								}
								['m', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2699);
									s.is_empty()
								}
								['p', 'e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2700);
									s.is_empty()
								}
								['s', 's', 'a', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							super::push(out, 1658);
							s.is_empty()
						}
						['o', 'c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2702);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3194);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 3345);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 2703);
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
			super::push(out, 167);
			match s {
				[] => true,
				['o', 'p', 'o', 't', 'a', 'm', 'o', 'n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 643);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2704);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2705);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3195);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 3196);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3197);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 3198);
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
					super::push(out, 647);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['p', 'i', 'l', 'l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1662);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1665);
							match s {
								[] => true,
								['o', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1669);
							match s {
								[] => true,
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2707);
									s.is_empty()
								}
								['r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'u', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n658(s, out),
				['b', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n661(s, out),
				['o', s @ ..] => {
					super::push(out, 654);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t0::n171(s, out),
		['n', s @ ..] => super::t0::n172(s, out),
		['p', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n174(s, out),
		['s', s @ ..] => super::t0::n175(s, out),
		['t', s @ ..] => super::t0::n176(s, out),
		['u', s @ ..] => super::t0::n177(s, out),
		['v', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['i', s @ ..] => super::t0::n694(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 686);
					match s {
						[] => true,
						['o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['a', 't', '\u{1}', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 181);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 182);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					super::push(out, 688);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1738);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2743);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1740);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2745);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 3215);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1744);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2746);
									match s {
										[] => true,
										['e', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1745);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1746);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2748);
									match s {
										[] => true,
										['r', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 3217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 698);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
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

/// Words ending -ma.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1748);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3218);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 3219);
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
					super::push(out, 703);
					s.is_empty()
				}
				['i', 'm', 'y', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 704);
					s.is_empty()
				}
				['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 708);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1753);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1754);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['c', 'a', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['c', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1758);
							s.is_empty()
						}
						['p', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1760);
							s.is_empty()
						}
						['s', 'p', 'a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1762);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1765);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				['t', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				['y', 's', '\u{1}', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 1768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 738);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['c', 'n', 'e', 'r', 'h', 'p', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 1769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1770);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 743);
					match s {
						[] => true,
						['c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						['h', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'a', '\u{1}', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['g', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 746);
					match s {
						[] => true,
						['t', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1775);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 751);
					match s {
						[] => true,
						['o', 'c', 's', 'o', 'n', 'g', 'i', 'g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 755);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 759);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1779);
							s.is_empty()
						}
						['e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1780);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1781);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 1782);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 762);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1784);
							match s {
								[] => true,
								['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2754);
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
			super::push(out, 202);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 765);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['h', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1787);
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 2755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1788);
							s.is_empty()
						}
						['f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1789);
							s.is_empty()
						}
						['p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 1790);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1791);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 770);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		['u', 'r', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['a', 'r', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['i', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['i', 'n', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1798);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2756);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2757);
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
			super::push(out, 216);
			match s {
				[] => true,
				['t', 'a', 'b', 'o', 'l', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['i', 'r', 'e', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				['v', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1800);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2758);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2759);
									match s {
										[] => true,
										['t', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1803);
							match s {
								[] => true,
								['l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2760);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['s', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1806);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 2761);
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
			super::push(out, 221);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['n', 'i', 'g', 'i', 'l', 'u', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['r', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				['d', 'n', 'a', 'm', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['u', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['o', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2762);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 810);
					match s {
						[] => true,
						['o', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 811);
					match s {
						[] => true,
						['c', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 817);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['o', 'h', s @ ..] => {
							super::push(out, 1819);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2764);
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
			super::push(out, 230);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				['l', 'e', 'a', 'r', 's', 'i', '\u{1}', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				['m', 'e', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['n', 'e', 'g', 'o', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 826);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 827);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 830);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1826);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2765);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 2766);
									s.is_empty()
								}
								['l', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2767);
									s.is_empty()
								}
								['r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2768);
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
			super::push(out, 232);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 832);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'e', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 835);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 836);
					match s {
						[] => true,
						['r', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 837);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 839);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 841);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1834);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2769);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3221);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						['r', 'u', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 842);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 236);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 844);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 845);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 239);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 848);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1843);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2770);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['l', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['b', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 851);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1845);
							match s {
								[] => true,
								['e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2771);
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
			super::push(out, 244);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xa.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['f', 'e', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ya.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	s.is_empty()
}

/// Words ending -za.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	s.is_empty()
}

/// The word tibicinium.
pub(crate) fn n3280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3223);
	s.is_empty()
}

/// Words ending -ctinium.
pub(crate) fn n2819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2778);
	s.is_empty()
}

/// Words ending -monium.
pub(crate) fn n1890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1863);
	s.is_empty()
}

/// Words ending -rnium.
pub(crate) fn n1891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1864);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rium.
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 890);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', s @ ..] => {
			super::push(out, 1866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tium.
pub(crate) fn n906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 891);
	s.is_empty()
}

/// Words ending -uium.
pub(crate) fn n907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 892);
	s.is_empty()
}

/// Words ending -vium.
pub(crate) fn n908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lum.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 894);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2781);
							s.is_empty()
						}
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1870);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2783);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3224);
									s.is_empty()
								}
								_ => false,
							}
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
		['o', s @ ..] => {
			super::push(out, 897);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1872);
					match s {
						[] => true,
						['n', 'l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mum.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -num.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 900);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1879);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1880);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2785);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3225);
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
			super::push(out, 903);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pum.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	s.is_empty()
}

/// Words ending -rum.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 905);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 906);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 907);
			match s {
				[] => true,
				['e', 'a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1883);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 2786);
							match s {
								[] => true,
								['o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3226);
									s.is_empty()
								}
								['y', 'x', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3227);
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
			super::push(out, 908);
			match s {
				[] => true,
				['g', 'u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 909);
			s.is_empty()
		}
		['p', 'u', s @ ..] => {
			super::push(out, 910);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1886);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sum.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	s.is_empty()
}

/// Words ending -tum.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1888);
					match s {
						[] => true,
						['u', 't', s @ ..] => {
							super::push(out, 2788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 915);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1889);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2789);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 917);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uum.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	s.is_empty()
}

/// Words ending -vum.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['i', 'd', s @ ..] => {
			super::push(out, 920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xum.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	s.is_empty()
}

/// Words ending -tichon.
pub(crate) fn n2833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2792);
	s.is_empty()
}

/// The word archon.
pub(crate) fn n1931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1904);
	s.is_empty()
}

/// Words ending -phon.
pub(crate) fn n957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 942);
	s.is_empty()
}

/// Words ending -thon.
pub(crate) fn n958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 943);
	match s {
		[] => true,
		['u', 'l', 'o', 'c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1905);
			s.is_empty()
		}
		['y', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ion.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 328);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1907);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3230);
									s.is_empty()
								}
								['r', 't', 'o', 'o', 'z', '\u{1}', s @ ..] => {
									super::push(out, 3231);
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
			super::push(out, 946);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 947);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 948);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1908);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1909);
					match s {
						[] => true,
						['l', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2795);
							match s {
								[] => true,
								['r', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3232);
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
		['z', 'a', 'p', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lon.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 329);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['y', 'h', 'p', 'o', 'n', 'i', 'l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mon.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['a', 'g', 'n', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 951);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		['g', 'e', 'l', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 953);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 954);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'h', 't', 'h', 'p', 'o', 'r', 'u', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				['z', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		['s', 'i', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 957);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -non.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['h', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['p', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pon.
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	s.is_empty()
}

/// Words ending -ron.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -son.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ton.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 968);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['e', 't', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 970);
			match s {
				[] => true,
				['g', 'o', 'm', 'a', 't', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1921);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1923);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 2796);
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

/// Words ending -xon.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yon.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zon.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1928);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['r', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['y', 'n', 'g', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['n', 'g', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gio.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 1011);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hio.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	s.is_empty()
}

/// Words ending -lio.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1013);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2800);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nio.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1016);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pio.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1019);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1020);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			super::push(out, 1022);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rio.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 1023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sio.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1024);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1943);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1945);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2802);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tio.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1950);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2803);
							match s {
								[] => true,
								['r', 'p', 'm', s @ ..] => {
									super::push(out, 3234);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3350);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['c', 'r', 'e', 'o', 'c', '\u{1}', s @ ..] => {
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
		['p', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1955);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2805);
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

/// Words ending -xio.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	s.is_empty()
}

/// Words ending -lo.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['n', 'n', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 372);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1038);
					s.is_empty()
				}
				['r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 374);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1042);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	s.is_empty()
}

/// Words ending -vo.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xo.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fer.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1072);
			s.is_empty()
		}
		['e', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		['g', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1074);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1075);
			s.is_empty()
		}
		['n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1076);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 1971);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word siler.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	s.is_empty()
}

/// The word vomer.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	s.is_empty()
}

/// Words ending -ner.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1080);
			s.is_empty()
		}
		['s', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1082);
			s.is_empty()
		}
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1083);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1084);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1085);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['e', 'h', 't', 'i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1973);
					s.is_empty()
				}
				['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1974);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1975);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1976);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1977);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1978);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1087);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1088);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1981);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1983);
					s.is_empty()
				}
				['p', 'i', 'c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1985);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['a', 'h', 'c', 'o', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1986);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1987);
					match s {
						[] => true,
						['v', 'i', s @ ..] => {
							super::push(out, 2811);
							match s {
								[] => true,
								['b', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3235);
									s.is_empty()
								}
								['r', 'g', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 3236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1092);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2812);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3237);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3351);
											s.is_empty()
										}
										['f', 'u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 3352);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3238);
									match s {
										[] => true,
										['c', 'u', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 3353);
											s.is_empty()
										}
										['h', 't', 'n', 'a', 'x', '\u{1}', s @ ..] => {
											super::push(out, 3354);
											s.is_empty()
										}
										['n', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3355);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3356);
											match s {
												[] => true,
												['h', 't', 'y', 'r', 'e', '\u{1}', s @ ..] => {
													super::push(out, 3379);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3357);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3239);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2814);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1990);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1991);
					s.is_empty()
				}
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1993);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'b', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1095);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word puer.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	s.is_empty()
}

/// Words ending -ver.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1995);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ir.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['d', 'd', 'a', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		['m', 'e', '\u{1}', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		['x', 'i', 'l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 419);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				['e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['o', 'c', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
				['r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['t', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1998);
							s.is_empty()
						}
						['v', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2001);
							s.is_empty()
						}
						['t', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 428);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2005);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2816);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3240);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2006);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2007);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2818);
									match s {
										[] => true,
										['l', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3241);
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
					super::push(out, 1115);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2009);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2819);
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
			super::push(out, 431);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['e', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				['o', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['l', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1124);
					s.is_empty()
				}
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'l', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				['t', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yr.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['m', 'r', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 441);
			s.is_empty()
		}
		['s', 's', 'a', '\u{1}', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1131);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nas.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1164);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'y', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1165);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ras.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['e', 'o', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1169);
			s.is_empty()
		}
		['o', 'g', 'a', 'r', 'd', 'n', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1170);
			s.is_empty()
		}
		['r', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1171);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tas.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1173);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2035);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2825);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 3242);
									match s {
										[] => true,
										['a', 't', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3358);
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
				['r', 'u', s @ ..] => {
					super::push(out, 2036);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2037);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3243);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 3359);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3244);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3360);
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
					super::push(out, 2038);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vas.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1174);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word hexas.
pub(crate) fn n465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	s.is_empty()
}

/// Words ending -yas.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1175);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2039);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bs.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		['r', 'u', '\u{1}', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -es.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				['p', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1182);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2041);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1184);
					match s {
						[] => true,
						['n', 'i', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2044);
							match s {
								[] => true,
								['o', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2827);
									s.is_empty()
								}
								['p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'm', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['n', 'o', 'r', 't', 'a', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['i', 'p', 'e', 'l', 'c', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2049);
							match s {
								[] => true,
								['g', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['c', 'a', 's', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						['e', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2055);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2830);
									match s {
										[] => true,
										['m', 'u', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2056);
							match s {
								[] => true,
								['m', 'a', 's', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2832);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2833);
									match s {
										[] => true,
										['o', 'g', 'y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3246);
											s.is_empty()
										}
										['y', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3247);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2834);
									match s {
										[] => true,
										['d', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3248);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'b', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						['t', 'o', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['f', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1197);
					match s {
						[] => true,
						['b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2060);
							s.is_empty()
						}
						['l', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['d', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						['t', 's', 'a', 'm', 'o', 'r', 'e', 'm', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'r', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2066);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1204);
					match s {
						[] => true,
						['e', 'o', 'c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2069);
							s.is_empty()
						}
						['y', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2071);
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
				['c', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2072);
							s.is_empty()
						}
						['e', s @ ..] => super::t0::n2101(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t0::n1224(s, out),
				['n', s @ ..] => super::t0::n1225(s, out),
				['r', s @ ..] => super::t0::n1226(s, out),
				['t', s @ ..] => super::t0::n1227(s, out),
				['u', s @ ..] => super::t0::n1228(s, out),
				['v', s @ ..] => super::t0::n1229(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n478(s, out),
		['m', s @ ..] => super::t0::n479(s, out),
		['n', s @ ..] => super::t0::n480(s, out),
		['p', s @ ..] => super::t0::n481(s, out),
		['r', s @ ..] => super::t0::n482(s, out),
		['s', s @ ..] => super::t0::n483(s, out),
		['t', s @ ..] => super::t0::n484(s, out),
		['u', s @ ..] => super::t0::n485(s, out),
		['v', 'i', 'd', '\u{1}', s @ ..] => super::t0::n486(s, out),
		['x', 'r', 'e', 'x', '\u{1}', s @ ..] => super::t0::n487(s, out),
		['y', 'h', 'p', 'i', 'd', '\u{1}', s @ ..] => super::t0::n488(s, out),
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 485);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 486);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2139);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2877);
									s.is_empty()
								}
								['n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2140);
							s.is_empty()
						}
						['s', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1266);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2143);
							s.is_empty()
						}
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2144);
							s.is_empty()
						}
						['o', 'b', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1269);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2147);
							s.is_empty()
						}
						['i', 'b', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['s', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1273);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2149);
							s.is_empty()
						}
						['t', 's', 'o', 'c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1275);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', 'r', 'o', 'd', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2879);
									s.is_empty()
								}
								['r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2152);
							match s {
								[] => true,
								['a', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2881);
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
			super::push(out, 492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1279);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2153);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2882);
									match s {
										[] => true,
										['v', s @ ..] => {
											super::push(out, 3251);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2154);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2883);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3252);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2155);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2156);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2885);
									s.is_empty()
								}
								['i', 'd', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2886);
									s.is_empty()
								}
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3253);
											s.is_empty()
										}
										['b', '\u{1}', s @ ..] => {
											super::push(out, 3254);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'y', '\u{1}', s @ ..] => {
									super::push(out, 2887);
									s.is_empty()
								}
								['r', 'e', 'b', 'u', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2157);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2889);
									s.is_empty()
								}
								['y', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2158);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2159);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 2891);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 3255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2160);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2161);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['c', 's', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2162);
							s.is_empty()
						}
						['h', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2163);
							s.is_empty()
						}
						['u', s @ ..] => super::t0::n2195(s, out),
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1281);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n1299(s, out),
				['l', s @ ..] => super::t0::n1300(s, out),
				['o', s @ ..] => super::t0::n1301(s, out),
				['t', 's', '\u{1}', s @ ..] => super::t0::n1302(s, out),
				['u', s @ ..] => super::t0::n1303(s, out),
				['y', 'g', 'n', 'o', 'g', '\u{1}', s @ ..] => super::t0::n1304(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t0::n497(s, out),
		['n', s @ ..] => super::t0::n498(s, out),
		['o', s @ ..] => super::t0::n499(s, out),
		['p', s @ ..] => super::t0::n500(s, out),
		['r', s @ ..] => super::t0::n501(s, out),
		['s', s @ ..] => super::t0::n502(s, out),
		['t', s @ ..] => super::t0::n503(s, out),
		['u', s @ ..] => super::t0::n504(s, out),
		['v', s @ ..] => super::t0::n505(s, out),
		['x', s @ ..] => super::t0::n506(s, out),
		_ => false,
	}
}

/// The word puls.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	s.is_empty()
}

/// Words ending -ms.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ns.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['g', 'i', 't', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1357);
					s.is_empty()
				}
				['h', 'p', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1358);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1359);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2307);
							match s {
								[] => true,
								['u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1360);
					match s {
						[] => true,
						['i', 'm', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1361);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2309);
							match s {
								[] => true,
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1362);
					match s {
						[] => true,
						['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2310);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1363);
					s.is_empty()
				}
				['b', 'm', 'u', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1364);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1365);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2312);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2962);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 3288);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 3367);
													s.is_empty()
												}
												['o', '\u{1}', s @ ..] => {
													super::push(out, 3368);
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
							super::push(out, 2313);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1366);
					match s {
						[] => true,
						['r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1367);
					match s {
						[] => true,
						['c', 'i', 'f', 'f', 'e', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2317);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2318);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 2963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1369);
					match s {
						[] => true,
						['o', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -os.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		['a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1375);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1377);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1378);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1380);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1381);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['t', 'o', 'h', 'p', 'l', 'e', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['l', 'e', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1387);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1390);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1391);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1392);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1393);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1394);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1397);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1398);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1399);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 1400);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1402);
					s.is_empty()
				}
				['e', 'm', s @ ..] => {
					super::push(out, 1403);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1404);
					s.is_empty()
				}
				['i', 'l', s @ ..] => {
					super::push(out, 1405);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1406);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2329);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2965);
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
			super::push(out, 520);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2330);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2331);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1408);
					s.is_empty()
				}
				['r', 'a', 'c', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1410);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2333);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2966);
									match s {
										[] => true,
										['n', 'i', 'h', 'r', '\u{1}', s @ ..] => {
											super::push(out, 3289);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2335);
							s.is_empty()
						}
						['p', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2336);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 2337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2340);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2341);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2342);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['a', 'r', 'e', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2344);
							s.is_empty()
						}
						['e', 'b', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2345);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2346);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2349);
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

/// Words ending -ps.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 526);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1428);
					match s {
						[] => true,
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2350);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2351);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2352);
							s.is_empty()
						}
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'e', 'i', 'h', '\u{1}', s @ ..] => {
			super::push(out, 529);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['c', 'u', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'i', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['y', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rs.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 535);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2357);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 2971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2358);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2359);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2360);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2362);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2363);
							match s {
								[] => true,
								['i', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2364);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2365);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2973);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2366);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2975);
									match s {
										[] => true,
										['a', 'i', '\u{1}', s @ ..] => {
											super::push(out, 3290);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2368);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2369);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2371);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2372);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2374);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2977);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2978);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3291);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 2979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 2376);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2379);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1447);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2381);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2981);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2982);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2382);
							match s {
								[] => true,
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 2983);
									s.is_empty()
								}
								['u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 2984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2383);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2385);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2386);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2387);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2986);
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
			super::push(out, 538);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2391);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1453);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1454);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2394);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2395);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1457);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n543(s, out),
		['g', s @ ..] => super::t0::n544(s, out),
		['h', s @ ..] => super::t0::n545(s, out),
		['i', s @ ..] => super::t0::n546(s, out),
		['l', s @ ..] => super::t0::n547(s, out),
		['m', s @ ..] => super::t0::n548(s, out),
		['n', s @ ..] => super::t0::n549(s, out),
		['o', s @ ..] => super::t0::n550(s, out),
		['p', s @ ..] => super::t0::n551(s, out),
		['r', s @ ..] => super::t0::n552(s, out),
		['s', s @ ..] => super::t0::n553(s, out),
		['t', s @ ..] => super::t0::n554(s, out),
		['u', s @ ..] => super::t0::n555(s, out),
		['v', s @ ..] => super::t0::n556(s, out),
		['x', s @ ..] => super::t0::n557(s, out),
		_ => false,
	}
}

/// Words ending -ys.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['h', 'c', 'a', 'r', 'b', 'i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 554);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				['e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['l', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'i', 'p', 'e', 'a', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nix.
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 582);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1615);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['o', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1617);
			match s {
				[] => true,
				['d', 'r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2676);
					s.is_empty()
				}
				['t', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2678);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rix.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 583);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1620);
			s.is_empty()
		}
		['h', 't', 'i', 'l', 'l', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1621);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1622);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2680);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3188);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3189);
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

/// Words ending -tix.
pub(crate) fn n588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 584);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1623);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word arvix.
pub(crate) fn n1646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1624);
	s.is_empty()
}

/// Words ending -lx.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 586);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nx.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2683);
							match s {
								[] => true,
								['h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3190);
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
			super::push(out, 588);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 590);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ox.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 591);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 592);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 593);
			s.is_empty()
		}
		['r', 'b', 'o', 'l', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rx.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	s.is_empty()
}

/// Words ending -ux.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 596);
			s.is_empty()
		}
		['i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 597);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 598);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1629);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yx.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['c', 'c', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 600);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2685);
							s.is_empty()
						}
						['h', 'p', 'm', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['d', 'r', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2687);
							s.is_empty()
						}
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['b', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'r', 'o', '\u{1}', s @ ..] => {
			super::push(out, 604);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -alia.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1671);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1672);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1673);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'o', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1674);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1675);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1676);
			s.is_empty()
		}
		['r', 'r', s @ ..] => {
			super::push(out, 1677);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2712);
					s.is_empty()
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1678);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1679);
			s.is_empty()
		}
		['u', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1680);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ilia.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['b', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1681);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1682);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1683);
			s.is_empty()
		}
		['n', 'e', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1684);
			s.is_empty()
		}
		['r', 'u', 'a', 't', 'e', 'v', 'o', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1685);
			s.is_empty()
		}
		['s', 'n', 'e', 't', 'u', '\u{1}', s @ ..] => {
			super::push(out, 1686);
			s.is_empty()
		}
		['t', 'c', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1687);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mia.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 656);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 657);
			match s {
				[] => true,
				['d', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 659);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nia.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1689);
					match s {
						[] => true,
						['c', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['c', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['h', 'p', 'o', s @ ..] => {
					super::push(out, 1694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 667);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1695);
					match s {
						[] => true,
						['r', 'e', 'c', 'o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1696);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ria.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2716);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				['e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1701);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1702);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2719);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 3199);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 3200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2720);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3201);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 3346);
											s.is_empty()
										}
										['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3347);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'g', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'p', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1703);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['s', 'e', 'c', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1706);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2723);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3203);
									s.is_empty()
								}
								['l', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3204);
									s.is_empty()
								}
								['r', 'c', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 3205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2725);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2726);
							match s {
								[] => true,
								['a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3206);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3207);
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
			super::push(out, 671);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['u', 'q', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sia.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['h', 't', 's', 'e', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1721);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2731);
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

/// Words ending -tia.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1723);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2732);
							match s {
								[] => true,
								['h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3208);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 3209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1724);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1727);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2734);
							match s {
								[] => true,
								['a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2735);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2736);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3211);
									match s {
										[] => true,
										['u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3348);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2737);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1728);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2739);
							match s {
								[] => true,
								['i', 'v', 'e', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1729);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2740);
							match s {
								[] => true,
								['e', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1730);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2741);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 3349);
											s.is_empty()
										}
										['h', 'c', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3377);
													s.is_empty()
												}
												['u', 'e', '\u{1}', s @ ..] => {
													super::push(out, 3378);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['c', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 3214);
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
			super::push(out, 684);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uia.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2742);
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

/// Words ending -ivia.
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 685);
	s.is_empty()
}

/// Words ending -ecies.
pub(crate) fn n2101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2073);
	match s {
		[] => true,
		['p', 's', '\u{1}', s @ ..] => {
			super::push(out, 2837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dies.
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1207);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2074);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2075);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nies.
pub(crate) fn n1225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1208);
	s.is_empty()
}

/// Words ending -ries.
pub(crate) fn n1226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1209);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2076);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2838);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['g', 'i', s @ ..] => {
					super::push(out, 2840);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', s @ ..] => {
					super::push(out, 2841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ties.
pub(crate) fn n1227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1210);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2842);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2843);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2844);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uies.
pub(crate) fn n1228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1211);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 2079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vies.
pub(crate) fn n1229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1212);
	s.is_empty()
}

/// Words ending -les.
pub(crate) fn n478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2080);
					s.is_empty()
				}
				['h', 't', 'i', 's', 'y', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1214);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1215);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2846);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2082);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1216);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1217);
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1218);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1219);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1220);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mes.
pub(crate) fn n479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1222);
			match s {
				[] => true,
				['c', 'e', s @ ..] => {
					super::push(out, 2089);
					match s {
						[] => true,
						['c', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2848);
							s.is_empty()
						}
						['s', 'r', 'e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2849);
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
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2090);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nes.
pub(crate) fn n480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1224);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2091);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2850);
							s.is_empty()
						}
						['o', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2093);
					match s {
						[] => true,
						['t', 'n', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1226);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1227);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pes.
pub(crate) fn n481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1229);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1230);
			match s {
				[] => true,
				['r', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2094);
					s.is_empty()
				}
				['s', 'u', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2096);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1231);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2097);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1232);
			match s {
				[] => true,
				['c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2098);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2099);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2100);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1233);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 2101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1235);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 2102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1236);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1237);
			match s {
				[] => true,
				['i', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 2103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2104);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2105);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1239);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2106);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2858);
							s.is_empty()
						}
						['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 2859);
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

/// Words ending -res.
pub(crate) fn n482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 478);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1240);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1241);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2860);
							s.is_empty()
						}
						['a', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1242);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2111);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2862);
							s.is_empty()
						}
						['o', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1243);
			match s {
				[] => true,
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1244);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1245);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2113);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'm', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1246);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ses.
pub(crate) fn n483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 479);
	match s {
		[] => true,
		['b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 1247);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1248);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1249);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1250);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tes.
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 480);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1251);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2117);
					match s {
						[] => true,
						['c', 'a', 'm', 'æ', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2119);
					match s {
						[] => true,
						['c', 'o', 'e', 'h', 'p', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2866);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						['h', 'p', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1252);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2121);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1253);
			match s {
				[] => true,
				['p', 'o', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1254);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2123);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2124);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2125);
					match s {
						[] => true,
						['e', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2126);
					match s {
						[] => true,
						['g', 'a', 'r', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2127);
					match s {
						[] => true,
						['o', 'h', 'c', 'e', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2128);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 2874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1255);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2130);
					match s {
						[] => true,
						['g', 'i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1256);
			match s {
				[] => true,
				['t', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1257);
			s.is_empty()
		}
		['r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1258);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1259);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2132);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2133);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2134);
					s.is_empty()
				}
				['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1260);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['d', 'o', 'n', 'e', 't', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2136);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ues.
pub(crate) fn n485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 481);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				['r', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word dives.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 482);
	s.is_empty()
}

/// The word xerxes.
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 483);
	s.is_empty()
}

/// The word diphyes.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 484);
	s.is_empty()
}

/// Words ending -uelis.
pub(crate) fn n2195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2164);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2893);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ilis.
pub(crate) fn n1299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1282);
	match s {
		[] => true,
		['c', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 2165);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2166);
			s.is_empty()
		}
		['h', 'p', 'y', 's', '\u{1}', s @ ..] => {
			super::push(out, 2167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -llis.
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1283);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2168);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3257);
							s.is_empty()
						}
						['o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2895);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2169);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'h', 't', 'n', 'a', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2170);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2171);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olis.
pub(crate) fn n1301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1284);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 2173);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2174);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2900);
					match s {
						[] => true,
						['c', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3260);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3261);
							match s {
								[] => true,
								['t', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3361);
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

/// The word stlis.
pub(crate) fn n1302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1285);
	s.is_empty()
}

/// Words ending -ulis.
pub(crate) fn n1303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1286);
	match s {
		[] => true,
		['b', 'i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 2175);
			s.is_empty()
		}
		['i', '\u{1}', s @ ..] => {
			super::push(out, 2176);
			s.is_empty()
		}
		['p', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2177);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gongylis.
pub(crate) fn n1304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1287);
	s.is_empty()
}

/// Words ending -mis.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1288);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1289);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1291);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2181);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2183);
					match s {
						[] => true,
						['d', 'i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 2903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['c', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'd', 'i', 'd', 'i', 'p', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nis.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2186);
					s.is_empty()
				}
				['e', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2188);
					s.is_empty()
				}
				['i', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2189);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['m', 'e', 'a', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2191);
					s.is_empty()
				}
				['v', 'u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1297);
			s.is_empty()
		}
		['h', 'c', 'y', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1298);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1299);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['a', 'r', 'y', 't', '\u{1}', s @ ..] => {
					super::push(out, 2194);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1301);
			match s {
				[] => true,
				['e', 'l', 'u', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1302);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ois.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['r', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pis.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1305);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2198);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2904);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1306);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1307);
			s.is_empty()
		}
		['m', 'a', 'l', 'i', 'm', 'z', '\u{1}', s @ ..] => {
			super::push(out, 1308);
			s.is_empty()
		}
		['p', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1309);
			s.is_empty()
		}
		['r', 'i', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 1310);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ris.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2201);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2906);
							s.is_empty()
						}
						['a', 'n', 'n', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2908);
							s.is_empty()
						}
						['c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2204);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 't', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2206);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2207);
					match s {
						[] => true,
						['e', 't', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 3262);
									s.is_empty()
								}
								['i', 'g', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 3264);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 3265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2913);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3362);
											s.is_empty()
										}
										['i', 'r', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3363);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3364);
											s.is_empty()
										}
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3365);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3266);
									s.is_empty()
								}
								['t', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2208);
					s.is_empty()
				}
				['p', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1313);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1314);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				['u', 'l', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['b', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2214);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2215);
					s.is_empty()
				}
				['m', 'e', 'h', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				['t', 'p', 'o', 'y', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1316);
			match s {
				[] => true,
				['a', 'e', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2220);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 2221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2223);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1319);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1320);
			match s {
				[] => true,
				['o', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				['s', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1321);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1322);
			match s {
				[] => true,
				['s', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sis.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2234);
					match s {
						[] => true,
						['r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						['t', 'n', 'a', 'h', 'p', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2921);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2922);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2923);
							s.is_empty()
						}
						['s', 'a', s @ ..] => {
							super::push(out, 2924);
							match s {
								[] => true,
								['t', 'a', 'c', 'o', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3269);
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
			super::push(out, 1324);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2238);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2925);
							match s {
								[] => true,
								['o', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'e', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2240);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2241);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2927);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 3271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 2928);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2242);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2930);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3272);
									match s {
										[] => true,
										['o', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3366);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2243);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2244);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2931);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2932);
							match s {
								[] => true,
								['h', 'p', 'a', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3274);
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
			super::push(out, 1325);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2245);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'u', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1326);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2246);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2934);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3275);
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
			super::push(out, 1328);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2247);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['h', 'p', 'r', 'o', 'm', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2249);
					s.is_empty()
				}
				['l', 'u', 'c', 'r', 'e', 'b', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 2250);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2935);
							s.is_empty()
						}
						['s', 'e', 'd', 'n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2251);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'o', 'h', 'c', 'n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 2937);
							s.is_empty()
						}
						['e', 'l', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3276);
									s.is_empty()
								}
								['o', 'i', 'r', 'e', 't', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'a', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'u', 'a', 'c', 'o', 'l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1329);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2253);
					match s {
						[] => true,
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2254);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2255);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2940);
							match s {
								[] => true,
								['c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3278);
									s.is_empty()
								}
								['l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2256);
					match s {
						[] => true,
						['n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1330);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1331);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2257);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2943);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2944);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2258);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2259);
					match s {
						[] => true,
						['c', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2945);
							s.is_empty()
						}
						['n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'e', 't', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1332);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tis.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1334);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2261);
					s.is_empty()
				}
				['i', 'n', 'o', 'm', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2262);
					s.is_empty()
				}
				['m', 'e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2263);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2264);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2265);
					s.is_empty()
				}
				['s', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1335);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2267);
					s.is_empty()
				}
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['h', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2269);
					s.is_empty()
				}
				['o', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2270);
					s.is_empty()
				}
				['p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1337);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2272);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2273);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 2947);
							match s {
								[] => true,
								['r', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 3281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2274);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2948);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2949);
							match s {
								[] => true,
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2275);
					match s {
						[] => true,
						['d', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2276);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2951);
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 3283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1338);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['a', 'm', 'o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2282);
					s.is_empty()
				}
				['y', 'c', 'o', 'l', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['d', 'i', 'p', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2285);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3284);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3285);
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
		['r', 'y', 's', '\u{1}', s @ ..] => {
			super::push(out, 1342);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2286);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 2954);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['r', 'g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uis.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2957);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2958);
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

/// Words ending -vis.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 501);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2295);
					match s {
						[] => true,
						['b', 'i', s @ ..] => {
							super::push(out, 2959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'u', 't', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 1351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xis.
pub(crate) fn n506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1353);
			match s {
				[] => true,
				['h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1354);
			match s {
				[] => true,
				['d', 'o', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1355);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1356);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eus.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1458);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1459);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1460);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1461);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['n', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['p', 'r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['i', 's', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1464);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gus.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2405);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2990);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2408);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2409);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1468);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1469);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2410);
					s.is_empty()
				}
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2411);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1470);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1471);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1472);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2413);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2993);
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

/// Words ending -hus.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1473);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 2414);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1476);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2417);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2419);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						['i', 'r', s @ ..] => {
							super::push(out, 2995);
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

/// Words ending -ius.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1477);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2420);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1479);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1480);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1481);
			match s {
				[] => true,
				['a', 'd', 'n', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2422);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2423);
					match s {
						[] => true,
						['d', 'n', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2997);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1482);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2425);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2427);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2999);
							s.is_empty()
						}
						['m', 'e', 'a', 'd', 'e', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1483);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2428);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3001);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3002);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3293);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['c', 'u', 'b', '\u{1}', s @ ..] => {
													super::push(out, 3380);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 3381);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3445(s, out),
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
									super::push(out, 3294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3003);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3004);
							match s {
								[] => true,
								['e', 'c', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3295);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3296);
									match s {
										[] => true,
										['s', 'u', 'a', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 3369);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'e', 'm', 'a', 'l', s @ ..] => {
							super::push(out, 3005);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3006);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3297);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3298);
									match s {
										[] => true,
										['i', 'b', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3370);
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
					super::push(out, 2429);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3007);
							s.is_empty()
						}
						['t', 'n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['c', 'r', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1484);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2432);
					match s {
						[] => true,
						['n', 'n', 'o', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'a', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lus.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2435);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2436);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2437);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2438);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3012);
							s.is_empty()
						}
						['i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1488);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2442);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1489);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1490);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2444);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3015);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3016);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3299);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2445);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1491);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2447);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2448);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1492);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2451);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3020);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									super::push(out, 3301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2453);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3022);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1493);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2455);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mus.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1494);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1495);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1496);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2456);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1497);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1498);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2458);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2459);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1499);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1500);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1501);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2463);
					s.is_empty()
				}
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2464);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2465);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2467);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3024);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1502);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2468);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2469);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3026);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3027);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 3028);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3029);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3030);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3031);
							s.is_empty()
						}
						['t', 'a', s @ ..] => {
							super::push(out, 3032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1503);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1504);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nus.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1505);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2472);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3033);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1506);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2475);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2476);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2478);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3035);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3036);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2480);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2481);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1507);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1508);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3040);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 3303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2486);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2487);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2490);
					match s {
						[] => true,
						['a', 'm', 's', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3043);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3044);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3046);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2492);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1509);
			match s {
				[] => true,
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2494);
					s.is_empty()
				}
				['u', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1510);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1511);
			match s {
				[] => true,
				['a', 'v', s @ ..] => {
					super::push(out, 2496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2497);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1513);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2499);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1514);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2501);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ous.
pub(crate) fn n550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	s.is_empty()
}

/// Words ending -pus.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1515);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1518);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1519);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1520);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2508);
					s.is_empty()
				}
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2510);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2511);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						['u', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2512);
					s.is_empty()
				}
				['l', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2513);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2514);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3051);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3052);
							s.is_empty()
						}
						['y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2515);
					match s {
						[] => true,
						['o', 's', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1522);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1523);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2516);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2517);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2518);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1524);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1525);
			match s {
				[] => true,
				['s', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2519);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rus.
pub(crate) fn n552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 548);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1526);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1527);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1528);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2522);
					s.is_empty()
				}
				['i', 'l', 'l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2523);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2524);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1530);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1531);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2526);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2527);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1532);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2529);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['c', 'o', 'n', 'a', 'b', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1534);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2532);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2534);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2535);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1536);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2537);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2538);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2539);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2540);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1537);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2541);
					match s {
						[] => true,
						['n', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 3059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2543);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2544);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2546);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3062);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2548);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1539);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sus.
pub(crate) fn n553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 549);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1540);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1541);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2549);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2550);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3065);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1542);
			s.is_empty()
		}
		['e', 'r', 'a', 'd', 'n', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1543);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1544);
			match s {
				[] => true,
				['d', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2552);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2553);
					s.is_empty()
				}
				['t', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2555);
					s.is_empty()
				}
				['e', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2556);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1545);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2558);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 3067);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2559);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1546);
			match s {
				[] => true,
				['l', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2560);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2562);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 2563);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1547);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2565);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3305);
									s.is_empty()
								}
								['a', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2566);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2567);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 3071);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3073);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1548);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['r', 'p', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3075);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2571);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3077);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3307);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 3308);
									match s {
										[] => true,
										['u', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3371);
											s.is_empty()
										}
										['y', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3372);
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
						['c', 'r', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3078);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3079);
							s.is_empty()
						}
						['r', 'a', 'p', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2572);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2574);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3083);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1550);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tus.
pub(crate) fn n554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 550);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1551);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1552);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3085);
							s.is_empty()
						}
						['a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3086);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3087);
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3089);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3090);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3091);
							match s {
								[] => true,
								['f', 'i', 'd', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3309);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 3373);
											s.is_empty()
										}
										['u', 'l', '\u{1}', s @ ..] => {
											super::push(out, 3374);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3092);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 3310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'i', 'd', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2580);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2581);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2582);
					match s {
						[] => true,
						['e', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3093);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3094);
							s.is_empty()
						}
						['l', 'u', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3095);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'd', 'e', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3311);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2583);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3096);
							s.is_empty()
						}
						['g', 'e', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3097);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3313);
									s.is_empty()
								}
								['o', 't', 'c', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3099);
							s.is_empty()
						}
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3100);
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3101);
							s.is_empty()
						}
						['i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 3102);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3103);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3315);
									match s {
										[] => true,
										['i', 'n', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 3375);
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
					super::push(out, 2585);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3104);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3105);
							s.is_empty()
						}
						['o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3107);
							s.is_empty()
						}
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3108);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3109);
							match s {
								[] => true,
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3110);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3111);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2587);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3113);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3114);
							s.is_empty()
						}
						['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 3115);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3116);
							match s {
								[] => true,
								['f', 'i', 'p', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3117);
							s.is_empty()
						}
						['o', 'p', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3118);
							s.is_empty()
						}
						['t', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3119);
							s.is_empty()
						}
						['u', 'i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3121);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2590);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3123);
							s.is_empty()
						}
						['n', 'a', 'l', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3124);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3125);
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2591);
					s.is_empty()
				}
				['y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n1573(s, out),
		['e', s @ ..] => {
			super::push(out, 1554);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2599);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2600);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2601);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2602);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2603);
					s.is_empty()
				}
				['p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2604);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3143);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n1575(s, out),
		['l', s @ ..] => super::t0::n1576(s, out),
		['n', s @ ..] => super::t1::n1577(s, out),
		['o', s @ ..] => {
			super::push(out, 1558);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2621);
					s.is_empty()
				}
				['m', s @ ..] => super::t1::n2661(s, out),
				['p', '\u{1}', s @ ..] => super::t1::n2662(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t1::n1579(s, out),
		['r', s @ ..] => super::t1::n1580(s, out),
		['s', s @ ..] => super::t1::n1581(s, out),
		['t', s @ ..] => super::t1::n1582(s, out),
		['u', s @ ..] => super::t1::n1583(s, out),
		['x', 'e', s @ ..] => super::t1::n1584(s, out),
		['y', s @ ..] => super::t1::n1585(s, out),
		_ => false,
	}
}

/// Words ending -uus.
pub(crate) fn n555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 551);
	s.is_empty()
}

/// Words ending -vus.
pub(crate) fn n556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 552);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1565);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1566);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2653);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 2654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1567);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xus.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 553);
	match s {
		[] => true,
		['a', 't', '\u{1}', s @ ..] => {
			super::push(out, 1569);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1570);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1571);
			match s {
				[] => true,
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'd', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1572);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1573);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2657);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3341);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3342);
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

/// The word sellarius.
pub(crate) fn n3445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3382);
	s.is_empty()
}

/// Words ending -ctus.
pub(crate) fn n1573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1553);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2593);
			match s {
				[] => true,
				['b', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3321);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2594);
			match s {
				[] => true,
				['f', 'e', s @ ..] => {
					super::push(out, 3131);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'b', s @ ..] => {
					super::push(out, 3132);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3325);
							s.is_empty()
						}
						['a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3326);
							s.is_empty()
						}
						['e', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3327);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3329);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2595);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3137);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3331);
							s.is_empty()
						}
						['e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2596);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2597);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2598);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', 'u', 'q', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3334);
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

/// Words ending -itus.
pub(crate) fn n1575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1555);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2605);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2606);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2607);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				['e', 'p', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				['u', 'a', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2608);
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				['c', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				['l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 't', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2610);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['e', 'g', 'i', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2612);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3157);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3335);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', 's', 'i', 'g', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3160);
					s.is_empty()
				}
				['o', 'p', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 2614);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2615);
			match s {
				[] => true,
				['r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltus.
pub(crate) fn n1576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1556);
	match s {
		[] => true,
		['a', 'p', 's', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2616);
			s.is_empty()
		}
		['u', 's', s @ ..] => {
			super::push(out, 2617);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3163);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 3337);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

