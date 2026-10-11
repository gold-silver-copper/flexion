use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => super::t0::n1(s, out),
		['r', s @ ..] => super::t0::n2(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['s', s @ ..] => super::t0::n3(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n4(s, out),
		['e', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 33);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 249);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 250);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 814);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 34);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 251);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 816);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 817);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1631);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 252);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 253);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 254);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 819);
									match s {
										[] => true,
										['j', '\u{1}', s @ ..] => {
											super::push(out, 1632);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => super::t0::n35(s, out),
				['d', s @ ..] => super::t0::n36(s, out),
				['e', s @ ..] => {
					super::push(out, 37);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 266);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 267);
							s.is_empty()
						}
						['s', s @ ..] => super::t0::n268(s, out),
						['v', s @ ..] => super::t0::n269(s, out),
						_ => false,
					}
				}
				['g', s @ ..] => super::t0::n38(s, out),
				['h', s @ ..] => super::t0::n39(s, out),
				['i', s @ ..] => super::t0::n40(s, out),
				['j', s @ ..] => super::t0::n41(s, out),
				['l', s @ ..] => super::t0::n42(s, out),
				['m', s @ ..] => super::t0::n43(s, out),
				['n', s @ ..] => super::t0::n44(s, out),
				['o', s @ ..] => super::t0::n45(s, out),
				['p', s @ ..] => super::t0::n46(s, out),
				['r', s @ ..] => super::t0::n47(s, out),
				['s', s @ ..] => super::t0::n48(s, out),
				['t', s @ ..] => super::t0::n49(s, out),
				['v', s @ ..] => super::t0::n50(s, out),
				['y', s @ ..] => super::t0::n51(s, out),
				['ñ', s @ ..] => super::t0::n52(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n6(s, out),
		['í', s @ ..] => super::t0::n7(s, out),
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['r', s @ ..] => super::t0::n8(s, out),
		_ => false,
	}
}

/// Words ending -ar.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 78);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 79);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 80);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 81);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 82);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 428);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 429);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 430);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 431);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1184);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1185);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1186);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1187);
									match s {
										[] => true,
										['a', 'e', s @ ..] => {
											super::push(out, 2092);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 433);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 83);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 435);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 436);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1189);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 1190);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 2093);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1191);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 84);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 85);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 86);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 87);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 438);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 439);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 88);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 440);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 441);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1194);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1195);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1196);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2094);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 89);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 443);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 444);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1197);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 1198);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2095);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 2096);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 90);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 445);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 446);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 447);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 448);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1200);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 1201);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 1202);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 1203);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1204);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1205);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 449);
							s.is_empty()
						}
						['r', s @ ..] => super::t0::n453(s, out),
						['t', s @ ..] => {
							super::push(out, 451);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 452);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n91(s, out),
				['s', s @ ..] => super::t0::n92(s, out),
				['u', s @ ..] => super::t0::n93(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n11(s, out),
		['e', s @ ..] => super::t0::n12(s, out),
		['f', s @ ..] => super::t0::n13(s, out),
		['g', s @ ..] => super::t0::n14(s, out),
		['h', s @ ..] => super::t0::n15(s, out),
		['i', s @ ..] => super::t0::n16(s, out),
		['j', s @ ..] => super::t0::n17(s, out),
		['l', s @ ..] => super::t0::n18(s, out),
		['m', s @ ..] => super::t0::n19(s, out),
		['n', s @ ..] => super::t0::n20(s, out),
		['o', s @ ..] => {
			super::push(out, 21);
			s.is_empty()
		}
		['p', s @ ..] => super::t0::n22(s, out),
		['r', s @ ..] => super::t0::n23(s, out),
		['s', s @ ..] => super::t0::n24(s, out),
		['t', s @ ..] => super::t0::n25(s, out),
		['u', s @ ..] => super::t0::n26(s, out),
		['v', s @ ..] => super::t0::n27(s, out),
		['x', s @ ..] => super::t0::n28(s, out),
		['y', s @ ..] => super::t0::n29(s, out),
		['z', s @ ..] => super::t0::n30(s, out),
		['ç', s @ ..] => super::t0::n31(s, out),
		['ñ', s @ ..] => super::t0::n32(s, out),
		_ => false,
	}
}

/// Words ending -cer.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['s', 'i', s @ ..] => {
							super::push(out, 1633);
							match s {
								[] => true,
								['t', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 2322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 821);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1634);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['e', 'r', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 828);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2323);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 829);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1638);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2325);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2713);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 2714);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2821);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 2822);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 2326);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1641);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2328);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 832);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 833);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 834);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1648);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2330);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 2715);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 2716);
											s.is_empty()
										}
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
					super::push(out, 835);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1649);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2331);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2332);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2717);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2823);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 2718);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 258);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 837);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['t', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -der.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1655);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2333);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2719);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'r', s @ ..] => {
									super::push(out, 2334);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2720);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 2335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2336);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2338);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2339);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 2721);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2824);
													s.is_empty()
												}
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
							super::push(out, 1656);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1658);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2340);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 845);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 847);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -seer.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	s.is_empty()
}

/// Words ending -veer.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 850);
			s.is_empty()
		}
		['o', 'r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				['s', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	s.is_empty()
}

/// Words ending -her.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	s.is_empty()
}

/// Words ending -ier.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	s.is_empty()
}

/// Words ending -jer.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	s.is_empty()
}

/// Words ending -ler.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 272);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				['l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mer.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	s.is_empty()
}

/// Words ending -ner.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['e', 'c', 's', s @ ..] => {
					super::push(out, 859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oer.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	s.is_empty()
}

/// Words ending -per.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['m', 'o', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 860);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 861);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['e', 'd', s @ ..] => {
							super::push(out, 1668);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -rer.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 278);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 280);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 281);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ver.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 862);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1672);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2343);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 864);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yer.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	s.is_empty()
}

/// Words ending -ñer.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['a', 't', 'a', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ir.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 53);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 292);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 867);
							match s {
								[] => true,
								['c', 's', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 293);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 868);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 869);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1674);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1675);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1676);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 1677);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 871);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1678);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 1679);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1680);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2722);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 2723);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 2724);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 2345);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 1681);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1682);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2346);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 2347);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1683);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 872);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 299);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 874);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 875);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 303);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 877);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 878);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						['ñ', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 881);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 882);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 1685);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 884);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1687);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 1688);
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
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 306);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 311);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 312);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['c', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 885);
							match s {
								[] => true,
								['h', 'e', s @ ..] => {
									super::push(out, 1689);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 886);
							match s {
								[] => true,
								['h', 'e', s @ ..] => {
									super::push(out, 1690);
									s.is_empty()
								}
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
			super::push(out, 59);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 314);
					s.is_empty()
				}
				['b', 'a', 't', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 318);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t0::n61(s, out),
		['n', s @ ..] => super::t0::n62(s, out),
		['p', s @ ..] => {
			super::push(out, 63);
			s.is_empty()
		}
		['r', s @ ..] => super::t0::n64(s, out),
		['s', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 341);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t0::n66(s, out),
		['u', s @ ..] => super::t0::n67(s, out),
		['v', s @ ..] => super::t0::n68(s, out),
		['ñ', s @ ..] => super::t0::n69(s, out),
		['ü', s @ ..] => {
			super::push(out, 70);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ír.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 932);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1723);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 73);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rse.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n74(s, out),
		['e', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 394);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1092);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 395);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1094);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1095);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1097);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2029);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2617);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2618);
											match s {
												[] => true,
												['i', 't', 'a', 's', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 2801);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 2030);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2619);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 2620);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2621);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 2031);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1098);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2033);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2622);
											s.is_empty()
										}
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2623);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 2624);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 2625);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1099);
							match s {
								[] => true,
								['e', 'v', 'n', s @ ..] => {
									super::push(out, 2035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1100);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2036);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2626);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2627);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1101);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1103);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2038);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t0::n1126(s, out),
						['o', s @ ..] => {
							super::push(out, 1105);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1106);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2042);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2043);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2044);
									match s {
										[] => true,
										['m', 'e', s @ ..] => {
											super::push(out, 2634);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n401(s, out),
				['g', s @ ..] => super::t0::n402(s, out),
				['j', s @ ..] => {
					super::push(out, 400);
					match s {
						[] => true,
						['e', 't', 's', s @ ..] => super::t0::n1132(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n404(s, out),
				['m', s @ ..] => super::t0::n405(s, out),
				['n', s @ ..] => super::t0::n406(s, out),
				['o', s @ ..] => super::t0::n407(s, out),
				['p', s @ ..] => super::t0::n408(s, out),
				['r', s @ ..] => super::t0::n409(s, out),
				['s', s @ ..] => super::t0::n410(s, out),
				['t', s @ ..] => super::t0::n411(s, out),
				['v', s @ ..] => super::t0::n412(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n76(s, out),
		['í', s @ ..] => super::t0::n77(s, out),
		_ => false,
	}
}

/// Words ending -rocar.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1207);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1208);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1209);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcar.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 454);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -scar.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 456);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2101);
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
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucar.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dar.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 94);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 462);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1216);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1217);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1218);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 464);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1220);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1221);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1222);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1223);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2102);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2103);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2104);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1226);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1230);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1231);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1232);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1233);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2105);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2107);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2108);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2109);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2110);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2111);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 467);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 471);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 472);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1236);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 475);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1238);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1239);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1240);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2113);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1241);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1242);
							match s {
								[] => true,
								['a', 'e', s @ ..] => {
									super::push(out, 2115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1243);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1244);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 102);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ear.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 478);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1246);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 2116);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2117);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 2118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1248);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 481);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 107);
			s.is_empty()
		}
		['l', 'a', 't', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1253);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2119);
									s.is_empty()
								}
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
			super::push(out, 110);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1254);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2120);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 2121);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1255);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2123);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 2648);
											s.is_empty()
										}
										['j', s @ ..] => {
											super::push(out, 2649);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 2650);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2124);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 486);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 488);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 489);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 1257);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2126);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 2651);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 490);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1259);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2127);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 2652);
											s.is_empty()
										}
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
					super::push(out, 491);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 1260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 492);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 493);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['q', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1264);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2128);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2129);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 2653);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 2812);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 2813);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 2654);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 2655);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 2814);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 2815);
													s.is_empty()
												}
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

/// Words ending -far.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gar.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 497);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1269);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2130);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2131);
									s.is_empty()
								}
								['s', 'e', 'd', 'e', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2133);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2134);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1273);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						['r', 'o', 's', s @ ..] => {
							super::push(out, 1276);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2136);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2137);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1277);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1279);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2138);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2139);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1280);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 514);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 1282);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 515);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1285);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 124);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 518);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -har.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1290);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2142);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 2143);
									s.is_empty()
								}
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
			super::push(out, 129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iar.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['m', 'e', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1294);
							match s {
								[] => true,
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1295);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1296);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 534);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1301);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1302);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1304);
							s.is_empty()
						}
						['r', 'g', 'o', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2147);
									s.is_empty()
								}
								['t', 'r', s @ ..] => {
									super::push(out, 2148);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2656);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 2657);
											s.is_empty()
										}
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
					super::push(out, 540);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1305);
							match s {
								[] => true,
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2149);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 541);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['o', 'c', 'e', s @ ..] => {
							super::push(out, 1307);
							s.is_empty()
						}
						['u', 'g', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 542);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['u', 't', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					super::push(out, 545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1313);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2150);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						['t', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1317);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1319);
							s.is_empty()
						}
						['p', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1321);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 553);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1323);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 555);
					s.is_empty()
				}
				['u', 'c', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n569(s, out),
				['m', s @ ..] => {
					super::push(out, 561);
					s.is_empty()
				}
				['o', s @ ..] => super::t0::n571(s, out),
				['s', s @ ..] => super::t0::n572(s, out),
				['x', s @ ..] => super::t0::n573(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n141(s, out),
		['s', s @ ..] => super::t0::n142(s, out),
		['t', s @ ..] => super::t0::n143(s, out),
		['u', s @ ..] => super::t0::n144(s, out),
		['v', s @ ..] => super::t0::n145(s, out),
		['x', s @ ..] => super::t0::n146(s, out),
		_ => false,
	}
}

/// Words ending -jar.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 149);
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
				['f', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['a', s @ ..] => {
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
				['l', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1386);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2175);
									s.is_empty()
								}
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
			super::push(out, 151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lar.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1390);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2176);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2177);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2658);
											s.is_empty()
										}
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
					super::push(out, 610);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 615);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1397);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1398);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 618);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 157);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1402);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 625);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 626);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 627);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 1405);
							match s {
								[] => true,
								['a', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2179);
									s.is_empty()
								}
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
			super::push(out, 159);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 1406);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 631);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1407);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1408);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1409);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2181);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 2182);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 2183);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2184);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1413);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1415);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2186);
									s.is_empty()
								}
								['s', s @ ..] => super::t0::n2246(s, out),
								_ => false,
							}
						}
						['f', s @ ..] => super::t0::n1452(s, out),
						['g', s @ ..] => super::t0::n1453(s, out),
						['h', s @ ..] => super::t0::n1454(s, out),
						['m', s @ ..] => {
							super::push(out, 1419);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1420);
							s.is_empty()
						}
						['r', s @ ..] => super::t0::n1457(s, out),
						['s', s @ ..] => super::t0::n1458(s, out),
						['t', s @ ..] => super::t0::n1459(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t0::n643(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n160(s, out),
		['p', s @ ..] => super::t0::n161(s, out),
		['r', s @ ..] => super::t0::n162(s, out),
		['s', s @ ..] => super::t0::n163(s, out),
		['u', s @ ..] => super::t0::n164(s, out),
		_ => false,
	}
}

/// Words ending -mar.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 169);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						['u', 'p', s @ ..] => {
							super::push(out, 1459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nar.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['o', 'b', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 174);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1473);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 2661);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 2662);
											s.is_empty()
										}
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
					super::push(out, 670);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1474);
							match s {
								[] => true,
								['a', 'c', 's', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1478);
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
								['i', s @ ..] => {
									super::push(out, 2209);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2210);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2211);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2213);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2215);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2216);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2218);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1482);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2220);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2221);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1484);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1485);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2223);
									s.is_empty()
								}
								['m', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2663);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2664);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 2224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1486);
							s.is_empty()
						}
						['v', 'n', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2225);
									s.is_empty()
								}
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n698(s, out),
				['u', s @ ..] => super::t0::n699(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n179(s, out),
		['z', s @ ..] => super::t0::n180(s, out),
		_ => false,
	}
}

/// Words ending -par.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 181);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 182);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rar.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 185);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1498);
							match s {
								[] => true,
								['q', 'i', 't', s @ ..] => {
									super::push(out, 2230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1500);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2231);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2232);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2233);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2667);
											s.is_empty()
										}
										['s', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2668);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 2234);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2669);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 187);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1505);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2235);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
							super::push(out, 1506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 1507);
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
				['p', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 190);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1512);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2236);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2237);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 708);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1513);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1514);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 714);
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
					super::push(out, 715);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1520);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1521);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2239);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 2671);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 2672);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1522);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2240);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2241);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1523);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1524);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1525);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1526);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1527);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2242);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2673);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 2674);
											s.is_empty()
										}
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
					super::push(out, 717);
					s.is_empty()
				}
				['u', s @ ..] => super::t0::n732(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t0::n195(s, out),
		['u', s @ ..] => super::t0::n196(s, out),
		_ => false,
	}
}

/// Words ending -sar.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1537);
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
				['t', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1538);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2250);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1540);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2252);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2253);
									s.is_empty()
								}
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
			super::push(out, 200);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', s @ ..] => {
							super::push(out, 1541);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1543);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2254);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2255);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2256);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2257);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2258);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1544);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 738);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tar.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1552);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2260);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 2677);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2261);
									s.is_empty()
								}
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
			super::push(out, 209);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 746);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['c', 'n', s @ ..] => {
							super::push(out, 1555);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2262);
									s.is_empty()
								}
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
			super::push(out, 210);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 751);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 752);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1558);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2263);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 2678);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 2679);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2264);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => super::t0::n1600(s, out),
						['g', s @ ..] => {
							super::push(out, 1560);
							s.is_empty()
						}
						['i', s @ ..] => super::t0::n1602(s, out),
						['j', s @ ..] => super::t0::n1603(s, out),
						['l', s @ ..] => super::t0::n1604(s, out),
						['m', s @ ..] => super::t0::n1605(s, out),
						['r', s @ ..] => super::t0::n1606(s, out),
						['s', s @ ..] => super::t0::n1607(s, out),
						['t', s @ ..] => super::t0::n1608(s, out),
						['u', s @ ..] => super::t0::n1609(s, out),
						['v', s @ ..] => super::t0::n1610(s, out),
						['y', s @ ..] => super::t0::n1611(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n768(s, out),
				['o', s @ ..] => super::t0::n769(s, out),
				['u', s @ ..] => super::t0::n770(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n212(s, out),
		['p', s @ ..] => super::t0::n213(s, out),
		['r', s @ ..] => super::t0::n214(s, out),
		['s', s @ ..] => super::t0::n215(s, out),
		['u', s @ ..] => super::t0::n216(s, out),
		_ => false,
	}
}

/// Words ending -uar.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['i', 'l', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['a', 'v', 'e', 'r', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 770);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'a', 'n', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -var.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1606);
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
		['i', s @ ..] => {
			super::push(out, 228);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['c', 'n', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['e', 's', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xar.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	s.is_empty()
}

/// Words ending -yar.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['u', 'h', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zar.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 235);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['f', 's', s @ ..] => {
							super::push(out, 1611);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 794);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 's', s @ ..] => {
									super::push(out, 2310);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2711);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 2311);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 2712);
											s.is_empty()
										}
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
					super::push(out, 795);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1612);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2312);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1613);
							s.is_empty()
						}
						['r', 'r', s @ ..] => {
							super::push(out, 1614);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 796);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1616);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1617);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2314);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2315);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 2316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 799);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 238);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['i', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1618);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1619);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 805);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1621);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2317);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2318);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1623);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1624);
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
				['a', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1625);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1626);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1627);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2320);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -çar.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['r', 'o', 'm', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ñar.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 244);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 811);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1629);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mir.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 887);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['r', 'p', 'm', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 323);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nir.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1695);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1696);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 327);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 893);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1698);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2349);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2350);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rir.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1700);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 1701);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 899);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1706);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							super::push(out, 1707);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 901);
					match s {
						[] => true,
						['e', 'r', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1708);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1709);
							match s {
								[] => true,
								['d', 'e', s @ ..] => {
									super::push(out, 2353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 903);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 1711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 904);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1712);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1713);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 334);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 338);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 339);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tir.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 344);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 346);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 910);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['v', 'e', s @ ..] => {
							super::push(out, 1715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2355);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1716);
							s.is_empty()
						}
						['v', 'n', 'i', 'e', s @ ..] => {
							super::push(out, 1717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uir.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 351);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 352);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 922);
					match s {
						[] => true,
						['e', 'e', s @ ..] => {
							super::push(out, 1718);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2357);
									s.is_empty()
								}
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
			super::push(out, 355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 358);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vir.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 362);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1721);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
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

/// Words ending -ñir.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 364);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 365);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 366);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -arse.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1725);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2359);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 2360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1727);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1728);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2361);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 2362);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2725);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1730);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1731);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1732);
							match s {
								[] => true,
								['v', 'e', s @ ..] => {
									super::push(out, 2364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1734);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1735);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1736);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2365);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2726);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1740);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2367);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2368);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1741);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1743);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1744);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1745);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2373);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 2374);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2375);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 946);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 947);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 950);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 951);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n975(s, out),
				['n', s @ ..] => super::t0::n976(s, out),
				['o', s @ ..] => {
					super::push(out, 955);
					match s {
						[] => true,
						['n', s @ ..] => super::t0::n1810(s, out),
						['r', s @ ..] => super::t0::n1811(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n978(s, out),
				['u', s @ ..] => super::t0::n979(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n377(s, out),
		['f', s @ ..] => super::t0::n378(s, out),
		['g', s @ ..] => super::t0::n379(s, out),
		['h', s @ ..] => super::t0::n380(s, out),
		['i', s @ ..] => super::t0::n381(s, out),
		['j', s @ ..] => super::t0::n382(s, out),
		['l', s @ ..] => super::t0::n383(s, out),
		['m', s @ ..] => super::t0::n384(s, out),
		['n', s @ ..] => super::t0::n385(s, out),
		['o', s @ ..] => super::t0::n386(s, out),
		['p', s @ ..] => super::t0::n387(s, out),
		['r', s @ ..] => super::t0::n388(s, out),
		['s', s @ ..] => super::t0::n389(s, out),
		['t', s @ ..] => super::t0::n390(s, out),
		['u', s @ ..] => super::t0::n391(s, out),
		['v', s @ ..] => super::t0::n392(s, out),
		['x', s @ ..] => super::t0::n393(s, out),
		['y', s @ ..] => super::t0::n394(s, out),
		['z', s @ ..] => super::t0::n395(s, out),
		['ñ', s @ ..] => super::t0::n396(s, out),
		_ => false,
	}
}

/// Words ending -nderse.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1104);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2040);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2628);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2802);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2630);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2631);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2632);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2804);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eerse.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 1107);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2045);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gerse.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1108);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stejerse.
pub(crate) fn n1132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	s.is_empty()
}

/// Words ending -lerse.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1111);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1112);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -merse.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 2046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1115);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nerse.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1117);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1118);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oerse.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	s.is_empty()
}

/// Words ending -perse.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['m', 'o', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2048);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rerse.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2049);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2050);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2806);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2807);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2808);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2809);
									s.is_empty()
								}
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

/// Words ending -serse.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2051);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2052);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -terse.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1120);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1121);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -verse.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1122);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1123);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2636);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1125);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irse.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 410);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2054);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2638);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2056);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1130);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1132);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1133);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2060);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1139);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['c', 'n', 'e', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1142);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1144);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2066);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1151);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2640);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2068);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2070);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['o', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1159);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1161);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2642);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 2643);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 2810);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2811);
											s.is_empty()
										}
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
					super::push(out, 1162);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2078);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1169);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2083);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n426(s, out),
		['v', s @ ..] => super::t0::n427(s, out),
		['ñ', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				['e', s @ ..] => super::t0::n1209(s, out),
				['u', s @ ..] => super::t0::n1210(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -írse.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2647);
									s.is_empty()
								}
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

/// Words ending -ipiar.
pub(crate) fn n569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1327);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1328);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1329);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -opiar.
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	s.is_empty()
}

/// Words ending -spiar.
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	s.is_empty()
}

/// Words ending -xpiar.
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	s.is_empty()
}

/// Words ending -riar.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					super::push(out, 1331);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1332);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1333);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1334);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2152);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 566);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1338);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'e', s @ ..] => {
			super::push(out, 568);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1341);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 2154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 571);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1346);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['g', 'a', s @ ..] => {
							super::push(out, 2155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', 'r', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 574);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2156);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1350);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1351);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1352);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2158);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1353);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1354);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2160);
							s.is_empty()
						}
						['u', s @ ..] => {
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
			super::push(out, 577);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -siar.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1356);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1357);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1358);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2162);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 578);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 579);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 580);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 581);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiar.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 583);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1359);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 585);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 586);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 587);
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
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1363);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uiar.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1366);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1367);
					match s {
						[] => true,
						['a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 2164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 590);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2165);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1369);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2167);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1372);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1373);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 2171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -viar.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 591);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1375);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1376);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2172);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1377);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 592);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 593);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 596);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 597);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1381);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1382);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xiar.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	s.is_empty()
}

/// Words ending -scollar.
pub(crate) fn n2246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2187);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2817);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -follar.
pub(crate) fn n1452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1416);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2188);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2189);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gollar.
pub(crate) fn n1453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1417);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2191);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hollar.
pub(crate) fn n1454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1418);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2193);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rollar.
pub(crate) fn n1457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1421);
	s.is_empty()
}

/// Words ending -sollar.
pub(crate) fn n1458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1422);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2195);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tollar.
pub(crate) fn n1459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1423);
	s.is_empty()
}

/// Words ending -ullar.
pub(crate) fn n643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 633);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1425);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1426);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olar.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 634);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2198);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2200);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2659);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2660);
									s.is_empty()
								}
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
			super::push(out, 635);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 637);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		['j', 'a', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 641);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 642);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 643);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2202);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 645);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 646);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 647);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ñ', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plar.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['m', 'e', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rlar.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	s.is_empty()
}

/// Words ending -slar.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ular.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 652);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1452);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ornar.
pub(crate) fn n698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 684);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2665);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1488);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urnar.
pub(crate) fn n699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 685);
	s.is_empty()
}

/// Words ending -unar.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 686);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 687);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -znar.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 688);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 690);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urrar.
pub(crate) fn n732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 718);
	s.is_empty()
}

/// Words ending -trar.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 719);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 720);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 723);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 724);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1531);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1532);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['o', 'm', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2245);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2246);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2247);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2675);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2676);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 2818);
											s.is_empty()
										}
										['r', 't', s @ ..] => {
											super::push(out, 2819);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urar.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	s.is_empty()
}

/// Words ending -dentar.
pub(crate) fn n1600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1559);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2266);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2267);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2680);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ientar.
pub(crate) fn n1602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1561);
	s.is_empty()
}

/// Words ending -jentar.
pub(crate) fn n1603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1562);
	s.is_empty()
}

/// Words ending -lentar.
pub(crate) fn n1604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1563);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2268);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2682);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2269);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2270);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2271);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mentar.
pub(crate) fn n1605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1564);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2273);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2274);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2684);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2275);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2687);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2689);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2690);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2691);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2276);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2277);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2693);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2694);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2695);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2278);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rentar.
pub(crate) fn n1606(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2696);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2697);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2279);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2280);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2281);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2699);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sentar.
pub(crate) fn n1607(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2282);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2283);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2284);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2285);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tentar.
pub(crate) fn n1608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1565);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2287);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2288);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2701);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', s @ ..] => {
			super::push(out, 2289);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2290);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2703);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2704);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uentar.
pub(crate) fn n1609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1566);
	s.is_empty()
}

/// Words ending -ventar.
pub(crate) fn n1610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1567);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2705);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2291);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2292);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2293);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2294);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yentar.
pub(crate) fn n1611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1568);
	s.is_empty()
}

/// Words ending -intar.
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 753);
	s.is_empty()
}

/// Words ending -ontar.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 754);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1569);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1570);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -untar.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 755);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1572);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1573);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -otar.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ptar.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	s.is_empty()
}

/// Words ending -rtar.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 758);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				['p', s @ ..] => {
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
			super::push(out, 760);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2298);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -star.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 761);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 762);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2300);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2707);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 2708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2302);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2303);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2304);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2710);
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
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 764);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2305);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['a', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1603);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utar.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	s.is_empty()
}

/// Words ending -ldarse.
pub(crate) fn n975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 953);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1753);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1754);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2377);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2378);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ndarse.
pub(crate) fn n976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2381);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2382);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2383);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1758);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2385);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2386);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2387);
					match s {
						[] => true,
						['g', 's', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nodarse.
pub(crate) fn n1810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1761);
	match s {
		[] => true,
		['e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2389);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rodarse.
pub(crate) fn n1811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1762);
	s.is_empty()
}

/// Words ending -rdarse.
pub(crate) fn n978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 956);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1763);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1764);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2390);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1765);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2392);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2393);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udarse.
pub(crate) fn n979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 957);
	s.is_empty()
}

/// Words ending -earse.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1767);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2395);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['g', 'a', s @ ..] => {
					super::push(out, 1768);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -farse.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -garse.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 965);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2398);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1775);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2400);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2401);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2402);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2404);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2405);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2406);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2407);
							s.is_empty()
						}
						['t', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 2409);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1784);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 2411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 969);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2412);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1787);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -harse.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 973);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 974);
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
				['i', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['h', 'c', 'n', 'e', s @ ..] => {
							super::push(out, 2416);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'p', 'a', 'c', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2418);
							s.is_empty()
						}
						['i', 'p', s @ ..] => {
							super::push(out, 2419);
							match s {
								[] => true,
								['m', 'o', s @ ..] => {
									super::push(out, 2734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', 'n', s @ ..] => {
							super::push(out, 2420);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', s @ ..] => {
							super::push(out, 2421);
							match s {
								[] => true,
								['r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2736);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1795);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['o', 'c', s @ ..] => {
							super::push(out, 2422);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'c', 'r', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['u', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iarse.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1798);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['t', 'n', s @ ..] => {
							super::push(out, 2424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1800);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1803);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2425);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2426);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2427);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1804);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2429);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1809);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2432);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1810);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1811);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 978);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1814);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1815);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1816);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1818);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1819);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 979);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 980);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 981);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1822);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1823);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1827);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2738);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2739);
									s.is_empty()
								}
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
			super::push(out, 984);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1828);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1829);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1831);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1832);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1834);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1835);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2442);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2740);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1836);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1837);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1838);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						['t', 'u', 'c', 'e', 'j', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1839);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2447);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2448);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2742);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1842);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1843);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1844);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1846);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2451);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2453);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2454);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n1901(s, out),
				['e', s @ ..] => {
					super::push(out, 1849);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n1903(s, out),
				['s', s @ ..] => super::t0::n1904(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jarse.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 991);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2746);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2747);
									s.is_empty()
								}
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
			super::push(out, 992);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 994);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 995);
			match s {
				[] => true,
				['o', 'f', 'l', s @ ..] => {
					super::push(out, 1855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -larse.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 380);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 997);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1856);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2461);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1858);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1859);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2463);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1861);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1862);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1863);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2748);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1864);
					match s {
						[] => true,
						['e', 'm', 'e', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1001);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1866);
					match s {
						[] => true,
						['h', 'c', 'e', s @ ..] => {
							super::push(out, 2468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1868);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2469);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2471);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1872);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2473);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2474);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2475);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2476);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2477);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2478);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2479);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1873);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 2481);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 2750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2482);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2751);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2483);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2485);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1876);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2753);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2825);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 2826);
											s.is_empty()
										}
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
					super::push(out, 1877);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2489);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1879);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2491);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2754);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2493);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2494);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2756);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1880);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1881);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1006);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2495);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => super::t0::n2917(s, out),
										['s', s @ ..] => super::t0::n2918(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t0::n2574(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n1939(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n1029(s, out),
		['s', s @ ..] => super::t0::n1030(s, out),
		['u', s @ ..] => super::t0::n1031(s, out),
		_ => false,
	}
}

/// Words ending -marse.
pub(crate) fn n384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1010);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1011);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1012);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1890);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2501);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1013);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1893);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1894);
					match s {
						[] => true,
						['f', 'a', s @ ..] => {
							super::push(out, 2503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1016);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1897);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1898);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1899);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -narse.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1017);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1019);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1902);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2504);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2505);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1905);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							super::push(out, 2507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1906);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1907);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1908);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2508);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1909);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1915);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2510);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2511);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2759);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2512);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2515);
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
		['r', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1919);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2517);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2518);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1921);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2521);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1024);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1923);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oarse.
pub(crate) fn n386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	s.is_empty()
}

/// Words ending -parse.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1025);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1030);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1927);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2525);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1928);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rarse.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1932);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1933);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2530);
							match s {
								[] => true,
								['q', 'i', 't', s @ ..] => {
									super::push(out, 2761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1934);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1935);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2533);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2534);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2762);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2535);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2764);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1936);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2537);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2766);
									s.is_empty()
								}
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
			super::push(out, 1036);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1937);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1938);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							super::push(out, 2540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1939);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2541);
							match s {
								[] => true,
								['s', 'n', 'o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2829);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 2830);
											s.is_empty()
										}
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
		['f', s @ ..] => {
			super::push(out, 1039);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2542);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1942);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1943);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['o', 'h', 's', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1946);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2544);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1948);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1951);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2548);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2550);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2551);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2552);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2553);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1957);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2554);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1958);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2556);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2768);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 2831);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 2769);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2832);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2557);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2558);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2559);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 2770);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2833);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sarse.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1048);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1961);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1051);
			match s {
				[] => true,
				['o', 'b', 'a', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1964);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					super::push(out, 1966);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2567);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1968);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tarse.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1056);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1970);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1972);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2572);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1973);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1060);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1974);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1975);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1061);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1977);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1978);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2576);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2772);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 2834);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 2835);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2577);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2578);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2774);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2836);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 2837);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 2775);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2579);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2777);
									s.is_empty()
								}
								['i', 'c', s @ ..] => {
									super::push(out, 2778);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 2838);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2779);
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 2841);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 2842);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2780);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2781);
									s.is_empty()
								}
								['g', 'n', s @ ..] => {
									super::push(out, 2782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2783);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 2784);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2786);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 2787);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2581);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2839);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 2840);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 2789);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1979);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1980);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2582);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 1982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1983);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1984);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2585);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1985);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1986);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2590);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t0::n1087(s, out),
		['u', s @ ..] => super::t0::n1088(s, out),
		_ => false,
	}
}

/// Words ending -uarse.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1067);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1992);
					match s {
						[] => true,
						['v', 'e', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1993);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1068);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1997);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1072);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -varse.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1999);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1075);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1076);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2605);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2002);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['c', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2004);
					s.is_empty()
				}
				['n', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1078);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2006);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xarse.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yarse.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	s.is_empty()
}

/// Words ending -zarse.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1082);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2008);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2009);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2010);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1083);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2012);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2013);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2609);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						['r', 'o', 'p', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2014);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2612);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2613);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2796);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 2797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2015);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2016);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2017);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2019);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2614);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1085);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2021);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2023);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2799);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1086);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1087);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2025);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1088);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ñarse.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1089);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2028);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uirse.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1172);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1173);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2087);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1175);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1176);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1177);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -virse.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1179);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2645);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eñirse.
pub(crate) fn n1209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	s.is_empty()
}

/// Words ending -uñirse.
pub(crate) fn n1210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	s.is_empty()
}

/// Words ending -aviarse.
pub(crate) fn n1901(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2456);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2457);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2744);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2458);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iviarse.
pub(crate) fn n1903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1850);
	s.is_empty()
}

/// Words ending -sviarse.
pub(crate) fn n1904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1851);
	s.is_empty()
}

/// Words ending -atemplarse.
pub(crate) fn n2917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2827);
	s.is_empty()
}

/// Words ending -stemplarse.
pub(crate) fn n2918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2828);
	s.is_empty()
}

/// Words ending -implarse.
pub(crate) fn n2574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2496);
	s.is_empty()
}

/// Words ending -oplarse.
pub(crate) fn n1939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1885);
	match s {
		[] => true,
		['c', s @ ..] => {
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

/// Words ending -rlarse.
pub(crate) fn n1029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1886);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1887);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2499);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2500);
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

/// Words ending -slarse.
pub(crate) fn n1030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1008);
	s.is_empty()
}

/// Words ending -ularse.
pub(crate) fn n1031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1009);
	s.is_empty()
}

/// Words ending -starse.
pub(crate) fn n1087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1065);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1987);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1988);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2591);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2592);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2593);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2594);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2595);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2596);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1989);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2598);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2599);
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2600);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utarse.
pub(crate) fn n1088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1066);
	s.is_empty()
}

