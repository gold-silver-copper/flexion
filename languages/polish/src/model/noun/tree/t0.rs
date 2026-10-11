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
					super::push(out, 58);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 726);
							match s {
								[] => true,
								['e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 3245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 727);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 728);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3246);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3247);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3248);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 3249);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7404);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 3250);
									s.is_empty()
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 3251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 729);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3253);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3254);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 59);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 731);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 732);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 3256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 733);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 3257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 734);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3258);
									s.is_empty()
								}
								['z', 'o', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 735);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 3260);
									s.is_empty()
								}
								['ż', s @ ..] => {
									super::push(out, 3261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 736);
							match s {
								[] => true,
								['ź', '\u{1}', s @ ..] => {
									super::push(out, 3262);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 737);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 738);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 60);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'i', 'g', 'j', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 740);
							s.is_empty()
						}
						['e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 61);
					match s {
						[] => true,
						['o', 'l', 'u', s @ ..] => {
							super::push(out, 742);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 3264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n63(s, out),
				['r', s @ ..] => super::t0::n64(s, out),
				['u', s @ ..] => super::t0::n65(s, out),
				['y', s @ ..] => super::t0::n66(s, out),
				['z', s @ ..] => {
					super::push(out, 66);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => super::t0::n68(s, out),
				['ą', s @ ..] => super::t0::n69(s, out),
				['ę', s @ ..] => {
					super::push(out, 69);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 70);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						_ => false,
					}
				}
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
		['j', s @ ..] => super::t0::n10(s, out),
		['k', s @ ..] => super::t0::n11(s, out),
		['l', s @ ..] => super::t0::n12(s, out),
		['m', s @ ..] => super::t0::n13(s, out),
		['n', s @ ..] => super::t0::n14(s, out),
		['o', s @ ..] => super::t0::n15(s, out),
		['p', s @ ..] => super::t0::n16(s, out),
		['r', s @ ..] => super::t0::n17(s, out),
		['s', s @ ..] => super::t0::n18(s, out),
		['t', s @ ..] => super::t0::n19(s, out),
		['u', s @ ..] => super::t0::n20(s, out),
		['v', s @ ..] => super::t0::n21(s, out),
		['w', s @ ..] => super::t0::n22(s, out),
		['x', s @ ..] => super::t0::n23(s, out),
		['y', s @ ..] => super::t0::n24(s, out),
		['z', s @ ..] => super::t0::n25(s, out),
		['ć', s @ ..] => super::t0::n26(s, out),
		['ę', s @ ..] => super::t0::n27(s, out),
		['ł', s @ ..] => super::t0::n28(s, out),
		['ń', s @ ..] => super::t0::n29(s, out),
		['ś', s @ ..] => super::t0::n30(s, out),
		['ź', s @ ..] => super::t0::n31(s, out),
		['ż', s @ ..] => super::t0::n32(s, out),
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 429);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 430);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2483);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 432);
					match s {
						[] => true,
						['u', 'z', s @ ..] => {
							super::push(out, 2485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 433);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6575);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6577);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 6578);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', 'g', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 434);
					match s {
						[] => true,
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 2489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 435);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 2490);
							match s {
								[] => true,
								['s', 'a', s @ ..] => {
									super::push(out, 6579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['r', 'k', s @ ..] => {
							super::push(out, 2491);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 437);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 438);
					match s {
						[] => true,
						['g', 'y', 'z', 'c', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ń', s @ ..] => {
					super::push(out, 439);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 2493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 440);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 2494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ź', s @ ..] => {
					super::push(out, 441);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2495);
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 442);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							super::push(out, 2496);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 6582);
									s.is_empty()
								}
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
			super::push(out, 34);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 443);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 444);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'p', s @ ..] => {
							super::push(out, 2498);
							match s {
								[] => true,
								['ł', s @ ..] => {
									super::push(out, 6584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 445);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 446);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 447);
					s.is_empty()
				}
				['i', s @ ..] => super::t0::n450(s, out),
				['j', s @ ..] => {
					super::push(out, 449);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2503);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 6599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2504);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 6600);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ó', s @ ..] => {
							super::push(out, 2505);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10639);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 10640);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 6601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 450);
					s.is_empty()
				}
				['m', s @ ..] => super::t0::n453(s, out),
				['p', s @ ..] => super::t0::n454(s, out),
				['r', s @ ..] => super::t0::n455(s, out),
				['s', 'i', 'p', 'o', 'j', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => super::t0::n456(s, out),
				['w', s @ ..] => super::t0::n457(s, out),
				['y', s @ ..] => super::t0::n458(s, out),
				['z', s @ ..] => super::t0::n459(s, out),
				['ą', s @ ..] => super::t0::n460(s, out),
				['ń', s @ ..] => super::t0::n461(s, out),
				['ź', s @ ..] => super::t0::n462(s, out),
				['ż', s @ ..] => super::t0::n463(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n35(s, out),
		['e', s @ ..] => super::t0::n36(s, out),
		['f', s @ ..] => super::t0::n37(s, out),
		['g', s @ ..] => super::t0::n38(s, out),
		['h', s @ ..] => super::t0::n39(s, out),
		['i', s @ ..] => super::t0::n40(s, out),
		['j', s @ ..] => super::t1::n41(s, out),
		['k', s @ ..] => super::t1::n42(s, out),
		['l', s @ ..] => super::t1::n43(s, out),
		['m', s @ ..] => super::t1::n44(s, out),
		['n', s @ ..] => super::t1::n45(s, out),
		['p', s @ ..] => super::t1::n46(s, out),
		['r', s @ ..] => super::t1::n47(s, out),
		['s', s @ ..] => super::t1::n48(s, out),
		['t', s @ ..] => super::t1::n49(s, out),
		['u', s @ ..] => super::t1::n50(s, out),
		['v', s @ ..] => super::t1::n51(s, out),
		['w', s @ ..] => super::t1::n52(s, out),
		['y', s @ ..] => super::t1::n53(s, out),
		['z', s @ ..] => super::t1::n54(s, out),
		['ł', s @ ..] => super::t1::n55(s, out),
		['ź', s @ ..] => super::t1::n56(s, out),
		['ż', s @ ..] => super::t1::n57(s, out),
		_ => false,
	}
}

/// Words ending -ob.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 743);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7405);
							s.is_empty()
						}
						['l', 'g', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7406);
							s.is_empty()
						}
						['s', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 744);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 745);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 746);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3265);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rb.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 749);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 750);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3267);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3268);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3269);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				['w', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ub.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 751);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 752);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 3272);
					s.is_empty()
				}
				['o', 'm', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 754);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7408);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3277);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3278);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3280);
					s.is_empty()
				}
				['d', 'o', 'b', 'ę', 'z', '\u{1}', s @ ..] => {
					super::push(out, 3281);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yb.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 758);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3282);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3284);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7412);
							s.is_empty()
						}
						['a', 'd', 'e', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ób.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 762);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['z', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7414);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7415);
							match s {
								[] => true,
								['t', 'ł', 'ó', 'ż', '\u{1}', s @ ..] => {
									super::push(out, 11082);
									s.is_empty()
								}
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
			super::push(out, 764);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3285);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3287);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3288);
					match s {
						[] => true,
						['i', 'c', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7418);
							s.is_empty()
						}
						['k', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['ż', '\u{1}', s @ ..] => {
					super::push(out, 3291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąb.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 3292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3293);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 7420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3294);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3295);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7421);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7422);
							match s {
								[] => true,
								['e', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 11083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3296);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7423);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 11084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7424);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3297);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3298);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3299);
					s.is_empty()
				}
				['s', 'o', 'w', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3300);
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
			super::push(out, 71);
			match s {
				[] => true,
				['j', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 3301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n73(s, out),
		['i', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 3331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 795);
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
					super::push(out, 796);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 3335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 798);
					match s {
						[] => true,
						['l', 'g', 'i', 'm', '\u{1}', s @ ..] => {
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
			super::push(out, 77);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 800);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['e', 'z', s @ ..] => {
							super::push(out, 3339);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['ł', s @ ..] => {
							super::push(out, 3342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 805);
					match s {
						[] => true,
						['m', 'z', 's', 't', 'l', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3345);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 811);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['ą', 'r', 'o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3348);
							s.is_empty()
						}
						['ę', 'i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7510);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 7511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['d', 'z', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ń', 'a', 't', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -d.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3349);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 7512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3350);
							match s {
								[] => true,
								['ą', s @ ..] => {
									super::push(out, 7513);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11203);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 3351);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3352);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11204);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 11205);
											s.is_empty()
										}
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
					super::push(out, 820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3353);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3354);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 7514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3355);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['a', 'ż', '\u{1}', s @ ..] => {
											super::push(out, 11206);
											s.is_empty()
										}
										['y', 'r', s @ ..] => {
											super::push(out, 11207);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 13577);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 7515);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 11208);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'ż', s @ ..] => {
									super::push(out, 7516);
									match s {
										[] => true,
										['ą', 'i', 's', 'k', '\u{1}', s @ ..] => {
											super::push(out, 11209);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 7517);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11210);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', 'e', s @ ..] => {
									super::push(out, 7518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'o', 'm', 'r', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 821);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3357);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 822);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 3359);
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', s @ ..] => {
							super::push(out, 3360);
							match s {
								[] => true,
								['s', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 7521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 3361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 825);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3362);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3363);
							match s {
								[] => true,
								['o', 'i', 'n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 3364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 828);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 830);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3366);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3367);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3368);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 7524);
									s.is_empty()
								}
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
			super::push(out, 86);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 832);
					match s {
						[] => true,
						['a', 'ł', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 834);
					match s {
						[] => true,
						['g', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['o', s @ ..] => super::t1::n847(s, out),
				['u', s @ ..] => {
					super::push(out, 843);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 88);
			s.is_empty()
		}
		['l', s @ ..] => super::t1::n90(s, out),
		['n', s @ ..] => super::t1::n91(s, out),
		['o', s @ ..] => super::t1::n92(s, out),
		['r', s @ ..] => super::t1::n93(s, out),
		['u', s @ ..] => super::t1::n94(s, out),
		['y', s @ ..] => super::t1::n95(s, out),
		['z', s @ ..] => super::t1::n96(s, out),
		['ó', s @ ..] => super::t1::n97(s, out),
		['ą', s @ ..] => super::t1::n98(s, out),
		['ę', s @ ..] => super::t1::n99(s, out),
		['ł', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['o', 'ż', s @ ..] => super::t1::n897(s, out),
				_ => false,
			}
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
			super::push(out, 100);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 892);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 3423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3425);
							match s {
								[] => true,
								['d', 'o', s @ ..] => {
									super::push(out, 7559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 3426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 897);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 899);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3427);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				['ń', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				['ź', s @ ..] => {
					super::push(out, 902);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['a', 'r', 'g', 'p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 104);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n107(s, out),
		['j', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 918);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 920);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3460);
							s.is_empty()
						}
						['z', 'r', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 921);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'w', 'd', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3463);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', 'm', 'y', 'r', 'p', 's', 'k', 'e', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3466);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 7615);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3467);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3468);
							match s {
								[] => true,
								['e', 'p', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7617);
									s.is_empty()
								}
								['o', 'm', '-', 'e', 'l', 'e', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3469);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 929);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3471);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3472);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 3473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3475);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3476);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 3479);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7619);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 7620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7621);
									s.is_empty()
								}
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7622);
									s.is_empty()
								}
								['d', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7623);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11305);
											s.is_empty()
										}
										['i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 11306);
											s.is_empty()
										}
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
					super::push(out, 934);
					match s {
						[] => true,
						['e', 'b', 'm', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 3483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['ę', s @ ..] => {
							super::push(out, 3484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ź', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['l', s @ ..] => super::t1::n3601(s, out),
						_ => false,
					}
				}
				['j', s @ ..] => super::t1::n949(s, out),
				['m', s @ ..] => super::t1::n950(s, out),
				['n', s @ ..] => super::t1::n951(s, out),
				['o', s @ ..] => super::t1::n952(s, out),
				['r', s @ ..] => super::t1::n953(s, out),
				['s', s @ ..] => super::t1::n954(s, out),
				['w', s @ ..] => super::t1::n955(s, out),
				['ź', s @ ..] => super::t1::n956(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t1::n113(s, out),
		['r', s @ ..] => super::t1::n114(s, out),
		['s', s @ ..] => super::t1::n115(s, out),
		['t', s @ ..] => super::t1::n116(s, out),
		['v', s @ ..] => super::t1::n117(s, out),
		['w', s @ ..] => super::t1::n118(s, out),
		['z', s @ ..] => super::t1::n119(s, out),
		['ł', s @ ..] => super::t1::n120(s, out),
		['ż', s @ ..] => super::t1::n121(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7634);
									s.is_empty()
								}
								['i', 'l', s @ ..] => {
									super::push(out, 7635);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11317);
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 13687);
													s.is_empty()
												}
												['d', s @ ..] => {
													super::push(out, 13688);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 13689);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 11318);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 11319);
											match s {
												[] => true,
												['e', 'd', s @ ..] => {
													super::push(out, 13690);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n15965(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 11320);
											match s {
												[] => true,
												['o', 's', s @ ..] => {
													super::push(out, 13691);
													s.is_empty()
												}
												['y', s @ ..] => {
													super::push(out, 13692);
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', 'o', s @ ..] => {
											super::push(out, 11321);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 11322);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 13693);
													s.is_empty()
												}
												['r', 'a', 'k', '\u{1}', s @ ..] => {
													super::push(out, 13694);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7636);
									s.is_empty()
								}
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
			super::push(out, 122);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['i', 't', 's', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3512);
							s.is_empty()
						}
						['d', 'y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3513);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['y', 's', '\u{1}', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'y', 't', '\u{1}', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 986);
					match s {
						[] => true,
						['m', 's', '\u{1}', s @ ..] => {
							super::push(out, 3515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 987);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3516);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3518);
							match s {
								[] => true,
								['z', 's', '\u{1}', s @ ..] => {
									super::push(out, 7637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3519);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 990);
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
			super::push(out, 135);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['k', 'r', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 7639);
									s.is_empty()
								}
								['p', 'o', 'r', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3520);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'n', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3523);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3525);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7641);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11323);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 7642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3526);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 7643);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'r', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 3528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1000);
					match s {
						[] => true,
						['r', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['l', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1002);
					match s {
						[] => true,
						['ź', 'd', 'u', s @ ..] => {
							super::push(out, 3531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 138);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1003);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 3532);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 3533);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1004);
					match s {
						[] => true,
						['b', 'b', 'u', 'd', s @ ..] => {
							super::push(out, 3535);
							s.is_empty()
						}
						['c', 'r', s @ ..] => {
							super::push(out, 3536);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7644);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7645);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7647);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7648);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3537);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7649);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 11324);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3538);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7650);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7651);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7652);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11325);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 7653);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7654);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7655);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11326);
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 13695);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 13696);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'c', '\u{1}', s @ ..] => {
											super::push(out, 11327);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => super::t1::n8034(s, out),
								_ => false,
							}
						}
						['m', s @ ..] => super::t1::n3660(s, out),
						['n', s @ ..] => super::t1::n3661(s, out),
						['p', s @ ..] => {
							super::push(out, 3541);
							s.is_empty()
						}
						['r', s @ ..] => super::t1::n3663(s, out),
						['s', s @ ..] => super::t1::n3664(s, out),
						['t', s @ ..] => super::t1::n3665(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n1014(s, out),
				['u', 'l', s @ ..] => super::t1::n1015(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n141(s, out),
		['r', s @ ..] => super::t1::n142(s, out),
		['u', s @ ..] => super::t1::n143(s, out),
		['y', s @ ..] => super::t1::n144(s, out),
		['z', s @ ..] => super::t1::n145(s, out),
		['ó', s @ ..] => super::t1::n146(s, out),
		['ą', s @ ..] => super::t1::n147(s, out),
		['ę', s @ ..] => super::t1::n148(s, out),
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['l', 'l', 'o', 't', 'a', 'j', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n1046(s, out),
				['e', s @ ..] => {
					super::push(out, 1037);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3607);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3608);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7708);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t1::n3733(s, out),
						['l', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3610);
							s.is_empty()
						}
						['m', s @ ..] => super::t1::n3735(s, out),
						['p', s @ ..] => super::t1::n3736(s, out),
						['z', s @ ..] => super::t1::n3737(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n1048(s, out),
				['l', s @ ..] => super::t1::n1049(s, out),
				['n', s @ ..] => super::t1::n1050(s, out),
				['o', s @ ..] => super::t1::n1051(s, out),
				['r', s @ ..] => super::t1::n1052(s, out),
				['s', s @ ..] => super::t1::n1053(s, out),
				['t', s @ ..] => super::t1::n1054(s, out),
				['u', s @ ..] => super::t1::n1055(s, out),
				['y', s @ ..] => super::t1::n1056(s, out),
				['z', s @ ..] => super::t1::n1057(s, out),
				['ę', s @ ..] => super::t1::n1058(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t1::n151(s, out),
		['o', s @ ..] => super::t1::n152(s, out),
		['s', s @ ..] => super::t1::n153(s, out),
		['u', s @ ..] => super::t1::n154(s, out),
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['z', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['b', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 3654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3655);
							match s {
								[] => true,
								['f', 'm', 'u', 't', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 7761);
									s.is_empty()
								}
								['g', 'o', 'b', 'o', 'w', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7762);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 7763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3656);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7764);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7765);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3657);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3659);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3660);
							s.is_empty()
						}
						['ą', s @ ..] => {
							super::push(out, 3661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3662);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3663);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 7767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1066);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7768);
									s.is_empty()
								}
								['l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7769);
									s.is_empty()
								}
								['r', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 7770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', 'o', 'p', 's', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3665);
							s.is_empty()
						}
						['o', 'z', 'r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3666);
							s.is_empty()
						}
						['u', 's', 'o', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1068);
					match s {
						[] => true,
						['o', 'n', 'ś', 'o', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 3668);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 3669);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1070);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3671);
							s.is_empty()
						}
						['e', 'd', s @ ..] => {
							super::push(out, 3672);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3673);
							s.is_empty()
						}
						['u', 'm', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 3674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t1::n1081(s, out),
				['t', s @ ..] => {
					super::push(out, 1072);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3685);
							match s {
								[] => true,
								['n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3686);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3687);
							s.is_empty()
						}
						['l', 'a', 'i', 'n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1074);
					s.is_empty()
				}
				['y', s @ ..] => super::t1::n1085(s, out),
				['z', s @ ..] => super::t1::n1086(s, out),
				['ę', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['i', 'z', 'd', '\u{1}', s @ ..] => super::t1::n3822(s, out),
						['r', s @ ..] => super::t1::n3823(s, out),
						_ => false,
					}
				}
				['ł', s @ ..] => super::t1::n1088(s, out),
				['ń', s @ ..] => super::t1::n1089(s, out),
				['ż', s @ ..] => super::t1::n1090(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n159(s, out),
		['t', s @ ..] => super::t1::n160(s, out),
		['w', s @ ..] => super::t1::n161(s, out),
		['z', 'z', 'a', 'r', 'a', 'p', 'a', 'p', '\u{1}', s @ ..] => super::t1::n162(s, out),
		_ => false,
	}
}

/// Words ending -j.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1090);
					match s {
						[] => true,
						['e', 'z', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 3708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3709);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 7803);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', s @ ..] => {
							super::push(out, 3710);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3711);
							s.is_empty()
						}
						['z', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['z', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							super::push(out, 3715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1095);
					match s {
						[] => true,
						['h', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['g', 'o', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3717);
							s.is_empty()
						}
						['h', 'c', 'a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 3718);
							s.is_empty()
						}
						['k', 'y', 'ż', 'e', 'i', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3719);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3720);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3721);
							match s {
								[] => true,
								['m', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 7805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7806);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11390);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3722);
							match s {
								[] => true,
								['u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3723);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3724);
							match s {
								[] => true,
								['o', 'n', 'o', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['o', 'r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3726);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 7810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 3727);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3728);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3729);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3730);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 7811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', 'd', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3732);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3733);
							s.is_empty()
						}
						['p', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3734);
							s.is_empty()
						}
						['s', 'o', 'k', 'z', 'c', 'e', 'r', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3736);
							match s {
								[] => true,
								['d', 'o', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['a', 'z', 'c', '\u{1}', s @ ..] => {
													super::push(out, 13715);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 13716);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ł', 'z', '\u{1}', s @ ..] => {
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
				['k', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 7812);
									s.is_empty()
								}
								['ż', s @ ..] => {
									super::push(out, 7813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3737);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3738);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3739);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3740);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3741);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => super::t1::n8206(s, out),
								['u', '\u{1}', s @ ..] => super::t1::n8207(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t1::n3871(s, out),
						['s', s @ ..] => super::t1::n3872(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n1119(s, out),
				['p', s @ ..] => super::t1::n1120(s, out),
				['r', s @ ..] => super::t1::n1121(s, out),
				['s', s @ ..] => super::t1::n1122(s, out),
				['z', s @ ..] => super::t1::n1123(s, out),
				['ż', s @ ..] => super::t1::n1124(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n166(s, out),
		['o', s @ ..] => super::t1::n167(s, out),
		['u', s @ ..] => super::t1::n168(s, out),
		['y', s @ ..] => super::t1::n169(s, out),
		['ó', s @ ..] => super::t1::n170(s, out),
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['a', s @ ..] => super::t1::n172(s, out),
		['c', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3906);
							match s {
								[] => true,
								['d', 'e', 'e', 'f', 'o', s @ ..] => {
									super::push(out, 8026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 3907);
							match s {
								[] => true,
								['k', 'c', 'a', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 3908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1151);
					match s {
						[] => true,
						['t', 's', 'p', 'a', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 3909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['l', 'r', 'e', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 3910);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n174(s, out),
		['i', s @ ..] => super::t1::n175(s, out),
		['j', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['r', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8581);
									s.is_empty()
								}
								['i', 'm', 'a', 'ł', '\u{1}', s @ ..] => {
									super::push(out, 8582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1187);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 4145);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4146);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4148);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 4150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4151);
							s.is_empty()
						}
						['d', 'b', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4152);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4156);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4158);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4159);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1196);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4161);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4162);
							match s {
								[] => true,
								['b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 8586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'b', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1198);
					match s {
						[] => true,
						['a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 4165);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4167);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4168);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 8587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 4169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4170);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4171);
							match s {
								[] => true,
								['g', 'n', 'i', 'n', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8588);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4172);
							match s {
								[] => true,
								['o', 'n', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4173);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							super::push(out, 4174);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4175);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4176);
							match s {
								[] => true,
								['e', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 8590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n1215(s, out),
				['o', s @ ..] => super::t1::n1216(s, out),
				['p', s @ ..] => super::t1::n1217(s, out),
				['r', s @ ..] => super::t1::n1218(s, out),
				['t', s @ ..] => super::t1::n1219(s, out),
				['w', s @ ..] => super::t1::n1220(s, out),
				['z', s @ ..] => super::t1::n1221(s, out),
				['ł', s @ ..] => super::t1::n1222(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n180(s, out),
		['s', s @ ..] => super::t1::n181(s, out),
		['u', s @ ..] => super::t1::n182(s, out),
		['w', s @ ..] => super::t1::n183(s, out),
		['y', s @ ..] => super::t1::n184(s, out),
		['ó', s @ ..] => super::t1::n185(s, out),
		['ü', s @ ..] => super::t1::n186(s, out),
		['ą', s @ ..] => super::t1::n187(s, out),
		['ę', s @ ..] => super::t1::n188(s, out),
		['ł', s @ ..] => super::t1::n189(s, out),
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4295);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4296);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4297);
							s.is_empty()
						}
						['r', 'e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4298);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 4299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4301);
							s.is_empty()
						}
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4302);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4303);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4304);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 12209);
											match s {
												[] => true,
												['s', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n16137(s, out),
														['s', s @ ..] => super::t1::n16138(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['w', s @ ..] => {
											super::push(out, 12210);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 14214);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', 'z', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1257);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4306);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1258);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4308);
							s.is_empty()
						}
						['e', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8712);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4309);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4310);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4311);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4312);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8714);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ą', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1261);
					match s {
						[] => true,
						['g', 'a', 'l', 'e', 'p', 'y', s @ ..] => {
							super::push(out, 4314);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 4315);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1262);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4317);
							match s {
								[] => true,
								['z', 's', '\u{1}', s @ ..] => {
									super::push(out, 8716);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4318);
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4319);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4320);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8717);
									s.is_empty()
								}
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1263);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4321);
							match s {
								[] => true,
								['i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n1278(s, out),
				['o', s @ ..] => super::t1::n1279(s, out),
				['p', s @ ..] => super::t1::n1280(s, out),
				['r', s @ ..] => super::t1::n1281(s, out),
				['s', s @ ..] => super::t1::n1282(s, out),
				['t', s @ ..] => super::t1::n1283(s, out),
				['v', s @ ..] => super::t1::n1284(s, out),
				['w', s @ ..] => super::t1::n1285(s, out),
				['z', s @ ..] => super::t1::n1286(s, out),
				['ż', s @ ..] => super::t1::n1287(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t1::n191(s, out),
		['e', s @ ..] => super::t1::n192(s, out),
		['f', s @ ..] => super::t1::n193(s, out),
		['i', s @ ..] => super::t1::n194(s, out),
		['j', s @ ..] => super::t1::n195(s, out),
		['k', s @ ..] => super::t1::n196(s, out),
		['l', s @ ..] => super::t1::n197(s, out),
		['o', s @ ..] => super::t1::n198(s, out),
		['r', s @ ..] => super::t1::n199(s, out),
		['t', s @ ..] => super::t1::n200(s, out),
		['u', s @ ..] => super::t1::n201(s, out),
		['y', s @ ..] => super::t1::n202(s, out),
		['ó', s @ ..] => super::t1::n203(s, out),
		['ś', s @ ..] => super::t1::n204(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['e', 'r', 't', 's', 'n', s @ ..] => {
					super::push(out, 1358);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1359);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8892);
									s.is_empty()
								}
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1360);
					match s {
						[] => true,
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 4533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1361);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1362);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1363);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1364);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4537);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 8895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8896);
									s.is_empty()
								}
								['a', 'k', s @ ..] => {
									super::push(out, 8897);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 12261);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'l', s @ ..] => {
									super::push(out, 8898);
									match s {
										[] => true,
										['i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 12262);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 12263);
											s.is_empty()
										}
										['l', 'i', s @ ..] => {
											super::push(out, 12264);
											match s {
												[] => true,
												['k', '\u{1}', s @ ..] => {
													super::push(out, 14226);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'k', 'e', s @ ..] => {
											super::push(out, 12265);
											match s {
												[] => true,
												['h', '\u{1}', s @ ..] => {
													super::push(out, 14227);
													s.is_empty()
												}
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
					super::push(out, 1365);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4539);
							match s {
								[] => true,
								['p', 'o', 'p', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1366);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'r', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1369);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8900);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1371);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4542);
							match s {
								[] => true,
								['z', 'u', 't', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4543);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4544);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4545);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1372);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['l', 's', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1377);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1378);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
							super::push(out, 4547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4548);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1380);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4550);
							s.is_empty()
						}
						['n', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 4552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4553);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8904);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1384);
					s.is_empty()
				}
				['n', s @ ..] => super::t1::n1401(s, out),
				['r', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4557);
							s.is_empty()
						}
						['b', s @ ..] => super::t1::n4722(s, out),
						['d', s @ ..] => super::t1::n4723(s, out),
						['h', s @ ..] => super::t1::n4724(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t1::n1403(s, out),
				['t', s @ ..] => super::t1::n1404(s, out),
				['ł', s @ ..] => super::t1::n1405(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n212(s, out),
		['t', s @ ..] => super::t1::n213(s, out),
		['u', s @ ..] => super::t1::n214(s, out),
		['y', s @ ..] => super::t1::n215(s, out),
		['z', s @ ..] => super::t1::n216(s, out),
		['ł', s @ ..] => super::t1::n217(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => super::t1::n218(s, out),
		['b', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4698);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									super::push(out, 9031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4699);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4701);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4702);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9032);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 9033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4703);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4705);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 9034);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 12344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4706);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9035);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 12345);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 4707);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['k', 'r', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 4712);
							s.is_empty()
						}
						['l', 'e', 't', 'n', 'e', 'ż', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4713);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4714);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4715);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'e', 't', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 9037);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 9038);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12346);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', 'e', 'ż', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4717);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4718);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 9039);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 9040);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 12347);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4719);
							match s {
								[] => true,
								['w', 'ś', '\u{1}', s @ ..] => {
									super::push(out, 9042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4720);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4721);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 4723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1447);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4724);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4726);
							match s {
								[] => true,
								['ł', 'b', '\u{1}', s @ ..] => {
									super::push(out, 9043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 4727);
							match s {
								[] => true,
								['e', 'z', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4728);
							s.is_empty()
						}
						['u', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 220);
			s.is_empty()
		}
		['i', s @ ..] => super::t1::n222(s, out),
		['j', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['i', 'ż', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n226(s, out),
		['p', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['a', 'g', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4867);
							match s {
								[] => true,
								['i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 9161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n229(s, out),
		['w', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['o', s @ ..] => super::t1::n1532(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t1::n231(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['m', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1526);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1528);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['p', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1530);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['a', 'i', 'n', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1536);
					s.is_empty()
				}
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1537);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1539);
					match s {
						[] => true,
						['l', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4925);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1540);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 4927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1541);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4928);
							s.is_empty()
						}
						['b', 'o', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4929);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4930);
							s.is_empty()
						}
						['p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 4933);
							s.is_empty()
						}
						['u', 'd', 'a', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4935);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4936);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4937);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4938);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9189);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 9190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 4939);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4940);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 9191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1548);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', 'i', 'z', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9192);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 9193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'd', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4942);
							s.is_empty()
						}
						['e', 'p', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['a', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1555);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1556);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1557);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1559);
					match s {
						[] => true,
						['a', 'm', s @ ..] => super::t1::n5132(s, out),
						['o', s @ ..] => super::t1::n5133(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n1580(s, out),
				['r', s @ ..] => super::t1::n1581(s, out),
				['s', s @ ..] => super::t1::n1582(s, out),
				['t', s @ ..] => super::t1::n1583(s, out),
				['w', s @ ..] => super::t1::n1584(s, out),
				['y', s @ ..] => super::t1::n1585(s, out),
				['z', s @ ..] => super::t1::n1586(s, out),
				['ł', s @ ..] => super::t1::n1587(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n241(s, out),
		['m', s @ ..] => super::t1::n242(s, out),
		['n', s @ ..] => super::t2::n243(s, out),
		['p', s @ ..] => super::t2::n244(s, out),
		['r', s @ ..] => super::t2::n245(s, out),
		['s', s @ ..] => super::t2::n246(s, out),
		['t', s @ ..] => super::t2::n247(s, out),
		['w', s @ ..] => super::t2::n248(s, out),
		['z', s @ ..] => super::t2::n249(s, out),
		['ł', s @ ..] => super::t2::n250(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5017);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5018);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 9244);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1635);
					s.is_empty()
				}
				['l', 'u', 'k', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1636);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5020);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 9245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5021);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 5022);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1638);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1639);
					match s {
						[] => true,
						['h', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5023);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9247);
									s.is_empty()
								}
								['k', 'r', 'u', 'z', 'c', 'z', 's', 'a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 9248);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['t', 'a', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 12439);
											s.is_empty()
										}
										['u', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
											super::push(out, 12440);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 251);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 1641);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1642);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 5026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1644);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 5027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1645);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 5028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1646);
					match s {
						[] => true,
						['s', 'b', s @ ..] => {
							super::push(out, 5029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1647);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 5030);
							match s {
								[] => true,
								['ł', s @ ..] => {
									super::push(out, 9249);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 5031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 5033);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 1653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', 'o', 'p', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9250);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 9251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5036);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 9252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 5037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1656);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 5038);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 5039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1657);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 5040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5041);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9253);
									s.is_empty()
								}
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 9254);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 9255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 's', s @ ..] => {
							super::push(out, 5042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9256);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n1682(s, out),
				['m', s @ ..] => {
					super::push(out, 1661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n1684(s, out),
				['p', s @ ..] => super::t2::n1685(s, out),
				['r', s @ ..] => super::t2::n1686(s, out),
				['t', s @ ..] => super::t2::n1687(s, out),
				['z', s @ ..] => super::t2::n1688(s, out),
				['ł', s @ ..] => super::t2::n1689(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n257(s, out),
		['u', s @ ..] => super::t2::n258(s, out),
		['y', s @ ..] => super::t2::n259(s, out),
		['ą', s @ ..] => super::t2::n260(s, out),
		['ę', s @ ..] => super::t2::n261(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5070);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 5071);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12446);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 12447);
											s.is_empty()
										}
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
					super::push(out, 1686);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5072);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5073);
							match s {
								[] => true,
								['e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9275);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5074);
							match s {
								[] => true,
								['p', 's', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5075);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 9277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5077);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9278);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5078);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1689);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5080);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5081);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5082);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5083);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5084);
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 9279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 5085);
							match s {
								[] => true,
								['t', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 9280);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 5086);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 5087);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 5088);
							match s {
								[] => true,
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 9281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1690);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5089);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9282);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 12448);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12449);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9283);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 9284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5091);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5092);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 9285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5093);
							s.is_empty()
						}
						['z', 'd', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['o', s @ ..] => super::t2::n5290(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t2::n1715(s, out),
				['l', s @ ..] => super::t2::n1716(s, out),
				['m', s @ ..] => super::t2::n1717(s, out),
				['n', s @ ..] => super::t2::n1718(s, out),
				['p', s @ ..] => super::t2::n1719(s, out),
				['s', s @ ..] => super::t2::n1720(s, out),
				['t', s @ ..] => super::t2::n1721(s, out),
				['u', s @ ..] => super::t2::n1722(s, out),
				['v', 'i', 'l', 'o', 'b', '\u{1}', s @ ..] => super::t2::n1723(s, out),
				['w', s @ ..] => super::t2::n1724(s, out),
				['z', s @ ..] => super::t2::n1725(s, out),
				['ż', s @ ..] => super::t2::n1726(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t2::n263(s, out),
		['d', s @ ..] => super::t2::n264(s, out),
		['e', s @ ..] => super::t2::n265(s, out),
		['f', s @ ..] => super::t2::n266(s, out),
		['g', s @ ..] => super::t2::n267(s, out),
		['h', s @ ..] => super::t2::n268(s, out),
		['i', s @ ..] => super::t2::n269(s, out),
		['j', s @ ..] => super::t2::n270(s, out),
		['k', s @ ..] => super::t2::n271(s, out),
		['n', s @ ..] => super::t2::n272(s, out),
		['o', s @ ..] => super::t2::n273(s, out),
		['p', 'y', 'z', 's', '\u{1}', s @ ..] => super::t2::n274(s, out),
		['r', s @ ..] => super::t2::n275(s, out),
		['t', s @ ..] => super::t2::n276(s, out),
		['u', s @ ..] => super::t2::n277(s, out),
		['w', s @ ..] => super::t2::n278(s, out),
		['y', s @ ..] => super::t2::n279(s, out),
		['ó', s @ ..] => super::t2::n280(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5413);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5414);
							match s {
								[] => true,
								['t', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 9628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'g', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 5415);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5416);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5417);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5418);
							match s {
								[] => true,
								['i', 'z', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9629);
									s.is_empty()
								}
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 9630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'z', 'c', 'r', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1814);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5420);
							s.is_empty()
						}
						['n', 'o', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5421);
							s.is_empty()
						}
						['u', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5422);
							s.is_empty()
						}
						['ę', 'z', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 5423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1815);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5424);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 9631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1816);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5426);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 9632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1817);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5427);
							s.is_empty()
						}
						['n', 'e', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5428);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5429);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 5430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1818);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5431);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5433);
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 9634);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 9635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'y', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5434);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5435);
							match s {
								[] => true,
								['w', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 9636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5436);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5437);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 9637);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12662);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 9638);
									match s {
										[] => true,
										['a', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 12663);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5438);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1821);
					match s {
						[] => true,
						['a', 'ł', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5440);
							s.is_empty()
						}
						['y', 'r', 'p', s @ ..] => {
							super::push(out, 5441);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1822);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5442);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5443);
							match s {
								[] => true,
								['c', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9641);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5445);
							s.is_empty()
						}
						['i', 'c', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5446);
							s.is_empty()
						}
						['j', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5447);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5448);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9642);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 12664);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5449);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9643);
									s.is_empty()
								}
								['m', 'a', 's', '\u{1}', s @ ..] => super::t2::n10169(s, out),
								['t', 'o', 'k', 's', '\u{1}', s @ ..] => super::t2::n10170(s, out),
								_ => false,
							}
						}
						['s', 'u', 'b', '\u{1}', s @ ..] => super::t2::n5665(s, out),
						['u', s @ ..] => super::t2::n5666(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t2::n1846(s, out),
				['s', s @ ..] => super::t2::n1847(s, out),
				['t', s @ ..] => super::t2::n1848(s, out),
				['w', s @ ..] => super::t2::n1849(s, out),
				['z', s @ ..] => super::t2::n1850(s, out),
				['ł', s @ ..] => super::t2::n1851(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t2::n282(s, out),
		['d', 'a', 'r', 't', '\u{1}', s @ ..] => super::t2::n283(s, out),
		['e', s @ ..] => super::t2::n284(s, out),
		['h', s @ ..] => super::t2::n285(s, out),
		['i', s @ ..] => super::t2::n286(s, out),
		['j', s @ ..] => super::t2::n287(s, out),
		['k', s @ ..] => super::t2::n288(s, out),
		['l', s @ ..] => super::t2::n289(s, out),
		['m', s @ ..] => super::t2::n290(s, out),
		['n', s @ ..] => super::t2::n291(s, out),
		['o', s @ ..] => super::t2::n292(s, out),
		['p', s @ ..] => super::t2::n293(s, out),
		['r', s @ ..] => super::t2::n294(s, out),
		['s', s @ ..] => super::t2::n295(s, out),
		['t', s @ ..] => super::t2::n296(s, out),
		['u', s @ ..] => super::t2::n297(s, out),
		['w', s @ ..] => super::t2::n298(s, out),
		['y', s @ ..] => super::t2::n299(s, out),
		['ą', s @ ..] => super::t2::n300(s, out),
		['ę', s @ ..] => super::t2::n301(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1923);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5668);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5669);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 9793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5670);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5671);
							match s {
								[] => true,
								['o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 9794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1924);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 5673);
							s.is_empty()
						}
						['ł', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 5674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1925);
					match s {
						[] => true,
						['r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1926);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5676);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5677);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9795);
									s.is_empty()
								}
								['r', 'g', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'b', s @ ..] => {
							super::push(out, 5678);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 9797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1927);
					match s {
						[] => true,
						['c', 'r', s @ ..] => {
							super::push(out, 5679);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1928);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5681);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5682);
							match s {
								[] => true,
								['o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 9798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'ś', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5683);
							s.is_empty()
						}
						['r', s @ ..] => super::t2::n5910(s, out),
						['w', s @ ..] => super::t2::n5911(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t2::n1955(s, out),
				['l', s @ ..] => super::t2::n1956(s, out),
				['m', s @ ..] => super::t2::n1957(s, out),
				['n', s @ ..] => super::t2::n1958(s, out),
				['p', s @ ..] => super::t2::n1959(s, out),
				['r', s @ ..] => super::t2::n1960(s, out),
				['s', s @ ..] => {
					super::push(out, 1935);
					match s {
						[] => true,
						['a', s @ ..] => super::t2::n5951(s, out),
						['e', s @ ..] => super::t2::n5952(s, out),
						['s', s @ ..] => super::t2::n5953(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t2::n1962(s, out),
				['v', s @ ..] => super::t2::n1963(s, out),
				['w', s @ ..] => super::t2::n1964(s, out),
				['z', s @ ..] => super::t2::n1965(s, out),
				['ł', s @ ..] => super::t2::n1966(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t2::n303(s, out),
		['e', s @ ..] => super::t2::n304(s, out),
		['f', s @ ..] => super::t2::n305(s, out),
		['h', s @ ..] => super::t2::n306(s, out),
		['i', s @ ..] => super::t2::n307(s, out),
		['j', s @ ..] => super::t2::n308(s, out),
		['k', s @ ..] => super::t2::n309(s, out),
		['l', s @ ..] => super::t2::n310(s, out),
		['n', s @ ..] => super::t2::n311(s, out),
		['o', s @ ..] => super::t2::n312(s, out),
		['p', s @ ..] => super::t2::n313(s, out),
		['r', s @ ..] => super::t2::n314(s, out),
		['s', s @ ..] => super::t2::n315(s, out),
		['u', s @ ..] => super::t2::n316(s, out),
		['y', s @ ..] => super::t2::n317(s, out),
		['z', s @ ..] => super::t2::n318(s, out),
		['ó', s @ ..] => super::t2::n319(s, out),
		['ą', s @ ..] => super::t2::n320(s, out),
		['ę', s @ ..] => super::t2::n321(s, out),
		['ł', s @ ..] => super::t2::n322(s, out),
		['ń', s @ ..] => super::t2::n323(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 323);
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
		['i', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2046);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -w.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 2048);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2049);
					match s {
						[] => true,
						['ę', s @ ..] => {
							super::push(out, 6017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2050);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 6018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2052);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6020);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6021);
							match s {
								[] => true,
								['ż', '\u{1}', s @ ..] => {
									super::push(out, 10098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2053);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2054);
					match s {
						[] => true,
						['ż', s @ ..] => {
							super::push(out, 6022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2055);
					match s {
						[] => true,
						['c', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 6023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2056);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10099);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['d', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2057);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2058);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2059);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6026);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 6027);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6028);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 10101);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6029);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6030);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6032);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6033);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10102);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 10103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6034);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10104);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6035);
							match s {
								[] => true,
								['k', 'w', 'ó', 'r', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10105);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2062);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6037);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 6038);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2064);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6040);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 10106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6041);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10107);
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 14429);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n16229(s, out),
														_ => false,
													}
												}
												['ł', s @ ..] => {
													super::push(out, 14430);
													match s {
														[] => true,
														['a', s @ ..] => super::t2::n16230(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 10108);
									match s {
										[] => true,
										['o', 'z', s @ ..] => {
											super::push(out, 12800);
											match s {
												[] => true,
												['r', 'e', 'i', 'w', 'z', '\u{1}', s @ ..] => {
													super::push(out, 14431);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10109);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 12801);
											match s {
												[] => true,
												['i', 'c', '\u{1}', s @ ..] => {
													super::push(out, 14432);
													s.is_empty()
												}
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
							super::push(out, 6042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['h', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['z', 'd', 'o', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2069);
					s.is_empty()
				}
				['t', 'a', 'z', 'c', 'r', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n2104(s, out),
				['e', s @ ..] => super::t2::n2105(s, out),
				['u', s @ ..] => super::t2::n2106(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['y', s @ ..] => super::t2::n334(s, out),
		['ó', s @ ..] => super::t2::n335(s, out),
		['ł', s @ ..] => super::t2::n336(s, out),
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['f', 'i', 't', 'n', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['b', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				['d', 'n', 'e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2087);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2088);
					match s {
						[] => true,
						['t', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['b', 'm', 'o', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -y.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2090);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2091);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 6061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2092);
					match s {
						[] => true,
						['c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2093);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2094);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2096);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6063);
							s.is_empty()
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2097);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2098);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 2099);
					match s {
						[] => true,
						['r', 't', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2100);
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2101);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2102);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2103);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 6069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 2104);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 6070);
							match s {
								[] => true,
								['a', 'z', s @ ..] => {
									super::push(out, 10125);
									s.is_empty()
								}
								['u', 'l', s @ ..] => {
									super::push(out, 10126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2105);
					match s {
						[] => true,
						['i', 'n', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2106);
					match s {
						[] => true,
						['ł', s @ ..] => {
							super::push(out, 6073);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 10127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 'b', 'm', 'e', 'k', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2107);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2108);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 6074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2111);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2112);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2113);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2114);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 6075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2115);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2116);
					match s {
						[] => true,
						['f', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6076);
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
					super::push(out, 2117);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6077);
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 10128);
									match s {
										[] => true,
										['a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 12808);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 6078);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'r', s @ ..] => {
							super::push(out, 6079);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 10130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6080);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 6081);
							match s {
								[] => true,
								['o', 'h', s @ ..] => {
									super::push(out, 10132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 6082);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 10133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2119);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2120);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2121);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2122);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6085);
							match s {
								[] => true,
								['d', 'e', s @ ..] => {
									super::push(out, 10134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 2123);
					match s {
						[] => true,
						['y', 'c', 'b', 'r', 'o', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2124);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6087);
							match s {
								[] => true,
								['i', 'p', s @ ..] => {
									super::push(out, 10135);
									match s {
										[] => true,
										['y', 's', '\u{1}', s @ ..] => {
											super::push(out, 12809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6088);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 10136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2125);
					s.is_empty()
				}
				['n', s @ ..] => super::t2::n2160(s, out),
				['o', s @ ..] => super::t2::n2161(s, out),
				['p', s @ ..] => super::t2::n2162(s, out),
				['r', s @ ..] => super::t2::n2163(s, out),
				['t', s @ ..] => super::t2::n2164(s, out),
				['w', s @ ..] => super::t2::n2165(s, out),
				['y', s @ ..] => super::t2::n2166(s, out),
				['z', s @ ..] => super::t2::n2167(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n348(s, out),
		['p', s @ ..] => super::t2::n349(s, out),
		['r', s @ ..] => super::t2::n350(s, out),
		['s', s @ ..] => super::t2::n351(s, out),
		['t', s @ ..] => super::t2::n352(s, out),
		['w', s @ ..] => super::t2::n353(s, out),
		['z', s @ ..] => super::t2::n354(s, out),
		['ł', s @ ..] => super::t2::n355(s, out),
		['ż', s @ ..] => super::t2::n356(s, out),
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2187);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6163);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2188);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2189);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6164);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6165);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2192);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6166);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6167);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10166);
									match s {
										[] => true,
										['ę', 'z', 's', 'w', '\u{1}', s @ ..] => {
											super::push(out, 12817);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 10167);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10168);
									match s {
										[] => true,
										['ó', 'g', '\u{1}', s @ ..] => {
											super::push(out, 12818);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10169);
									match s {
										[] => true,
										['o', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 12819);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', 'e', 'z', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'u', s @ ..] => {
							super::push(out, 6168);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n358(s, out),
		['d', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10281);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10282);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10283);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12941);
											match s {
												[] => true,
												['o', 'k', 's', '\u{1}', s @ ..] => {
													super::push(out, 14482);
													s.is_empty()
												}
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
					super::push(out, 2209);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 2212);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 6225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 2213);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10284);
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 12942);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 12943);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 12944);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 10285);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10286);
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 12945);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 12946);
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 10287);
									s.is_empty()
								}
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
			super::push(out, 358);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2215);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2216);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 6227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2217);
					match s {
						[] => true,
						['o', 'r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2218);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6229);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6230);
							match s {
								[] => true,
								['l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2220);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2223);
					match s {
						[] => true,
						['r', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2224);
					match s {
						[] => true,
						['r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 6234);
							s.is_empty()
						}
						['o', 'p', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6235);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 6236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				['t', 'p', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				['n', 'r', 'e', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2233);
					s.is_empty()
				}
				['u', s @ ..] => super::t2::n2268(s, out),
				['ł', s @ ..] => super::t2::n2269(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n363(s, out),
		['s', s @ ..] => super::t2::n364(s, out),
		['u', s @ ..] => super::t2::n365(s, out),
		['y', s @ ..] => super::t2::n366(s, out),
		['z', s @ ..] => super::t2::n367(s, out),
		['ó', s @ ..] => super::t2::n368(s, out),
		['ą', s @ ..] => super::t2::n369(s, out),
		['ę', s @ ..] => super::t2::n370(s, out),
		_ => false,
	}
}

/// Words ending -ć.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2276);
					match s {
						[] => true,
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6351);
							match s {
								[] => true,
								['a', 'ł', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 10455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6352);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6353);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 6354);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6355);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10456);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 13034);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 10457);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 10458);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13035);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6356);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 10459);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 10460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6357);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 10461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6358);
							s.is_empty()
						}
						['w', 'd', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2279);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2280);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6360);
							match s {
								[] => true,
								['i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10462);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 10463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6361);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2282);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6363);
							match s {
								[] => true,
								['p', 'i', 'w', 'r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 10464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['b', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2283);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2289);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6367);
							match s {
								[] => true,
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 10465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 6368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2290);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 6369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['r', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 6370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'z', 'r', '\u{1}', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['ą', 'p', '\u{1}', s @ ..] => {
			super::push(out, 377);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['i', 'z', s @ ..] => {
					super::push(out, 2296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ś', s @ ..] => super::t2::n381(s, out),
		_ => false,
	}
}

/// Words ending -ę.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['ą', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ł.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2312);
					match s {
						[] => true,
						['m', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6393);
							s.is_empty()
						}
						['y', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2313);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6395);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6396);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', 'e', 'c', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				['g', 'e', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2316);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 6398);
							s.is_empty()
						}
						['m', 'e', s @ ..] => {
							super::push(out, 6399);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6400);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 10512);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											super::push(out, 13115);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', 'd', 'u', s @ ..] => {
							super::push(out, 6401);
							match s {
								[] => true,
								['ł', s @ ..] => {
									super::push(out, 10513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 2317);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', 'f', s @ ..] => {
									super::push(out, 10514);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 13116);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', s @ ..] => {
									super::push(out, 10515);
									match s {
										[] => true,
										['w', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 13117);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2318);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 6402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2319);
					match s {
						[] => true,
						['i', 'g', 'y', s @ ..] => {
							super::push(out, 6403);
							s.is_empty()
						}
						['o', 'j', 'c', 'a', s @ ..] => {
							super::push(out, 6404);
							match s {
								[] => true,
								['r', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 6405);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10517);
									match s {
										[] => true,
										['r', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 13118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 10518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'y', 'c', 'n', 'y', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2320);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2321);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10519);
									match s {
										[] => true,
										['i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 13119);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10520);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 10521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6406);
							match s {
								[] => true,
								['m', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10522);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2323);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 6408);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6409);
							match s {
								[] => true,
								['ę', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6410);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10524);
									match s {
										[] => true,
										['o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 13120);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['r', 'd', 'z', 'i', 'w', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 6412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2325);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6413);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2326);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6415);
							match s {
								[] => true,
								['r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10525);
									s.is_empty()
								}
								['y', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 10526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 6416);
							match s {
								[] => true,
								['z', 's', '\u{1}', s @ ..] => {
									super::push(out, 10527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6417);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 10528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2327);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2328);
					match s {
						[] => true,
						['e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6418);
							s.is_empty()
						}
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6419);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6420);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6422);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 10529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6423);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 6424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 6425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n388(s, out),
		['o', s @ ..] => super::t2::n389(s, out),
		['s', s @ ..] => super::t2::n390(s, out),
		['u', s @ ..] => super::t2::n391(s, out),
		['y', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['ó', s @ ..] => super::t2::n393(s, out),
		_ => false,
	}
}

/// Words ending -ń.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6451);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 10543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2353);
					match s {
						[] => true,
						['z', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6454);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 10544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 6455);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10545);
									match s {
										[] => true,
										['i', 'w', 'k', '\u{1}', s @ ..] => {
											super::push(out, 13129);
											s.is_empty()
										}
										['l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 13130);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 10546);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 13131);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 10547);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 10548);
									match s {
										[] => true,
										['ó', 'ż', '\u{1}', s @ ..] => {
											super::push(out, 13132);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ś', s @ ..] => {
									super::push(out, 10549);
									match s {
										[] => true,
										['r', 'e', 'i', 's', s @ ..] => {
											super::push(out, 13133);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 14593);
													s.is_empty()
												}
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
							super::push(out, 6456);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 10550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6457);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10551);
									match s {
										[] => true,
										['e', 'z', 'c', '\u{1}', s @ ..] => {
											super::push(out, 13134);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10552);
									s.is_empty()
								}
								['e', 'l', s @ ..] => {
									super::push(out, 10553);
									match s {
										[] => true,
										['ś', '\u{1}', s @ ..] => {
											super::push(out, 13135);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => super::t2::n11148(s, out),
								['o', s @ ..] => super::t2::n11149(s, out),
								['r', s @ ..] => super::t2::n11150(s, out),
								['u', s @ ..] => super::t2::n11151(s, out),
								['ę', s @ ..] => super::t2::n11152(s, out),
								_ => false,
							}
						}
						['s', s @ ..] => super::t2::n6719(s, out),
						['w', s @ ..] => super::t2::n6720(s, out),
						['z', s @ ..] => super::t2::n6721(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n2391(s, out),
				['r', s @ ..] => super::t2::n2392(s, out),
				['s', s @ ..] => super::t2::n2393(s, out),
				['t', s @ ..] => super::t2::n2394(s, out),
				['z', s @ ..] => super::t2::n2395(s, out),
				['ż', s @ ..] => super::t2::n2396(s, out),
				_ => false,
			}
		}
		['m', 'e', 'i', 'c', '\u{1}', s @ ..] => super::t2::n396(s, out),
		['o', s @ ..] => super::t2::n397(s, out),
		['p', s @ ..] => super::t2::n398(s, out),
		['r', s @ ..] => super::t2::n399(s, out),
		['u', s @ ..] => super::t2::n400(s, out),
		['y', s @ ..] => super::t2::n401(s, out),
		['ś', s @ ..] => super::t2::n402(s, out),
		['ź', s @ ..] => super::t2::n403(s, out),
		_ => false,
	}
}

/// Words ending -ś.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['d', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2384);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2385);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2386);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2387);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2388);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2390);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6495);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6496);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10592);
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2391);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2393);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2394);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2395);
					match s {
						[] => true,
						['d', 'a', 'ł', 'k', 'u', '\u{1}', s @ ..] => {
							super::push(out, 6497);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6498);
							s.is_empty()
						}
						['k', 'ę', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6499);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6500);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 10594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'k', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2396);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'n', 'o', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 2398);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2399);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6504);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6506);
							s.is_empty()
						}
						['z', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2405);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6508);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6509);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['a', 'i', 'z', 'd', '\u{1}', s @ ..] => {
											super::push(out, 13157);
											s.is_empty()
										}
										['i', 'z', 'd', '\u{1}', s @ ..] => {
											super::push(out, 13158);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6510);
							s.is_empty()
						}
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2407);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6512);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 6513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2408);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2409);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 6514);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2410);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 10595);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 10596);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2411);
					match s {
						[] => true,
						['a', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6516);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 6517);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10597);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6518);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 10598);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 10599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6519);
							s.is_empty()
						}
						['n', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2413);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 2414);
					match s {
						[] => true,
						['ę', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2415);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6523);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2416);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6525);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6526);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10600);
									s.is_empty()
								}
								['o', 'ł', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2417);
					match s {
						[] => true,
						['o', 'r', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 6527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ź.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2422);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2423);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6529);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 10602);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10603);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 10604);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 13159);
											match s {
												[] => true,
												['p', 'o', 'd', s @ ..] => {
													super::push(out, 14603);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n16293(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ź', s @ ..] => {
											super::push(out, 13160);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 14604);
													match s {
														[] => true,
														['e', 'i', 'n', '\u{1}', s @ ..] => super::t2::n16294(s, out),
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
							super::push(out, 6530);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10605);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10606);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 10607);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13161);
											s.is_empty()
										}
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
					super::push(out, 2424);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2425);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 6531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 2427);
					match s {
						[] => true,
						['b', 'a', 'ł', '\u{1}', s @ ..] => {
							super::push(out, 6532);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 6533);
							match s {
								[] => true,
								['o', 'ż', '\u{1}', s @ ..] => {
									super::push(out, 10608);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 2428);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6534);
							s.is_empty()
						}
						['ł', 'o', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 6535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ź', s @ ..] => {
					super::push(out, 2429);
					match s {
						[] => true,
						['ó', s @ ..] => {
							super::push(out, 6536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['t', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'h', '\u{1}', s @ ..] => {
			super::push(out, 413);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['ł', s @ ..] => {
					super::push(out, 2431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['i', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6537);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ż.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2432);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6539);
							match s {
								[] => true,
								['z', 'r', 'p', 's', '-', s @ ..] => {
									super::push(out, 10609);
									match s {
										[] => true,
										['o', 'n', 'p', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 13162);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6540);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10610);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 13163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6541);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'a', 't', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 2433);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2434);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2435);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10612);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 10613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 2436);
					match s {
						[] => true,
						['o', 'w', 'k', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2437);
					match s {
						[] => true,
						['b', 'm', 'a', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						['e', 'n', 'e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6546);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2438);
					match s {
						[] => true,
						['a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6548);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2439);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2440);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2441);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6550);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6551);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 10614);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13164);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2442);
					match s {
						[] => true,
						['a', 'p', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2443);
					match s {
						[] => true,
						['e', 'i', 'k', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6554);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'w', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13165);
											s.is_empty()
										}
										['z', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 13166);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 10615);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6555);
							s.is_empty()
						}
						['r', 'o', 'p', 'l', s @ ..] => {
							super::push(out, 6556);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2444);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2445);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2446);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2447);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2448);
					match s {
						[] => true,
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 2449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2450);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2451);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6560);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6561);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10616);
									match s {
										[] => true,
										['r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 13167);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6562);
							match s {
								[] => true,
								['d', 'o', 'ł', s @ ..] => {
									super::push(out, 10618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2454);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2455);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 2456);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 2457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2458);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2459);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['i', 'w', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 2464);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['m', 'r', s @ ..] => {
					super::push(out, 2467);
					s.is_empty()
				}
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t2::n426(s, out),
		['ó', s @ ..] => super::t2::n427(s, out),
		['ą', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2478);
					s.is_empty()
				}
				['z', s @ ..] => super::t2::n2518(s, out),
				_ => false,
			}
		}
		['ę', s @ ..] => super::t2::n429(s, out),
		['ł', s @ ..] => super::t2::n430(s, out),
		_ => false,
	}
}

/// Words ending -ica.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2500);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['g', 'a', s @ ..] => {
					super::push(out, 6585);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 10621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'g', s @ ..] => {
					super::push(out, 6586);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'r', s @ ..] => {
					super::push(out, 6587);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 10623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ź', s @ ..] => {
					super::push(out, 6588);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 10624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2501);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6589);
					match s {
						[] => true,
						['j', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10625);
							s.is_empty()
						}
						['r', 'g', s @ ..] => {
							super::push(out, 10626);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 13168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 6590);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'z', s @ ..] => {
					super::push(out, 6591);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6592);
					match s {
						[] => true,
						['c', 'ó', s @ ..] => {
							super::push(out, 10629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6593);
					match s {
						[] => true,
						['ł', 'p', s @ ..] => {
							super::push(out, 10630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6594);
					s.is_empty()
				}
				['ź', s @ ..] => {
					super::push(out, 6595);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2502);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 6596);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 10632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6597);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 10634);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 10635);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 10636);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 10637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6598);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mca.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	s.is_empty()
}

/// Words ending -pca.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['ę', s @ ..] => {
			super::push(out, 2506);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rca.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2507);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6602);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 6603);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10641);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 13169);
									match s {
										[] => true,
										['ą', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 14605);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13170);
									s.is_empty()
								}
								['k', 'a', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13171);
									s.is_empty()
								}
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
			super::push(out, 2508);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 2509);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dziejopisca.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	s.is_empty()
}

/// Words ending -wca.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2510);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6604);
					s.is_empty()
				}
				['m', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6605);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yca.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2512);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2513);
			match s {
				[] => true,
				['a', 'd', s @ ..] => {
					super::push(out, 6607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2514);
			match s {
				[] => true,
				['c', 'z', s @ ..] => {
					super::push(out, 6608);
					match s {
						[] => true,
						['s', 'u', s @ ..] => {
							super::push(out, 10643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', 'd', s @ ..] => {
			super::push(out, 2515);
			match s {
				[] => true,
				['ż', 'a', s @ ..] => {
					super::push(out, 6609);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zca.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	s.is_empty()
}

/// Words ending -ąca.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['i', 'p', s @ ..] => {
			super::push(out, 2516);
			s.is_empty()
		}
		['z', 'r', s @ ..] => {
			super::push(out, 2517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ńca.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	s.is_empty()
}

/// Words ending -źca.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żca.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	s.is_empty()
}

/// Words ending -da.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2519);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2520);
					match s {
						[] => true,
						['n', 'g', s @ ..] => {
							super::push(out, 6611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'ą', 's', '\u{1}', s @ ..] => {
							super::push(out, 6612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2521);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
							super::push(out, 6613);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					super::push(out, 2522);
					match s {
						[] => true,
						['ż', s @ ..] => {
							super::push(out, 6614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2523);
					s.is_empty()
				}
				['w', 's', s @ ..] => {
					super::push(out, 2524);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 2525);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2527);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6616);
							s.is_empty()
						}
						['r', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 6617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2528);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2529);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2530);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 10646);
									s.is_empty()
								}
								['i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 6620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 2533);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2534);
					match s {
						[] => true,
						['b', 'a', 'g', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6621);
							s.is_empty()
						}
						['k', 'e', 's', 'o', 'r', s @ ..] => {
							super::push(out, 6622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2535);
					s.is_empty()
				}
				['b', 'o', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 2536);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2537);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6623);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 10648);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13172);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 6624);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2538);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6626);
							s.is_empty()
						}
						['š', s @ ..] => {
							super::push(out, 6627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2539);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2540);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2541);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2542);
					match s {
						[] => true,
						['s', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6628);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6629);
							match s {
								[] => true,
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2543);
					match s {
						[] => true,
						['b', 'i', 'l', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6630);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6631);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6632);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6633);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2544);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6635);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6636);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6637);
							match s {
								[] => true,
								['c', 'ą', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10650);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 13173);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 13174);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 10651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2546);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6638);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10652);
									s.is_empty()
								}
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
			super::push(out, 469);
			match s {
				[] => true,
				['a', 'g', s @ ..] => {
					super::push(out, 2547);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6639);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', 'y', 'z', 'u', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2548);
					s.is_empty()
				}
				['o', 'm', 'y', s @ ..] => {
					super::push(out, 2549);
					match s {
						[] => true,
						['z', 'c', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['b', 'a', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 2550);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2551);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2552);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2553);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6643);
							s.is_empty()
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
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2557);
					s.is_empty()
				}
				['ł', 'b', s @ ..] => super::t2::n2602(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t2::n473(s, out),
		['y', s @ ..] => super::t2::n474(s, out),
		['z', s @ ..] => super::t2::n475(s, out),
		['ę', s @ ..] => super::t2::n476(s, out),
		_ => false,
	}
}

/// Words ending -ea.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 475);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 476);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['a', 'l', 'l', 'e', 'c', 's', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2564);
					s.is_empty()
				}
				['y', 'c', 'l', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fa.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 479);
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
			super::push(out, 480);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2566);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 2567);
					match s {
						[] => true,
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 6650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['d', 'o', s @ ..] => {
							super::push(out, 6651);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['i', 'b', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2569);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2571);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2572);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6655);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['g', 'a', 'n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 2574);
					s.is_empty()
				}
				['l', 'k', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2576);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6657);
							s.is_empty()
						}
						['i', 's', 'w', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6658);
							s.is_empty()
						}
						['r', 'e', 't', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2577);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2578);
					s.is_empty()
				}
				['z', 'c', 'y', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 2579);
					s.is_empty()
				}
				['ł', 'd', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2580);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 2581);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6661);
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
			match s {
				[] => true,
				['o', 'm', s @ ..] => {
					super::push(out, 2582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2583);
					match s {
						[] => true,
						['c', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2584);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2585);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6664);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2586);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10656);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 10657);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 13175);
											s.is_empty()
										}
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
			super::push(out, 488);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 2587);
					match s {
						[] => true,
						['i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['d', 'r', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2590);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2591);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6667);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 10658);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 10659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 6668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2592);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6669);
							match s {
								[] => true,
								['ł', '\u{1}', s @ ..] => {
									super::push(out, 10660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'ó', 'ł', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', 'o', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2593);
					s.is_empty()
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
		['a', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'n', 'w', 'ó', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6671);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 6672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2594);
					match s {
						[] => true,
						['l', 'k', s @ ..] => {
							super::push(out, 6673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'b', s @ ..] => {
							super::push(out, 6674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2595);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2596);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2597);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10662);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 10663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2598);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2599);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6678);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10664);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 6679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'y', s @ ..] => {
							super::push(out, 6680);
							match s {
								[] => true,
								['z', 'c', 'r', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 10666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 's', s @ ..] => {
							super::push(out, 6681);
							s.is_empty()
						}
						['r', 'k', s @ ..] => {
							super::push(out, 6682);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 6683);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 10667);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13176);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 's', s @ ..] => {
					super::push(out, 2600);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ia.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2601);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6684);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6685);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 10668);
									match s {
										[] => true,
										['r', 'a', s @ ..] => {
											super::push(out, 13177);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 10669);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13178);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 13179);
											s.is_empty()
										}
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
					super::push(out, 2602);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2603);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2604);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6686);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6687);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 10670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 2605);
					match s {
						[] => true,
						['ł', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 2606);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6689);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2607);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10671);
									s.is_empty()
								}
								['a', 'r', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13180);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 13181);
											s.is_empty()
										}
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
					super::push(out, 2608);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2609);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2610);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6692);
							match s {
								[] => true,
								['i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2611);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2612);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 6693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2613);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10673);
									s.is_empty()
								}
								['ł', '\u{1}', s @ ..] => {
									super::push(out, 10674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2614);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2615);
					s.is_empty()
				}
				['ń', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2617);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6694);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 10675);
									s.is_empty()
								}
								['r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 10676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2618);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2619);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'k', s @ ..] => {
					super::push(out, 2620);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2621);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6697);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10677);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 13182);
											s.is_empty()
										}
										['o', s @ ..] => super::t2::n14048(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n2671(s, out),
				['o', s @ ..] => super::t2::n2672(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n499(s, out),
		['h', s @ ..] => super::t2::n500(s, out),
		['k', 'a', 'm', s @ ..] => super::t2::n501(s, out),
		['l', s @ ..] => super::t2::n502(s, out),
		['m', s @ ..] => super::t2::n503(s, out),
		['n', s @ ..] => super::t2::n504(s, out),
		['p', s @ ..] => super::t2::n505(s, out),
		['r', s @ ..] => super::t2::n506(s, out),
		['s', s @ ..] => super::t2::n507(s, out),
		['t', s @ ..] => super::t2::n508(s, out),
		['w', s @ ..] => super::t2::n509(s, out),
		['z', s @ ..] => super::t2::n510(s, out),
		_ => false,
	}
}

