use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => super::t0::n1(s, out),
		['b', s @ ..] => super::t0::n2(s, out),
		['c', s @ ..] => super::t0::n3(s, out),
		['d', s @ ..] => super::t0::n4(s, out),
		['e', s @ ..] => super::t0::n5(s, out),
		['f', s @ ..] => super::t0::n6(s, out),
		['g', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 96);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 740);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2760);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 741);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2762);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => super::t0::n750(s, out),
						['n', s @ ..] => {
							super::push(out, 743);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 744);
							match s {
								[] => true,
								['r', 'i', 't', s @ ..] => super::t0::n2844(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						['v', s @ ..] => super::t0::n754(s, out),
						['z', s @ ..] => super::t0::n755(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n97(s, out),
				['i', s @ ..] => super::t0::n98(s, out),
				['l', s @ ..] => super::t0::n99(s, out),
				['n', s @ ..] => super::t0::n100(s, out),
				['o', s @ ..] => super::t0::n101(s, out),
				['r', s @ ..] => super::t0::n102(s, out),
				['u', s @ ..] => super::t0::n103(s, out),
				['y', s @ ..] => super::t0::n104(s, out),
				_ => false,
			}
		}
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
					super::push(out, 231);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1501);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4393);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1502);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4395);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1503);
							match s {
								[] => true,
								['i', 'e', 'n', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 10074);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10075);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 10076);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10077);
													s.is_empty()
												}
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
							super::push(out, 1504);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1506);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 232);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4397);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t0::n233(s, out),
				['e', s @ ..] => {
					super::push(out, 234);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n236(s, out),
				['l', s @ ..] => super::t0::n237(s, out),
				['n', s @ ..] => super::t0::n238(s, out),
				['o', s @ ..] => super::t0::n239(s, out),
				['r', s @ ..] => super::t0::n240(s, out),
				['s', s @ ..] => super::t0::n241(s, out),
				['u', s @ ..] => super::t0::n242(s, out),
				['y', s @ ..] => super::t0::n243(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n27(s, out),
		['d', s @ ..] => super::t0::n28(s, out),
		['e', s @ ..] => super::t0::n29(s, out),
		['f', s @ ..] => super::t0::n30(s, out),
		['g', s @ ..] => super::t0::n31(s, out),
		['h', s @ ..] => super::t0::n32(s, out),
		['i', s @ ..] => super::t0::n33(s, out),
		['l', s @ ..] => super::t0::n34(s, out),
		['m', s @ ..] => super::t0::n35(s, out),
		['n', s @ ..] => super::t0::n36(s, out),
		['p', s @ ..] => super::t0::n37(s, out),
		['q', s @ ..] => super::t0::n38(s, out),
		['r', s @ ..] => super::t0::n39(s, out),
		['s', s @ ..] => super::t1::n40(s, out),
		['t', s @ ..] => super::t1::n41(s, out),
		['u', s @ ..] => super::t1::n42(s, out),
		['v', s @ ..] => super::t1::n43(s, out),
		['x', s @ ..] => super::t1::n44(s, out),
		['z', s @ ..] => super::t1::n45(s, out),
		_ => false,
	}
}

/// Words ending -b.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 46);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 385);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1907);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1908);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4844);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 7974);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 7975);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 4845);
									match s {
										[] => true,
										['a', 'b', s @ ..] => {
											super::push(out, 7976);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1909);
							match s {
								[] => true,
								['l', 'l', 'i', 'f', 'o', 'r', 'm', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 4846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1910);
							match s {
								[] => true,
								['n', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 4847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 387);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 388);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1911);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1912);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1913);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 7977);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 10209);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 10210);
													s.is_empty()
												}
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
					super::push(out, 389);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1914);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4848);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4849);
									s.is_empty()
								}
								['u', 's', 't', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 1915);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 7978);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 7979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 't', s @ ..] => {
							super::push(out, 1916);
							match s {
								[] => true,
								['a', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 4851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 391);
					match s {
						[] => true,
						['t', 'i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4852);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t1::n13098(s, out),
														['s', s @ ..] => super::t1::n13099(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 10211);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 7980);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 4853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 1918);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4854);
									s.is_empty()
								}
								['t', 'u', 'l', s @ ..] => {
									super::push(out, 4855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'i', 't', s @ ..] => {
							super::push(out, 1919);
							match s {
								[] => true,
								['u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 393);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1920);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4857);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 7981);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 7982);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 4858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 394);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1921);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 7983);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 7984);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n398(s, out),
				['v', s @ ..] => super::t1::n399(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n47(s, out),
		['i', s @ ..] => super::t1::n48(s, out),
		['l', s @ ..] => super::t1::n49(s, out),
		['o', s @ ..] => super::t1::n50(s, out),
		['r', s @ ..] => super::t1::n51(s, out),
		['u', s @ ..] => super::t1::n52(s, out),
		['y', s @ ..] => super::t1::n53(s, out),
		_ => false,
	}
}

/// Words ending -c.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1977);
							match s {
								[] => true,
								['b', s @ ..] => super::t1::n5158(s, out),
								['n', s @ ..] => {
									super::push(out, 4894);
									s.is_empty()
								}
								['t', s @ ..] => super::t1::n5160(s, out),
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						['o', s @ ..] => super::t1::n2020(s, out),
						['u', s @ ..] => super::t1::n2021(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t1::n441(s, out),
				['e', s @ ..] => super::t1::n442(s, out),
				['i', s @ ..] => super::t1::n443(s, out),
				['l', s @ ..] => super::t1::n444(s, out),
				['m', s @ ..] => super::t1::n445(s, out),
				['n', s @ ..] => super::t1::n446(s, out),
				['p', s @ ..] => super::t1::n447(s, out),
				['r', s @ ..] => super::t1::n448(s, out),
				['s', s @ ..] => super::t1::n449(s, out),
				['t', s @ ..] => super::t1::n450(s, out),
				['u', s @ ..] => super::t1::n451(s, out),
				['v', s @ ..] => super::t1::n452(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n55(s, out),
		['h', s @ ..] => super::t1::n56(s, out),
		['i', s @ ..] => super::t1::n57(s, out),
		['l', s @ ..] => super::t1::n58(s, out),
		['n', s @ ..] => super::t1::n59(s, out),
		['o', s @ ..] => super::t1::n60(s, out),
		['r', s @ ..] => super::t1::n61(s, out),
		['u', s @ ..] => super::t1::n62(s, out),
		['y', s @ ..] => super::t1::n63(s, out),
		_ => false,
	}
}

/// Words ending -d.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 519);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2214);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5176);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5177);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'c', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 8322);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8323);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 10423);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10424);
													s.is_empty()
												}
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
					super::push(out, 524);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2217);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2218);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2219);
							match s {
								[] => true,
								['c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2221);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['r', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['u', 'r', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n65(s, out),
		['i', s @ ..] => super::t1::n66(s, out),
		['o', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5414);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5415);
									match s {
										[] => true,
										['r', 'a', s @ ..] => {
											match s {
												[] => true,
												['n', 's', s @ ..] => {
													super::push(out, 10512);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 10513);
													s.is_empty()
												}
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
					super::push(out, 568);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 569);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2361);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5416);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5417);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 5418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2362);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2363);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5419);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2364);
							match s {
								[] => true,
								['r', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2365);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5422);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5424);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 5425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2368);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8476);
											s.is_empty()
										}
										['t', 'a', s @ ..] => {
											match s {
												[] => true,
												['n', 's', s @ ..] => {
													super::push(out, 10514);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 10515);
													s.is_empty()
												}
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
							super::push(out, 2369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t1::n579(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['a', 'c', 'o', 'n', 'i', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n69(s, out),
		['y', s @ ..] => {
			super::push(out, 70);
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
		['b', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5440);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2379);
							match s {
								[] => true,
								['e', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2380);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5443);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 5444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2385);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5445);
									match s {
										[] => true,
										['c', 't', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 8486);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5446);
									match s {
										[] => true,
										['n', 'a', 'n', 's', '\u{1}', s @ ..] => {
											super::push(out, 8487);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5448);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5449);
									s.is_empty()
								}
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
			super::push(out, 73);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 601);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5450);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5451);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2387);
							match s {
								[] => true,
								['i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 5453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2388);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5454);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2389);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2391);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5456);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 8488);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 8489);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5457);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2392);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2393);
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5459);
									s.is_empty()
								}
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
			super::push(out, 74);
			match s {
				[] => true,
				['f', s @ ..] => super::t1::n611(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n75(s, out),
		['i', s @ ..] => super::t1::n76(s, out),
		['l', s @ ..] => super::t1::n77(s, out),
		['m', s @ ..] => super::t1::n78(s, out),
		['n', s @ ..] => super::t1::n79(s, out),
		['o', s @ ..] => {
			super::push(out, 80);
			s.is_empty()
		}
		['p', s @ ..] => super::t1::n81(s, out),
		['q', s @ ..] => {
			super::push(out, 82);
			s.is_empty()
		}
		['r', s @ ..] => super::t1::n83(s, out),
		['s', s @ ..] => super::t1::n84(s, out),
		['t', s @ ..] => super::t1::n85(s, out),
		['u', s @ ..] => super::t1::n86(s, out),
		['v', s @ ..] => super::t1::n87(s, out),
		['x', s @ ..] => super::t1::n88(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 680);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2605);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5654);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 5655);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 5656);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5657);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2608);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5659);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5660);
									match s {
										[] => true,
										['c', 'a', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 10604);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 10605);
													s.is_empty()
												}
												['t', s @ ..] => {
													match s {
														[] => true,
														['o', s @ ..] => super::t1::n13406(s, out),
														['u', s @ ..] => super::t1::n13407(s, out),
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
							super::push(out, 2609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 681);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2610);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5661);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5662);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 8642);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 8643);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2611);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5663);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5664);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 8644);
											match s {
												[] => true,
												['r', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 10606);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 8645);
											match s {
												[] => true,
												['r', 'o', 's', 'i', 's', s @ ..] => {
													super::push(out, 10607);
													s.is_empty()
												}
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
							super::push(out, 2612);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5665);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 8646);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 8647);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						['l', s @ ..] => super::t1::n2684(s, out),
						['s', s @ ..] => super::t1::n2685(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n692(s, out),
				['n', s @ ..] => super::t1::n693(s, out),
				['r', s @ ..] => super::t1::n694(s, out),
				['s', s @ ..] => super::t1::n695(s, out),
				['t', s @ ..] => super::t1::n696(s, out),
				['u', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['v', s @ ..] => super::t1::n698(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n90(s, out),
		['i', s @ ..] => super::t1::n91(s, out),
		['l', s @ ..] => super::t1::n92(s, out),
		['o', s @ ..] => super::t1::n93(s, out),
		['r', s @ ..] => super::t1::n94(s, out),
		['u', s @ ..] => super::t1::n95(s, out),
		_ => false,
	}
}

/// Words ending -lag.
pub(crate) fn n750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2764);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2765);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2766);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5796);
					match s {
						[] => true,
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2767);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2768);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8724);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tirrag.
pub(crate) fn n2844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2770);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 8726);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vag.
pub(crate) fn n754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 746);
	match s {
		[] => true,
		['i', 's', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5798);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5799);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zag.
pub(crate) fn n755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 747);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2771);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2772);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eg.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 748);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 2773);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8728);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8729);
									s.is_empty()
								}
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
			super::push(out, 750);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					super::push(out, 2774);
					match s {
						[] => true,
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2775);
					match s {
						[] => true,
						['r', 'a', 'l', s @ ..] => {
							super::push(out, 5801);
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10646);
											match s {
												[] => true,
												['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 2776);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5802);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2777);
					match s {
						[] => true,
						['i', 'l', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8730);
									s.is_empty()
								}
								['t', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2778);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5804);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 752);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5806);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10647);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10648);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5808);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8732);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8733);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10649);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11651);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11652);
													s.is_empty()
												}
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
							super::push(out, 5809);
							match s {
								[] => true,
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ig.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['g', 'i', 'v', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2779);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lg.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2780);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2781);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2782);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2783);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['s', 'c', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5811);
							s.is_empty()
						}
						['s', s @ ..] => {
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
			super::push(out, 758);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5813);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2786);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8735);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 10650);
											match s {
												[] => true,
												['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 11653);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 8736);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8737);
									match s {
										[] => true,
										['s', 'i', 's', s @ ..] => {
											super::push(out, 10651);
											s.is_empty()
										}
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
					super::push(out, 2788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2789);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2790);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5815);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8738);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8739);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8740);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 11654);
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													super::push(out, 11655);
													s.is_empty()
												}
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
							super::push(out, 5816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ng.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 760);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 761);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 762);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -og.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	s.is_empty()
}

/// Words ending -rg.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2791);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2792);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2793);
					s.is_empty()
				}
				['i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2794);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5817);
							match s {
								[] => true,
								['n', 'i', 'c', 'o', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2795);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5819);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 10652);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10653);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5820);
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2796);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2797);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5822);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8742);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 8743);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8744);
									match s {
										[] => true,
										['s', 'i', 's', s @ ..] => {
											super::push(out, 10654);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2798);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5824);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8745);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8746);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5825);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8747);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8748);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5826);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 764);
			match s {
				[] => true,
				['g', 'a', 't', s @ ..] => {
					super::push(out, 2799);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8749);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8750);
									s.is_empty()
								}
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
			super::push(out, 765);
			match s {
				[] => true,
				['s', 'e', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5827);
							s.is_empty()
						}
						['o', 'c', 'h', 'r', 'o', 'm', 'o', 'g', 'e', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5828);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5829);
							match s {
								[] => true,
								['c', 'e', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 8751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 766);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 767);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 768);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ug.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 769);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['a', 'n', 'i', s @ ..] => {
					super::push(out, 2800);
					match s {
						[] => true,
						['c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5832);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5833);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yg.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 772);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 773);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5835);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5836);
							s.is_empty()
						}
						_ => false,
					}
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
			super::push(out, 105);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2801);
							match s {
								[] => true,
								['n', 'u', s @ ..] => {
									super::push(out, 5837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5838);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2803);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5840);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 5841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2804);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2805);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2806);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2809);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5842);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['c', s @ ..] => {
											super::push(out, 8752);
											match s {
												[] => true,
												['e', 'n', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2811);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2812);
							match s {
								[] => true,
								['a', 't', 'u', 'r', s @ ..] => {
									super::push(out, 5844);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 8753);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2813);
							match s {
								[] => true,
								['e', 't', 'i', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 8754);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 8755);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['i', 'p', 't', 'e', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2815);
							s.is_empty()
						}
						['t', 'a', 'p', 'h', 'y', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2817);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5846);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 8756);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2818);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2819);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5848);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 5849);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 8757);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5850);
									s.is_empty()
								}
								['t', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2822);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5852);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5853);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2826);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n107(s, out),
		['o', s @ ..] => super::t1::n108(s, out),
		['u', s @ ..] => super::t1::n109(s, out),
		['y', s @ ..] => super::t1::n110(s, out),
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2863);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2864);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5878);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 8782);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 8783);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 8784);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2865);
							match s {
								[] => true,
								['l', 'a', 't', s @ ..] => {
									super::push(out, 5880);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8785);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 8786);
											s.is_empty()
										}
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
					super::push(out, 810);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2866);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5881);
									s.is_empty()
								}
								['i', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 5882);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 814);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2868);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 112);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2870);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							super::push(out, 2872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2873);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 821);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5884);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 8787);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 823);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2876);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 8788);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 8789);
											match s {
												[] => true,
												['u', s @ ..] => {
													match s {
														[] => true,
														['r', s @ ..] => super::t1::n13432(s, out),
														['s', s @ ..] => super::t1::n13433(s, out),
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
		['g', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2877);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5886);
									s.is_empty()
								}
								['v', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 8790);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 8791);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2878);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5887);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2879);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5889);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 10666);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10667);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => super::t1::n2972(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 117);
			s.is_empty()
		}
		['l', s @ ..] => super::t1::n118(s, out),
		['m', s @ ..] => super::t1::n119(s, out),
		['n', s @ ..] => super::t1::n120(s, out),
		['o', s @ ..] => super::t1::n121(s, out),
		['p', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		['r', s @ ..] => super::t1::n123(s, out),
		['s', s @ ..] => super::t1::n124(s, out),
		['t', s @ ..] => super::t1::n125(s, out),
		['u', s @ ..] => super::t1::n126(s, out),
		['v', s @ ..] => {
			super::push(out, 127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['i', 'l', 'o', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['n', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['e', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3042);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6225);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6226);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 9027);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 9028);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3043);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3044);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 6227);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3045);
							match s {
								[] => true,
								['a', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 6229);
									s.is_empty()
								}
								['o', 's', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3046);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6231);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9029);
											s.is_empty()
										}
										['i', 'o', s @ ..] => {
											super::push(out, 9030);
											match s {
												[] => true,
												['s', s @ ..] => {
													match s {
														[] => true,
														['i', s @ ..] => super::t1::n13498(s, out),
														['u', s @ ..] => super::t1::n13499(s, out),
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
									super::push(out, 6232);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3047);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3048);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => super::t2::n893(s, out),
				['e', s @ ..] => super::t2::n894(s, out),
				['i', s @ ..] => {
					super::push(out, 884);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3055);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n897(s, out),
				['o', s @ ..] => super::t2::n898(s, out),
				['p', s @ ..] => super::t2::n899(s, out),
				['q', s @ ..] => super::t2::n900(s, out),
				['r', s @ ..] => super::t2::n901(s, out),
				['s', s @ ..] => super::t2::n902(s, out),
				['t', s @ ..] => super::t2::n903(s, out),
				['u', s @ ..] => super::t2::n904(s, out),
				['v', s @ ..] => super::t2::n905(s, out),
				['x', s @ ..] => super::t2::n906(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n131(s, out),
		['i', s @ ..] => super::t2::n132(s, out),
		['o', s @ ..] => super::t2::n133(s, out),
		['u', s @ ..] => super::t2::n134(s, out),
		['y', s @ ..] => super::t2::n135(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3158);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6355);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 6356);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6357);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9108);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 9109);
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', 'u', 's', '\u{1}', s @ ..] => super::t2::n13522(s, out),
														['s', '\u{1}', s @ ..] => super::t2::n13523(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', 'u', 's', '\u{1}', s @ ..] => super::t2::n13524(s, out),
														['s', '\u{1}', s @ ..] => super::t2::n13525(s, out),
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
									super::push(out, 6358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3159);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3160);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3161);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6359);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t2::n3276(s, out),
						['u', s @ ..] => super::t2::n3277(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t2::n952(s, out),
				['e', s @ ..] => super::t2::n953(s, out),
				['g', s @ ..] => super::t2::n954(s, out),
				['i', s @ ..] => super::t2::n955(s, out),
				['l', s @ ..] => super::t2::n956(s, out),
				['m', s @ ..] => super::t2::n957(s, out),
				['n', s @ ..] => super::t2::n958(s, out),
				['o', s @ ..] => super::t2::n959(s, out),
				['r', s @ ..] => super::t2::n960(s, out),
				['s', s @ ..] => super::t2::n961(s, out),
				['t', s @ ..] => super::t2::n962(s, out),
				['u', s @ ..] => super::t2::n963(s, out),
				['x', s @ ..] => super::t2::n964(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n137(s, out),
		['i', s @ ..] => super::t2::n138(s, out),
		['o', s @ ..] => super::t2::n139(s, out),
		['u', s @ ..] => super::t2::n140(s, out),
		['y', s @ ..] => super::t2::n141(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 999);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 6508);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6509);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1003);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3292);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3293);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3294);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 9189);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 9190);
											s.is_empty()
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 10908);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10909);
													s.is_empty()
												}
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
					super::push(out, 1004);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3295);
							match s {
								[] => true,
								['r', 'a', 'e', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 6511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1005);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3296);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6512);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 6513);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 9191);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 9192);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 10910);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10911);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6514);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6515);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 9193);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3297);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6516);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6517);
									match s {
										[] => true,
										['n', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 9194);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 6518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3298);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6519);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 3299);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 6521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3300);
							match s {
								[] => true,
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 6522);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3302);
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9195);
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9196);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3303);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6523);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n143(s, out),
		['i', s @ ..] => super::t2::n144(s, out),
		['o', s @ ..] => super::t2::n145(s, out),
		['u', s @ ..] => super::t2::n146(s, out),
		['y', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3358);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3359);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3360);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3361);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6571);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9224);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 9225);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6572);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 3363);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 9226);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 9227);
											s.is_empty()
										}
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
					super::push(out, 1062);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3364);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3365);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3366);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3367);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6574);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 9228);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 9229);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 6575);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6576);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9230);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 9231);
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 10924);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10925);
													s.is_empty()
												}
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
				['e', s @ ..] => super::t2::n1074(s, out),
				['f', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['u', 's', s @ ..] => super::t2::n3507(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n1076(s, out),
				['l', s @ ..] => super::t2::n1077(s, out),
				['m', s @ ..] => {
					super::push(out, 1067);
					s.is_empty()
				}
				['n', s @ ..] => super::t2::n1079(s, out),
				['o', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['r', s @ ..] => super::t2::n1082(s, out),
				['s', s @ ..] => super::t2::n1083(s, out),
				['t', s @ ..] => super::t2::n1084(s, out),
				['u', s @ ..] => super::t2::n1085(s, out),
				['v', s @ ..] => super::t2::n1086(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n149(s, out),
		['d', s @ ..] => super::t2::n150(s, out),
		['e', s @ ..] => super::t2::n151(s, out),
		['f', s @ ..] => super::t2::n152(s, out),
		['l', s @ ..] => super::t2::n153(s, out),
		['m', s @ ..] => super::t2::n154(s, out),
		['n', s @ ..] => super::t2::n155(s, out),
		['p', s @ ..] => super::t2::n156(s, out),
		['r', s @ ..] => super::t2::n157(s, out),
		['s', s @ ..] => super::t2::n158(s, out),
		['t', s @ ..] => super::t2::n159(s, out),
		['v', s @ ..] => super::t2::n160(s, out),
		['x', s @ ..] => super::t2::n161(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['u', 'l', 'a', 't', s @ ..] => {
							super::push(out, 3486);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6653);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3487);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 6655);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6656);
									s.is_empty()
								}
								['t', s @ ..] => super::t2::n7160(s, out),
								_ => false,
							}
						}
						['h', 'y', 'p', 'h', 'y', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => super::t2::n3630(s, out),
						['i', s @ ..] => super::t2::n3631(s, out),
						['t', s @ ..] => super::t2::n3632(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t2::n1140(s, out),
				['g', s @ ..] => super::t2::n1141(s, out),
				['k', s @ ..] => super::t2::n1142(s, out),
				['l', s @ ..] => super::t2::n1143(s, out),
				['m', s @ ..] => super::t2::n1144(s, out),
				['n', s @ ..] => super::t2::n1145(s, out),
				['p', s @ ..] => super::t2::n1146(s, out),
				['r', s @ ..] => super::t2::n1147(s, out),
				['s', s @ ..] => super::t2::n1148(s, out),
				['t', s @ ..] => super::t2::n1149(s, out),
				['u', s @ ..] => super::t2::n1150(s, out),
				['v', s @ ..] => super::t2::n1151(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n163(s, out),
		['h', s @ ..] => super::t2::n164(s, out),
		['i', s @ ..] => super::t2::n165(s, out),
		['l', s @ ..] => super::t2::n166(s, out),
		['n', s @ ..] => super::t2::n167(s, out),
		['o', s @ ..] => super::t2::n168(s, out),
		['r', s @ ..] => super::t2::n169(s, out),
		['s', s @ ..] => super::t2::n170(s, out),
		['t', s @ ..] => super::t2::n171(s, out),
		['u', s @ ..] => super::t2::n172(s, out),
		['y', s @ ..] => super::t2::n173(s, out),
		_ => false,
	}
}

/// Words ending -q.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1201);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3717);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9627);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 11125);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 11126);
													s.is_empty()
												}
												['t', 'a', s @ ..] => {
													super::push(out, 11127);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 9628);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11128);
													match s {
														[] => true,
														['e', s @ ..] => super::t2::n13756(s, out),
														['u', s @ ..] => super::t2::n13757(s, out),
														_ => false,
													}
												}
												['f', s @ ..] => {
													super::push(out, 11129);
													s.is_empty()
												}
												['g', s @ ..] => {
													super::push(out, 11130);
													match s {
														[] => true,
														['a', 'r', s @ ..] => super::t2::n13758(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 9629);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3718);
							match s {
								[] => true,
								['s', 't', 'u', s @ ..] => {
									super::push(out, 7005);
									match s {
										[] => true,
										['o', 's', 'i', 's', s @ ..] => {
											super::push(out, 9630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3719);
							s.is_empty()
						}
						['m', 'm', 'a', 'g', 'n', 'u', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'u', 'm', 'q', 'u', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3720);
							match s {
								[] => true,
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['l', 'u', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['c', 'u', 'm', 'q', 'u', 'e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['c', 'u', 'm', 'q', 'u', 'e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 3721);
							match s {
								[] => true,
								['t', 'a', 'l', 'i', 't', s @ ..] => {
									super::push(out, 7006);
									match s {
										[] => true,
										['i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9631);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3722);
							match s {
								[] => true,
								['s', 'a', 'b', s @ ..] => {
									super::push(out, 7007);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 9632);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 9633);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3724);
							match s {
								[] => true,
								['n', 's', 'l', 'a', 'n', 'd', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3725);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 7009);
									match s {
										[] => true,
										['u', 'e', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11131);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11132);
													s.is_empty()
												}
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
							super::push(out, 3726);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n1215(s, out),
				['o', s @ ..] => super::t2::n1216(s, out),
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
			super::push(out, 175);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['i', 'o', 's', 'u', 'l', s @ ..] => {
							super::push(out, 3732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1207);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3733);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3734);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7015);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9646);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 9647);
											match s {
												[] => true,
												['u', 's', s @ ..] => {
													super::push(out, 11135);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 7016);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1209);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['o', 's', 'i', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3736);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3738);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7018);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 7019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3739);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3740);
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									super::push(out, 7020);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11136);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11137);
													s.is_empty()
												}
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
					super::push(out, 1213);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3741);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3742);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3744);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3745);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 7021);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3746);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3748);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n176(s, out),
		['h', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3831);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1237);
					match s {
						[] => true,
						['c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3834);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7164);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3835);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 9731);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 11190);
													match s {
														[] => true,
														['s', '\u{1}', s @ ..] => super::t2::n13789(s, out),
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
									super::push(out, 7166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3836);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['a', s @ ..] => super::t2::n3996(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				['p', s @ ..] => super::t2::n1254(s, out),
				['s', s @ ..] => super::t2::n1255(s, out),
				['t', s @ ..] => {
					super::push(out, 1244);
					s.is_empty()
				}
				['v', s @ ..] => super::t2::n1257(s, out),
				['x', s @ ..] => super::t2::n1258(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n179(s, out),
		['u', s @ ..] => super::t2::n180(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1269);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3872);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
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
									super::push(out, 7206);
									match s {
										[] => true,
										['m', 'e', 'n', 't', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 9743);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 9744);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 9745);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7207);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7208);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 9746);
											s.is_empty()
										}
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
					super::push(out, 1271);
					match s {
						[] => true,
						['v', 'i', s @ ..] => {
							super::push(out, 3873);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 7209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1272);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3874);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3875);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7211);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7212);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3876);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 7214);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1273);
					match s {
						[] => true,
						['a', 'r', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3878);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7216);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 7217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3879);
							s.is_empty()
						}
						['i', s @ ..] => super::t2::n4048(s, out),
						['l', s @ ..] => super::t2::n4049(s, out),
						['o', s @ ..] => super::t2::n4050(s, out),
						['s', s @ ..] => super::t2::n4051(s, out),
						['t', s @ ..] => super::t2::n4052(s, out),
						['u', s @ ..] => super::t2::n4053(s, out),
						['v', s @ ..] => super::t2::n4054(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t2::n1287(s, out),
				['n', s @ ..] => super::t2::n1288(s, out),
				['p', s @ ..] => super::t2::n1289(s, out),
				['r', s @ ..] => super::t2::n1290(s, out),
				['t', s @ ..] => super::t2::n1291(s, out),
				['u', s @ ..] => super::t2::n1292(s, out),
				['v', s @ ..] => super::t2::n1293(s, out),
				['x', s @ ..] => super::t2::n1294(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n182(s, out),
		['e', s @ ..] => super::t2::n183(s, out),
		['i', s @ ..] => super::t2::n184(s, out),
		['l', s @ ..] => super::t2::n185(s, out),
		['m', s @ ..] => super::t2::n186(s, out),
		['o', s @ ..] => super::t2::n187(s, out),
		['p', s @ ..] => super::t2::n188(s, out),
		['q', s @ ..] => super::t2::n189(s, out),
		['t', s @ ..] => super::t3::n190(s, out),
		['u', s @ ..] => super::t3::n191(s, out),
		['y', s @ ..] => super::t3::n192(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1364);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4125);
							match s {
								[] => true,
								['c', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4126);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 7479);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7480);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7481);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7482);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4127);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1365);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4129);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4130);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1366);
					match s {
						[] => true,
						['t', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1367);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1368);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1369);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4132);
							match s {
								[] => true,
								['r', 'i', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 9927);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 9928);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4133);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7484);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9929);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 9930);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', s @ ..] => {
					super::push(out, 1370);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1372);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4134);
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									super::push(out, 7486);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11299);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11300);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1375);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 4136);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 9931);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 9932);
											s.is_empty()
										}
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
			super::push(out, 194);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1377);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4137);
							s.is_empty()
						}
						['t', s @ ..] => super::t3::n4318(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t3::n1392(s, out),
				['m', s @ ..] => super::t3::n1393(s, out),
				['n', s @ ..] => super::t3::n1394(s, out),
				['p', s @ ..] => super::t3::n1395(s, out),
				['r', s @ ..] => super::t3::n1396(s, out),
				['s', s @ ..] => super::t3::n1397(s, out),
				['t', s @ ..] => super::t3::n1398(s, out),
				['x', s @ ..] => super::t3::n1399(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t3::n195(s, out),
		['i', s @ ..] => super::t3::n196(s, out),
		['o', s @ ..] => super::t3::n197(s, out),
		['r', s @ ..] => super::t3::n198(s, out),
		['s', s @ ..] => super::t3::n199(s, out),
		['u', s @ ..] => super::t3::n200(s, out),
		['w', s @ ..] => super::t3::n201(s, out),
		['y', s @ ..] => super::t3::n202(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7569);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9984);
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11334);
													match s {
														[] => true,
														['u', 's', '\u{1}', s @ ..] => super::t3::n13870(s, out),
														_ => false,
													}
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'o', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['c', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4244);
							s.is_empty()
						}
						['q', 'u', 'i', 't', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['a', 'i', 'n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7570);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7571);
									s.is_empty()
								}
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
			super::push(out, 205);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4246);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'o', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4248);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4249);
							match s {
								[] => true,
								['n', 'i', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 7572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4250);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['l', 'a', 't', s @ ..] => {
							super::push(out, 4251);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 9985);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 9986);
											s.is_empty()
										}
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
			super::push(out, 208);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4252);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7574);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9987);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 9988);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 11335);
													s.is_empty()
												}
												['u', 's', s @ ..] => {
													super::push(out, 11336);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 7575);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7576);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7577);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9989);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 9990);
											s.is_empty()
										}
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
					super::push(out, 1426);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1429);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4253);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7578);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4254);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t3::n1444(s, out),
				['g', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['i', s @ ..] => super::t3::n1446(s, out),
				['o', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		['r', s @ ..] => super::t3::n211(s, out),
		['s', s @ ..] => super::t3::n212(s, out),
		['t', s @ ..] => super::t3::n213(s, out),
		['v', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
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
			super::push(out, 216);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4276);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7595);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 4277);
							match s {
								[] => true,
								['a', 'r', 'i', 'c', s @ ..] => {
									super::push(out, 7597);
									match s {
										[] => true,
										['i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9997);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9998);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 9999);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4278);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4279);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4280);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7598);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4281);
							match s {
								[] => true,
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7600);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4283);
							match s {
								[] => true,
								['b', 'u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7601);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7602);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7603);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4284);
							match s {
								[] => true,
								['n', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 7604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1456);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4286);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7605);
									match s {
										[] => true,
										['i', 'c', s @ ..] => {
											match s {
												[] => true,
												['e', 'n', 'd', s @ ..] => {
													super::push(out, 11340);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11341);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 7606);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7607);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7608);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4287);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4288);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 10000);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 10001);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10002);
											match s {
												[] => true,
												['u', s @ ..] => {
													match s {
														[] => true,
														['r', s @ ..] => super::t3::n13871(s, out),
														['s', s @ ..] => super::t3::n13872(s, out),
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
									super::push(out, 7610);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1457);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4289);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4290);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4291);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4292);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1458);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4294);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4295);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n1473(s, out),
				['s', s @ ..] => super::t3::n1474(s, out),
				['t', s @ ..] => {
					super::push(out, 1461);
					match s {
						[] => true,
						['a', s @ ..] => super::t3::n4494(s, out),
						['i', s @ ..] => super::t3::n4495(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n217(s, out),
		['i', s @ ..] => super::t3::n218(s, out),
		['o', s @ ..] => super::t3::n219(s, out),
		['u', s @ ..] => super::t3::n220(s, out),
		_ => false,
	}
}

/// Words ending -w.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['i', 's', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['c', 'o', 'n', 's', 'i', 'n', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 224);
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
			super::push(out, 225);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 4392);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10072);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10073);
											s.is_empty()
										}
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
					super::push(out, 1496);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['e', 'c', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['n', 'a', 'r', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['g', 'o', 'm', 'a', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dba.
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 233);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1509);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4399);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1510);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7718);
							match s {
								[] => true,
								['t', 'o', s @ ..] => {
									super::push(out, 10078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7719);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7720);
							s.is_empty()
						}
						['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1511);
			match s {
				[] => true,
				['m', 'i', 'n', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 4402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7722);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7723);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4403);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iba.
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1513);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1514);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4404);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1515);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1516);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1517);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1518);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4406);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4408);
					match s {
						[] => true,
						['i', 'c', 'a', 't', 'i', s @ ..] => {
							super::push(out, 7725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4409);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4410);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7726);
							match s {
								[] => true,
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 7727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4411);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7728);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lba.
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1520);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4412);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 4413);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4414);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7730);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10080);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10081);
									s.is_empty()
								}
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
			super::push(out, 1521);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1522);
			match s {
				[] => true,
				['g', 'u', 'r', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7731);
							match s {
								[] => true,
								['e', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10082);
									s.is_empty()
								}
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10083);
									s.is_empty()
								}
								['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1523);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1524);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4415);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4416);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7733);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10085);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10086);
									s.is_empty()
								}
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

/// Words ending -nba.
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1525);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1526);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1527);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4417);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1528);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4419);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4420);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4421);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oba.
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4422);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7734);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1530);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1531);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4424);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7736);
							s.is_empty()
						}
						['g', 'i', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10087);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7737);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4425);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rba.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1532);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4426);
					s.is_empty()
				}
				['s', 'u', 's', s @ ..] => {
					super::push(out, 4427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4428);
					match s {
						[] => true,
						['u', 'n', 't', 'i', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10089);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10090);
									s.is_empty()
								}
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
			super::push(out, 1534);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4429);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4430);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4431);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7739);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1536);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sba.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1537);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4432);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7741);
							match s {
								[] => true,
								['e', 'n', 's', s @ ..] => {
									super::push(out, 10091);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4433);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 7743);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7744);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 10092);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 10093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1538);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1539);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1540);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4436);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10094);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10095);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11363);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 11364);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11365);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 7745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1541);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1542);
			match s {
				[] => true,
				['r', 'u', s @ ..] => {
					super::push(out, 4438);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7746);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11366);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11367);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1543);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4439);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4440);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uba.
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1544);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1545);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yba.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1547);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1548);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ca.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4442);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4443);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4444);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7748);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7749);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4445);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1554);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4447);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10096);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 10097);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10098);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 10099);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4448);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7750);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7751);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 7752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4449);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7753);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7755);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1555);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4450);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10101);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10102);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10103);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10104);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4451);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10105);
											match s {
												[] => true,
												['n', 's', s @ ..] => {
													super::push(out, 11368);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11369);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 10106);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 7757);
									s.is_empty()
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
				['r', s @ ..] => {
					super::push(out, 1557);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7758);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10107);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1558);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4453);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4454);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4455);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7759);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7760);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4456);
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									super::push(out, 7762);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10109);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10110);
											s.is_empty()
										}
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
		['e', s @ ..] => super::t3::n246(s, out),
		['h', s @ ..] => super::t3::n247(s, out),
		['i', s @ ..] => super::t3::n248(s, out),
		['m', s @ ..] => super::t3::n249(s, out),
		['o', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		['q', s @ ..] => super::t3::n251(s, out),
		['r', s @ ..] => super::t3::n252(s, out),
		['t', s @ ..] => super::t3::n253(s, out),
		['u', s @ ..] => super::t3::n254(s, out),
		_ => false,
	}
}

/// Words ending -da.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 4485);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10114);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10115);
											s.is_empty()
										}
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
					super::push(out, 1585);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7775);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4486);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4488);
							match s {
								[] => true,
								['e', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 7777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4489);
							match s {
								[] => true,
								['m', 'a', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 7778);
									s.is_empty()
								}
								['r', 'a', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 7779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4490);
							match s {
								[] => true,
								['n', 'a', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 7780);
									s.is_empty()
								}
								['v', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 7781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7782);
									s.is_empty()
								}
								['t', 'u', 's', s @ ..] => {
									super::push(out, 7783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4491);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7784);
									s.is_empty()
								}
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
			super::push(out, 258);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4492);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4493);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7785);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10116);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 10117);
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', 'u', 's', '\u{1}', s @ ..] => super::t3::n13890(s, out),
														['s', '\u{1}', s @ ..] => super::t3::n13891(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', 'u', 's', '\u{1}', s @ ..] => super::t3::n13892(s, out),
														['s', '\u{1}', s @ ..] => super::t3::n13893(s, out),
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
					super::push(out, 1594);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4494);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7786);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4496);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4497);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7788);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7789);
									s.is_empty()
								}
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
			super::push(out, 259);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1602);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1603);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['a', s @ ..] => super::t3::n1624(s, out),
				['e', s @ ..] => super::t3::n1625(s, out),
				['i', s @ ..] => super::t3::n1626(s, out),
				['l', s @ ..] => super::t3::n1627(s, out),
				['r', s @ ..] => super::t3::n1628(s, out),
				['u', s @ ..] => super::t3::n1629(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t3::n261(s, out),
		['h', s @ ..] => super::t3::n262(s, out),
		['i', s @ ..] => super::t3::n263(s, out),
		['l', s @ ..] => super::t3::n264(s, out),
		['m', s @ ..] => super::t3::n265(s, out),
		['n', s @ ..] => super::t3::n266(s, out),
		['o', s @ ..] => super::t3::n267(s, out),
		['p', s @ ..] => super::t3::n268(s, out),
		['q', s @ ..] => super::t3::n269(s, out),
		['r', s @ ..] => super::t3::n270(s, out),
		['s', s @ ..] => super::t3::n271(s, out),
		['t', s @ ..] => super::t3::n272(s, out),
		['u', s @ ..] => super::t3::n273(s, out),
		['v', s @ ..] => super::t3::n274(s, out),
		['z', s @ ..] => super::t3::n275(s, out),
		_ => false,
	}
}

/// Words ending -ea.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 276);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 277);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4602);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1679);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4604);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7836);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4605);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 11380);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11381);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 10156);
											s.is_empty()
										}
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
					super::push(out, 1680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1681);
					match s {
						[] => true,
						['s', 'c', 'e', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 4606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'g', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4607);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7838);
									s.is_empty()
								}
								['l', 'i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10157);
											match s {
												[] => true,
												['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7839);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7840);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10158);
											match s {
												[] => true,
												['r', 'i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
													super::push(out, 11382);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 10159);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11383);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11384);
													s.is_empty()
												}
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
							super::push(out, 4608);
							match s {
								[] => true,
								['p', 'a', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', 'u', 's', '\u{1}', s @ ..] => super::t3::n13898(s, out),
														['s', '\u{1}', s @ ..] => super::t3::n13899(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', 'u', 's', '\u{1}', s @ ..] => super::t3::n13900(s, out),
														['s', '\u{1}', s @ ..] => super::t3::n13901(s, out),
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
		['r', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4609);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4611);
							s.is_empty()
						}
						['n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4612);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4613);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4615);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4617);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4618);
							match s {
								[] => true,
								['n', 'o', 's', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10160);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10161);
											s.is_empty()
										}
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
		['s', s @ ..] => super::t3::n283(s, out),
		['t', s @ ..] => {
			super::push(out, 284);
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
		['a', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1690);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4621);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4622);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 4623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4624);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4625);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4626);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7843);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7844);
									match s {
										[] => true,
										['r', 'a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t3::n13902(s, out),
														['s', s @ ..] => super::t3::n13903(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', s @ ..] => super::t3::n13904(s, out),
														['s', s @ ..] => super::t3::n13905(s, out),
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
							super::push(out, 4627);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4628);
							s.is_empty()
						}
						['x', 'u', 'r', s @ ..] => {
							super::push(out, 4629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4630);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7845);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4631);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4632);
							match s {
								[] => true,
								['c', 't', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10167);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10168);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11388);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11389);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['g', 'e', 'n', 's', s @ ..] => {
									super::push(out, 7847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4633);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7848);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 7849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1694);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1695);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 4634);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7850);
									s.is_empty()
								}
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
			super::push(out, 288);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4635);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 290);
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
		['g', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1699);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4637);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1701);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4639);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4641);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4642);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 4643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1704);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4644);
							s.is_empty()
						}
						['t', 'u', 's', s @ ..] => {
							super::push(out, 4645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4646);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ha.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['e', 'n', 'e', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ia.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				['s', s @ ..] => {
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
		['b', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7851);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4648);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7853);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4649);
							s.is_empty()
						}
						['n', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7855);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7856);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['p', 'u', s @ ..] => {
							super::push(out, 4651);
							match s {
								[] => true,
								['n', 'c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4652);
							match s {
								[] => true,
								['i', 'c', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7858);
									s.is_empty()
								}
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
			super::push(out, 299);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1720);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7859);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1721);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4654);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4655);
							match s {
								[] => true,
								['e', 'n', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1724);
					match s {
						[] => true,
						['r', 'i', 'n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7861);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7863);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1728);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1729);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7864);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7865);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1731);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['b', s @ ..] => super::t3::n4889(s, out),
						['c', s @ ..] => {
							super::push(out, 4658);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4659);
							s.is_empty()
						}
						['t', s @ ..] => super::t3::n4892(s, out),
						['u', s @ ..] => super::t3::n4893(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n1760(s, out),
				['i', s @ ..] => super::t3::n1761(s, out),
				['o', s @ ..] => super::t3::n1762(s, out),
				['u', s @ ..] => super::t3::n1763(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t3::n304(s, out),
		['s', s @ ..] => super::t3::n305(s, out),
		['t', s @ ..] => super::t3::n306(s, out),
		['u', s @ ..] => super::t3::n307(s, out),
		['v', s @ ..] => super::t3::n308(s, out),
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1745);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4676);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7878);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4677);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4679);
							match s {
								[] => true,
								['l', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7880);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1748);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1751);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4681);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10173);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10174);
											s.is_empty()
										}
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
					super::push(out, 1752);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							super::push(out, 4682);
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1753);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4683);
							match s {
								[] => true,
								['a', 't', 'o', s @ ..] => {
									super::push(out, 7883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4684);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7884);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7885);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['i', 'c', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4687);
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 7886);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7887);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4688);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7889);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7890);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4689);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4690);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'd', 'a', 'l', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1758);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1759);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1760);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4692);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['o', 't', 'u', 'r', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 314);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7892);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1768);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4696);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 318);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['c', 'h', s @ ..] => {
					super::push(out, 1770);
					match s {
						[] => true,
						['o', 'r', 'e', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', s @ ..] => {
					super::push(out, 1771);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1775);
					match s {
						[] => true,
						['c', 'a', 'v', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4700);
							s.is_empty()
						}
						['g', 'a', 'v', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['p', 'o', 'n', 'y', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['r', 'a', 'c', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['o', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4702);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1777);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7894);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4704);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1779);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4706);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4708);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 7896);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11390);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11391);
													s.is_empty()
												}
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
							super::push(out, 4709);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10175);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10176);
											s.is_empty()
										}
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
					super::push(out, 1782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1783);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4710);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							super::push(out, 4711);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1784);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4713);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4714);
							s.is_empty()
						}
						['x', 'u', 'r', s @ ..] => {
							super::push(out, 4715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4716);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4717);
							s.is_empty()
						}
						['t', 'a', s @ ..] => {
							super::push(out, 4718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['l', 'l', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7897);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4719);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4720);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4721);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 329);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4723);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 7900);
									match s {
										[] => true,
										['e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11392);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11393);
													s.is_empty()
												}
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
							super::push(out, 4724);
							match s {
								[] => true,
								['i', 'c', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', 'd', s @ ..] => super::t3::n11448(s, out),
										['t', s @ ..] => super::t3::n11449(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => super::t3::n4961(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n1816(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t3::n331(s, out),
		['x', s @ ..] => super::t3::n332(s, out),
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['a', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1792);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1793);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1795);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1796);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1797);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7904);
									match s {
										[] => true,
										['t', 'i', s @ ..] => {
											super::push(out, 10182);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 11394);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11395);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 7905);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4735);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4736);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7907);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1802);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4738);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4739);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1803);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4741);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7909);
									match s {
										[] => true,
										['s', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4742);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7910);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7911);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7912);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', 's', s @ ..] => {
													super::push(out, 11396);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11397);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 7913);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 7914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1804);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 4743);
							match s {
								[] => true,
								['i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7915);
									s.is_empty()
								}
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
			super::push(out, 337);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1805);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7916);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 338);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7918);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7919);
									s.is_empty()
								}
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

/// Words ending -qa.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1808);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4744);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4745);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4746);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4747);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7920);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 7921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1809);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4748);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4749);
							match s {
								[] => true,
								['a', 'n', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10184);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10185);
											s.is_empty()
										}
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

/// Words ending -ra.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['b', 'i', 'l', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1811);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 1813);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 341);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1815);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4750);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1816);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1818);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 4752);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10186);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10187);
											s.is_empty()
										}
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
					super::push(out, 1819);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 4753);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10188);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10189);
											s.is_empty()
										}
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
			super::push(out, 343);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4754);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'f', 'e', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1820);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4756);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1822);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1823);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4758);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4759);
							match s {
								[] => true,
								['c', 'o', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4760);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1826);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1827);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1828);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4762);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4763);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7924);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 7925);
									s.is_empty()
								}
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
			super::push(out, 346);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1829);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1830);
					s.is_empty()
				}
				['o', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1833);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4764);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4765);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 7926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1834);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7927);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1836);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4767);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7929);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1837);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4768);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4769);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4770);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 4771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 351);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1839);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4772);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 4773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1840);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 353);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					super::push(out, 1842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 1843);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

