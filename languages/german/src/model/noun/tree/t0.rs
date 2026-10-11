use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => super::t0::n1(s, out),
		['b', s @ ..] => super::t0::n2(s, out),
		['c', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 84);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 85);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 925);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 927);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 86);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 87);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 932);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 88);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 934);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4319);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4320);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 935);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 936);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 937);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4323);
									match s {
										[] => true,
										['u', 'p', '-', 'g', 'n', 'i', 'o', 'g', '\u{1}', s @ ..] => {
											super::push(out, 12387);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 938);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12388);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 12389);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 939);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 940);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4325);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4326);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 89);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 942);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 90);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 944);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 945);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 946);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 948);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 91);
					match s {
						[] => true,
						['-', 't', 'e', 'l', 'b', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 92);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 93);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 94);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 95);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 96);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 955);
							s.is_empty()
						}
						['-', s @ ..] => {
							match s {
								[] => true,
								['e', 't', 's', 'ä', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4328);
									s.is_empty()
								}
								['n', 'e', 'r', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n4(s, out),
		['e', s @ ..] => super::t0::n5(s, out),
		['f', s @ ..] => super::t0::n6(s, out),
		['g', s @ ..] => super::t0::n7(s, out),
		['h', s @ ..] => super::t0::n8(s, out),
		['i', s @ ..] => super::t0::n9(s, out),
		['j', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 225);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 226);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1781);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 1782);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 1783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 228);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 230);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t0::n11(s, out),
		['l', s @ ..] => super::t0::n12(s, out),
		['m', s @ ..] => super::t0::n13(s, out),
		['n', s @ ..] => super::t0::n14(s, out),
		['o', s @ ..] => super::t0::n15(s, out),
		['p', s @ ..] => super::t0::n16(s, out),
		['q', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 366);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2599);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n18(s, out),
		['s', s @ ..] => super::t0::n19(s, out),
		['t', s @ ..] => super::t0::n20(s, out),
		['u', s @ ..] => super::t0::n21(s, out),
		['v', s @ ..] => super::t0::n22(s, out),
		['w', s @ ..] => super::t0::n23(s, out),
		['x', s @ ..] => super::t0::n24(s, out),
		['y', s @ ..] => super::t0::n25(s, out),
		['z', s @ ..] => super::t0::n26(s, out),
		['ß', s @ ..] => super::t0::n27(s, out),
		['à', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['á', s @ ..] => {
			super::push(out, 29);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['m', 'a', 'n', 'i', 'p', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 3532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => super::t0::n30(s, out),
		['é', s @ ..] => super::t0::n31(s, out),
		['í', s @ ..] => super::t0::n32(s, out),
		['ó', s @ ..] => super::t0::n33(s, out),
		['õ', s @ ..] => super::t0::n34(s, out),
		['ö', s @ ..] => super::t0::n35(s, out),
		['ü', s @ ..] => super::t0::n36(s, out),
		['č', s @ ..] => super::t0::n37(s, out),
		['ş', s @ ..] => super::t0::n38(s, out),
		['ū', s @ ..] => super::t0::n39(s, out),
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 41);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 42);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3556);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3557);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3558);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3559);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3560);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3561);
							match s {
								[] => true,
								['i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 11778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 3562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3565);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 581);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 3567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3568);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11779);
									match s {
										[] => true,
										['u', 'm', 'o', '\u{1}', s @ ..] => {
											super::push(out, 22942);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 11780);
									s.is_empty()
								}
								['z', 'a', 'b', 'm', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 11781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3569);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 11782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3570);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3571);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11783);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 11784);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 11785);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11786);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 11787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3572);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3573);
							match s {
								[] => true,
								['n', 'a', 'h', 'k', '\u{1}', s @ ..] => {
									super::push(out, 11788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3574);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3575);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11789);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3577);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 586);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 587);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3580);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 11791);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3581);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 44);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3585);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 11793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3586);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 11794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3587);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 3589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3590);
							s.is_empty()
						}
						['n', s @ ..] => super::t0::n3600(s, out),
						['p', s @ ..] => {
							super::push(out, 3592);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n595(s, out),
				['o', s @ ..] => super::t0::n596(s, out),
				['r', s @ ..] => super::t0::n597(s, out),
				['s', s @ ..] => super::t0::n598(s, out),
				['u', s @ ..] => super::t0::n599(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n45(s, out),
		['e', s @ ..] => super::t0::n46(s, out),
		['f', s @ ..] => super::t0::n47(s, out),
		['g', s @ ..] => super::t0::n48(s, out),
		['h', s @ ..] => super::t0::n49(s, out),
		['i', s @ ..] => super::t0::n50(s, out),
		['j', s @ ..] => super::t0::n51(s, out),
		['k', s @ ..] => super::t0::n52(s, out),
		['l', s @ ..] => super::t0::n53(s, out),
		['m', s @ ..] => super::t0::n54(s, out),
		['n', s @ ..] => super::t1::n55(s, out),
		['o', s @ ..] => super::t1::n56(s, out),
		['p', s @ ..] => super::t1::n57(s, out),
		['r', s @ ..] => super::t1::n58(s, out),
		['s', s @ ..] => super::t1::n59(s, out),
		['t', s @ ..] => super::t1::n60(s, out),
		['u', s @ ..] => super::t1::n61(s, out),
		['v', s @ ..] => super::t1::n62(s, out),
		['w', s @ ..] => super::t1::n63(s, out),
		['x', s @ ..] => super::t1::n64(s, out),
		['y', s @ ..] => super::t1::n65(s, out),
		['z', s @ ..] => super::t1::n66(s, out),
		['ä', s @ ..] => super::t1::n67(s, out),
		['ç', s @ ..] => super::t1::n68(s, out),
		_ => false,
	}
}

/// Words ending -b.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 865);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4239);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 866);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4241);
							match s {
								[] => true,
								['k', 'r', 'e', 'n', 'ö', 'd', '\u{1}', s @ ..] => {
									super::push(out, 12326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 867);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 868);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 869);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4244);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 872);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4246);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 874);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4250);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12327);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 23100);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 32219);
													s.is_empty()
												}
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
							super::push(out, 4251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4252);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4253);
							match s {
								[] => true,
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 12328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4254);
							match s {
								[] => true,
								['e', 'n', 'i', 'h', 'c', 's', 'a', 'm', 'l', 'ü', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 12329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4255);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12330);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', 'g', s @ ..] => {
											super::push(out, 23101);
											s.is_empty()
										}
										['l', 'o', s @ ..] => {
											super::push(out, 23102);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 23103);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 23104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 'a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 23105);
											s.is_empty()
										}
										['i', 'r', s @ ..] => {
											super::push(out, 23106);
											match s {
												[] => true,
												['ü', 'p', '\u{1}', s @ ..] => {
													super::push(out, 32220);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ß', 'a', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23107);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											super::push(out, 23108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 878);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['t', 'i', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n881(s, out),
				['r', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n72(s, out),
		['i', s @ ..] => super::t1::n73(s, out),
		['l', s @ ..] => super::t1::n74(s, out),
		['m', s @ ..] => super::t1::n75(s, out),
		['o', s @ ..] => super::t1::n76(s, out),
		['p', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['r', s @ ..] => super::t1::n78(s, out),
		['s', s @ ..] => super::t1::n79(s, out),
		['t', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['s', s @ ..] => super::t1::n914(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n81(s, out),
		['v', s @ ..] => super::t1::n82(s, out),
		['w', s @ ..] => super::t1::n83(s, out),
		_ => false,
	}
}

/// Words ending -d.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['e', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4331);
							s.is_empty()
						}
						['l', 'e', 'd', s @ ..] => {
							super::push(out, 4332);
							match s {
								[] => true,
								['u', 'r', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 12390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'm', 'i', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4333);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4334);
							match s {
								[] => true,
								['e', 'i', s @ ..] => {
									super::push(out, 12391);
									s.is_empty()
								}
								['u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 12392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4335);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 4336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 959);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4337);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4338);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 12393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4339);
							match s {
								[] => true,
								['g', 'n', s @ ..] => {
									super::push(out, 12394);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 12395);
									s.is_empty()
								}
								['l', 'e', 'g', 'ü', s @ ..] => {
									super::push(out, 12396);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 12397);
									match s {
										[] => true,
										['l', 'g', 'g', 'u', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 23153);
											s.is_empty()
										}
										['t', 't', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 23154);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 's', s @ ..] => {
									super::push(out, 12398);
									s.is_empty()
								}
								['t', 'r', s @ ..] => {
									super::push(out, 12399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4340);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 23155);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 23156);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 965);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n968(s, out),
				['s', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4361);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 968);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 4364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n100(s, out),
		['f', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n985(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n986(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n104(s, out),
		['k', s @ ..] => super::t1::n105(s, out),
		['l', s @ ..] => super::t1::n106(s, out),
		['m', s @ ..] => super::t1::n107(s, out),
		['n', s @ ..] => super::t1::n108(s, out),
		['o', s @ ..] => super::t1::n109(s, out),
		['p', s @ ..] => super::t1::n110(s, out),
		['r', s @ ..] => super::t1::n111(s, out),
		['s', s @ ..] => super::t1::n112(s, out),
		['u', s @ ..] => super::t1::n113(s, out),
		['v', s @ ..] => super::t1::n114(s, out),
		['y', s @ ..] => super::t1::n115(s, out),
		['ü', s @ ..] => super::t1::n116(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 117);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 4588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4589);
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 12807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							super::push(out, 4590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4591);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 12808);
									match s {
										[] => true,
										['u', 'a', 'f', s @ ..] => super::t1::n24315(s, out),
										_ => false,
									}
								}
								['n', s @ ..] => super::t1::n12996(s, out),
								['s', s @ ..] => super::t1::n12997(s, out),
								_ => false,
							}
						}
						['h', s @ ..] => super::t1::n4616(s, out),
						['l', s @ ..] => super::t1::n4617(s, out),
						['n', s @ ..] => super::t1::n4618(s, out),
						['r', s @ ..] => super::t1::n4619(s, out),
						['t', s @ ..] => super::t1::n4620(s, out),
						['w', s @ ..] => super::t1::n4621(s, out),
						_ => false,
					}
				}
				['b', s @ ..] => super::t1::n1068(s, out),
				['e', s @ ..] => super::t1::n1069(s, out),
				['i', s @ ..] => super::t1::n1070(s, out),
				['l', s @ ..] => super::t1::n1071(s, out),
				['m', s @ ..] => super::t1::n1072(s, out),
				['o', s @ ..] => super::t1::n1073(s, out),
				['r', s @ ..] => super::t1::n1074(s, out),
				['s', s @ ..] => super::t1::n1075(s, out),
				['u', s @ ..] => super::t1::n1076(s, out),
				['ü', s @ ..] => super::t1::n1077(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n121(s, out),
		['d', s @ ..] => super::t1::n122(s, out),
		['e', s @ ..] => super::t1::n123(s, out),
		['f', s @ ..] => super::t1::n124(s, out),
		['g', s @ ..] => super::t1::n125(s, out),
		['h', s @ ..] => super::t1::n126(s, out),
		['i', s @ ..] => super::t1::n127(s, out),
		['j', s @ ..] => super::t1::n128(s, out),
		['k', s @ ..] => super::t1::n129(s, out),
		['l', s @ ..] => super::t1::n130(s, out),
		['m', s @ ..] => super::t1::n131(s, out),
		['n', s @ ..] => super::t1::n132(s, out),
		['o', s @ ..] => super::t1::n133(s, out),
		['p', s @ ..] => super::t1::n134(s, out),
		['r', s @ ..] => super::t1::n135(s, out),
		['s', s @ ..] => super::t1::n136(s, out),
		['t', s @ ..] => super::t1::n137(s, out),
		['u', s @ ..] => super::t2::n138(s, out),
		['v', s @ ..] => super::t2::n139(s, out),
		['w', s @ ..] => super::t2::n140(s, out),
		['x', s @ ..] => super::t2::n141(s, out),
		['y', s @ ..] => super::t2::n142(s, out),
		['z', s @ ..] => super::t2::n143(s, out),
		['ß', s @ ..] => super::t2::n144(s, out),
		['ä', s @ ..] => super::t2::n145(s, out),
		['é', s @ ..] => super::t2::n146(s, out),
		['ö', s @ ..] => super::t2::n147(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5846);
							match s {
								[] => true,
								['s', 't', s @ ..] => {
									super::push(out, 14695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5847);
							match s {
								[] => true,
								['c', 's', 'n', 'e', s @ ..] => {
									super::push(out, 14696);
									match s {
										[] => true,
										['h', 'c', 's', 'ö', 'r', 'n', 'r', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 24882);
											s.is_empty()
										}
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
					super::push(out, 1383);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14697);
									match s {
										[] => true,
										['r', 'a', 'p', 'd', s @ ..] => {
											super::push(out, 24883);
											match s {
												[] => true,
												['l', 'e', 'g', 'z', 'n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
													super::push(out, 33239);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 14698);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 14699);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 24884);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 14700);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 24885);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 33240);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t2::n41304(s, out),
														_ => false,
													}
												}
												['o', 'h', s @ ..] => {
													super::push(out, 33241);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n41305(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['p', s @ ..] => {
											super::push(out, 24886);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 33242);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n41306(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 24887);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 33243);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 33244);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 33245);
													match s {
														[] => true,
														['a', s @ ..] => super::t2::n41307(s, out),
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
				['s', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1387);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5851);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14701);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 14702);
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 24888);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5852);
							s.is_empty()
						}
						['r', s @ ..] => super::t2::n5912(s, out),
						['t', s @ ..] => {
							super::push(out, 5854);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 14703);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 24893);
											s.is_empty()
										}
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
					super::push(out, 1388);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t2::n151(s, out),
		['g', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['b', s @ ..] => super::t2::n1400(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n153(s, out),
		['l', s @ ..] => super::t2::n154(s, out),
		['n', s @ ..] => super::t2::n155(s, out),
		['o', s @ ..] => super::t2::n156(s, out),
		['p', s @ ..] => super::t2::n157(s, out),
		['q', s @ ..] => super::t2::n158(s, out),
		['r', s @ ..] => super::t2::n159(s, out),
		['t', s @ ..] => super::t2::n160(s, out),
		['u', s @ ..] => super::t2::n161(s, out),
		['v', s @ ..] => super::t2::n162(s, out),
		['ä', s @ ..] => super::t2::n163(s, out),
		_ => false,
	}
}

/// Words ending -g.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5972);
							match s {
								[] => true,
								['s', 't', 'a', 'l', 'u', 'm', '\u{1}', s @ ..] => super::t2::n15122(s, out),
								_ => false,
							}
						}
						['p', s @ ..] => super::t2::n6032(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n1447(s, out),
				['p', s @ ..] => super::t2::n1448(s, out),
				['r', s @ ..] => super::t2::n1449(s, out),
				['t', s @ ..] => super::t2::n1450(s, out),
				['w', s @ ..] => super::t2::n1451(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n166(s, out),
		['g', s @ ..] => super::t2::n167(s, out),
		['i', s @ ..] => super::t2::n168(s, out),
		['l', s @ ..] => super::t2::n169(s, out),
		['m', s @ ..] => super::t2::n170(s, out),
		['n', s @ ..] => super::t2::n171(s, out),
		['o', s @ ..] => super::t2::n172(s, out),
		['r', s @ ..] => super::t2::n173(s, out),
		['u', s @ ..] => super::t2::n174(s, out),
		['w', s @ ..] => super::t2::n175(s, out),
		['ô', s @ ..] => super::t2::n176(s, out),
		['ö', s @ ..] => super::t2::n177(s, out),
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 178);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1514);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1515);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1517);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15252);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 25526);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 15253);
									match s {
										[] => true,
										['h', 'e', 'k', '\u{1}', s @ ..] => {
											super::push(out, 25527);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15254);
									match s {
										[] => true,
										['t', 'a', 'y', 'a', '\u{1}', s @ ..] => {
											super::push(out, 25528);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 15255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1519);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6169);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6170);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6173);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6174);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 15256);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 15257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 6175);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 15258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1523);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n182(s, out),
		['e', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1539);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15485);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15486);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['i', 'e', 'w', '\u{1}', s @ ..] => {
													super::push(out, 34061);
													s.is_empty()
												}
												['n', 'u', 'h', '\u{1}', s @ ..] => {
													super::push(out, 34062);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 's', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 25795);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'c', s @ ..] => {
									match s {
										[] => true,
										['l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 25796);
											s.is_empty()
										}
										['s', 't', 'a', 'l', 'k', '\u{1}', s @ ..] => {
											super::push(out, 25797);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'c', 'e', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 15487);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 15488);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 25798);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => super::t2::n15833(s, out),
								['n', s @ ..] => super::t2::n15834(s, out),
								['r', s @ ..] => super::t2::n15835(s, out),
								['s', s @ ..] => super::t2::n15836(s, out),
								['t', s @ ..] => super::t2::n15837(s, out),
								['z', s @ ..] => super::t2::n15838(s, out),
								['ß', s @ ..] => super::t2::n15839(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t2::n1542(s, out),
				['l', s @ ..] => super::t2::n1543(s, out),
				['p', s @ ..] => super::t2::n1544(s, out),
				['r', s @ ..] => super::t2::n1545(s, out),
				['w', s @ ..] => super::t2::n1546(s, out),
				['z', s @ ..] => super::t2::n1547(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n184(s, out),
		['i', s @ ..] => super::t2::n185(s, out),
		['k', s @ ..] => super::t2::n186(s, out),
		['l', s @ ..] => super::t2::n187(s, out),
		['o', s @ ..] => super::t2::n188(s, out),
		['p', s @ ..] => super::t2::n189(s, out),
		['r', s @ ..] => super::t2::n190(s, out),
		['s', s @ ..] => super::t2::n191(s, out),
		['t', s @ ..] => super::t2::n192(s, out),
		['u', s @ ..] => super::t2::n193(s, out),
		['ä', s @ ..] => super::t2::n194(s, out),
		['ü', s @ ..] => super::t2::n195(s, out),
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1585);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6343);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['e', 's', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6346);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 15559);
									s.is_empty()
								}
								['r', 'b', 'e', 'z', '\u{1}', s @ ..] => {
									super::push(out, 15560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 6347);
							match s {
								[] => true,
								['n', 'a', 'l', 'n', 'ö', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 15561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'f', 'i', 'r', '-', 'n', 'e', 'z', 't', 'i', 'p', 's', 'z', s @ ..] => {
							super::push(out, 6348);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6349);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15562);
									s.is_empty()
								}
								['e', 'm', s @ ..] => {
									super::push(out, 15563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6350);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6351);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6352);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6353);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15564);
									s.is_empty()
								}
								['e', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 15565);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 15566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6354);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 15567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15568);
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
						['\u{1}', s @ ..] => {
							super::push(out, 6356);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6358);
							match s {
								[] => true,
								['u', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 15569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['u', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6362);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6363);
							match s {
								[] => true,
								['o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 15570);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6364);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15571);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1595);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6365);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6366);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6367);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6368);
							match s {
								[] => true,
								['b', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 15573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 6369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6370);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 6371);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6372);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6373);
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 15574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6374);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 15575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1599);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6375);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6376);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 15576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1601);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6379);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 6380);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['i', s @ ..] => super::t2::n6448(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t2::n1605(s, out),
				['n', s @ ..] => super::t2::n1606(s, out),
				['o', s @ ..] => super::t2::n1607(s, out),
				['r', s @ ..] => super::t2::n1608(s, out),
				['u', s @ ..] => super::t2::n1609(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n200(s, out),
		['d', s @ ..] => super::t2::n201(s, out),
		['e', s @ ..] => super::t2::n202(s, out),
		['f', s @ ..] => super::t2::n203(s, out),
		['g', s @ ..] => super::t2::n204(s, out),
		['h', s @ ..] => super::t2::n205(s, out),
		['i', s @ ..] => super::t2::n206(s, out),
		['j', s @ ..] => super::t2::n207(s, out),
		['k', s @ ..] => super::t2::n208(s, out),
		['l', s @ ..] => super::t2::n209(s, out),
		['m', s @ ..] => super::t2::n210(s, out),
		['n', s @ ..] => super::t2::n211(s, out),
		['o', s @ ..] => super::t2::n212(s, out),
		['p', s @ ..] => super::t2::n213(s, out),
		['q', s @ ..] => super::t2::n214(s, out),
		['r', s @ ..] => super::t2::n215(s, out),
		['s', s @ ..] => super::t2::n216(s, out),
		['t', s @ ..] => super::t2::n217(s, out),
		['u', s @ ..] => super::t2::n218(s, out),
		['v', s @ ..] => super::t2::n219(s, out),
		['w', s @ ..] => super::t2::n220(s, out),
		['x', s @ ..] => super::t2::n221(s, out),
		['y', s @ ..] => super::t2::n222(s, out),
		['z', s @ ..] => super::t2::n223(s, out),
		['ć', s @ ..] => super::t2::n224(s, out),
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6755);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 15849);
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 25969);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 15850);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 25970);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1788);
					match s {
						[] => true,
						['ä', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6758);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 15851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6759);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 15852);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 15853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6760);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 15854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['c', 's', 't', 'n', s @ ..] => {
							super::push(out, 6761);
							match s {
								[] => true,
								['o', 'l', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 15855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6762);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6763);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6764);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6765);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 15856);
									match s {
										[] => true,
										['o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 25971);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 6766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6767);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6769);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6771);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6772);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6773);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 15857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6774);
							match s {
								[] => true,
								['e', 'k', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 15858);
									s.is_empty()
								}
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 15859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6775);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 15860);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 15861);
									match s {
										[] => true,
										['r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 25972);
											s.is_empty()
										}
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
					super::push(out, 1795);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['g', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6777);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1798);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6780);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 15862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6781);
							s.is_empty()
						}
						['p', 's', '-', 'k', 'a', 'n', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6783);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1800);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1801);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1802);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6786);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1803);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1804);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 6788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 233);
			s.is_empty()
		}
		['c', s @ ..] => super::t2::n234(s, out),
		['e', s @ ..] => super::t2::n235(s, out),
		['f', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 1824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n239(s, out),
		['k', s @ ..] => {
			super::push(out, 240);
			s.is_empty()
		}
		['l', s @ ..] => super::t3::n241(s, out),
		['m', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t3::n243(s, out),
		['o', s @ ..] => super::t3::n244(s, out),
		['r', s @ ..] => super::t3::n245(s, out),
		['s', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1874);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7003);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n1877(s, out),
				['o', s @ ..] => super::t3::n1878(s, out),
				['r', s @ ..] => super::t3::n1879(s, out),
				['u', s @ ..] => super::t3::n1880(s, out),
				['y', s @ ..] => super::t3::n1881(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t3::n247(s, out),
		['w', s @ ..] => super::t3::n248(s, out),
		['y', s @ ..] => super::t3::n249(s, out),
		['ö', s @ ..] => super::t3::n250(s, out),
		['ü', s @ ..] => super::t3::n251(s, out),
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['a', s @ ..] => super::t3::n253(s, out),
		['d', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1919);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 7143);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 16375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 7144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'u', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 16376);
									s.is_empty()
								}
								['s', 'r', 'u', 't', 'a', 'r', 'a', 'p', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 16377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1921);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7146);
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7147);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26454);
											s.is_empty()
										}
										['e', 'g', 'a', 'n', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 26455);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 16378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7148);
							match s {
								[] => true,
								['ö', 'k', '\u{1}', s @ ..] => {
									super::push(out, 16379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1922);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 7149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n255(s, out),
		['f', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1948);
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t3::n258(s, out),
		['i', s @ ..] => super::t3::n259(s, out),
		['k', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1976);
					match s {
						[] => true,
						['a', 'h', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t3::n261(s, out),
		['m', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1986);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1988);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7425);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7426);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 16957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t3::n7508(s, out),
						['o', s @ ..] => {
							super::push(out, 7428);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 16958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1989);
					match s {
						[] => true,
						['y', 'l', 'g', 'n', s @ ..] => {
							super::push(out, 7429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t3::n1992(s, out),
				['e', s @ ..] => super::t3::n1993(s, out),
				['g', s @ ..] => {
					super::push(out, 1992);
					match s {
						[] => true,
						['o', s @ ..] => super::t3::n7516(s, out),
						_ => false,
					}
				}
				['h', s @ ..] => super::t3::n1995(s, out),
				['i', s @ ..] => super::t3::n1996(s, out),
				['k', s @ ..] => super::t3::n1997(s, out),
				['l', s @ ..] => super::t3::n1998(s, out),
				['m', s @ ..] => super::t3::n1999(s, out),
				['n', s @ ..] => super::t3::n2000(s, out),
				['o', s @ ..] => super::t3::n2001(s, out),
				['p', s @ ..] => super::t3::n2002(s, out),
				['r', s @ ..] => super::t3::n2003(s, out),
				['s', s @ ..] => super::t3::n2004(s, out),
				['t', s @ ..] => super::t3::n2005(s, out),
				['u', s @ ..] => super::t3::n2006(s, out),
				['z', s @ ..] => super::t3::n2007(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t3::n264(s, out),
		['r', s @ ..] => super::t3::n265(s, out),
		['s', s @ ..] => super::t3::n266(s, out),
		['t', s @ ..] => super::t3::n267(s, out),
		['u', s @ ..] => super::t3::n268(s, out),
		['v', s @ ..] => super::t3::n269(s, out),
		['w', s @ ..] => super::t3::n270(s, out),
		['x', s @ ..] => super::t3::n271(s, out),
		['y', s @ ..] => super::t3::n272(s, out),
		['z', s @ ..] => super::t3::n273(s, out),
		['ä', s @ ..] => super::t3::n274(s, out),
		['å', s @ ..] => super::t3::n275(s, out),
		['ö', s @ ..] => super::t3::n276(s, out),
		['ü', s @ ..] => super::t3::n277(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 278);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2069);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7561);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 17065);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 17066);
									match s {
										[] => true,
										['k', 'c', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 27118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 17067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7562);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7563);
							match s {
								[] => true,
								['e', 'f', 'f', 'o', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 27119);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 27120);
											s.is_empty()
										}
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
					super::push(out, 2070);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7564);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7565);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 17068);
									match s {
										[] => true,
										['r', 'u', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 27121);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 17069);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 34953);
													match s {
														[] => true,
														['l', 'u', 'g', '\u{1}', s @ ..] => super::t3::n42521(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 34954);
													match s {
														[] => true,
														['i', s @ ..] => super::t3::n42522(s, out),
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
							super::push(out, 7566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2071);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7567);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 17070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7568);
							match s {
								[] => true,
								['a', 'm', 'a', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 27122);
											s.is_empty()
										}
										['m', 'u', 'i', s @ ..] => {
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 34955);
													match s {
														[] => true,
														['t', 'i', 'l', '\u{1}', s @ ..] => super::t3::n42523(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 34956);
													s.is_empty()
												}
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
							super::push(out, 7569);
							match s {
								[] => true,
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 17071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2072);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17072);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 17073);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 27123);
											match s {
												[] => true,
												['e', 'r', 'k', '\u{1}', s @ ..] => {
													super::push(out, 34957);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t3::n7659(s, out),
						_ => false,
					}
				}
				['j', s @ ..] => super::t3::n2077(s, out),
				['l', s @ ..] => super::t3::n2078(s, out),
				['m', s @ ..] => super::t3::n2079(s, out),
				['n', s @ ..] => super::t3::n2080(s, out),
				['p', s @ ..] => super::t3::n2081(s, out),
				['r', s @ ..] => super::t3::n2082(s, out),
				['s', s @ ..] => super::t3::n2083(s, out),
				['t', s @ ..] => super::t3::n2084(s, out),
				['w', s @ ..] => super::t3::n2085(s, out),
				['y', s @ ..] => super::t3::n2086(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t3::n280(s, out),
		['d', s @ ..] => super::t3::n281(s, out),
		['e', s @ ..] => super::t3::n282(s, out),
		['f', s @ ..] => super::t3::n283(s, out),
		['h', s @ ..] => super::t3::n284(s, out),
		['i', s @ ..] => super::t3::n285(s, out),
		['l', s @ ..] => super::t3::n286(s, out),
		['m', s @ ..] => super::t3::n287(s, out),
		['o', s @ ..] => super::t3::n288(s, out),
		['r', s @ ..] => super::t3::n289(s, out),
		['s', s @ ..] => super::t3::n290(s, out),
		['t', s @ ..] => super::t3::n291(s, out),
		['u', s @ ..] => super::t3::n292(s, out),
		['w', s @ ..] => super::t3::n293(s, out),
		['y', s @ ..] => super::t3::n294(s, out),
		['ä', s @ ..] => super::t3::n295(s, out),
		['ö', s @ ..] => super::t3::n296(s, out),
		['ü', s @ ..] => super::t3::n297(s, out),
		['ē', s @ ..] => super::t3::n298(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 299);
			s.is_empty()
		}
		['a', s @ ..] => super::t3::n300(s, out),
		['b', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2199);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n302(s, out),
		['g', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8208);
							match s {
								[] => true,
								['r', 'e', 'v', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 18083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8210);
							s.is_empty()
						}
						['b', s @ ..] => super::t3::n8308(s, out),
						['h', s @ ..] => {
							super::push(out, 8212);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18091);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 18092);
									match s {
										[] => true,
										['u', 'f', 'p', '\u{1}', s @ ..] => {
											super::push(out, 28138);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => super::t3::n18590(s, out),
								['p', s @ ..] => super::t3::n18591(s, out),
								['r', s @ ..] => super::t3::n18592(s, out),
								['s', s @ ..] => super::t3::n18593(s, out),
								_ => false,
							}
						}
						['j', s @ ..] => super::t3::n8310(s, out),
						['k', s @ ..] => super::t3::n8311(s, out),
						['r', s @ ..] => super::t3::n8312(s, out),
						['w', s @ ..] => super::t3::n8313(s, out),
						['z', s @ ..] => super::t3::n8314(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n2233(s, out),
				['i', s @ ..] => super::t3::n2234(s, out),
				['o', s @ ..] => super::t3::n2235(s, out),
				['u', s @ ..] => super::t3::n2236(s, out),
				['ö', s @ ..] => super::t3::n2237(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n305(s, out),
		['l', s @ ..] => super::t3::n306(s, out),
		['n', s @ ..] => super::t3::n307(s, out),
		['o', s @ ..] => super::t3::n308(s, out),
		['r', s @ ..] => super::t3::n309(s, out),
		['s', s @ ..] => super::t3::n310(s, out),
		['u', s @ ..] => super::t3::n311(s, out),
		['w', s @ ..] => super::t3::n312(s, out),
		['y', s @ ..] => super::t3::n313(s, out),
		['z', s @ ..] => super::t3::n314(s, out),
		['á', s @ ..] => super::t3::n315(s, out),
		['ä', s @ ..] => super::t3::n316(s, out),
		['ë', s @ ..] => super::t3::n317(s, out),
		['ó', s @ ..] => super::t3::n318(s, out),
		['ö', s @ ..] => super::t3::n319(s, out),
		['ü', s @ ..] => super::t3::n320(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 321);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2338);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2339);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2340);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2341);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2342);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8684);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2343);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8686);
							match s {
								[] => true,
								['h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 18941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2344);
					s.is_empty()
				}
				['ç', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2346);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8687);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 8688);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8689);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 8690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2347);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2348);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2350);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8693);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 18942);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 18943);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 18944);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 18945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8694);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8695);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 18946);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 18947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2351);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2353);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8697);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 8698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8699);
							match s {
								[] => true,
								['s', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 18948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8700);
							match s {
								[] => true,
								['i', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 18949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2356);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2357);
					match s {
						[] => true,
						['o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2359);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8703);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8704);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8705);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2360);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2361);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8708);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 18950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 8709);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 8710);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 8711);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8712);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8713);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8714);
							match s {
								[] => true,
								['e', s @ ..] => super::t3::n19488(s, out),
								['o', s @ ..] => super::t3::n19489(s, out),
								['p', s @ ..] => super::t3::n19490(s, out),
								_ => false,
							}
						}
						['s', s @ ..] => super::t3::n8815(s, out),
						['t', s @ ..] => super::t3::n8816(s, out),
						['z', 'u', 'r', 'c', '\u{1}', s @ ..] => super::t3::n8817(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n2366(s, out),
				['i', s @ ..] => super::t3::n2367(s, out),
				['l', s @ ..] => super::t3::n2368(s, out),
				['n', s @ ..] => super::t3::n2369(s, out),
				['o', s @ ..] => super::t3::n2370(s, out),
				['r', s @ ..] => super::t3::n2371(s, out),
				['u', s @ ..] => super::t3::n2372(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n326(s, out),
		['f', s @ ..] => super::t3::n327(s, out),
		['g', s @ ..] => super::t3::n328(s, out),
		['h', s @ ..] => super::t3::n329(s, out),
		['i', s @ ..] => super::t3::n330(s, out),
		['j', s @ ..] => super::t3::n331(s, out),
		['k', s @ ..] => super::t3::n332(s, out),
		['l', s @ ..] => super::t3::n333(s, out),
		['m', s @ ..] => super::t3::n334(s, out),
		['n', s @ ..] => super::t4::n335(s, out),
		['o', s @ ..] => super::t4::n336(s, out),
		['p', s @ ..] => super::t4::n337(s, out),
		['r', s @ ..] => super::t4::n338(s, out),
		['s', s @ ..] => super::t4::n339(s, out),
		['t', s @ ..] => super::t4::n340(s, out),
		['u', s @ ..] => super::t4::n341(s, out),
		['v', s @ ..] => super::t4::n342(s, out),
		['x', s @ ..] => super::t4::n343(s, out),
		['y', s @ ..] => super::t4::n344(s, out),
		['z', s @ ..] => super::t4::n345(s, out),
		['ñ', s @ ..] => super::t4::n346(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2523);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9009);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28961);
											s.is_empty()
										}
										['r', 'e', 'n', 'ö', 'd', '\u{1}', s @ ..] => {
											super::push(out, 28962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 9010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2524);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 9011);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9012);
							s.is_empty()
						}
						['l', 'l', 'a', 'b', 'e', 's', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2525);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9014);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2526);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2527);
					match s {
						[] => true,
						['-', 'y', 'a', 'p', '-', 'r', 'e', 'd', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2528);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2529);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2530);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9017);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2531);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2532);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 2533);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2534);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9021);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 9022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2535);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2537);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 9024);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9025);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 19157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2538);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2539);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2540);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9026);
							match s {
								[] => true,
								['b', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 19158);
									s.is_empty()
								}
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
			super::push(out, 351);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 2541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2544);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2545);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9029);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9030);
							match s {
								[] => true,
								['n', 'w', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 19159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2547);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2548);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 9032);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2549);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2550);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2551);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9035);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9036);
							match s {
								[] => true,
								['i', 'r', 'p', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 28963);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 36084);
													match s {
														[] => true,
														['n', 'a', 'h', 't', 'm', 'a', 's', 'e', 'g', '\u{1}', s @ ..] => super::t4::n43114(s, out),
														_ => false,
													}
												}
												['g', 'n', 'u', 'r', s @ ..] => {
													super::push(out, 36085);
													match s {
														[] => true,
														['e', 'i', 'd', 'r', 'a', 't', 'e', 'r', '\u{1}', s @ ..] => super::t4::n43115(s, out),
														['h', 'ü', 'f', 's', 'u', 'a', 't', 's', 'i', 'e', 'm', '\u{1}', s @ ..] => super::t4::n43116(s, out),
														_ => false,
													}
												}
												['n', 'o', 'i', 't', 'a', 'd', s @ ..] => {
													super::push(out, 36086);
													s.is_empty()
												}
												['t', s @ ..] => super::t4::n38095(s, out),
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
		['l', s @ ..] => super::t4::n354(s, out),
		['m', s @ ..] => super::t4::n355(s, out),
		['n', s @ ..] => super::t4::n356(s, out),
		['o', s @ ..] => super::t4::n357(s, out),
		['p', s @ ..] => super::t4::n358(s, out),
		['r', s @ ..] => super::t4::n359(s, out),
		['s', s @ ..] => super::t4::n360(s, out),
		['t', s @ ..] => super::t4::n361(s, out),
		['u', s @ ..] => super::t4::n362(s, out),
		['v', s @ ..] => super::t4::n363(s, out),
		['w', s @ ..] => super::t4::n364(s, out),
		['y', s @ ..] => super::t4::n365(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2601);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 9131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2602);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => super::t4::n369(s, out),
		['d', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n371(s, out),
		['h', s @ ..] => super::t4::n372(s, out),
		['i', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2665);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9491);
							match s {
								[] => true,
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 20043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 9492);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9493);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2666);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2667);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9495);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 20044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 9496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2668);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9498);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 9499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2670);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9501);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9502);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 9503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2672);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2673);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9504);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9505);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 20045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9506);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 9507);
							match s {
								[] => true,
								['r', 'e', 's', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 20046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2674);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9508);
							s.is_empty()
						}
						['m', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2675);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 9510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9511);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9512);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 9513);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 20047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2677);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n376(s, out),
		['p', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2699);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9581);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9582);
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 20149);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 9583);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 20150);
									match s {
										[] => true,
										['r', 'i', 'w', 'n', s @ ..] => {
											super::push(out, 30065);
											s.is_empty()
										}
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
					super::push(out, 2700);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 20151);
									match s {
										[] => true,
										['r', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 30066);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 20152);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 30067);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 20153);
									match s {
										[] => true,
										['g', 'n', s @ ..] => {
											super::push(out, 30068);
											match s {
												[] => true,
												['a', 'f', 'p', 'm', 'e', '\u{1}', s @ ..] => {
													super::push(out, 36837);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['z', 'n', s @ ..] => {
									super::push(out, 20154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2701);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9584);
							match s {
								[] => true,
								['c', 's', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['e', 'e', s @ ..] => {
											super::push(out, 30069);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 30070);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 30071);
											s.is_empty()
										}
										['r', 'e', 'b', 'l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 30072);
											s.is_empty()
										}
										['s', 'k', s @ ..] => {
											super::push(out, 30073);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => super::t4::n9698(s, out),
						['w', s @ ..] => super::t4::n9699(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t4::n2706(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t4::n379(s, out),
		['t', s @ ..] => super::t4::n380(s, out),
		['u', s @ ..] => super::t4::n381(s, out),
		['y', s @ ..] => super::t4::n382(s, out),
		['ä', s @ ..] => super::t4::n383(s, out),
		['é', s @ ..] => super::t4::n384(s, out),
		['ö', s @ ..] => super::t4::n385(s, out),
		['ü', s @ ..] => super::t4::n386(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2750);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2751);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9676);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', 'r', 'a', 'm', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2753);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9678);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9679);
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 20241);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 20242);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2754);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2755);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9681);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 9682);
							s.is_empty()
						}
						['e', 'n', s @ ..] => {
							super::push(out, 9683);
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 20243);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 30128);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 20244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 9684);
							match s {
								[] => true,
								['c', 'a', s @ ..] => {
									super::push(out, 20245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'o', s @ ..] => {
							super::push(out, 9685);
							s.is_empty()
						}
						['m', 'u', s @ ..] => {
							super::push(out, 9686);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9687);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 20246);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 30129);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 30130);
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
								['e', 's', s @ ..] => {
									super::push(out, 20247);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 20248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9688);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 20249);
									s.is_empty()
								}
								['u', 'o', s @ ..] => {
									super::push(out, 20250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2756);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9690);
							match s {
								[] => true,
								['s', 'l', 'r', 'e', 'h', 'c', 'r', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 20251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t4::n2761(s, out),
				['k', s @ ..] => super::t4::n2762(s, out),
				['l', s @ ..] => super::t4::n2763(s, out),
				['m', s @ ..] => super::t4::n2764(s, out),
				['n', s @ ..] => super::t4::n2765(s, out),
				['o', s @ ..] => {
					super::push(out, 2762);
					match s {
						[] => true,
						['r', s @ ..] => super::t4::n9829(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t4::n2767(s, out),
				['r', s @ ..] => super::t4::n2768(s, out),
				['s', s @ ..] => super::t4::n2769(s, out),
				['t', s @ ..] => super::t4::n2770(s, out),
				['v', s @ ..] => super::t4::n2771(s, out),
				['w', s @ ..] => super::t4::n2772(s, out),
				['x', s @ ..] => super::t4::n2773(s, out),
				['y', s @ ..] => super::t4::n2774(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t4::n390(s, out),
		['c', s @ ..] => super::t4::n391(s, out),
		['d', s @ ..] => super::t4::n392(s, out),
		['e', s @ ..] => super::t4::n393(s, out),
		['g', s @ ..] => super::t4::n394(s, out),
		['h', s @ ..] => super::t4::n395(s, out),
		['i', s @ ..] => super::t4::n396(s, out),
		['k', s @ ..] => super::t4::n397(s, out),
		['l', s @ ..] => super::t4::n398(s, out),
		['m', s @ ..] => super::t4::n399(s, out),
		['n', s @ ..] => super::t4::n400(s, out),
		['o', s @ ..] => super::t4::n401(s, out),
		['p', s @ ..] => super::t4::n402(s, out),
		['r', s @ ..] => super::t4::n403(s, out),
		['s', s @ ..] => super::t4::n404(s, out),
		['t', s @ ..] => super::t4::n405(s, out),
		['u', s @ ..] => super::t4::n406(s, out),
		['w', s @ ..] => super::t4::n407(s, out),
		['y', s @ ..] => super::t4::n408(s, out),
		['á', s @ ..] => super::t4::n409(s, out),
		['ä', s @ ..] => super::t4::n410(s, out),
		['å', s @ ..] => super::t4::n411(s, out),
		['ö', s @ ..] => super::t4::n412(s, out),
		['ü', s @ ..] => super::t4::n413(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['a', s @ ..] => super::t4::n415(s, out),
		['b', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2966);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10408);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2968);
					match s {
						[] => true,
						['f', 'f', 'e', '\u{1}', s @ ..] => {
							super::push(out, 10410);
							s.is_empty()
						}
						['l', 'l', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10411);
							s.is_empty()
						}
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2969);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 't', 's', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10413);
							s.is_empty()
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10415);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10416);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 30674);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 37190);
													match s {
														[] => true,
														['f', s @ ..] => super::t4::n43794(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 37191);
													s.is_empty()
												}
												['s', 'm', s @ ..] => {
													super::push(out, 37192);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 30675);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 37193);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 30676);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 37194);
													match s {
														[] => true,
														['e', 'd', 'i', '\u{1}', s @ ..] => super::t4::n43795(s, out),
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
													super::push(out, 37195);
													match s {
														[] => true,
														['k', s @ ..] => super::t4::n43796(s, out),
														_ => false,
													}
												}
												['e', s @ ..] => {
													super::push(out, 37196);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'h', s @ ..] => {
											super::push(out, 30677);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 37197);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 30678);
											s.is_empty()
										}
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
					super::push(out, 2971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n419(s, out),
		['f', s @ ..] => super::t4::n420(s, out),
		['g', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3006);
					match s {
						[] => true,
						['d', 'e', 'r', 'p', 'g', s @ ..] => {
							super::push(out, 10531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3007);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 10532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21304);
									s.is_empty()
								}
								['b', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 21305);
									s.is_empty()
								}
								['n', 'e', 's', 'i', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 21306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t4::n422(s, out),
		['i', s @ ..] => super::t4::n423(s, out),
		['k', s @ ..] => super::t4::n424(s, out),
		['l', s @ ..] => super::t4::n425(s, out),
		['m', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3048);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10756);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 21686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 10757);
							match s {
								[] => true,
								['a', 'i', 'd', s @ ..] => {
									super::push(out, 21687);
									s.is_empty()
								}
								['l', 'e', 't', 'r', 'a', 'k', 's', s @ ..] => {
									super::push(out, 21688);
									match s {
										[] => true,
										['e', 'd', 'n', 'u', s @ ..] => {
											super::push(out, 31179);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 10758);
							match s {
								[] => true,
								['e', 't', 'e', s @ ..] => {
									super::push(out, 21689);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'l', s @ ..] => super::t4::n10886(s, out),
						['s', s @ ..] => super::t4::n10887(s, out),
						['t', s @ ..] => super::t4::n10888(s, out),
						['u', s @ ..] => super::t4::n10889(s, out),
						['z', s @ ..] => super::t4::n10890(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t4::n3054(s, out),
				['i', s @ ..] => super::t4::n3055(s, out),
				['m', s @ ..] => super::t4::n3056(s, out),
				['u', s @ ..] => super::t4::n3057(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t4::n427(s, out),
		['o', s @ ..] => super::t4::n428(s, out),
		['p', s @ ..] => super::t4::n429(s, out),
		['r', s @ ..] => super::t4::n430(s, out),
		['s', s @ ..] => super::t4::n431(s, out),
		['t', s @ ..] => super::t5::n432(s, out),
		['u', s @ ..] => super::t5::n433(s, out),
		['x', s @ ..] => super::t5::n434(s, out),
		['y', s @ ..] => super::t5::n435(s, out),
		['z', s @ ..] => super::t5::n436(s, out),
		['ä', s @ ..] => super::t5::n437(s, out),
		['ö', s @ ..] => super::t5::n438(s, out),
		['ü', s @ ..] => super::t5::n439(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		['a', s @ ..] => super::t5::n441(s, out),
		['b', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3170);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11272);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 11273);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 22542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11274);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11275);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3175);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				['l', 'e', 'd', 'd', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3179);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3180);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3181);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11279);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 22543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 11280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3182);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 11281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22544);
									s.is_empty()
								}
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
			super::push(out, 445);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3185);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11282);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11283);
							match s {
								[] => true,
								['a', 't', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 22545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11284);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 11285);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 31950);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 38261);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 31951);
											match s {
												[] => true,
												['e', 'g', 'o', 'v', '\u{1}', s @ ..] => {
													super::push(out, 38262);
													s.is_empty()
												}
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
							super::push(out, 11286);
							match s {
								[] => true,
								['e', 'd', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 22546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 11287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3187);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11288);
							match s {
								[] => true,
								['m', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 22547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 11289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11290);
							s.is_empty()
						}
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3190);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3191);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11293);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11294);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 22548);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 22549);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 38263);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 38264);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 38265);
													match s {
														[] => true,
														['a', 'm', '\u{1}', s @ ..] => super::t5::n44681(s, out),
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
				['w', s @ ..] => {
					super::push(out, 3192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 3193);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3194);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 11295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3195);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3196);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3197);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 11296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3198);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 11297);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3199);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3200);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3201);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3202);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 11300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t5::n448(s, out),
		['j', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['o', s @ ..] => super::t5::n3214(s, out),
				['u', s @ ..] => {
					super::push(out, 3210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t5::n450(s, out),
		['l', s @ ..] => super::t5::n451(s, out),
		['m', s @ ..] => super::t5::n452(s, out),
		['n', s @ ..] => super::t5::n453(s, out),
		['o', s @ ..] => super::t5::n454(s, out),
		['p', s @ ..] => super::t5::n455(s, out),
		['r', s @ ..] => super::t5::n456(s, out),
		['s', s @ ..] => super::t5::n457(s, out),
		['t', s @ ..] => super::t5::n458(s, out),
		['v', s @ ..] => super::t5::n459(s, out),
		['w', s @ ..] => super::t5::n460(s, out),
		['y', s @ ..] => super::t5::n461(s, out),
		['z', s @ ..] => super::t5::n462(s, out),
		['ä', s @ ..] => super::t5::n463(s, out),
		['ç', s @ ..] => super::t5::n464(s, out),
		_ => false,
	}
}

/// Words ending -v.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3264);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 11338);
							match s {
								[] => true,
								['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 22560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3266);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3267);
					match s {
						[] => true,
						['k', 'o', 'r', s @ ..] => {
							super::push(out, 11340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 3268);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3271);
					match s {
						[] => true,
						['c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3272);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11343);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11344);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 22561);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 22562);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 31952);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 22563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3273);
					match s {
						[] => true,
						['a', s @ ..] => super::t5::n11476(s, out),
						['i', s @ ..] => {
							super::push(out, 11346);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 22576);
									match s {
										[] => true,
										['d', 'a', 'f', 'f', 'o', 't', 's', 't', 's', 'n', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 31967);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 22577);
									s.is_empty()
								}
								['s', 'o', 'p', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 31968);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 31969);
											s.is_empty()
										}
										['ä', s @ ..] => {
											super::push(out, 31970);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 22578);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 11347);
							match s {
								[] => true,
								['a', 'f', s @ ..] => {
									super::push(out, 22579);
									s.is_empty()
								}
								['e', s @ ..] => super::t5::n23278(s, out),
								['n', s @ ..] => {
									super::push(out, 22580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t5::n11479(s, out),
						['o', s @ ..] => {
							super::push(out, 11348);
							s.is_empty()
						}
						['u', s @ ..] => super::t5::n11481(s, out),
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 3274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t5::n469(s, out),
		['n', s @ ..] => super::t5::n470(s, out),
		['r', s @ ..] => super::t5::n471(s, out),
		['s', s @ ..] => super::t5::n472(s, out),
		['t', s @ ..] => super::t5::n473(s, out),
		['u', s @ ..] => super::t5::n474(s, out),
		['w', s @ ..] => super::t5::n475(s, out),
		['ü', s @ ..] => super::t5::n476(s, out),
		_ => false,
	}
}

/// Words ending -w.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 477);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3285);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3286);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11351);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3287);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11353);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3288);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11355);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 11356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3289);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3290);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11357);
							match s {
								[] => true,
								['o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 22589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3291);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 3292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 3293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3294);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 22590);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 38279);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 38280);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 38281);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 22591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11358);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3298);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3300);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3302);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 11361);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 22592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 11362);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 22593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11363);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 11364);
							match s {
								[] => true,
								['o', 'n', 'k', '\u{1}', s @ ..] => {
									super::push(out, 22594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3303);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11365);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 11366);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 11367);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 22595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11368);
							match s {
								[] => true,
								['-', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 22596);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3304);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3305);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 3306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 486);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 488);
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
		['\u{1}', s @ ..] => {
			super::push(out, 489);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11371);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11372);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3311);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 11374);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 11375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3312);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11376);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 22597);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 22598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11377);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3314);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11379);
							match s {
								[] => true,
								['o', 'l', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 22599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3316);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11380);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 11381);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 22600);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 11382);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 22601);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31979);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31980);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 31981);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 38282);
													s.is_empty()
												}
												['m', 'u', s @ ..] => {
													super::push(out, 38283);
													match s {
														[] => true,
														['e', 'n', 'p', '\u{1}', s @ ..] => super::t5::n44684(s, out),
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
				['s', s @ ..] => {
					super::push(out, 3317);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11384);
							match s {
								[] => true,
								['y', 's', 'o', s @ ..] => {
									super::push(out, 22602);
									s.is_empty()
								}
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
			super::push(out, 491);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3319);
					s.is_empty()
				}
				['d', s @ ..] => super::t5::n3325(s, out),
				['f', s @ ..] => {
					super::push(out, 3321);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11387);
							match s {
								[] => true,
								['t', 'n', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 22606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3322);
					match s {
						[] => true,
						['t', 'r', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t5::n3328(s, out),
				['m', s @ ..] => {
					super::push(out, 3324);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 11395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3325);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3326);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11396);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3327);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t5::n3333(s, out),
				['t', s @ ..] => super::t5::n3334(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t5::n492(s, out),
		['n', s @ ..] => super::t5::n493(s, out),
		['o', s @ ..] => super::t5::n494(s, out),
		['u', s @ ..] => super::t5::n495(s, out),
		['y', s @ ..] => super::t5::n496(s, out),
		['ü', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3359);
					s.is_empty()
				}
				['g', 'n', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3360);
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
		['\u{1}', s @ ..] => {
			super::push(out, 497);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3361);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11450);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3363);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 11452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3364);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3365);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11453);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 22637);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 32008);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 22638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3367);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3368);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11454);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 22639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 11455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11456);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'k', s @ ..] => {
											super::push(out, 32009);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 32010);
											s.is_empty()
										}
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
					super::push(out, 3369);
					match s {
						[] => true,
						['s', 'a', s @ ..] => {
							super::push(out, 11457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3370);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11458);
							s.is_empty()
						}
						['e', 't', s @ ..] => {
							super::push(out, 11459);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 11460);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 22641);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11461);
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 22642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3371);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3372);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 22643);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 22644);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 22645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3373);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3374);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 11464);
							match s {
								[] => true,
								['r', '-', 'r', 'e', 's', 's', 'a', 'w', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 22646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3377);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3378);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11466);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11467);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 22647);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3379);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3380);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 22648);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 22649);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 32011);
											match s {
												[] => true,
												['t', 'p', s @ ..] => {
													super::push(out, 38302);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 22650);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3381);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11469);
							match s {
								[] => true,
								['o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 22651);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'g', s @ ..] => {
							super::push(out, 11470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22652);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t5::n503(s, out),
		['g', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3393);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3395);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3396);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11491);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11492);
							match s {
								[] => true,
								['o', 'k', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 22661);
									s.is_empty()
								}
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
			super::push(out, 506);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3397);
					match s {
						[] => true,
						['r', 'e', 't', 't', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3398);
					s.is_empty()
				}
				['l', s @ ..] => super::t5::n3404(s, out),
				['o', s @ ..] => super::t5::n3405(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t5::n508(s, out),
		['n', s @ ..] => super::t5::n509(s, out),
		['o', s @ ..] => super::t5::n510(s, out),
		['p', s @ ..] => super::t5::n511(s, out),
		['r', s @ ..] => super::t5::n512(s, out),
		['s', s @ ..] => super::t5::n513(s, out),
		['t', s @ ..] => super::t5::n514(s, out),
		['x', s @ ..] => super::t5::n515(s, out),
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 515);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3428);
					match s {
						[] => true,
						['ö', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3429);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3430);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11540);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3432);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3433);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3434);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3435);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3436);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3437);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 3439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 520);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3441);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11544);
							s.is_empty()
						}
						['u', 'j', 'ä', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3442);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11546);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 11547);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11548);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 11549);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 22678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3443);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3444);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3445);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3446);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3447);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3448);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11552);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 22679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3449);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 11554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3451);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11555);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 22680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 11556);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 22681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11557);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 22682);
									match s {
										[] => true,
										['c', 's', 'r', s @ ..] => {
											super::push(out, 32026);
											match s {
												[] => true,
												['e', 'd', s @ ..] => {
													super::push(out, 38303);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 22683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11558);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 22684);
									match s {
										[] => true,
										['c', 'o', s @ ..] => {
											super::push(out, 32027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', 'f', s @ ..] => {
											super::push(out, 32028);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 32029);
											match s {
												[] => true,
												['a', 't', 'e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 38304);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 22685);
									match s {
										[] => true,
										['u', 'i', 'h', s @ ..] => {
											super::push(out, 32030);
											match s {
												[] => true,
												['t', 'i', 'l', '\u{1}', s @ ..] => {
													super::push(out, 38305);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'r', s @ ..] => {
									super::push(out, 22686);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 32031);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 't', s @ ..] => {
									super::push(out, 22687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 11559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3452);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11560);
							match s {
								[] => true,
								['h', 'c', 's', 'n', s @ ..] => {
									super::push(out, 22688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t5::n3459(s, out),
				['o', s @ ..] => super::t5::n3460(s, out),
				['u', s @ ..] => {
					super::push(out, 3455);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => super::t5::n3462(s, out),
				['ö', s @ ..] => super::t5::n3463(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t5::n524(s, out),
		['r', s @ ..] => super::t5::n525(s, out),
		['s', s @ ..] => super::t5::n526(s, out),
		['t', s @ ..] => super::t5::n527(s, out),
		['u', s @ ..] => super::t5::n528(s, out),
		['z', s @ ..] => super::t5::n529(s, out),
		['ä', s @ ..] => super::t5::n530(s, out),
		['ö', s @ ..] => super::t5::n531(s, out),
		_ => false,
	}
}

/// Words ending -ß.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3489);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3490);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3491);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3492);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11682);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11683);
							s.is_empty()
						}
						['n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 11684);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11685);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32159);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 32160);
											s.is_empty()
										}
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
					super::push(out, 3493);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11686);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 11687);
							match s {
								[] => true,
								['c', 'i', 'e', 'l', s @ ..] => {
									super::push(out, 22876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 11688);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11689);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 32161);
											s.is_empty()
										}
										['g', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 38407);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 38408);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 32162);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 22877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11690);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 32163);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 32164);
											match s {
												[] => true,
												['d', 'a', 'r', '\u{1}', s @ ..] => {
													super::push(out, 38409);
													s.is_empty()
												}
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
							super::push(out, 11691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3494);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11692);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11693);
							s.is_empty()
						}
						['d', 'r', 'a', 'h', 't', 't', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11694);
							s.is_empty()
						}
						['e', 's', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 11695);
							s.is_empty()
						}
						['f', 'u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11696);
							s.is_empty()
						}
						['g', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 11697);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 22878);
									s.is_empty()
								}
								['k', 's', '\u{1}', s @ ..] => {
									super::push(out, 22879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 22880);
									s.is_empty()
								}
								['i', 'a', 'r', 'b', 'm', 'u', '\u{1}', s @ ..] => {
									super::push(out, 22881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11698);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11699);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', 'u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 22882);
									s.is_empty()
								}
								['n', 'n', 'e', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 22883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11700);
							match s {
								[] => true,
								['a', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 22884);
									s.is_empty()
								}
								['g', 'r', 'i', 'b', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 22885);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 22886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3496);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11701);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 11702);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 22887);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 22888);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 22889);
									s.is_empty()
								}
								['n', 'e', 'h', 'c', s @ ..] => {
									super::push(out, 22890);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 22891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11703);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 22892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3497);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11704);
							s.is_empty()
						}
						['i', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3498);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t5::n534(s, out),
		['i', s @ ..] => super::t5::n535(s, out),
		['o', s @ ..] => super::t5::n536(s, out),
		['u', s @ ..] => super::t5::n537(s, out),
		['y', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['m', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3528);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 539);
			s.is_empty()
		}
		['ü', s @ ..] => super::t5::n541(s, out),
		_ => false,
	}
}

/// Words ending -ä.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -é.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['a', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3534);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['a', 'c', 's', 'e', s @ ..] => {
					super::push(out, 3536);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11765);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 22934);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 22935);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 22936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 11766);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 22937);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 22938);
									s.is_empty()
								}
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
			super::push(out, 549);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3537);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3538);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3539);
					match s {
						[] => true,
						['i', 'b', 'a', 'h', 's', 'é', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3541);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11768);
							s.is_empty()
						}
						['r', 'c', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3542);
					match s {
						[] => true,
						['o', 's', s @ ..] => {
							super::push(out, 11770);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 22939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'i', 't', s @ ..] => {
			super::push(out, 551);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3543);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3544);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3546);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3547);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11772);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22940);
									s.is_empty()
								}
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
			super::push(out, 555);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 3548);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3549);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -í.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 557);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ó.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -õ.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['ó', s @ ..] => {
			super::push(out, 560);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ö.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 561);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 562);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3550);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3551);
					match s {
						[] => true,
						['r', 'o', 'g', s @ ..] => {
							super::push(out, 11774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3552);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ü.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 565);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11775);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 11776);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 22941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 's', 'l', 'g', 'n', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -č.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ş.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ū.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['y', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nica.
pub(crate) fn n3600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3591);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 11795);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 11796);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nca.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 594);
	s.is_empty()
}

/// Words ending -oca.
pub(crate) fn n596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 595);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3594);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3595);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rca.
pub(crate) fn n597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 596);
	s.is_empty()
}

/// Words ending -sca.
pub(crate) fn n598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 597);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3596);
			match s {
				[] => true,
				['u', 'h', 'a', 'y', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uca.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 598);
	match s {
		[] => true,
		['b', 'm', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 3598);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -da.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3599);
					match s {
						[] => true,
						['m', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3600);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3601);
					match s {
						[] => true,
						['i', 't', 'n', 'i', '-', s @ ..] => {
							super::push(out, 11799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3602);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3603);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3604);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3605);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11802);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 22943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3606);
					match s {
						[] => true,
						['t', 'n', s @ ..] => {
							super::push(out, 11804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3607);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3608);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3609);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 601);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3610);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3611);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3612);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11807);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11809);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 604);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3616);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3618);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3619);
					match s {
						[] => true,
						['e', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3620);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 11812);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3621);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3622);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3624);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 11815);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 22945);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 22946);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 32182);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11816);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11817);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 22947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 11818);
							match s {
								[] => true,
								['r', 'a', 'y', 'n', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 22948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3625);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 11819);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11820);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 11821);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3626);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3627);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11823);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 11824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3628);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3629);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3630);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 607);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 608);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3632);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3633);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 610);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ea.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 612);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 614);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 615);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3637);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3639);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11826);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 618);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3640);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3641);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fa.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 624);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 625);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3644);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 626);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 627);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 628);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 629);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'k', '\u{1}', s @ ..] => {
			super::push(out, 631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ga.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 632);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3650);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3651);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3653);
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
					super::push(out, 3654);
					s.is_empty()
				}
				['l', 'l', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3655);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3656);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 11829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3657);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11830);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3658);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3660);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3662);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3663);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 11833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 637);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3664);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 11834);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11835);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11836);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3665);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3666);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3667);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 11838);
							s.is_empty()
						}
						['l', 'a', 'b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11839);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 3668);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3669);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3670);
					match s {
						[] => true,
						['-', 'n', 'e', 'g', 'e', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 11842);
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
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3671);
					match s {
						[] => true,
						['m', 'h', 'c', 's', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3672);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3674);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ha.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 3678);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3679);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11844);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 11845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3680);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3681);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11846);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3682);
					match s {
						[] => true,
						['o', 'c', s @ ..] => {
							super::push(out, 11848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3683);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11849);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 22951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3684);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3685);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11850);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 22952);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 22953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11851);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 22954);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 22955);
									match s {
										[] => true,
										['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 32183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 11852);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 11853);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 22956);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 22957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11854);
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 22958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3686);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3687);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 646);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3689);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 648);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3691);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 650);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3693);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 652);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3696);
					match s {
						[] => true,
						['r', 'g', 'i', 'p', 'e', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3697);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 11858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['y', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3701);
					match s {
						[] => true,
						['n', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3703);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3704);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 11862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3706);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11863);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3707);
					match s {
						[] => true,
						['p', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ia.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 656);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 657);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3708);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 658);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 659);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3712);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11867);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 22959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3714);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11869);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 22960);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 22961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11870);
							match s {
								[] => true,
								['a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 22962);
									s.is_empty()
								}
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
			super::push(out, 662);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3718);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3720);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11871);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3721);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3722);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3723);
					match s {
						[] => true,
						['e', 'h', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11873);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 11874);
							s.is_empty()
						}
						['m', 'a', 'f', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 666);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3725);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11876);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 11877);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3726);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3727);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 11880);
							s.is_empty()
						}
						['t', 'g', 'n', 'i', 'l', 'l', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['u', 'q', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3729);
					match s {
						[] => true,
						['l', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 11882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3730);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3732);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11883);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 11884);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11885);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 22963);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 22964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11886);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 11887);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 11888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3733);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11889);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 11890);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11891);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 22965);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 22966);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 22967);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 22968);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 32184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 11892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3734);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11893);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11894);
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 22969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3735);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3736);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3737);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11896);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11897);
							match s {
								[] => true,
								['l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 22970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11898);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['o', 'r', s @ ..] => {
					super::push(out, 3739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3740);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3741);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 11900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', 'r', s @ ..] => {
					super::push(out, 3742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 3743);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['e', s @ ..] => super::t5::n3755(s, out),
				['i', s @ ..] => super::t5::n3756(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t5::n676(s, out),
		_ => false,
	}
}

/// Words ending -ja.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 676);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3750);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3751);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3752);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11901);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['n', 'u', 'f', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 680);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3756);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3757);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3758);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11903);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3759);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 11905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 682);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3761);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11906);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 11907);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 11908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3762);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3763);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 686);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3767);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3768);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3769);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3770);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ka.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3771);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3773);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3774);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3775);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 11910);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3777);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 11912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3778);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 11913);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11914);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 22971);
									s.is_empty()
								}
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
			super::push(out, 689);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 690);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['c', 's', 'u', 'l', s @ ..] => {
					super::push(out, 3783);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 11915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3784);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11916);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11917);
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 22972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3785);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11918);
							match s {
								[] => true,
								['l', 'w', '\u{1}', s @ ..] => {
									super::push(out, 22973);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3786);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11920);
							s.is_empty()
						}
						['o', 'h', s @ ..] => {
							super::push(out, 11921);
							match s {
								[] => true,
								['o', 'k', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 22974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3787);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 22975);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 22976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3788);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11923);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									super::push(out, 22977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3789);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3790);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22978);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 22979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3791);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11926);
							match s {
								[] => true,
								['n', 'a', 'v', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 22980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11927);
							match s {
								[] => true,
								['a', 'w', 's', '\u{1}', s @ ..] => {
									super::push(out, 22981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3793);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11929);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 11930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3794);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3795);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11931);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3796);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11933);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 698);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3797);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11935);
							match s {
								[] => true,
								['c', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 22982);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 11936);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11937);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3798);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11939);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 700);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3800);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 22983);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 22984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3801);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3802);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3803);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3804);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 701);
			s.is_empty()
		}
		['r', s @ ..] => super::t5::n703(s, out),
		['s', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3809);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['a', s @ ..] => super::t5::n3821(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n706(s, out),
		['w', s @ ..] => super::t5::n707(s, out),
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3817);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3818);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 11947);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11948);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 22985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3819);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3820);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 11949);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 22986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3821);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3822);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 32185);
											s.is_empty()
										}
										['t', 'r', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 38416);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 38417);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['a', s @ ..] => {
									super::push(out, 22987);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 22988);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 22989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3824);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3825);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3826);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11953);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 11954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3827);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11955);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3828);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3829);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 11957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3830);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11958);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 11959);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11960);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 709);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3832);
					match s {
						[] => true,
						['n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3833);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3835);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3836);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11963);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3838);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3839);
					match s {
						[] => true,
						['q', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 11966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3840);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3841);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11968);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 11969);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22990);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 22991);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 22992);
									match s {
										[] => true,
										['l', 'o', 'p', 'l', 'a', 'v', '\u{1}', s @ ..] => {
											super::push(out, 32186);
											s.is_empty()
										}
										['n', 'u', 't', '\u{1}', s @ ..] => {
											super::push(out, 32187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 11970);
							match s {
								[] => true,
								['e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 22993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 11971);
							s.is_empty()
						}
						['n', s @ ..] => super::t5::n12122(s, out),
						['r', s @ ..] => {
							super::push(out, 11973);
							s.is_empty()
						}
						['t', s @ ..] => super::t5::n12124(s, out),
						['z', s @ ..] => {
							super::push(out, 11975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t5::n3852(s, out),
				['o', s @ ..] => super::t5::n3853(s, out),
				['u', s @ ..] => super::t5::n3854(s, out),
				['y', s @ ..] => super::t5::n3855(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t5::n714(s, out),
		['p', s @ ..] => super::t5::n715(s, out),
		['r', s @ ..] => super::t5::n716(s, out),
		['s', s @ ..] => super::t5::n717(s, out),
		['u', s @ ..] => super::t5::n718(s, out),
		['á', s @ ..] => super::t5::n719(s, out),
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 719);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3874);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3875);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3876);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 12003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3877);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 12004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3878);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12005);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3879);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3880);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3881);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3882);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 12010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12011);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12012);
							match s {
								[] => true,
								['n', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 23009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12013);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3884);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 12015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 721);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3885);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3886);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3887);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3888);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12017);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32194);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 32195);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 32196);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 38418);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'n', s @ ..] => {
											super::push(out, 32197);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12018);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12019);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23010);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 23011);
									match s {
										[] => true,
										['p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 32198);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 32199);
											s.is_empty()
										}
										['k', 'n', 'u', 'p', 'r', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 32200);
											s.is_empty()
										}
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
					super::push(out, 3889);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 12020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3890);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12021);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 12022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12023);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12024);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12025);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 23012);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 23013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32201);
											s.is_empty()
										}
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
					super::push(out, 3892);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12026);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12027);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3893);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12029);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12030);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 12031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3894);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 12032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3895);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t5::n725(s, out),
		['l', s @ ..] => {
			super::push(out, 725);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['g', '-', s @ ..] => {
							super::push(out, 12044);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 12045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3906);
					match s {
						[] => true,
						['l', 'i', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23023);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 23024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3907);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3908);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 12046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t5::n728(s, out),
		['r', s @ ..] => super::t5::n729(s, out),
		['s', s @ ..] => super::t5::n730(s, out),
		['t', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n732(s, out),
		['y', s @ ..] => super::t5::n733(s, out),
		['ä', s @ ..] => super::t5::n734(s, out),
		_ => false,
	}
}

