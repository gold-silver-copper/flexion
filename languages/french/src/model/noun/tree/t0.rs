use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{10}', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 111);
							s.is_empty()
						}
						['l', 'j', '\u{1}', s @ ..] => {
							super::push(out, 112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['d', 'r', '\u{1}', s @ ..] => {
							super::push(out, 113);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 114);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 115);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 116);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 117);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 118);
							match s {
								[] => true,
								['d', 'p', '\u{1}', s @ ..] => {
									super::push(out, 709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 710);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 120);
							s.is_empty()
						}
						['r', 'm', '\u{1}', s @ ..] => {
							super::push(out, 121);
							s.is_empty()
						}
						['z', 'b', '\u{1}', s @ ..] => {
							super::push(out, 122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 123);
							s.is_empty()
						}
						['f', 'b', '\u{1}', s @ ..] => {
							super::push(out, 124);
							s.is_empty()
						}
						['s', 'd', 'p', '\u{1}', s @ ..] => {
							super::push(out, 125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 126);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 10);
					match s {
						[] => true,
						['n', 'p', '\u{1}', s @ ..] => {
							super::push(out, 128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'b', 's', '\u{1}', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 129);
							match s {
								[] => true,
								['q', '\u{1}', s @ ..] => {
									super::push(out, 711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 15);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 130);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 712);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 16);
					match s {
						[] => true,
						['d', 'p', '\u{1}', s @ ..] => {
							super::push(out, 132);
							s.is_empty()
						}
						['p', 'r', '\u{1}', s @ ..] => {
							super::push(out, 133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 134);
							s.is_empty()
						}
						['g', 'm', '\u{1}', s @ ..] => {
							super::push(out, 135);
							s.is_empty()
						}
						['l', 't', '\u{1}', s @ ..] => {
							super::push(out, 136);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 137);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 138);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 140);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 713);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 141);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 142);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 143);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 144);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 145);
							s.is_empty()
						}
						['s', 's', 'c', '\u{1}', s @ ..] => {
							super::push(out, 146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 147);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 20);
					match s {
						[] => true,
						['g', 't', '\u{1}', s @ ..] => {
							super::push(out, 149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['º', 'n', '\u{1}', s @ ..] => {
					super::push(out, 21);
					s.is_empty()
				}
				['ʳ', 'm', '\u{1}', s @ ..] => {
					super::push(out, 22);
					s.is_empty()
				}
				['ᵉ', 'ᵐ', 'm', '\u{1}', s @ ..] => {
					super::push(out, 23);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => super::t0::n2(s, out),
		['\u{12}', s @ ..] => super::t0::n3(s, out),
		['\u{13}', s @ ..] => super::t0::n4(s, out),
		_ => false,
	}
}

/// Words ending - (1 syllables).
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 150);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 151);
					s.is_empty()
				}
				['g', 'p', '\u{1}', s @ ..] => {
					super::push(out, 152);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 153);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 154);
					s.is_empty()
				}
				['m', 'p', '\u{1}', s @ ..] => {
					super::push(out, 155);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 156);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 157);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 715);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 716);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'r', '\u{1}', s @ ..] => {
					super::push(out, 158);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 718);
							s.is_empty()
						}
						['r', 'm', '\u{1}', s @ ..] => {
							super::push(out, 719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 160);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ï', s @ ..] => {
					super::push(out, 161);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 721);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1762);
									s.is_empty()
								}
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
			super::push(out, 25);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 162);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 163);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 164);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 723);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 165);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 166);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 168);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 725);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 726);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 169);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 170);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 171);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 728);
							s.is_empty()
						}
						['m', 's', '\u{1}', s @ ..] => {
							super::push(out, 729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 172);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 173);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 730);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 731);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 174);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 175);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 176);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 177);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 178);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 735);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'a', s @ ..] => {
					super::push(out, 179);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 180);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 181);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 737);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 738);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1764);
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
						['s', '\u{1}', s @ ..] => {
							super::push(out, 739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 184);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 185);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 741);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 1765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 186);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 744);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 187);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 746);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1768);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2966);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 2967);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 's', '\u{1}', s @ ..] => {
							super::push(out, 747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 748);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 749);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 189);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 751);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n192(s, out),
				['t', s @ ..] => super::t0::n193(s, out),
				['u', s @ ..] => {
					super::push(out, 192);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => super::t0::n195(s, out),
				['y', s @ ..] => super::t0::n196(s, out),
				['é', s @ ..] => super::t0::n197(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t0::n30(s, out),
		['g', s @ ..] => super::t0::n31(s, out),
		['h', s @ ..] => super::t0::n32(s, out),
		['i', s @ ..] => super::t0::n33(s, out),
		['j', s @ ..] => super::t0::n34(s, out),
		['k', s @ ..] => super::t0::n35(s, out),
		['l', s @ ..] => super::t0::n36(s, out),
		['m', s @ ..] => super::t0::n37(s, out),
		['n', s @ ..] => super::t0::n38(s, out),
		['o', s @ ..] => super::t0::n39(s, out),
		['p', s @ ..] => super::t0::n40(s, out),
		['q', 'a', 'f', '\u{1}', s @ ..] => super::t0::n41(s, out),
		['r', s @ ..] => super::t0::n42(s, out),
		['s', s @ ..] => super::t0::n43(s, out),
		['t', s @ ..] => super::t0::n44(s, out),
		['u', s @ ..] => super::t0::n45(s, out),
		['v', s @ ..] => super::t0::n46(s, out),
		['w', s @ ..] => super::t0::n47(s, out),
		['x', s @ ..] => super::t0::n48(s, out),
		['y', s @ ..] => super::t0::n49(s, out),
		['z', s @ ..] => super::t0::n50(s, out),
		['é', s @ ..] => super::t0::n51(s, out),
		['ë', s @ ..] => super::t0::n52(s, out),
		['ï', s @ ..] => super::t0::n53(s, out),
		['ô', 'h', 'r', '\u{1}', s @ ..] => super::t0::n54(s, out),
		['û', 'd', '\u{1}', s @ ..] => super::t0::n55(s, out),
		_ => false,
	}
}

/// Words ending - (2 syllables).
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 323);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						['b', 'm', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 863);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1788);
									match s {
										[] => true,
										['g', 'n', '\u{1}', s @ ..] => {
											super::push(out, 2968);
											s.is_empty()
										}
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
					super::push(out, 324);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 865);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1789);
									match s {
										[] => true,
										['g', 'n', '\u{1}', s @ ..] => {
											super::push(out, 2969);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 867);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 868);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 869);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 870);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 871);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 872);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'a', s @ ..] => {
					super::push(out, 326);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 327);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 328);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 875);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 329);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 876);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1796);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2970);
											s.is_empty()
										}
										['t', 's', '\u{1}', s @ ..] => {
											super::push(out, 2971);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 877);
							match s {
								[] => true,
								['a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 330);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 331);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 332);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 881);
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1798);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1799);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 2972);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 2973);
											s.is_empty()
										}
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
					super::push(out, 333);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 882);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 884);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 334);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 335);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 888);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 889);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 890);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 1801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 891);
							match s {
								[] => true,
								['a', s @ ..] => super::t0::n1836(s, out),
								['e', s @ ..] => super::t0::n1837(s, out),
								_ => false,
							}
						}
						['ê', s @ ..] => super::t0::n905(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t0::n343(s, out),
				['x', s @ ..] => super::t0::n344(s, out),
				['y', s @ ..] => super::t0::n345(s, out),
				['z', s @ ..] => super::t0::n346(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t0::n57(s, out),
		['c', s @ ..] => super::t0::n58(s, out),
		['d', s @ ..] => super::t0::n59(s, out),
		['e', s @ ..] => super::t0::n60(s, out),
		['f', s @ ..] => super::t0::n61(s, out),
		['g', s @ ..] => super::t0::n62(s, out),
		['h', s @ ..] => super::t0::n63(s, out),
		['i', s @ ..] => super::t0::n64(s, out),
		['j', 'd', 'u', 'o', 'b', 'b', 'e', 'z', '\u{1}', s @ ..] => super::t0::n65(s, out),
		['k', s @ ..] => super::t0::n66(s, out),
		['l', s @ ..] => super::t0::n67(s, out),
		['m', s @ ..] => super::t0::n68(s, out),
		['n', s @ ..] => super::t0::n69(s, out),
		['o', s @ ..] => super::t0::n70(s, out),
		['p', s @ ..] => super::t0::n71(s, out),
		['r', s @ ..] => super::t0::n72(s, out),
		['s', s @ ..] => super::t0::n73(s, out),
		['t', s @ ..] => super::t0::n74(s, out),
		['u', s @ ..] => super::t0::n75(s, out),
		['v', s @ ..] => super::t0::n76(s, out),
		['w', s @ ..] => super::t0::n77(s, out),
		['x', s @ ..] => super::t0::n78(s, out),
		['y', s @ ..] => super::t0::n79(s, out),
		['z', s @ ..] => super::t0::n80(s, out),
		['é', s @ ..] => super::t0::n81(s, out),
		['ï', s @ ..] => super::t0::n82(s, out),
		_ => false,
	}
}

/// Words ending - (3 syllables).
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 518);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1232);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1233);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1234);
							match s {
								[] => true,
								['l', 'i', 'b', '-', 'o', 'g', 'k', 'n', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1236);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['n', 'a', 't', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1238);
							s.is_empty()
						}
						['é', 'm', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1240);
							s.is_empty()
						}
						['g', 'a', 'l', 'l', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 1242);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1243);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1244);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1245);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 1246);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1247);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 2066);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1248);
							match s {
								[] => true,
								['j', '-', 'e', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1250);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2068);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 3058);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 4105);
													match s {
														[] => true,
														['c', 's', s @ ..] => super::t0::n4985(s, out),
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
							super::push(out, 1251);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1253);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ï', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1256);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						['l', 'e', 't', s @ ..] => {
							super::push(out, 1258);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1259);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1260);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						['i', 'l', '-', 'i', 'k', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						['m', 'a', 'g', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1266);
							match s {
								[] => true,
								['g', 'i', 'r', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2070);
									s.is_empty()
								}
								['k', 'a', 't', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 2071);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', 't', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1268);
							match s {
								[] => true,
								['p', '-', 'n', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2073);
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
							super::push(out, 1269);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1270);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2074);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1271);
							match s {
								[] => true,
								['g', s @ ..] => super::t0::n2123(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n1299(s, out),
						['é', s @ ..] => super::t0::n1300(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t0::n543(s, out),
				['t', s @ ..] => super::t0::n544(s, out),
				['u', s @ ..] => super::t0::n545(s, out),
				['v', s @ ..] => super::t0::n546(s, out),
				['z', s @ ..] => super::t0::n547(s, out),
				['é', s @ ..] => super::t0::n548(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t0::n84(s, out),
		['c', s @ ..] => super::t0::n85(s, out),
		['d', s @ ..] => super::t0::n86(s, out),
		['e', s @ ..] => super::t0::n87(s, out),
		['f', s @ ..] => super::t0::n88(s, out),
		['g', s @ ..] => super::t0::n89(s, out),
		['h', s @ ..] => super::t0::n90(s, out),
		['i', s @ ..] => super::t0::n91(s, out),
		['k', s @ ..] => super::t0::n92(s, out),
		['l', s @ ..] => super::t0::n93(s, out),
		['m', s @ ..] => super::t0::n94(s, out),
		['n', s @ ..] => super::t0::n95(s, out),
		['o', s @ ..] => super::t0::n96(s, out),
		['p', s @ ..] => super::t0::n97(s, out),
		['q', 'u', 'i', 'm', 'm', 'u', 'v', 'a', 'n', 'u', 'n', '\u{1}', s @ ..] => super::t0::n98(s, out),
		['r', s @ ..] => super::t0::n99(s, out),
		['s', s @ ..] => super::t0::n100(s, out),
		['t', s @ ..] => super::t0::n101(s, out),
		['u', s @ ..] => super::t0::n102(s, out),
		['v', s @ ..] => super::t0::n103(s, out),
		['w', s @ ..] => super::t0::n104(s, out),
		['x', s @ ..] => super::t0::n105(s, out),
		['y', s @ ..] => super::t0::n106(s, out),
		['z', s @ ..] => super::t0::n107(s, out),
		['à', s @ ..] => super::t0::n108(s, out),
		['é', s @ ..] => super::t0::n109(s, out),
		['ï', s @ ..] => super::t0::n110(s, out),
		['ô', s @ ..] => super::t0::n111(s, out),
		_ => false,
	}
}

/// Words ending -re (1 syllables).
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -te (1 syllables).
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 754);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 755);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ve (1 syllables).
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	s.is_empty()
}

/// Words ending -ye (1 syllables).
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	s.is_empty()
}

/// Words ending -ée (1 syllables).
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['t', 'l', '\u{1}', s @ ..] => {
			super::push(out, 756);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 757);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -f (1 syllables).
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['e', 'i', 'n', 'z', '\u{1}', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'v', 'e', '\u{1}', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 202);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -g (1 syllables).
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'm', 's', '\u{1}', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['u', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['v', 'e', '\u{1}', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -h (1 syllables).
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['e', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						['u', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						['e', s @ ..] => {
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
			super::push(out, 211);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					super::push(out, 772);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -i (1 syllables).
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['d', 'c', '\u{1}', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['e', 'e', '\u{1}', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 217);
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['q', '\u{1}', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['r', 'j', '\u{1}', s @ ..] => {
			super::push(out, 220);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['x', '\u{1}', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -j (1 syllables).
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['à', 'm', '\u{1}', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -k (1 syllables).
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', 'i', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['c', 'a', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -l (1 syllables).
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['-', 'n', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				['u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['é', 'r', '\u{1}', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'o', '\u{1}', s @ ..] => {
					super::push(out, 791);
					s.is_empty()
				}
				['œ', '\u{1}', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		['m', 'a', '\u{1}', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 234);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 793);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 236);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -m (1 syllables).
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['-', 'v', 'a', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 237);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'o', '\u{1}', s @ ..] => {
			super::push(out, 239);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 240);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -n (1 syllables).
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		['e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 247);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -o (1 syllables).
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['b', '\u{1}', s @ ..] => {
			super::push(out, 253);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 255);
			s.is_empty()
		}
		['j', '\u{1}', s @ ..] => {
			super::push(out, 256);
			s.is_empty()
		}
		['k', '\u{1}', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 258);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -p (1 syllables).
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'e', '\u{1}', s @ ..] => {
			super::push(out, 261);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				['i', '\u{1}', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 265);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 267);
			s.is_empty()
		}
		['é', 'd', '\u{1}', s @ ..] => {
			super::push(out, 268);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word faq (1 syllables).
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	s.is_empty()
}

/// Words ending -r (1 syllables).
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -s (1 syllables).
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 276);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['m', 'e', '\u{1}', s @ ..] => {
			super::push(out, 278);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -t (1 syllables).
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['i', 'u', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 833);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['é', 'd', '\u{1}', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['e', 'u', 'o', '\u{1}', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -u (1 syllables).
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ʻ', 'ū', 'l', '\u{1}', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1786);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'p', '\u{1}', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['k', '\u{1}', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 845);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 847);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', '\u{1}', s @ ..] => {
			super::push(out, 302);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['œ', 'v', '\u{1}', s @ ..] => {
			super::push(out, 304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -v (1 syllables).
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 305);
			s.is_empty()
		}
		['i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -w (1 syllables).
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	s.is_empty()
}

/// Words ending -x (1 syllables).
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -y (1 syllables).
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	s.is_empty()
}

/// Words ending -z (1 syllables).
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -é (1 syllables).
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 316);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 857);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ë (1 syllables).
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ï (1 syllables).
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word rhô (1 syllables).
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	s.is_empty()
}

/// The word dû (1 syllables).
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	s.is_empty()
}

/// Words ending -asta (2 syllables).
pub(crate) fn n1836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1802);
	s.is_empty()
}

/// Words ending -esta (2 syllables).
pub(crate) fn n1837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1803);
	s.is_empty()
}

/// Words ending -êta (2 syllables).
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 892);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1804);
			s.is_empty()
		}
		['b', '\u{1}', s @ ..] => {
			super::push(out, 1805);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2974);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2976);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -va (2 syllables).
pub(crate) fn n343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xa (2 syllables).
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 894);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ya (2 syllables).
pub(crate) fn n345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -za (2 syllables).
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['o', 'y', 'g', '\u{1}', s @ ..] => {
			super::push(out, 896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -b (2 syllables).
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'b', s @ ..] => {
			super::push(out, 341);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -c (2 syllables).
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['b', '-', 'c', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 899);
					match s {
						[] => true,
						['c', '-', 't', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1808);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2978);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4088);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', '-', 't', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2979);
									s.is_empty()
								}
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
			super::push(out, 346);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['d', '-', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -d (2 syllables).
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['g', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 903);
					match s {
						[] => true,
						['h', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['i', 'p', '-', s @ ..] => {
					super::push(out, 905);
					match s {
						[] => true,
						['n', 'i', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 906);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['f', '-', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4089);
											s.is_empty()
										}
										['g', 'n', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 4090);
											s.is_empty()
										}
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
		['n', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['o', 'r', '-', 'l', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['o', 'f', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e (2 syllables).
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['-', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 354);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['m', 'i', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 912);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1812);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', 'é', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'a', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['è', 'l', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 919);
					match s {
						[] => true,
						['u', 'o', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['è', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['ï', s @ ..] => {
					super::push(out, 921);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 358);
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 922);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1826);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['n', 'a', 'r', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['è', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1830);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1834);
							s.is_empty()
						}
						['â', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						['ê', 'r', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						['m', 'y', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1840);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 362);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						['u', 'o', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						['â', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						['è', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1848);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 2983);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'u', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						['i', s @ ..] => super::t0::n1887(s, out),
						['y', s @ ..] => super::t0::n1888(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n966(s, out),
				['p', s @ ..] => super::t0::n967(s, out),
				['u', s @ ..] => super::t0::n968(s, out),
				['y', s @ ..] => super::t0::n969(s, out),
				['â', s @ ..] => super::t0::n970(s, out),
				['ê', 'r', s @ ..] => super::t0::n971(s, out),
				['ô', 'r', s @ ..] => super::t0::n972(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t0::n372(s, out),
		['n', s @ ..] => super::t0::n373(s, out),
		['p', s @ ..] => super::t0::n374(s, out),
		['r', s @ ..] => super::t0::n375(s, out),
		['s', s @ ..] => super::t0::n376(s, out),
		['t', s @ ..] => super::t0::n377(s, out),
		['u', s @ ..] => super::t0::n378(s, out),
		['v', s @ ..] => super::t0::n379(s, out),
		['x', s @ ..] => super::t0::n380(s, out),
		['y', s @ ..] => super::t0::n381(s, out),
		['z', 'n', s @ ..] => super::t0::n382(s, out),
		['é', s @ ..] => super::t0::n383(s, out),
		_ => false,
	}
}

/// Words ending -f (2 syllables).
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1028);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1029);
					match s {
						[] => true,
						['l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -g (2 syllables).
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1030);
					match s {
						[] => true,
						['p', 'é', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1930);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 1931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -h (2 syllables).
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1031);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['o', 'm', 'i', 'd', 'z', '\u{1}', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1032);
					match s {
						[] => true,
						['l', 'o', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -i (2 syllables).
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['-', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['é', s @ ..] => {
					super::push(out, 1034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1035);
					match s {
						[] => true,
						['u', 'o', 'j', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 391);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 393);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 394);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1036);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 1935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1037);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1038);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['n', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1040);
					s.is_empty()
				}
				['v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1042);
					match s {
						[] => true,
						['-', 'n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1043);
					s.is_empty()
				}
				['s', '-', 'z', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				['m', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1048);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 1050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1051);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1938);
							s.is_empty()
						}
						['o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1939);
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
		['t', 't', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 399);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1940);
							match s {
								[] => true,
								['-', 'i', 'u', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', 'z', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 401);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word zebboudj (2 syllables).
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	s.is_empty()
}

/// Words ending -k (2 syllables).
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 't', 'a', s @ ..] => {
			super::push(out, 404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l (2 syllables).
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['b', 'f', 'r', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1055);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1941);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1942);
							s.is_empty()
						}
						['z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1058);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1944);
							match s {
								[] => true,
								['i', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4094);
											s.is_empty()
										}
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4095);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1945);
							s.is_empty()
						}
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1946);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1947);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1948);
							s.is_empty()
						}
						['è', '\u{1}', s @ ..] => {
							super::push(out, 1949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1950);
							s.is_empty()
						}
						['r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1951);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1953);
							s.is_empty()
						}
						['s', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['c', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						['l', 'e', 'z', 't', '\u{1}', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 1958);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 3009);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3010);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1959);
							s.is_empty()
						}
						['é', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3011);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 3012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						['r', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'w', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				['é', 'r', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['t', 'é', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['e', 'l', '-', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3013);
									s.is_empty()
								}
								['é', '\u{1}', s @ ..] => {
									super::push(out, 3014);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3015);
									s.is_empty()
								}
								['t', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', s @ ..] => {
							super::push(out, 1963);
							match s {
								[] => true,
								['a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3017);
									s.is_empty()
								}
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3018);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['o', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 408);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['ï', s @ ..] => {
							super::push(out, 1967);
							match s {
								[] => true,
								['a', 's', 'i', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 4096);
											s.is_empty()
										}
										['r', 't', '\u{1}', s @ ..] => {
											super::push(out, 4097);
											s.is_empty()
										}
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

/// Words ending -m (2 syllables).
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 412);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						['é', '\u{1}', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'd', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['æ', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1971);
							match s {
								[] => true,
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -n (2 syllables).
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 1974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1083);
					match s {
						[] => true,
						['s', 's', 'o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1976);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 3020);
									s.is_empty()
								}
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
			super::push(out, 417);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'é', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['c', '-', 'n', 'i', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1093);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1094);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1095);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3021);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1985);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1987);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1989);
							match s {
								[] => true,
								['o', 'x', '\u{1}', s @ ..] => {
									super::push(out, 3025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1105);
					match s {
						[] => true,
						['i', 'a', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						['s', 'e', s @ ..] => {
							super::push(out, 1992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 1109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['û', s @ ..] => {
			super::push(out, 421);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -o (2 syllables).
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 422);
			s.is_empty()
		}
		['c', 's', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['d', 'a', s @ ..] => {
			super::push(out, 424);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1111);
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
				['n', 'o', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 's', s @ ..] => {
			super::push(out, 427);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['d', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['m', 'o', s @ ..] => {
			super::push(out, 430);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 431);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		['s', 'n', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['-', 'o', 'y', '\u{1}', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -p (2 syllables).
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['l', 'f', '-', 'p', 'i', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['d', 'l', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1995);
							s.is_empty()
						}
						['n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['l', '-', 'n', 'e', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -r (2 syllables).
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['a', 'f', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1128);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1998);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1999);
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									super::push(out, 3026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'g', s @ ..] => {
							super::push(out, 2000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['p', 'i', 'r', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1133);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['k', 'n', 'e', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2005);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2006);
							match s {
								[] => true,
								['s', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2007);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2008);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3029);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3030);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3031);
									match s {
										[] => true,
										['-', 'u', 'o', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4098);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 3032);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2009);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2010);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2011);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2012);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3035);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4099);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 4810);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3036);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2013);
							match s {
								[] => true,
								['s', 'o', s @ ..] => {
									super::push(out, 3037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2014);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3038);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 3039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'g', s @ ..] => {
							super::push(out, 2015);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3040);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2017);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['œ', s @ ..] => {
					super::push(out, 1141);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -s (2 syllables).
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['u', 'g', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1143);
					match s {
						[] => true,
						['n', 'r', s @ ..] => {
							super::push(out, 2020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1145);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'i', 'f', '-', 'u', 'a', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 448);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 451);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['e', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2021);
							s.is_empty()
						}
						['æ', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['æ', 'r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['é', 'l', 'c', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -t (2 syllables).
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['s', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['u', 'g', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2024);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3042);
									s.is_empty()
								}
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
			super::push(out, 456);
			match s {
				[] => true,
				['-', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1159);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2025);
							match s {
								[] => true,
								['n', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3043);
									s.is_empty()
								}
								['u', 'e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2026);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2027);
							match s {
								[] => true,
								['c', '-', 'n', 'i', 'a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2028);
							match s {
								[] => true,
								['h', '-', 't', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1164);
					match s {
						[] => true,
						['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['o', 'p', '-', s @ ..] => {
							super::push(out, 2030);
							match s {
								[] => true,
								['d', 'n', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3047);
									s.is_empty()
								}
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
			super::push(out, 458);
			match s {
				[] => true,
				['b', '-', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				['g', 'a', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['o', 's', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2031);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2032);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1172);
					match s {
						[] => true,
						['s', 's', s @ ..] => {
							super::push(out, 2034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', '-', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2036);
							match s {
								[] => true,
								['-', 't', 'i', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['â', s @ ..] => {
			super::push(out, 462);
			s.is_empty()
		}
		['ê', s @ ..] => {
			super::push(out, 463);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -u (2 syllables).
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['d', 'n', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				['r', 'r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 2039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['s', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['b', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['f', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2040);
							match s {
								[] => true,
								['-', 'e', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2041);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3052);
									match s {
										[] => true,
										['f', 'e', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4102);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['k', 'u', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2043);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3053);
									match s {
										[] => true,
										['c', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4103);
											s.is_empty()
										}
										['s', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4104);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['u', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						['é', '\u{1}', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['u', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['s', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						['u', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 469);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 471);
			s.is_empty()
		}
		['i', 'c', 'c', 'o', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		['j', 'é', 't', '\u{1}', s @ ..] => {
			super::push(out, 473);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 475);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['b', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1198);
					match s {
						[] => true,
						['u', 'o', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1200);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						['u', 'o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', 'i', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['m', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2056);
							match s {
								[] => true,
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 3054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1207);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2057);
							match s {
								[] => true,
								['-', 'u', 'o', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['g', 'n', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ç', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -v (2 syllables).
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['a', 'c', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 485);
			s.is_empty()
		}
		['u', 'o', 'r', 'é', '\u{1}', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -w (2 syllables).
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	s.is_empty()
}

/// Words ending -x (2 syllables).
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['b', 'm', 'o', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['e', 'i', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['r', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -y (2 syllables).
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 3056);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 3057);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'r', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 493);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				['d', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 495);
			s.is_empty()
		}
		['g', 'g', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 496);
			s.is_empty()
		}
		['k', 's', 'i', 'h', 'w', '\u{1}', s @ ..] => {
			super::push(out, 497);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		['n', 'o', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1221);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2060);
							s.is_empty()
						}
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['l', 'a', 'y', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -z (2 syllables).
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['a', 'g', 'o', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['i', 'm', 'é', 'r', '\u{1}', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		['n', 'i', 'r', 'p', 'n', 'o', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 506);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -é (2 syllables).
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		['i', 't', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['c', '-', 't', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['h', 'p', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 516);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['a', 'y', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ï (2 syllables).
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	s.is_empty()
}

/// Words ending -schnacka (3 syllables).
pub(crate) fn n4985(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'c', 's', 'i', 'e', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5224);
			s.is_empty()
		}
		['i', 'e', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5225);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gora (3 syllables).
pub(crate) fn n2123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2076);
	match s {
		[] => true,
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rra (3 syllables).
pub(crate) fn n1299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1272);
	s.is_empty()
}

/// Words ending -éra (3 syllables).
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1273);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sa (3 syllables).
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1274);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ta (3 syllables).
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 531);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1276);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1277);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ua (3 syllables).
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 532);
	s.is_empty()
}

/// Words ending -va (3 syllables).
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 533);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1279);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -za (3 syllables).
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	s.is_empty()
}

/// Words ending -éa (3 syllables).
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	s.is_empty()
}

/// Words ending -b (3 syllables).
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['p', '-', 'o', 't', 's', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1281);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -c (3 syllables).
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1282);
					match s {
						[] => true,
						['a', 't', '-', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1283);
					match s {
						[] => true,
						['-', 'e', 'd', '-', 'l', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2081);
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
				['v', 'a', '-', 'e', 't', 's', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1285);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1286);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2082);
							match s {
								[] => true,
								['-', 'e', 'l', 'b', 'u', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'u', 's', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1287);
					s.is_empty()
				}
				['p', 'é', '-', 'c', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1290);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2083);
							match s {
								[] => true,
								['b', '-', 'n', 'o', 'l', 'l', 'i', 'u', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3062);
									s.is_empty()
								}
								['f', '-', 'u', 'a', '-', 'e', 'r', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 3063);
									s.is_empty()
								}
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
			super::push(out, 541);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1291);
					match s {
						[] => true,
						['s', 'e', '-', 'c', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						['é', '-', 'c', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['-', 't', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -d (3 syllables).
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['i', 'p', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2087);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', '-', 'u', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4106);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['o', 'p', 'e', 'r', '\u{1}', s @ ..] => {
													super::push(out, 4811);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 4812);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'u', 'p', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['e', 'h', 't', 'r', 'a', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1293);
					s.is_empty()
				}
				['h', 'a', 'j', 'd', 'u', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1294);
					s.is_empty()
				}
				['o', 'r', 'f', '-', 'e', 's', 's', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['a', 'r', 'g', '-', 'e', 'r', 'è', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1296);
					s.is_empty()
				}
				['o', 'f', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['-', 'e', 'l', 'b', 'u', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2089);
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
					super::push(out, 1298);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['-', 'e', 'h', 'c', 'u', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1299);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2091);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3066);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 4107);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 4813);
													s.is_empty()
												}
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
			super::push(out, 547);
			match s {
				[] => true,
				['a', 'g', 'i', 'n', '-', 'e', 'p', 'a', 'r', 't', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1300);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1301);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2092);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3067);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 4108);
											s.is_empty()
										}
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

/// Words ending -e (3 syllables).
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1302);
					match s {
						[] => true,
						['a', 'n', 'i', 'h', 's', 'i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2094);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3068);
									match s {
										[] => true,
										['e', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4109);
											s.is_empty()
										}
										['u', 'a', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4110);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2095);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1304);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2097);
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 3070);
									match s {
										[] => true,
										['-', 'n', 'e', '-', 'c', 'o', 'r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4111);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2098);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1305);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2099);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2100);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3074);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1306);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2102);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3076);
									match s {
										[] => true,
										['-', 'e', 't', 'n', 'i', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 4112);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 4113);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2103);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 3077);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1307);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2104);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1308);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2107);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3078);
									match s {
										[] => true,
										['e', 't', 'l', 'o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 4114);
											s.is_empty()
										}
										['à', '-', 'e', 'c', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4115);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2108);
							match s {
								[] => true,
								['g', '-', s @ ..] => {
									super::push(out, 3080);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4116);
											match s {
												[] => true,
												['s', 'i', 'r', 'b', '\u{1}', s @ ..] => {
													super::push(out, 4814);
													s.is_empty()
												}
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
							super::push(out, 2109);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2110);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2111);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n1338(s, out),
				['n', s @ ..] => super::t0::n1339(s, out),
				['o', s @ ..] => {
					super::push(out, 1312);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1313);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2122);
							s.is_empty()
						}
						['o', 'f', '-', 'e', 'é', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1314);
					match s {
						[] => true,
						['a', 's', '-', 'r', 'e', 'i', 'r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						['o', 'd', '-', 'e', 'l', 'l', 'i', 'a', 't', '\u{1}', s @ ..] => super::t0::n2172(s, out),
						['p', s @ ..] => super::t0::n2173(s, out),
						_ => false,
					}
				}
				['è', s @ ..] => super::t0::n1343(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n563(s, out),
		['e', s @ ..] => super::t0::n564(s, out),
		['f', s @ ..] => super::t0::n565(s, out),
		['g', s @ ..] => super::t0::n566(s, out),
		['h', s @ ..] => super::t0::n567(s, out),
		['i', s @ ..] => super::t0::n568(s, out),
		['k', s @ ..] => super::t0::n569(s, out),
		['l', s @ ..] => super::t0::n570(s, out),
		['m', s @ ..] => super::t0::n571(s, out),
		['n', s @ ..] => super::t0::n572(s, out),
		['p', s @ ..] => super::t0::n573(s, out),
		['r', s @ ..] => super::t1::n574(s, out),
		['s', s @ ..] => super::t1::n575(s, out),
		['t', s @ ..] => super::t1::n576(s, out),
		['u', s @ ..] => super::t1::n577(s, out),
		['v', s @ ..] => super::t1::n578(s, out),
		['x', s @ ..] => super::t1::n579(s, out),
		['z', s @ ..] => super::t1::n580(s, out),
		['é', s @ ..] => super::t1::n581(s, out),
		['ü', s @ ..] => super::t1::n582(s, out),
		_ => false,
	}
}

/// Words ending -f (3 syllables).
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['p', '-', 'u', 'a', '-', 'e', 't', 'u', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1484);
					match s {
						[] => true,
						['c', '-', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'r', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3857);
									s.is_empty()
								}
								['t', 'n', 'a', s @ ..] => {
									super::push(out, 3858);
									match s {
										[] => true,
										['d', 'u', 'j', 'd', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4673);
											s.is_empty()
										}
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
					super::push(out, 1485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 572);
			match s {
				[] => true,
				['o', 's', '-', 's', 'n', 'a', 's', '-', 't', 'i', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['-', 'g', 'n', 'i', 'k', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 574);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1489);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2669);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['œ', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2670);
							match s {
								[] => true,
								['-', 'e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4674);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5133);
													match s {
														[] => true,
														['l', 'i', 'œ', '\u{1}', s @ ..] => super::t1::n5553(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', 'ê', 'r', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4675);
											s.is_empty()
										}
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

/// Words ending -g (3 syllables).
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['a', 't', '-', 'n', 'o', 'i', 't', 's', 'e', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 575);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -h (3 syllables).
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 577);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1492);
					match s {
						[] => true,
						['b', 'a', 'r', 'é', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -i (3 syllables).
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2672);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3860);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4676);
											s.is_empty()
										}
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
		['c', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['u', 'o', 's', '-', 's', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2673);
							match s {
								[] => true,
								['s', 'è', 'r', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3861);
									s.is_empty()
								}
								['t', 'n', 'a', 'v', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['a', 'm', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2674);
							match s {
								[] => true,
								['f', 'u', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3863);
									s.is_empty()
								}
								['v', 'u', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'u', 'o', 'k', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 584);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 585);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				['é', 'b', '-', 'l', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['-', 'a', 'l', '-', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				['s', '-', 'à', '-', 't', 'n', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 1504);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1505);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2676);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3865);
									match s {
										[] => true,
										['s', 'n', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 4677);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'u', 'o', 'p', '-', 't', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 589);
			s.is_empty()
		}
		['u', 'o', '-', 'i', 'u', 'o', '-', 'i', 'n', 'é', 'b', '\u{1}', s @ ..] => {
			super::push(out, 590);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 591);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -k (3 syllables).
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 592);
			match s {
				[] => true,
				['m', 'r', 'a', 'b', 'h', 's', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 't', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2677);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3866);
									s.is_empty()
								}
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
			super::push(out, 594);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l (3 syllables).
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1509);
					match s {
						[] => true,
						['a', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2678);
							s.is_empty()
						}
						['o', 'r', 'b', 'm', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2679);
							s.is_empty()
						}
						['s', 'a', 'p', 'o', 't', 'c', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2681);
							s.is_empty()
						}
						['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['t', 's', 'e', 'l', 'é', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1512);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2684);
							match s {
								[] => true,
								['n', 'a', '-', 'e', 'm', 'm', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 3867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1513);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2685);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3868);
									match s {
										[] => true,
										['-', 't', 'n', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4678);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'c', 'c', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2686);
							match s {
								[] => true,
								['m', 'r', s @ ..] => {
									super::push(out, 3870);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4679);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2687);
							match s {
								[] => true,
								['i', 't', s @ ..] => {
									super::push(out, 3871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2688);
							match s {
								[] => true,
								['u', 'o', 'j', '-', 'e', 'r', 'v', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['u', 'f', 'r', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1515);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2691);
							s.is_empty()
						}
						['i', 'b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2693);
							s.is_empty()
						}
						['é', 'c', s @ ..] => {
							super::push(out, 2694);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 3873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1516);
					match s {
						[] => true,
						['a', 'n', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2695);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2696);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 3874);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4680);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5134);
													match s {
														[] => true,
														['e', 'u', 'q', 's', 'a', 'm', '\u{1}', s @ ..] => super::t1::n5554(s, out),
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
							super::push(out, 2697);
							match s {
								[] => true,
								['n', 'k', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 3875);
									s.is_empty()
								}
								['t', 's', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3876);
									s.is_empty()
								}
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
			super::push(out, 596);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1517);
					match s {
						[] => true,
						['c', '-', s @ ..] => {
							super::push(out, 2698);
							match s {
								[] => true,
								['e', 't', 't', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3877);
									s.is_empty()
								}
								['n', 'e', '-', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1518);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1519);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2700);
							match s {
								[] => true,
								['l', 'o', 'c', '-', 't', 'n', 'a', 'n', 'e', 't', 'u', 'e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['u', 'a', '-', 'e', 'r', 't', 'î', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 597);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['-', 'e', 's', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2702);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2703);
							match s {
								[] => true,
								['i', 'p', 'u', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1522);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2704);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3881);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4681);
											match s {
												[] => true,
												['e', 'n', 'i', 'h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 5135);
													s.is_empty()
												}
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
				['œ', '-', 'e', 'h', 'c', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1524);
					match s {
						[] => true,
						['b', 't', 'o', 'o', 'f', '-', 'n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['d', 'a', s @ ..] => {
							super::push(out, 2706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['-', 'e', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'è', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3882);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3883);
									match s {
										[] => true,
										['t', 'e', 'u', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4682);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n1563(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -m (3 syllables).
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['l', 'e', 'h', 'c', '-', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['a', 'f', '-', 'a', 'l', '-', 'e', 'v', 'è', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1532);
					s.is_empty()
				}
				['t', 'a', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 604);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['l', 'o', 'p', 'o', 'm', 'r', 'e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 2709);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2710);
							match s {
								[] => true,
								['i', 'l', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 3884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', s @ ..] => {
							super::push(out, 2711);
							match s {
								[] => true,
								['t', 'p', 'i', 'r', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 3885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['u', 'c', 'i', s @ ..] => {
							super::push(out, 2712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1536);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3886);
									s.is_empty()
								}
								['x', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['o', 'i', 'r', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'm', 'i', 'd', 'e', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2714);
							s.is_empty()
						}
						['p', 'i', 'r', 'c', 's', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2715);
							s.is_empty()
						}
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
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1539);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1542);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2716);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3888);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4683);
											match s {
												[] => true,
												['d', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
													super::push(out, 5136);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 5137);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'm', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 3889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'l', 't', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2717);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2718);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3890);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 3891);
									match s {
										[] => true,
										['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 4684);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4685);
											match s {
												[] => true,
												['i', 'n', 'n', 'e', 't', '\u{1}', s @ ..] => {
													super::push(out, 5138);
													s.is_empty()
												}
												['s', 'o', 'r', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5139);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 5140);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', 'b', 'g', 'u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 4686);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2719);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2720);
							match s {
								[] => true,
								['i', 'n', 'n', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 3893);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3894);
									match s {
										[] => true,
										['e', 'n', 'i', 's', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4687);
											s.is_empty()
										}
										['o', 'r', 'c', 'o', 'l', 'c', 'y', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4688);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 2721);
							match s {
								[] => true,
								['b', 'g', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3895);
									s.is_empty()
								}
								['r', 'a', 'l', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 3896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['c', 'é', '-', 'r', 'i', 'n', 'e', 'v', 'u', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				['g', 'a', 'w', 's', 'k', 'l', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1546);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2723);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2725);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3897);
									match s {
										[] => true,
										['u', 'r', 'i', 'h', 'c', '-', 'n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2726);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3898);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4690);
											match s {
												[] => true,
												['e', 'r', 't', 'î', 'a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 5141);
													s.is_empty()
												}
												['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 5142);
													s.is_empty()
												}
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
							super::push(out, 2727);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2728);
							match s {
								[] => true,
								['-', 'e', 'u', 'q', '-', 's', 'n', 'i', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3899);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3900);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4691);
											match s {
												[] => true,
												['a', 'r', 'b', 'a', 'l', '-', 't', 'e', '-', 'n', 'e', 'i', 'v', 'u', 'e', 'n', '-', 'e', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
													super::push(out, 5143);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['é', s @ ..] => {
									super::push(out, 3901);
									match s {
										[] => true,
										['t', 's', 'i', 'n', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4692);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2729);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3902);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4693);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3903);
									match s {
										[] => true,
										['u', s @ ..] => super::t1::n4860(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n621(s, out),
		['n', s @ ..] => super::t1::n622(s, out),
		['o', s @ ..] => super::t1::n623(s, out),
		['u', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['r', 'b', '-', 'e', 'g', 'u', 'o', 'r', '\u{1}', s @ ..] => super::t1::n1614(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -o (3 syllables).
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 612);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['é', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['a', 'c', '-', 'n', 'o', 'i', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['r', 'a', 'n', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['e', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['u', 'p', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['m', '-', 'i', 'l', 'é', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['y', 'r', 'c', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1591);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2788);
							match s {
								[] => true,
								['n', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 3951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['h', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2789);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1597);
					match s {
						[] => true,
						['i', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2792);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3952);
									match s {
										[] => true,
										['-', 'n', 'a', 'm', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 4721);
											s.is_empty()
										}
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
					super::push(out, 1599);
					s.is_empty()
				}
				['u', 'a', '-', 'i', 'x', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', 'd', 'i', 'v', '-', 'e', 'd', 'n', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -p (3 syllables).
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 623);
			s.is_empty()
		}
		['e', 'r', 'o', 't', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 624);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 625);
			s.is_empty()
		}
		['o', 'l', s @ ..] => {
			super::push(out, 626);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 627);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2793);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3953);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4722);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 5163);
													match s {
														[] => true,
														['-', 'e', s @ ..] => super::t1::n5560(s, out),
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word nunavummiuq (3 syllables).
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	s.is_empty()
}

/// Words ending -r (3 syllables).
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1603);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2794);
							match s {
								[] => true,
								['n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 3954);
									s.is_empty()
								}
								['o', 't', 's', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', '-', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1605);
					match s {
						[] => true,
						['l', 'o', 'd', 'o', 'r', s @ ..] => {
							super::push(out, 2795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 629);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1606);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2796);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2797);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									super::push(out, 3956);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4723);
											match s {
												[] => true,
												['c', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5164);
													s.is_empty()
												}
												['e', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
													super::push(out, 5165);
													s.is_empty()
												}
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
					super::push(out, 1607);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2798);
							match s {
								[] => true,
								['r', 'o', 's', '-', 'i', 't', 'n', 'e', 'r', 'p', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2799);
							match s {
								[] => true,
								['é', 'b', '-', 'e', 'r', 'u', 't', 'i', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2800);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3959);
									match s {
										[] => true,
										['p', '-', 'n', 'o', 'l', 'l', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4724);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2801);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3960);
									match s {
										[] => true,
										['p', '-', 'e', s @ ..] => {
											match s {
												[] => true,
												['i', 'a', 'n', 'n', 'o', 'm', '\u{1}', s @ ..] => {
													super::push(out, 5166);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 5167);
													match s {
														[] => true,
														['r', 'o', 'p', '\u{1}', s @ ..] => super::t1::n5561(s, out),
														['t', 'a', 'r', 'g', '\u{1}', s @ ..] => super::t1::n5562(s, out),
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
									super::push(out, 3961);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 4725);
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 5168);
													match s {
														[] => true,
														['-', s @ ..] => super::t1::n5563(s, out),
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
						['s', s @ ..] => {
							super::push(out, 2802);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3962);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 4726);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2803);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3963);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 4727);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2804);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3964);
									match s {
										[] => true,
										['e', 'c', '-', 'p', 'u', 'o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 4728);
											s.is_empty()
										}
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
					super::push(out, 1608);
					match s {
						[] => true,
						['l', 'a', '-', 's', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1609);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2806);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3965);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4729);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5169);
													match s {
														[] => true,
														['e', 'h', 'c', 'ê', 'b', '\u{1}', s @ ..] => super::t1::n5564(s, out),
														['n', 'o', 'd', 'r', 'a', 'c', '\u{1}', s @ ..] => super::t1::n5565(s, out),
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
				['n', s @ ..] => {
					super::push(out, 1610);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2808);
							match s {
								[] => true,
								['e', 'j', 'é', 'd', '-', s @ ..] => {
									super::push(out, 3966);
									match s {
										[] => true,
										['e', 's', 'u', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4730);
											s.is_empty()
										}
										['t', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4731);
											s.is_empty()
										}
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
					super::push(out, 1611);
					match s {
						[] => true,
						['a', 'p', '-', 'e', 'y', 'l', 'l', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'a', 'p', '-', 'z', 'e', 's', 's', 'i', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1612);
					s.is_empty()
				}
				['t', s @ ..] => super::t1::n1647(s, out),
				['v', s @ ..] => {
					super::push(out, 1614);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2813);
							match s {
								[] => true,
								['o', 'v', 'e', 'r', '-', 'e', 'h', 'c', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n643(s, out),
		['o', s @ ..] => super::t1::n644(s, out),
		['u', s @ ..] => super::t1::n645(s, out),
		_ => false,
	}
}

/// Words ending -s (3 syllables).
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				['d', 'l', 'i', 't', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['r', 'b', '-', 'à', '-', 'r', 'e', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1628);
					match s {
						[] => true,
						['g', 'n', 'o', '-', 'e', 'p', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2843);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['d', 'n', 'i', 'l', 'y', 'c', '-', 'c', 'o', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2845);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2846);
							match s {
								[] => true,
								['a', 'b', '-', 'e', 'd', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4011);
									s.is_empty()
								}
								_ => false,
							}
						}
						['è', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2848);
							match s {
								[] => true,
								['u', 'a', 'h', 'c', '-', 'e', 'd', '-', 't', 'u', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['n', 'a', 'd', 'n', 'e', 'c', 's', 'e', 'd', 'o', 'r', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2849);
							s.is_empty()
						}
						['o', 'n', '-', 'c', 'o', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2850);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2851);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4013);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 2852);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4015);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 4766);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5194);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['è', 's', 'b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4016);
									s.is_empty()
								}
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
			super::push(out, 635);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1635);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1636);
					match s {
						[] => true,
						['t', 'n', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['g', '-', 't', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2855);
							match s {
								[] => true,
								['-', 'i', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'o', 's', '-', 'e', 'v', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1638);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2857);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4018);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 4767);
											match s {
												[] => true,
												['æ', 't', '\u{1}', s @ ..] => {
													super::push(out, 5195);
													s.is_empty()
												}
												['é', 't', '\u{1}', s @ ..] => {
													super::push(out, 5196);
													s.is_empty()
												}
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
							super::push(out, 2858);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 2859);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1639);
					match s {
						[] => true,
						['e', 'l', '-', 't', 'n', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1640);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2861);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4021);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 4768);
											s.is_empty()
										}
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
		['n', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1641);
					match s {
						[] => true,
						['p', 'a', '-', 't', 'e', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2862);
							s.is_empty()
						}
						['s', '-', 'e', 'l', 'b', 'u', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1642);
					match s {
						[] => true,
						['l', 'l', 'i', 'e', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1644);
					match s {
						[] => true,
						['r', 'e', 'c', 'c', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['m', 'e', 't', '-', s @ ..] => {
					super::push(out, 1645);
					match s {
						[] => true,
						['e', 'c', 'a', 'p', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 640);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['a', 'h', 'c', '-', 'x', 'u', 'a', '-', 'e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1647);
					match s {
						[] => true,
						['a', 'd', 'n', 'e', 'c', 's', 'e', 'd', 'o', 'r', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						['e', 'm', 'e', 's', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['n', 'a', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['r', 'p', '-', 's', 'e', 'd', '-', 'e', 'n', 'i', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -t (3 syllables).
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['l', 'o', 's', '-', 'n', 'e', 'y', 'o', 't', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1651);
					match s {
						[] => true,
						['c', '-', s @ ..] => {
							super::push(out, 2870);
							match s {
								[] => true,
								['e', 'd', '-', 'l', 'i', 'œ', '\u{1}', s @ ..] => {
									super::push(out, 4022);
									s.is_empty()
								}
								['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 2871);
							match s {
								[] => true,
								['-', 'é', 't', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4024);
									s.is_empty()
								}
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
			super::push(out, 645);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 646);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1655);
					match s {
						[] => true,
						['s', 'a', 'b', '-', 'o', 't', 's', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1656);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2873);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4025);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4769);
											match s {
												[] => true,
												['r', 'b', '-', 'e', 'r', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
													super::push(out, 5197);
													s.is_empty()
												}
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
							super::push(out, 2874);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4026);
									match s {
										[] => true,
										['p', '-', 'u', 'a', 'e', 't', 'r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 4770);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4771);
											match s {
												[] => true,
												['b', 'a', 'c', '-', 'é', 'p', 'u', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5198);
													s.is_empty()
												}
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
				['n', s @ ..] => {
					super::push(out, 1657);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['o', 'l', 'c', '-', 'r', 'e', 't', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['n', 'i', 'v', '-', 'e', 'r', 't', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2876);
							match s {
								[] => true,
								['-', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'i', 't', '\u{1}', s @ ..] => {
											super::push(out, 4772);
											s.is_empty()
										}
										['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4773);
											s.is_empty()
										}
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
					super::push(out, 1661);
					match s {
						[] => true,
						['é', 'r', 'c', '-', 'e', 'c', 'n', 'a', 'r', 'u', 's', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2878);
							match s {
								[] => true,
								['n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4027);
									s.is_empty()
								}
								['u', 'a', '-', 'e', 's', 's', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4028);
									s.is_empty()
								}
								['é', 'p', 'a', 'n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'é', 'b', '-', 'l', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2879);
							match s {
								[] => true,
								['-', 'e', 'u', 'g', 'n', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1665);
					match s {
						[] => true,
						['i', 'p', 's', '-', 'e', 't', 'i', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1666);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2881);
							match s {
								[] => true,
								['p', '-', 'e', 'n', 'g', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1667);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2882);
							match s {
								[] => true,
								['r', 'i', 'c', '-', 't', 'r', 'u', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', '-', 'e', 'd', '-', 'e', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1668);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						['d', s @ ..] => super::t1::n2973(s, out),
						['f', s @ ..] => super::t1::n2974(s, out),
						['l', s @ ..] => super::t1::n2975(s, out),
						['n', s @ ..] => super::t1::n2976(s, out),
						['r', s @ ..] => super::t1::n2977(s, out),
						['s', s @ ..] => super::t1::n2978(s, out),
						['t', s @ ..] => super::t1::n2979(s, out),
						['v', s @ ..] => super::t1::n2980(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n1703(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n663(s, out),
		['r', s @ ..] => super::t1::n664(s, out),
		['s', s @ ..] => super::t1::n665(s, out),
		['u', s @ ..] => super::t1::n666(s, out),
		['ô', s @ ..] => super::t1::n667(s, out),
		_ => false,
	}
}

/// Words ending -u (3 syllables).
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2912);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4057);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 4787);
											match s {
												[] => true,
												['e', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5210);
													s.is_empty()
												}
												['é', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5211);
													s.is_empty()
												}
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
					super::push(out, 1684);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2913);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4058);
									match s {
										[] => true,
										['f', 'f', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4788);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2914);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4059);
									match s {
										[] => true,
										['r', 'u', 'o', 's', '-', 'e', 'v', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4789);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'b', 'u', 'o', 'd', '-', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2916);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2917);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4060);
									match s {
										[] => true,
										['a', 't', '-', 'e', 'l', 'l', 'i', 'u', 'o', 'n', 'e', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4790);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2918);
							match s {
								[] => true,
								['i', 'o', 'm', 'a', 'd', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2919);
							match s {
								[] => true,
								['r', 'a', 'm', '-', 'n', 'i', 'u', 'q', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4062);
									s.is_empty()
								}
								['â', 'g', '-', 'a', 'p', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1685);
					s.is_empty()
				}
				['t', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'm', 'u', 'g', 'a', 's', 'r', 'a', 'y', '\u{1}', s @ ..] => {
			super::push(out, 656);
			s.is_empty()
		}
		['c', 'n', 'i', 'a', 'v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 657);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 658);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2920);
							match s {
								[] => true,
								['r', '-', 'e', 't', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4064);
									s.is_empty()
								}
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
			super::push(out, 659);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1689);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2921);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'r', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4791);
											s.is_empty()
										}
										['l', '-', 'z', 'e', 's', 's', 'e', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4792);
											s.is_empty()
										}
										['m', 'u', 'l', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4793);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 4794);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4065);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4795);
											match s {
												[] => true,
												['e', 't', 'a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5212);
													s.is_empty()
												}
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
					super::push(out, 1690);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2922);
							match s {
								[] => true,
								['-', 'l', 'e', 't', 'ô', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4066);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'h', 'c', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['b', '-', 'n', 'o', 'd', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2924);
							s.is_empty()
						}
						['l', 'a', '-', 'c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['p', '-', 'e', 't', 'n', 'o', 'm', 'é', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2927);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4067);
									match s {
										[] => true,
										['-', s @ ..] => {
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 5213);
													match s {
														[] => true,
														['i', s @ ..] => super::t1::n5584(s, out),
														_ => false,
													}
												}
												['u', 'a', 'e', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5214);
													s.is_empty()
												}
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
		['g', 'i', 'b', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		['h', 'o', 'b', '-', 'u', 'h', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1694);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 663);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['c', s @ ..] => super::t1::n1730(s, out),
				['d', s @ ..] => super::t1::n1731(s, out),
				['h', s @ ..] => super::t1::n1732(s, out),
				['j', s @ ..] => super::t1::n1733(s, out),
				['l', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['r', s @ ..] => super::t1::n1735(s, out),
				['s', '-', 'e', 'p', 'p', 'i', 'r', 'g', '\u{1}', s @ ..] => super::t1::n1736(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n679(s, out),
		['s', 'i', 'm', 'a', 'r', 'i', 't', '\u{1}', s @ ..] => super::t1::n680(s, out),
		['t', s @ ..] => super::t1::n681(s, out),
		['v', s @ ..] => super::t1::n682(s, out),
		['ç', 'r', 'e', 'p', 'a', '\u{1}', s @ ..] => super::t1::n683(s, out),
		_ => false,
	}
}

/// Words ending -v (3 syllables).
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['i', 't', 'o', 'm', 't', 'i', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 671);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['i', 'n', 'h', 's', 'a', 'l', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -w (3 syllables).
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -x (3 syllables).
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 674);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1709);
					match s {
						[] => true,
						['n', 'o', 'p', 's', '-', 'o', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['d', 'r', 'e', 'p', '-', 'e', 'd', '-', 'l', 'i', 'œ', '\u{1}', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 677);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['r', 'r', 'e', 't', '-', 'l', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'h', 'c', '-', 'e', 'p', 'u', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -y (3 syllables).
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 679);
			s.is_empty()
		}
		['b', 'a', 'l', 'l', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 680);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['o', 't', 'a', 'v', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				['r', 'e', 'f', '-', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2938);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4070);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4803);
											match s {
												[] => true,
												['e', 'v', 'a', 'r', '\u{1}', s @ ..] => {
													super::push(out, 5218);
													s.is_empty()
												}
												['n', 'e', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
													super::push(out, 5219);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'z', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -z (3 syllables).
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'i', 'r', 'i', 'a', 'b', 'o', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -à (3 syllables).
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['-', 'i', 'o', 't', '-', 'e', 'h', 'c', 'u', 'o', 'c', '-', 'e', 'i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -é (3 syllables).
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['é', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2941);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4071);
									s.is_empty()
								}
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
			super::push(out, 688);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1722);
					match s {
						[] => true,
						['t', 'c', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 690);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1723);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2943);
							match s {
								[] => true,
								['-', 'e', s @ ..] => {
									super::push(out, 4072);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 4804);
											match s {
												[] => true,
												['s', s @ ..] => {
													match s {
														[] => true,
														['a', 'h', 'c', '\u{1}', s @ ..] => super::t1::n5587(s, out),
														['u', 'o', 'p', '\u{1}', s @ ..] => super::t1::n5588(s, out),
														_ => false,
													}
												}
												['u', 'a', 'p', '\u{1}', s @ ..] => {
													super::push(out, 5220);
													s.is_empty()
												}
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
			super::push(out, 691);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1726);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2944);
							match s {
								[] => true,
								['d', '-', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', '-', 'e', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['t', 'i', 'm', s @ ..] => {
					super::push(out, 1729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2945);
							match s {
								[] => true,
								['b', '-', 'é', 'l', 'u', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4074);
									s.is_empty()
								}
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
			super::push(out, 696);
			match s {
				[] => true,
				['i', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2946);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4075);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['m', 'e', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 5221);
													s.is_empty()
												}
												['n', 'r', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 5222);
													s.is_empty()
												}
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
					super::push(out, 1738);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1740);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1742);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1743);
					match s {
						[] => true,
						['n', 'o', 'h', '-', 't', 'n', 'i', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1744);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1745);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 1746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 700);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2948);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2949);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4076);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4805);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5223);
													s.is_empty()
												}
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
		['t', s @ ..] => {
			super::push(out, 701);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1750);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2950);
							s.is_empty()
						}
						['u', 'o', '-', 'n', 'o', 't', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', s @ ..] => {
					super::push(out, 1751);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n1786(s, out),
				['o', s @ ..] => {
					super::push(out, 1753);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2962);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2963);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 4085);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => super::t1::n1792(s, out),
				['ô', s @ ..] => super::t1::n1793(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n715(s, out),
		['v', s @ ..] => super::t1::n716(s, out),
		['z', s @ ..] => {
			super::push(out, 704);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ï (3 syllables).
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 705);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ô (3 syllables).
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 706);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ille (2 syllables).
pub(crate) fn n1887(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2984);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2985);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylle (2 syllables).
pub(crate) fn n1888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1851);
	s.is_empty()
}

/// Words ending -ole (2 syllables).
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['é', s @ ..] => {
			super::push(out, 1852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ple (2 syllables).
pub(crate) fn n967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 952);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1853);
			s.is_empty()
		}
		['o', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ule (2 syllables).
pub(crate) fn n968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 953);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yle (2 syllables).
pub(crate) fn n969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	s.is_empty()
}

/// Words ending -âle (2 syllables).
pub(crate) fn n970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 955);
	s.is_empty()
}

/// Words ending -rêle (2 syllables).
pub(crate) fn n971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 956);
	s.is_empty()
}

/// Words ending -rôle (2 syllables).
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 957);
	s.is_empty()
}

/// Words ending -me (2 syllables).
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		['h', 't', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		['r', 'u', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 965);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1858);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1859);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['è', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1861);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1862);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ne (2 syllables).
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1863);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1866);
					s.is_empty()
				}
				['e', 'y', s @ ..] => {
					super::push(out, 1867);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1871);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 2987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['â', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		['è', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ê', s @ ..] => {
			super::push(out, 974);
			s.is_empty()
		}
		['î', s @ ..] => {
			super::push(out, 975);
			s.is_empty()
		}
		['ô', s @ ..] => {
			super::push(out, 976);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pe (2 syllables).
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 977);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 978);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 980);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -re (2 syllables).
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				['è', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'i', s @ ..] => {
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
		['d', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1881);
					s.is_empty()
				}
				['è', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 984);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 985);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 't', s @ ..] => {
			super::push(out, 987);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1884);
					s.is_empty()
				}
				['u', 'e', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['i', 'a', s @ ..] => {
					super::push(out, 1886);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1887);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'o', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				['â', s @ ..] => {
					super::push(out, 1890);
					s.is_empty()
				}
				['î', 'u', s @ ..] => {
					super::push(out, 1891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['u', 'e', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				['è', 'l', s @ ..] => {
					super::push(out, 1895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['è', s @ ..] => {
			super::push(out, 991);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -se (2 syllables).
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 992);
			s.is_empty()
		}
		['n', 'a', s @ ..] => {
			super::push(out, 993);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 994);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 995);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1897);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1898);
					match s {
						[] => true,
						['o', s @ ..] => {
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
			match s {
				[] => true,
				['i', 'u', s @ ..] => {
					super::push(out, 1899);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te (2 syllables).
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 997);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1904);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'u', s @ ..] => {
			super::push(out, 999);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1906);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1001);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1002);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1908);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1909);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				['o', 'i', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 1004);
			s.is_empty()
		}
		['è', 'o', s @ ..] => {
			super::push(out, 1005);
			s.is_empty()
		}
		['î', s @ ..] => {
			super::push(out, 1006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ue (2 syllables).
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1913);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2997);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				['è', s @ ..] => {
					super::push(out, 1915);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2998);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 4093);
									s.is_empty()
								}
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
			super::push(out, 1007);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1008);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 1009);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2999);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', 'o', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3001);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3003);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1919);
					s.is_empty()
				}
				['è', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							super::push(out, 3005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ï', s @ ..] => {
					super::push(out, 1921);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['-', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1011);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ve (2 syllables).
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1016);
			s.is_empty()
		}
		['u', 'a', s @ ..] => {
			super::push(out, 1017);
			s.is_empty()
		}
		['è', s @ ..] => {
			super::push(out, 1018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xe (2 syllables).
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	s.is_empty()
}

/// Words ending -ye (2 syllables).
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	s.is_empty()
}

/// Words ending -nze (2 syllables).
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ée (2 syllables).
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1020);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1022);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1023);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		['r', 'o', s @ ..] => {
			super::push(out, 1025);
			s.is_empty()
		}
		['t', 'a', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ice (3 syllables).
pub(crate) fn n1338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1310);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2112);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 4117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2113);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3083);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 3084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3085);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2115);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2116);
			match s {
				[] => true,
				['t', 'a', 'p', s @ ..] => {
					super::push(out, 3088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2117);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4121);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 2118);
			match s {
				[] => true,
				['-', 'u', 'a', '-', 'e', 's', 's', 'u', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3090);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3091);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4123);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4815);
									match s {
										[] => true,
										['-', 'n', 'o', 'i', 't', 'a', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 5226);
											s.is_empty()
										}
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

/// Words ending -nce (3 syllables).
pub(crate) fn n1339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1311);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2119);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3092);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4124);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3093);
					match s {
						[] => true,
						['u', 't', s @ ..] => {
							super::push(out, 4126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', s @ ..] => {
					super::push(out, 3094);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3095);
					match s {
						[] => true,
						['s', 'i', 's', 'r', s @ ..] => {
							super::push(out, 4127);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4816);
									s.is_empty()
								}
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
			super::push(out, 2120);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3096);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 4128);
							match s {
								[] => true,
								['o', 'r', 'p', '-', 't', 'a', 't', 'é', '\u{1}', s @ ..] => {
									super::push(out, 4817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'c', s @ ..] => {
					super::push(out, 3098);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 4129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3099);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3100);
					match s {
						[] => true,
						['q', 'é', 's', '-', 'n', 'a', 'l', 'p', '\u{1}', s @ ..] => {
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
			super::push(out, 2121);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3101);
					match s {
						[] => true,
						['n', 'a', '-', 'e', 'd', 'n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word taille-douce (3 syllables).
pub(crate) fn n2172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2125);
	s.is_empty()
}

/// Words ending -puce (3 syllables).
pub(crate) fn n2173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2126);
	s.is_empty()
}

/// Words ending -èce (3 syllables).
pub(crate) fn n1343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1315);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2127);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['-', 'e', s @ ..] => {
							match s {
								[] => true,
								['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4818);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4819);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5227);
											s.is_empty()
										}
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

/// Words ending -de (3 syllables).
pub(crate) fn n563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 550);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1316);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2128);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2129);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2130);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3103);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2131);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2132);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 3104);
							s.is_empty()
						}
						['n', 'o', 'f', s @ ..] => {
							super::push(out, 3105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', s @ ..] => {
					super::push(out, 2133);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2134);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3106);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4134);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2137);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3108);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3109);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4135);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 4820);
											match s {
												[] => true,
												['-', 't', 'a', 't', 'n', 'e', 't', 't', 'a', '\u{1}', s @ ..] => {
													super::push(out, 5228);
													s.is_empty()
												}
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
							super::push(out, 3110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2140);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3112);
							s.is_empty()
						}
						['o', 's', s @ ..] => {
							super::push(out, 3113);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2141);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4136);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3115);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3116);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 3117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2142);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3118);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3119);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3120);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2143);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2144);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3121);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4140);
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
							super::push(out, 3122);
							match s {
								[] => true,
								['h', 'c', s @ ..] => {
									super::push(out, 4141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3123);
							s.is_empty()
						}
						['é', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4142);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 4821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4143);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4822);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4144);
									match s {
										[] => true,
										['é', s @ ..] => {
											super::push(out, 4823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'o', s @ ..] => {
					super::push(out, 2145);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3124);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4145);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4824);
											s.is_empty()
										}
										_ => false,
									}
								}
								['é', s @ ..] => {
									super::push(out, 4146);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 4825);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				['é', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3126);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2147);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3128);
							match s {
								[] => true,
								['-', 'e', 't', 'a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 3129);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4149);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n2198(s, out),
				['o', s @ ..] => super::t1::n2199(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n1347(s, out),
		['r', s @ ..] => super::t1::n1348(s, out),
		['u', s @ ..] => super::t1::n1349(s, out),
		['y', s @ ..] => super::t1::n1350(s, out),
		['è', s @ ..] => super::t1::n1351(s, out),
		['ï', s @ ..] => super::t1::n1352(s, out),
		_ => false,
	}
}

/// Words ending -ee (3 syllables).
pub(crate) fn n564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 551);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fe (3 syllables).
pub(crate) fn n565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 552);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1326);
			s.is_empty()
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2167);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2168);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2170);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1328);
			match s {
				[] => true,
				['o', 'r', 'b', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2171);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ge (3 syllables).
pub(crate) fn n566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 553);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1329);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2172);
					match s {
						[] => true,
						['-', 't', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3155);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 3156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2173);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3157);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3158);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4171);
									match s {
										[] => true,
										['r', 'c', s @ ..] => {
											super::push(out, 4835);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4836);
											match s {
												[] => true,
												['œ', s @ ..] => {
													super::push(out, 5229);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 4837);
											s.is_empty()
										}
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
					super::push(out, 2174);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3159);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3160);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2175);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3161);
							s.is_empty()
						}
						['ô', s @ ..] => {
							super::push(out, 3162);
							match s {
								[] => true,
								['h', 'c', '-', 'e', 'c', 'n', 'a', 'r', 'u', 's', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2176);
					match s {
						[] => true,
						['i', 'a', s @ ..] => {
							super::push(out, 3163);
							match s {
								[] => true,
								['r', 'r', s @ ..] => {
									super::push(out, 4175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							super::push(out, 3164);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 3165);
							match s {
								[] => true,
								['m', '-', 'e', 'u', 'm', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2177);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3166);
							match s {
								[] => true,
								['n', 'e', '-', 'e', 's', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', 'r', s @ ..] => {
							super::push(out, 3167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2178);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 3168);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3169);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 4179);
									match s {
										[] => true,
										['m', '-', s @ ..] => {
											super::push(out, 4838);
											match s {
												[] => true,
												['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
													super::push(out, 5230);
													s.is_empty()
												}
												['t', 'r', 'u', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5231);
													s.is_empty()
												}
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
							super::push(out, 3170);
							s.is_empty()
						}
						['é', 'i', s @ ..] => {
							super::push(out, 3171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2179);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3172);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4180);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 4839);
											match s {
												[] => true,
												['-', 'e', 'l', '-', 'z', 'e', 'd', 'é', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5232);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4181);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4840);
											s.is_empty()
										}
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
					super::push(out, 2180);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3173);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 3174);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 3175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2181);
					match s {
						[] => true,
						['o', 'h', s @ ..] => {
							super::push(out, 3176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				['y', 'o', s @ ..] => {
					super::push(out, 2183);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3178);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4182);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
													super::push(out, 5233);
													s.is_empty()
												}
												['s', 's', 'a', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5234);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4183);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 4841);
											s.is_empty()
										}
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
					super::push(out, 2185);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['-', 'n', 'o', 't', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n1360(s, out),
		['r', s @ ..] => super::t1::n1361(s, out),
		['u', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2193);
					s.is_empty()
				}
				['o', s @ ..] => super::t1::n2246(s, out),
				_ => false,
			}
		}
		['â', '-', 'e', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1334);
			s.is_empty()
		}
		['è', s @ ..] => super::t1::n1364(s, out),
		_ => false,
	}
}

/// Words ending -he (3 syllables).
pub(crate) fn n567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 554);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2196);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3191);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2197);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3193);
							match s {
								[] => true,
								['-', 'e', 'd', '-', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3194);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3195);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3196);
							match s {
								[] => true,
								['s', 'o', s @ ..] => {
									super::push(out, 4194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 3197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2198);
					match s {
						[] => true,
						['a', 'r', 'b', 'i', s @ ..] => {
							super::push(out, 3198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2199);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3199);
							match s {
								[] => true,
								['-', 'e', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3200);
							match s {
								[] => true,
								['c', '-', 'e', 'l', 'b', 'u', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4196);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3201);
							match s {
								[] => true,
								['-', 's', 'u', 'o', 's', '-', 'i', 'r', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4197);
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
							super::push(out, 3202);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2200);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3204);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3205);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3206);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4200);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4844);
											match s {
												[] => true,
												['e', 's', 'u', 'm', 'a', '\u{1}', s @ ..] => {
													super::push(out, 5237);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 4201);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 4202);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4845);
											match s {
												[] => true,
												['n', 'i', 'a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5238);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4203);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4846);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 5239);
													match s {
														[] => true,
														['d', '-', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => super::t1::n5589(s, out),
														_ => false,
													}
												}
												['u', s @ ..] => {
													super::push(out, 5240);
													match s {
														[] => true,
														['a', s @ ..] => super::t1::n5590(s, out),
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
									super::push(out, 4204);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4205);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4847);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 5241);
													match s {
														[] => true,
														['-', 'e', 't', 'n', 'i', 'a', 's', '\u{1}', s @ ..] => super::t1::n5591(s, out),
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
							super::push(out, 3207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['è', s @ ..] => {
					super::push(out, 2201);
					match s {
						[] => true,
						['i', 'r', 'g', '-', 'e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ê', s @ ..] => {
					super::push(out, 2202);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3209);
							match s {
								[] => true,
								['-', 'e', 't', 'ê', 't', '\u{1}', s @ ..] => {
									super::push(out, 4206);
									s.is_empty()
								}
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
			super::push(out, 1337);
			s.is_empty()
		}
		['p', s @ ..] => super::t1::n1367(s, out),
		['t', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3217);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 4212);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2209);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4213);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2210);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							super::push(out, 3219);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ie (3 syllables).
pub(crate) fn n568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2212);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2213);
					match s {
						[] => true,
						['b', '-', 'e', 's', 's', 'u', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3221);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'o', 'r', s @ ..] => {
			super::push(out, 1341);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['s', '-', 'n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2215);
					s.is_empty()
				}
				['é', 'p', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				['ï', 'o', 'l', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['o', 'l', 'o', s @ ..] => {
					match s {
						[] => true,
						['m', 'y', s @ ..] => {
							super::push(out, 3223);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 4216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', s @ ..] => {
							super::push(out, 3224);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3225);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 4218);
									s.is_empty()
								}
								['s', 'i', s @ ..] => {
									super::push(out, 4219);
									s.is_empty()
								}
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
			super::push(out, 1344);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2218);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 4220);
									s.is_empty()
								}
								['t', 'n', s @ ..] => {
									super::push(out, 4221);
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
						['a', 'r', 'g', 'o', 'm', 'é', s @ ..] => {
							super::push(out, 3226);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2219);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 2221);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['a', 'g', 'o', 's', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3230);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'n', s @ ..] => {
					super::push(out, 2223);
					match s {
						[] => true,
						['o', 't', s @ ..] => {
							super::push(out, 3232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['é', 'c', s @ ..] => {
					super::push(out, 2224);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3233);
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
					super::push(out, 2225);
					s.is_empty()
				}
				['o', 'g', 'o', s @ ..] => {
					super::push(out, 2226);
					match s {
						[] => true,
						['m', 'r', s @ ..] => {
							super::push(out, 3234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['é', 'g', 'o', 'h', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3235);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4222);
									match s {
										[] => true,
										['d', 'r', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4859);
											s.is_empty()
										}
										['r', 'i', 'a', 'l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4860);
											s.is_empty()
										}
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
		['p', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['-', 'e', 'd', '-', 'e', 'u', 'e', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				['a', 'r', 'é', 'h', 't', 'o', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				['o', 'c', 's', 'o', 'r', 'c', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['m', '-', 'n', 'i', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2234);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3237);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4223);
									match s {
										[] => true,
										['a', 'g', s @ ..] => {
											super::push(out, 4861);
											match s {
												[] => true,
												['-', 'e', 't', 'l', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 5253);
													s.is_empty()
												}
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
							super::push(out, 3238);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 4224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'n', 'o', 'ç', s @ ..] => {
							super::push(out, 3239);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3240);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3241);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3242);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4225);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4862);
											match s {
												[] => true,
												['a', 'r', 'é', 'p', 'o', '\u{1}', s @ ..] => {
													super::push(out, 5254);
													s.is_empty()
												}
												['t', 'e', 'l', 'l', 'a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5255);
													s.is_empty()
												}
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
				['t', s @ ..] => {
					super::push(out, 2235);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3243);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4226);
									match s {
										[] => true,
										['-', 'e', 'r', 'è', 'm', '\u{1}', s @ ..] => {
											super::push(out, 4863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['é', 'm', 'o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4227);
									s.is_empty()
								}
								['r', 'o', s @ ..] => {
									super::push(out, 4228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'c', s @ ..] => {
					super::push(out, 2236);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['g', 'o', s @ ..] => {
							super::push(out, 3244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 2238);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3245);
							s.is_empty()
						}
						['t', 'c', 'a', 'b', 'o', s @ ..] => {
							super::push(out, 3246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n1382(s, out),
		['t', s @ ..] => super::t1::n1383(s, out),
		['u', s @ ..] => super::t1::n1384(s, out),
		['v', s @ ..] => super::t1::n1385(s, out),
		['x', 'e', 'l', s @ ..] => super::t1::n1386(s, out),
		_ => false,
	}
}

/// Words ending -ke (3 syllables).
pub(crate) fn n569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 556);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -le (3 syllables).
pub(crate) fn n570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 557);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['d', 'n', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2242);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2243);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2244);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3250);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 4231);
									match s {
										[] => true,
										['c', 'n', s @ ..] => {
											super::push(out, 4864);
											s.is_empty()
										}
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
					super::push(out, 2245);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2247);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 3251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2249);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3253);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3254);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3255);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4232);
									s.is_empty()
								}
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
			super::push(out, 1358);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2250);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3256);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4233);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4865);
											match s {
												[] => true,
												['l', 'a', 'v', 'e', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5256);
													s.is_empty()
												}
												['u', 'd', '-', 'n', 'o', 't', 'â', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5257);
													s.is_empty()
												}
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
							super::push(out, 3257);
							s.is_empty()
						}
						['p', 'a', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3259);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3260);
							match s {
								[] => true,
								['-', 'e', 'd', '-', 's', 'u', 'o', 's', 's', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4234);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4235);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4236);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 4866);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 5258);
													match s {
														[] => true,
														['c', '-', s @ ..] => super::t1::n5594(s, out),
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
									super::push(out, 4237);
									s.is_empty()
								}
								['é', s @ ..] => {
									super::push(out, 4238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2251);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 3261);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4239);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 4867);
											s.is_empty()
										}
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
					super::push(out, 2252);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2253);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3263);
							match s {
								[] => true,
								['m', '-', 'e', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4240);
									s.is_empty()
								}
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
			super::push(out, 1359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2254);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2255);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3265);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2256);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3266);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4242);
									match s {
										[] => true,
										['d', 'n', 'a', 'r', 'g', '-', 'e', 'r', 'è', 'i', 'r', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4868);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'c', 'o', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['f', 'u', 'b', '-', 'd', 'u', 'a', 'p', 'a', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2258);
					s.is_empty()
				}
				['u', 'o', 'r', s @ ..] => {
					super::push(out, 2259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1361);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['a', '-', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3267);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2262);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3269);
							s.is_empty()
						}
						['o', 'm', s @ ..] => {
							super::push(out, 3270);
							match s {
								[] => true,
								['o', s @ ..] => super::t1::n4390(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', 'a', s @ ..] => super::t1::n2320(s, out),
				['d', s @ ..] => super::t1::n2321(s, out),
				['g', s @ ..] => super::t1::n2322(s, out),
				['h', s @ ..] => super::t1::n2323(s, out),
				['n', s @ ..] => super::t1::n2324(s, out),
				['o', s @ ..] => super::t1::n2325(s, out),
				['p', s @ ..] => super::t1::n2326(s, out),
				['r', s @ ..] => super::t1::n2327(s, out),
				['s', s @ ..] => super::t1::n2328(s, out),
				['t', s @ ..] => super::t1::n2329(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n1394(s, out),
		['o', s @ ..] => super::t1::n1395(s, out),
		['p', s @ ..] => super::t1::n1396(s, out),
		['r', s @ ..] => super::t1::n1397(s, out),
		['u', s @ ..] => super::t1::n1398(s, out),
		['y', s @ ..] => super::t1::n1399(s, out),
		['è', s @ ..] => super::t1::n1400(s, out),
		['ê', s @ ..] => super::t1::n1401(s, out),
		['ô', s @ ..] => super::t1::n1402(s, out),
		_ => false,
	}
}

/// Words ending -me (3 syllables).
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1372);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3332);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4892);
											s.is_empty()
										}
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
					super::push(out, 2308);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3334);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2312);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'i', s @ ..] => {
			super::push(out, 1373);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 't', 'i', 'r', 'a', s @ ..] => {
			super::push(out, 1374);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1375);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['c', '-', 'u', 'a', '-', 'e', 's', 's', 'u', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'é', s @ ..] => {
					super::push(out, 2316);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3337);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1376);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1377);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2319);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3338);
							s.is_empty()
						}
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['-', 'e', 'm', 'o', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4289);
									s.is_empty()
								}
								['a', 'n', s @ ..] => {
									super::push(out, 4290);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4291);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 4893);
											match s {
												[] => true,
												['é', s @ ..] => {
													super::push(out, 5267);
													s.is_empty()
												}
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
					super::push(out, 2320);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3339);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4292);
									match s {
										[] => true,
										['e', 'g', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 4894);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2321);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3341);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4293);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4895);
											match s {
												[] => true,
												['g', 'a', 's', '\u{1}', s @ ..] => {
													super::push(out, 5268);
													s.is_empty()
												}
												['é', 'n', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 5269);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'u', 'o', 'j', '\u{1}', s @ ..] => {
											super::push(out, 4896);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'i', 't', 'n', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4294);
									s.is_empty()
								}
								['n', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4295);
									s.is_empty()
								}
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
			super::push(out, 1378);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2323);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'y', s @ ..] => {
					super::push(out, 2324);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2326);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3344);
							match s {
								[] => true,
								['r', 't', 's', s @ ..] => {
									super::push(out, 4296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2328);
					match s {
						[] => true,
						['o', 'h', s @ ..] => {
							super::push(out, 3346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2329);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 3347);
							s.is_empty()
						}
						['s', s @ ..] => {
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
			super::push(out, 1379);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3349);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4297);
									match s {
										[] => true,
										['e', 'g', '-', 't', 'a', 't', 'é', '\u{1}', s @ ..] => {
											super::push(out, 4897);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3350);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3351);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2332);
					match s {
						[] => true,
						['f', s @ ..] => super::t1::n3462(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n1411(s, out),
		['u', s @ ..] => {
			super::push(out, 1381);
			match s {
				[] => true,
				['l', 'p', s @ ..] => {
					super::push(out, 2335);
					match s {
						[] => true,
						['-', 'e', 't', 'ê', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t1::n1413(s, out),
		['è', s @ ..] => super::t1::n1414(s, out),
		_ => false,
	}
}

/// Words ending -ne (3 syllables).
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['-', 'n', 'e', 'i', 'c', 'a', 'm', 'r', 'a', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1384);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1385);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2343);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3364);
							match s {
								[] => true,
								['a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2344);
					match s {
						[] => true,
						['-', 'e', 'd', '-', 'c', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3365);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3366);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4315);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 4905);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3367);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3368);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'i', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2346);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3369);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3370);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 4318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2347);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2348);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2349);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3372);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 4319);
									s.is_empty()
								}
								['t', 't', s @ ..] => {
									super::push(out, 4320);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2350);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 3374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2351);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3375);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4321);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4906);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3376);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2352);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3378);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3379);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2353);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3380);
							match s {
								[] => true,
								['l', 'r', s @ ..] => {
									super::push(out, 4324);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 3381);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3382);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3383);
							s.is_empty()
						}
						['u', 'b', s @ ..] => {
							super::push(out, 3384);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2354);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2355);
					match s {
						[] => true,
						['o', 'l', 'i', s @ ..] => {
							super::push(out, 3386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3387);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1386);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2358);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3389);
							s.is_empty()
						}
						['m', s @ ..] => super::t1::n3501(s, out),
						['t', s @ ..] => super::t1::n3502(s, out),
						['v', s @ ..] => {
							super::push(out, 3392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => super::t1::n2420(s, out),
				['c', s @ ..] => super::t1::n2421(s, out),
				['d', 'i', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				['e', s @ ..] => super::t1::n2423(s, out),
				['f', s @ ..] => super::t1::n2424(s, out),
				['g', 'a', s @ ..] => super::t1::n2425(s, out),
				['h', s @ ..] => super::t1::n2426(s, out),
				['k', s @ ..] => super::t1::n2427(s, out),
				['l', s @ ..] => super::t1::n2428(s, out),
				['m', s @ ..] => super::t1::n2429(s, out),
				['n', 'n', s @ ..] => super::t1::n2430(s, out),
				['o', s @ ..] => super::t1::n2431(s, out),
				['p', s @ ..] => super::t1::n2432(s, out),
				['r', s @ ..] => super::t1::n2433(s, out),
				['s', s @ ..] => super::t1::n2434(s, out),
				['t', s @ ..] => super::t1::n2435(s, out),
				['u', s @ ..] => super::t1::n2436(s, out),
				['v', 'a', 'l', s @ ..] => super::t1::n2437(s, out),
				['x', 'o', 't', 'o', 'n', s @ ..] => super::t1::n2438(s, out),
				['z', s @ ..] => super::t1::n2439(s, out),
				['é', 't', 'o', 'r', 'p', 'o', s @ ..] => super::t1::n2440(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t1::n1419(s, out),
		['n', s @ ..] => super::t1::n1420(s, out),
		['o', s @ ..] => super::t1::n1421(s, out),
		['r', s @ ..] => super::t1::n1422(s, out),
		['u', s @ ..] => super::t1::n1423(s, out),
		['y', s @ ..] => super::t1::n1424(s, out),
		['â', s @ ..] => super::t1::n1425(s, out),
		['è', s @ ..] => super::t1::n1426(s, out),
		['ê', s @ ..] => super::t1::n1427(s, out),
		['ï', s @ ..] => super::t1::n1428(s, out),
		['ô', 'c', 'i', s @ ..] => super::t1::n1429(s, out),
		_ => false,
	}
}

/// Words ending -pe (3 syllables).
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['r', 't', 't', s @ ..] => {
					super::push(out, 2414);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'c', s @ ..] => {
			super::push(out, 1399);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1400);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2415);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3475);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4362);
									match s {
										[] => true,
										['u', 'a', 'e', 't', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4928);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4363);
									s.is_empty()
								}
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
			super::push(out, 1401);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2416);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3476);
							match s {
								[] => true,
								['o', 'd', s @ ..] => {
									super::push(out, 4364);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4929);
											s.is_empty()
										}
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
					super::push(out, 2417);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3477);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2418);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3479);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							super::push(out, 3480);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1402);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3481);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1404);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 2422);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3483);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4367);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 4368);
									s.is_empty()
								}
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
			super::push(out, 1405);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['-', 't', 'r', 'a', 'c', 'é', '\u{1}', s @ ..] => {
							super::push(out, 3484);
							s.is_empty()
						}
						['o', 'n', s @ ..] => {
							super::push(out, 3485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

