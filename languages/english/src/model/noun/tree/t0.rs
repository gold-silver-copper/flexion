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
					super::push(out, 53);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 535);
							s.is_empty()
						}
						['\u{13}', 'r', s @ ..] => {
							super::push(out, 536);
							match s {
								[] => true,
								['a', 'y', 'g', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 54);
					s.is_empty()
				}
				['i', '\u{12}', 'r', s @ ..] => {
					super::push(out, 55);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 56);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 57);
					s.is_empty()
				}
				['o', '\u{12}', 'o', 'b', 'h', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 58);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 59);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 537);
							s.is_empty()
						}
						['\u{12}', 'a', s @ ..] => {
							super::push(out, 538);
							match s {
								[] => true,
								['b', 'u', 'h', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 60);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 539);
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									super::push(out, 1523);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 61);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 62);
					match s {
						[] => true,
						['\u{11}', 'v', s @ ..] => {
							super::push(out, 540);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1524);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1525);
									match s {
										[] => true,
										['a', 's', 'a', 'a', 'h', 'd', '\u{1}', s @ ..] => {
											super::push(out, 4186);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 4187);
											match s {
												[] => true,
												['z', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 7068);
													s.is_empty()
												}
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
				['d', '\u{10}', s @ ..] => {
					super::push(out, 63);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 64);
					match s {
						[] => true,
						['\u{11}', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1526);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{12}', 't', s @ ..] => {
							super::push(out, 542);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							match s {
								[] => true,
								['i', 'w', 'o', 'k', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1528);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 543);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1530);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 4188);
											match s {
												[] => true,
												['r', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 7069);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 544);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1531);
									match s {
										[] => true,
										['n', 'a', 'h', 't', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 4189);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 1532);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1533);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 4190);
											match s {
												[] => true,
												['x', s @ ..] => {
													super::push(out, 7070);
													match s {
														[] => true,
														['e', '\u{1}', s @ ..] => super::t0::n10053(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['v', 'o', 'v', 'i', 'l', 's', '\u{1}', s @ ..] => {
									super::push(out, 1534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 66);
					s.is_empty()
				}
				['n', '\u{11}', 'o', s @ ..] => {
					super::push(out, 67);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 68);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1535);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1536);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 4191);
											s.is_empty()
										}
										['q', '\u{1}', s @ ..] => {
											super::push(out, 4192);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 1537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{12}', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1538);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1539);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4193);
											match s {
												[] => true,
												['b', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n10054(s, out),
														['t', 'q', '\u{1}', s @ ..] => super::t0::n10055(s, out),
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
				['r', '\u{11}', 'a', s @ ..] => super::t0::n70(s, out),
				['s', s @ ..] => super::t0::n71(s, out),
				_ => false,
			}
		}
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
		_ => false,
	}
}

/// Words ending -a.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['\u{11}', 'w', '\u{1}', s @ ..] => {
					super::push(out, 475);
					s.is_empty()
				}
				['\u{12}', 'y', 'i', 'f', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['\u{13}', 'w', 'a', 'a', 'd', 'o', '\u{1}', s @ ..] => {
					super::push(out, 477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 479);
					match s {
						[] => true,
						['b', 'u', s @ ..] => {
							super::push(out, 1203);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1204);
							match s {
								[] => true,
								['l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1205);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1206);
							match s {
								[] => true,
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 480);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1207);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1208);
							match s {
								[] => true,
								['m', 'a', 'o', 'x', 'y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3692);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3693);
									match s {
										[] => true,
										['m', 'a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8976);
													s.is_empty()
												}
												['h', 't', 'n', 'a', 'c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 8977);
													s.is_empty()
												}
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
							super::push(out, 1209);
							s.is_empty()
						}
						['o', 't', s @ ..] => {
							super::push(out, 1210);
							match s {
								[] => true,
								['o', 'k', 'a', 'r', 'u', 'k', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3695);
									match s {
										[] => true,
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 6708);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', 'y', '\u{1}', s @ ..] => {
									super::push(out, 3696);
									s.is_empty()
								}
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
			super::push(out, 29);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1211);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 1212);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 3697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1213);
							match s {
								[] => true,
								['u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 3699);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1214);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 3701);
									match s {
										[] => true,
										['b', 'i', '\u{1}', s @ ..] => {
											super::push(out, 6709);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1215);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1216);
							match s {
								[] => true,
								['h', s @ ..] => super::t0::n3967(s, out),
								['t', s @ ..] => {
									super::push(out, 3703);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3704);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1217);
							match s {
								[] => true,
								['l', 'p', s @ ..] => super::t0::n3970(s, out),
								['n', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 3706);
									s.is_empty()
								}
								['t', 'a', 'm', 'l', 'a', 'd', '\u{1}', s @ ..] => super::t0::n3972(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t0::n1331(s, out),
						['o', s @ ..] => super::t0::n1332(s, out),
						['r', s @ ..] => super::t0::n1333(s, out),
						['s', s @ ..] => super::t0::n1334(s, out),
						['u', s @ ..] => super::t0::n1335(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n30(s, out),
		['e', s @ ..] => super::t0::n31(s, out),
		['f', s @ ..] => super::t0::n32(s, out),
		['g', s @ ..] => super::t0::n33(s, out),
		['h', s @ ..] => super::t0::n34(s, out),
		['i', s @ ..] => super::t0::n35(s, out),
		['j', s @ ..] => super::t0::n36(s, out),
		['k', s @ ..] => super::t0::n37(s, out),
		['l', s @ ..] => super::t0::n38(s, out),
		['m', s @ ..] => super::t0::n39(s, out),
		['n', s @ ..] => super::t0::n40(s, out),
		['o', s @ ..] => super::t0::n41(s, out),
		['p', s @ ..] => super::t0::n42(s, out),
		['q', s @ ..] => super::t0::n43(s, out),
		['r', s @ ..] => super::t0::n44(s, out),
		['s', s @ ..] => super::t0::n45(s, out),
		['t', s @ ..] => super::t0::n46(s, out),
		['u', s @ ..] => super::t0::n47(s, out),
		['v', s @ ..] => super::t0::n48(s, out),
		['w', s @ ..] => super::t0::n49(s, out),
		['x', s @ ..] => super::t0::n50(s, out),
		['y', s @ ..] => super::t1::n51(s, out),
		['z', s @ ..] => super::t1::n52(s, out),
		_ => false,
	}
}

/// The word exotic (3 syllables).
pub(crate) fn n10053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9193);
	s.is_empty()
}

/// The word bipoc (2 syllables).
pub(crate) fn n10054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9194);
	s.is_empty()
}

/// The word qtbipoc (2 syllables).
pub(crate) fn n10055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9195);
	s.is_empty()
}

/// Words ending -arc (1 syllables).
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['w', 'p', '\u{1}', s @ ..] => {
			super::push(out, 546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sc.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1541);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 4196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 71);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1545);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 4197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1546);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1547);
							match s {
								[] => true,
								['l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1548);
							match s {
								[] => true,
								['r', 'm', 'e', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4201);
									match s {
										[] => true,
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 7071);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4202);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 7072);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1550);
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									super::push(out, 4203);
									match s {
										[] => true,
										['a', 'p', 'i', 'c', 'i', 'n', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 7073);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'a', 'm', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1551);
							match s {
								[] => true,
								['h', 'e', 'l', 't', s @ ..] => {
									super::push(out, 4205);
									match s {
										[] => true,
										['t', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', '\u{11}', 'a', '\u{1}', s @ ..] => {
			super::push(out, 72);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				['\u{11}', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1554);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4206);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['y', 'w', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 553);
					match s {
						[] => true,
						['e', 'n', 'a', 'h', 'g', 'n', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 554);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1558);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 4207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1559);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4208);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 4209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1560);
							match s {
								[] => true,
								['s', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1561);
							match s {
								[] => true,
								['o', 'y', '\u{1}', s @ ..] => {
									super::push(out, 4211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1562);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1563);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4212);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 7076);
											match s {
												[] => true,
												['s', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 9196);
													s.is_empty()
												}
												['t', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 9197);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', 'l', 'i', 'v', 'i', 'r', 'p', 'r', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1564);
							match s {
								[] => true,
								['a', 'j', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1565);
							match s {
								[] => true,
								['l', 'e', 'g', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 4215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'b', 'a', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1566);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1567);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4216);
									match s {
										[] => true,
										['l', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 7077);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t1::n1695(s, out),
						['p', s @ ..] => {
							super::push(out, 1569);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1570);
							s.is_empty()
						}
						['s', s @ ..] => super::t1::n1698(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n76(s, out),
		['l', s @ ..] => super::t1::n77(s, out),
		['m', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['\u{10}', 'w', '\u{1}', s @ ..] => super::t1::n584(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n79(s, out),
		['o', s @ ..] => super::t1::n80(s, out),
		['p', s @ ..] => super::t1::n81(s, out),
		['r', s @ ..] => super::t1::n82(s, out),
		['u', s @ ..] => super::t1::n83(s, out),
		['z', s @ ..] => super::t1::n84(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['g', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1629);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1630);
							match s {
								[] => true,
								['t', 'h', 'c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1631);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4257);
									match s {
										[] => true,
										['o', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 7104);
											s.is_empty()
										}
										['u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 7105);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', 'v', 'e', 'a', 'v', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['e', 'h', s @ ..] => {
							super::push(out, 1632);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 4258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1634);
							match s {
								[] => true,
								['a', 'n', 'i', 'h', 's', 'i', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4259);
									s.is_empty()
								}
								['n', 'i', 'h', 's', 'i', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1635);
							match s {
								[] => true,
								['e', 'h', 'c', 's', 'r', 'a', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4261);
									s.is_empty()
								}
								['r', 'a', s @ ..] => {
									super::push(out, 4262);
									match s {
										[] => true,
										['t', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 7106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1636);
							match s {
								[] => true,
								['e', 'w', 'k', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 4263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1637);
							match s {
								[] => true,
								['a', 'k', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 581);
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1641);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4265);
									match s {
										[] => true,
										['l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 7107);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 4266);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4267);
									s.is_empty()
								}
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4268);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1643);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 4269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1644);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4270);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 4271);
									match s {
										[] => true,
										['r', 'e', 'k', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 7108);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 4272);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 7109);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 9203);
													match s {
														[] => true,
														['h', 'p', '\u{1}', s @ ..] => super::t1::n11420(s, out),
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
							super::push(out, 1645);
							match s {
								[] => true,
								['r', 't', s @ ..] => {
									super::push(out, 4273);
									match s {
										[] => true,
										['a', 's', 'i', s @ ..] => {
											super::push(out, 7110);
											match s {
												[] => true,
												['v', 'v', 'o', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9204);
													s.is_empty()
												}
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
							super::push(out, 1646);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4274);
									match s {
										[] => true,
										['v', 'i', 'n', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1647);
							match s {
								[] => true,
								['l', 'r', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4275);
									s.is_empty()
								}
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
			super::push(out, 88);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 583);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1649);
							match s {
								[] => true,
								['y', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1651);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n90(s, out),
		['f', s @ ..] => super::t1::n91(s, out),
		['g', s @ ..] => super::t1::n92(s, out),
		['h', s @ ..] => super::t1::n93(s, out),
		['i', s @ ..] => super::t1::n94(s, out),
		['j', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['\u{13}', 'n', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['a', 'h', 'z', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t1::n96(s, out),
		['l', s @ ..] => super::t1::n97(s, out),
		['m', s @ ..] => super::t1::n98(s, out),
		['n', s @ ..] => super::t1::n99(s, out),
		['o', s @ ..] => super::t1::n100(s, out),
		['p', s @ ..] => super::t1::n101(s, out),
		['r', s @ ..] => super::t1::n102(s, out),
		['s', s @ ..] => super::t1::n103(s, out),
		['t', s @ ..] => super::t1::n104(s, out),
		['u', s @ ..] => super::t1::n105(s, out),
		['v', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['\u{12}', 'g', s @ ..] => super::t1::n642(s, out),
				['\u{13}', 'k', 'e', 'p', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t1::n108(s, out),
		['z', s @ ..] => super::t1::n109(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 4512);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 4513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4514);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', 'o', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 7279);
											s.is_empty()
										}
										['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 7280);
											s.is_empty()
										}
										['h', 't', 'o', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 7281);
											s.is_empty()
										}
										['n', 'i', s @ ..] => {
											match s {
												[] => true,
												['h', 's', '\u{1}', s @ ..] => {
													super::push(out, 9311);
													s.is_empty()
												}
												['w', 't', '\u{1}', s @ ..] => {
													super::push(out, 9312);
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', 'o', 'r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 7282);
											s.is_empty()
										}
										['t', 'e', 'e', 'w', 's', '\u{1}', s @ ..] => {
											super::push(out, 7283);
											s.is_empty()
										}
										['y', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 7284);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1817);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1818);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4516);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 7285);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1820);
							match s {
								[] => true,
								['s', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4518);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 7286);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', 'v', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 7287);
											s.is_empty()
										}
										['n', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 7288);
											s.is_empty()
										}
										['o', 'n', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7289);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['d', 'n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 9313);
													s.is_empty()
												}
												['t', 'n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9314);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'l', 'r', 'a', 'g', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 110);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 112);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1824);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 4519);
									s.is_empty()
								}
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 4520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1825);
							match s {
								[] => true,
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 4521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['i', 'h', s @ ..] => {
							super::push(out, 1826);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4522);
									match s {
										[] => true,
										['k', 'a', 'e', 'n', 's', '\u{1}', s @ ..] => {
											super::push(out, 7290);
											s.is_empty()
										}
										['l', 'l', 'e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 7291);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', 'i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1827);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4523);
									match s {
										[] => true,
										['e', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7292);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7293);
											match s {
												[] => true,
												['e', s @ ..] => super::t1::n10179(s, out),
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
							super::push(out, 1828);
							s.is_empty()
						}
						['r', s @ ..] => super::t1::n1966(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t1::n115(s, out),
		['g', s @ ..] => super::t1::n116(s, out),
		['h', s @ ..] => super::t1::n117(s, out),
		['i', s @ ..] => super::t1::n118(s, out),
		['k', s @ ..] => super::t1::n119(s, out),
		['l', s @ ..] => super::t1::n120(s, out),
		['m', s @ ..] => super::t1::n121(s, out),
		['n', s @ ..] => super::t1::n122(s, out),
		['o', s @ ..] => super::t1::n123(s, out),
		['p', s @ ..] => super::t1::n124(s, out),
		['q', s @ ..] => super::t1::n125(s, out),
		['r', s @ ..] => super::t1::n126(s, out),
		['s', s @ ..] => super::t1::n127(s, out),
		['t', s @ ..] => super::t1::n128(s, out),
		['u', s @ ..] => super::t1::n129(s, out),
		['w', s @ ..] => super::t1::n130(s, out),
		_ => false,
	}
}

/// Words ending -g.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', 't', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['d', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						['i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1865);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['l', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7298);
											s.is_empty()
										}
										['w', 'o', 'n', 'k', '\u{1}', s @ ..] => {
											super::push(out, 7299);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', 'e', 'l', 'i', 'v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1867);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4547);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['h', 't', '\u{1}', s @ ..] => {
													super::push(out, 9320);
													s.is_empty()
												}
												['t', '\u{1}', s @ ..] => {
													super::push(out, 9321);
													s.is_empty()
												}
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
			super::push(out, 133);
			match s {
				[] => true,
				['\u{10}', '\u{1}', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1869);
							match s {
								[] => true,
								['n', 'e', 'f', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4548);
									s.is_empty()
								}
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
			super::push(out, 135);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1870);
							match s {
								[] => true,
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 4549);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4550);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 7300);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 7301);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1871);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 4551);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4552);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 7302);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													super::push(out, 9322);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4553);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 7303);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 1872);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1873);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 4554);
									s.is_empty()
								}
								['j', '\u{1}', s @ ..] => {
									super::push(out, 4555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => super::t1::n689(s, out),
				['\u{13}', s @ ..] => super::t1::n690(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1883);
							match s {
								[] => true,
								['c', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4581);
									s.is_empty()
								}
								['e', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', 'l', 'a', 'g', s @ ..] => super::t1::n692(s, out),
				_ => false,
			}
		}
		['p', '\u{10}', 'm', '\u{1}', s @ ..] => super::t1::n138(s, out),
		['r', s @ ..] => super::t1::n139(s, out),
		['u', s @ ..] => super::t1::n140(s, out),
		['x', '\u{10}', '\u{1}', s @ ..] => super::t1::n141(s, out),
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1884);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1885);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4583);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 7323);
											match s {
												[] => true,
												['m', '\u{1}', s @ ..] => {
													super::push(out, 9332);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 4584);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4585);
									match s {
										[] => true,
										['e', 's', '\u{1}', s @ ..] => {
											super::push(out, 7325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1886);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 4586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1888);
							match s {
								[] => true,
								['c', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 4587);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1889);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4588);
									s.is_empty()
								}
								['p', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1890);
							match s {
								[] => true,
								['l', 'a', 'h', s @ ..] => {
									super::push(out, 4590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1891);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'q', '\u{1}', s @ ..] => {
									super::push(out, 4591);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4592);
									match s {
										[] => true,
										['i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 7326);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'p', s @ ..] => {
							super::push(out, 1892);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1893);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4593);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 7327);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4594);
									match s {
										[] => true,
										['i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 7328);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1894);
							match s {
								[] => true,
								['t', 'i', s @ ..] => {
									super::push(out, 4595);
									match s {
										[] => true,
										['h', 's', '\u{1}', s @ ..] => {
											super::push(out, 7329);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1895);
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 4596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1896);
							match s {
								[] => true,
								['i', 'b', s @ ..] => {
									super::push(out, 4597);
									match s {
										[] => true,
										['a', 'h', 's', '\u{1}', s @ ..] => {
											super::push(out, 7330);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 'i', 'q', 'w', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 't', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t1::n696(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n144(s, out),
		['d', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				['\u{11}', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n146(s, out),
		['f', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['\u{11}', s @ ..] => super::t1::n708(s, out),
				['\u{12}', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['\u{13}', s @ ..] => super::t1::n710(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t1::n149(s, out),
		['i', s @ ..] => super::t1::n150(s, out),
		['k', s @ ..] => super::t1::n151(s, out),
		['l', s @ ..] => super::t1::n152(s, out),
		['m', s @ ..] => super::t1::n153(s, out),
		['n', s @ ..] => super::t1::n154(s, out),
		['o', s @ ..] => super::t1::n155(s, out),
		['p', s @ ..] => super::t1::n156(s, out),
		['r', s @ ..] => super::t1::n157(s, out),
		['s', s @ ..] => super::t1::n158(s, out),
		['t', s @ ..] => super::t1::n159(s, out),
		['u', s @ ..] => super::t1::n160(s, out),
		['v', s @ ..] => super::t1::n161(s, out),
		['w', s @ ..] => super::t1::n162(s, out),
		['z', s @ ..] => super::t1::n163(s, out),
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1995);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 4780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1996);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1997);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1998);
							s.is_empty()
						}
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1999);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 4782);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2001);
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4784);
									s.is_empty()
								}
								['n', 'u', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4785);
									s.is_empty()
								}
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'u', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 2002);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4787);
									s.is_empty()
								}
								['o', 'y', 'k', '\u{1}', s @ ..] => {
									super::push(out, 4788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'o', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 4789);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 't', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2005);
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 4791);
									s.is_empty()
								}
								['i', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4792);
									s.is_empty()
								}
								['o', 'y', '\u{1}', s @ ..] => {
									super::push(out, 4793);
									s.is_empty()
								}
								['u', 'o', 'y', '\u{1}', s @ ..] => {
									super::push(out, 4794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2006);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2007);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4796);
									s.is_empty()
								}
								['u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 4797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2008);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 4798);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4800);
									s.is_empty()
								}
								['n', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2009);
							match s {
								[] => true,
								['i', 'h', 'k', '\u{1}', s @ ..] => {
									super::push(out, 4802);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4803);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 9441);
													s.is_empty()
												}
												['t', '\u{1}', s @ ..] => {
													super::push(out, 9442);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', 's', '\u{1}', s @ ..] => {
											super::push(out, 7469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['b', 'u', s @ ..] => {
							super::push(out, 2010);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 4804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'u', 'b', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2011);
							s.is_empty()
						}
						['f', 'i', 'a', 'f', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 2012);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2013);
							s.is_empty()
						}
						['k', s @ ..] => super::t1::n2169(s, out),
						['l', s @ ..] => {
							super::push(out, 2014);
							s.is_empty()
						}
						['m', 'a', 'm', 'a', 'y', '\u{1}', s @ ..] => {
							super::push(out, 2015);
							s.is_empty()
						}
						['p', s @ ..] => super::t1::n2172(s, out),
						['r', s @ ..] => super::t1::n2173(s, out),
						['t', s @ ..] => super::t1::n2174(s, out),
						['v', 'i', 'a', 'v', 'i', 't', '\u{1}', s @ ..] => super::t1::n2175(s, out),
						['w', s @ ..] => super::t1::n2176(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => super::t1::n166(s, out),
		['c', s @ ..] => super::t1::n167(s, out),
		['d', s @ ..] => super::t1::n168(s, out),
		['e', s @ ..] => super::t1::n169(s, out),
		['f', s @ ..] => super::t1::n170(s, out),
		['g', s @ ..] => super::t1::n171(s, out),
		['h', s @ ..] => super::t1::n172(s, out),
		['i', s @ ..] => super::t1::n173(s, out),
		['j', s @ ..] => super::t1::n174(s, out),
		['k', s @ ..] => super::t1::n175(s, out),
		['l', s @ ..] => super::t1::n176(s, out),
		['m', s @ ..] => super::t1::n177(s, out),
		['n', s @ ..] => super::t1::n178(s, out),
		['o', s @ ..] => super::t1::n179(s, out),
		['p', s @ ..] => super::t1::n180(s, out),
		['q', s @ ..] => super::t1::n181(s, out),
		['r', s @ ..] => super::t1::n182(s, out),
		['s', s @ ..] => super::t1::n183(s, out),
		['t', s @ ..] => super::t1::n184(s, out),
		['u', s @ ..] => super::t1::n185(s, out),
		['v', s @ ..] => super::t1::n186(s, out),
		['w', s @ ..] => super::t1::n187(s, out),
		['x', s @ ..] => super::t1::n188(s, out),
		['y', s @ ..] => super::t1::n189(s, out),
		['z', s @ ..] => super::t1::n190(s, out),
		_ => false,
	}
}

/// Words ending -j.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2172);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 2173);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 2175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['j', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2176);
							s.is_empty()
						}
						['l', 'a', 'h', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2177);
							s.is_empty()
						}
						['m', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 2178);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['d', '\u{11}', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2180);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['\u{11}', 'z', '\u{1}', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2182);
							s.is_empty()
						}
						['o', 'r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2183);
							s.is_empty()
						}
						['v', 't', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', '\u{11}', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2185);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2186);
							match s {
								[] => true,
								['f', 's', '\u{1}', s @ ..] => {
									super::push(out, 5004);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 5005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', '\u{11}', 'o', 'y', '\u{1}', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['\u{13}', 'i', 'k', 'a', 'r', 'r', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 201);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 746);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2187);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 5006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 2188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2189);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2190);
							s.is_empty()
						}
						['i', 'm', s @ ..] => {
							super::push(out, 2191);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2192);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2193);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2194);
							match s {
								[] => true,
								['a', 'v', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5010);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2195);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 748);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2196);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2197);
							match s {
								[] => true,
								['a', 'p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5012);
									s.is_empty()
								}
								['u', 'z', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2198);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5014);
									match s {
										[] => true,
										['h', 's', 'o', s @ ..] => {
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 9475);
													s.is_empty()
												}
												['t', '\u{1}', s @ ..] => {
													super::push(out, 9476);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'o', s @ ..] => {
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 9477);
													s.is_empty()
												}
												['t', '\u{1}', s @ ..] => {
													super::push(out, 9478);
													s.is_empty()
												}
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
		['c', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5015);
									s.is_empty()
								}
								['j', 'p', 'i', s @ ..] => {
									super::push(out, 5016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'p', 's', s @ ..] => {
							super::push(out, 2199);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'd', s @ ..] => {
									super::push(out, 5017);
									match s {
										[] => true,
										['o', 'o', 'w', '\u{1}', s @ ..] => {
											super::push(out, 7605);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'd', s @ ..] => {
									super::push(out, 5018);
									match s {
										[] => true,
										['a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7606);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2202);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 5019);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 7607);
											match s {
												[] => true,
												['s', 'u', 'l', 'l', 'o', 'h', '\u{1}', s @ ..] => {
													super::push(out, 9479);
													s.is_empty()
												}
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
							super::push(out, 2203);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5020);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5021);
									match s {
										[] => true,
										['a', 'h', 'o', 'c', 'u', 'o', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 7608);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', 'e', s @ ..] => {
									super::push(out, 5022);
									match s {
										[] => true,
										['b', 'u', 't', '\u{1}', s @ ..] => {
											super::push(out, 7609);
											s.is_empty()
										}
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
			super::push(out, 203);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2204);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 5023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 751);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2205);
							s.is_empty()
						}
						['f', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 2206);
							s.is_empty()
						}
						['l', s @ ..] => super::t1::n2396(s, out),
						['n', s @ ..] => super::t1::n2397(s, out),
						['o', s @ ..] => super::t1::n2398(s, out),
						['r', s @ ..] => super::t1::n2399(s, out),
						['z', s @ ..] => super::t1::n2400(s, out),
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t1::n809(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n205(s, out),
		['l', s @ ..] => super::t1::n206(s, out),
		['n', s @ ..] => super::t1::n207(s, out),
		['o', s @ ..] => super::t1::n208(s, out),
		['r', s @ ..] => super::t1::n209(s, out),
		['s', s @ ..] => super::t1::n210(s, out),
		['u', s @ ..] => super::t1::n211(s, out),
		['y', s @ ..] => super::t1::n212(s, out),
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2240);
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5058);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 5059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2241);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5060);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2242);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5061);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 5062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2243);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 768);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2245);
							match s {
								[] => true,
								['n', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2246);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2247);
							match s {
								[] => true,
								['k', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 5065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2249);
							match s {
								[] => true,
								['r', 'u', s @ ..] => {
									super::push(out, 5066);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2250);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2251);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5067);
									match s {
										[] => true,
										['h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7628);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2252);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5068);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 7629);
											match s {
												[] => true,
												['e', 'm', s @ ..] => {
													super::push(out, 9484);
													s.is_empty()
												}
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
							super::push(out, 2253);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 5069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2254);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5070);
									match s {
										[] => true,
										['r', 'a', 'm', 't', 'r', 'u', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2255);
							match s {
								[] => true,
								['u', 'n', 'e', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2256);
							match s {
								[] => true,
								['i', 'g', s @ ..] => {
									super::push(out, 5072);
									match s {
										[] => true,
										['r', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 7631);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'g', s @ ..] => {
									super::push(out, 5073);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 7632);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													super::push(out, 9485);
													s.is_empty()
												}
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
							super::push(out, 2257);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2259);
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
				['\u{10}', 'b', '\u{1}', s @ ..] => {
					super::push(out, 770);
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
				['\u{12}', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5074);
									match s {
										[] => true,
										['i', 't', 'h', 's', '\u{1}', s @ ..] => {
											super::push(out, 7633);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2261);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2262);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 5076);
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 7634);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2263);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2264);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									super::push(out, 5078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2266);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2267);
							s.is_empty()
						}
						['u', 'q', 'n', s @ ..] => {
							super::push(out, 2268);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t1::n831(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n217(s, out),
		['l', s @ ..] => super::t1::n218(s, out),
		['m', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['o', s @ ..] => super::t1::n220(s, out),
		['r', s @ ..] => super::t1::n221(s, out),
		['t', s @ ..] => super::t1::n222(s, out),
		['u', s @ ..] => super::t1::n223(s, out),
		['w', s @ ..] => super::t1::n224(s, out),
		['y', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['\u{11}', 'e', 'r', s @ ..] => {
					super::push(out, 793);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 794);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2322);
							match s {
								[] => true,
								['c', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						['j', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						['l', 'l', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2327);
							match s {
								[] => true,
								['m', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 5123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2328);
							match s {
								[] => true,
								['n', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 5124);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 796);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2331);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{11}', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 228);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 798);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							super::push(out, 2333);
							match s {
								[] => true,
								['o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2334);
							match s {
								[] => true,
								['t', 'n', s @ ..] => {
									super::push(out, 5126);
									match s {
										[] => true,
										['a', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 7655);
											s.is_empty()
										}
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
			super::push(out, 230);
			match s {
				[] => true,
				['\u{10}', 's', s @ ..] => {
					super::push(out, 800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2336);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2337);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2338);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['h', 'p', 'a', 'r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2339);
							s.is_empty()
						}
						['l', 'i', 'h', 'p', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 804);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2341);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 5129);
									s.is_empty()
								}
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
			super::push(out, 235);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 805);
					match s {
						[] => true,
						['p', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{11}', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 236);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2343);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2344);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2345);
							match s {
								[] => true,
								['b', 'd', 'n', 'a', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5130);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['p', 'o', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 810);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2347);
							s.is_empty()
						}
						['f', 's', '\u{1}', s @ ..] => {
							super::push(out, 2348);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2349);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 2350);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2351);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2352);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 2353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{11}', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 239);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['\u{11}', 'e', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', 'e', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2355);
							match s {
								[] => true,
								['s', 'u', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 814);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 2356);
							match s {
								[] => true,
								['s', 'a', s @ ..] => {
									super::push(out, 5132);
									match s {
										[] => true,
										['r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 7656);
											s.is_empty()
										}
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
			super::push(out, 241);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 815);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2357);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{11}', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2359);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2360);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5133);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 7657);
											s.is_empty()
										}
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
		['t', s @ ..] => super::t1::n243(s, out),
		['u', s @ ..] => super::t1::n244(s, out),
		['v', s @ ..] => super::t1::n245(s, out),
		['w', s @ ..] => super::t1::n246(s, out),
		['x', s @ ..] => super::t1::n247(s, out),
		['y', s @ ..] => super::t1::n248(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => super::t1::n249(s, out),
		['c', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 828);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2451);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 5315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', s @ ..] => {
							super::push(out, 2452);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 5316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2453);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5317);
									s.is_empty()
								}
								['h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 2454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 829);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2455);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2456);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['o', 'y', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5319);
									s.is_empty()
								}
								['r', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5320);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2457);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'k', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2458);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2459);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5322);
									match s {
										[] => true,
										['h', 's', '\u{1}', s @ ..] => {
											super::push(out, 7976);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'l', 'a', s @ ..] => {
							super::push(out, 2460);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 5323);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2461);
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 5324);
									s.is_empty()
								}
								['h', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 5325);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5326);
									match s {
										[] => true,
										['r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 7977);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 7978);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'r', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 5327);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 7979);
											s.is_empty()
										}
										['u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 7980);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5328);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5329);
									match s {
										[] => true,
										['l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 7981);
											s.is_empty()
										}
										['n', '\u{1}', s @ ..] => {
											super::push(out, 7982);
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											super::push(out, 7983);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2462);
							match s {
								[] => true,
								['i', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 5330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2463);
							s.is_empty()
						}
						['p', 'p', s @ ..] => {
							super::push(out, 2464);
							match s {
								[] => true,
								['a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5331);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2465);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5332);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t1::n2666(s, out),
						['t', s @ ..] => super::t1::n2667(s, out),
						['y', s @ ..] => super::t1::n2668(s, out),
						['z', s @ ..] => super::t1::n2669(s, out),
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t1::n889(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t1::n253(s, out),
		['g', s @ ..] => super::t1::n254(s, out),
		['h', s @ ..] => super::t1::n255(s, out),
		['i', s @ ..] => super::t1::n256(s, out),
		['k', s @ ..] => super::t1::n257(s, out),
		['l', s @ ..] => super::t1::n258(s, out),
		['m', s @ ..] => super::t1::n259(s, out),
		['n', s @ ..] => super::t1::n260(s, out),
		['o', s @ ..] => super::t1::n261(s, out),
		['p', s @ ..] => super::t1::n262(s, out),
		['r', s @ ..] => super::t1::n263(s, out),
		['s', s @ ..] => super::t1::n264(s, out),
		['t', s @ ..] => super::t1::n265(s, out),
		['u', s @ ..] => super::t1::n266(s, out),
		['w', s @ ..] => super::t1::n267(s, out),
		['x', s @ ..] => super::t1::n268(s, out),
		['y', s @ ..] => super::t1::n269(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 269);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5530);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 5531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'j', '\u{1}', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2582);
							match s {
								[] => true,
								['g', 'n', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5532);
									s.is_empty()
								}
								['n', 'a', 'u', 'y', '\u{1}', s @ ..] => {
									super::push(out, 5533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2583);
							match s {
								[] => true,
								['j', 'n', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 5534);
									s.is_empty()
								}
								['n', 'n', 'e', 'h', 'z', '\u{1}', s @ ..] => {
									super::push(out, 5535);
									s.is_empty()
								}
								['p', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 5536);
									s.is_empty()
								}
								['t', 'u', 'o', 'y', '\u{1}', s @ ..] => {
									super::push(out, 5537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2584);
							match s {
								[] => true,
								['o', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 5538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'u', s @ ..] => {
							super::push(out, 2585);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2586);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['b', 'g', 'n', 'o', 'l', 'o', 'a', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						['d', 'e', 'u', 'y', 'n', 'a', 'y', '\u{1}', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2590);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8187);
											s.is_empty()
										}
										['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8188);
											s.is_empty()
										}
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
		['b', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				['\u{12}', 'm', s @ ..] => {
					super::push(out, 857);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 858);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2592);
							match s {
								[] => true,
								['l', 'g', 'g', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2593);
							match s {
								[] => true,
								['b', 'u', 'r', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5541);
									s.is_empty()
								}
								['h', 'o', 'b', 'o', 'h', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5542);
									s.is_empty()
								}
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
			super::push(out, 272);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 859);
					match s {
						[] => true,
						['c', 'u', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2595);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2596);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 860);
					match s {
						[] => true,
						['a', 'd', 'n', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2598);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5544);
									match s {
										[] => true,
										['t', 't', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8189);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2599);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2600);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5545);
									match s {
										[] => true,
										['i', 'f', 'n', 'a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 8190);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 8191);
											match s {
												[] => true,
												['n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'a', 'f', 'a', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2602);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 861);
					s.is_empty()
				}
				['\u{12}', s @ ..] => super::t1::n927(s, out),
				['\u{13}', s @ ..] => super::t1::n928(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['\u{12}', s @ ..] => super::t1::n929(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 275);
			s.is_empty()
		}
		['g', s @ ..] => super::t1::n277(s, out),
		['h', s @ ..] => super::t1::n278(s, out),
		['i', s @ ..] => super::t2::n279(s, out),
		['j', s @ ..] => super::t2::n280(s, out),
		['k', s @ ..] => super::t2::n281(s, out),
		['l', s @ ..] => super::t2::n282(s, out),
		['m', s @ ..] => super::t2::n283(s, out),
		['n', s @ ..] => super::t2::n284(s, out),
		['o', s @ ..] => super::t2::n285(s, out),
		['p', s @ ..] => super::t2::n286(s, out),
		['r', s @ ..] => super::t2::n287(s, out),
		['s', s @ ..] => super::t2::n288(s, out),
		['t', s @ ..] => super::t2::n289(s, out),
		['u', s @ ..] => super::t2::n290(s, out),
		['v', s @ ..] => super::t2::n291(s, out),
		['w', s @ ..] => super::t2::n292(s, out),
		['x', s @ ..] => super::t2::n293(s, out),
		['y', s @ ..] => super::t2::n294(s, out),
		['z', s @ ..] => super::t2::n295(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', '\u{11}', 'u', '\u{1}', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['\u{10}', 'k', '\u{1}', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['\u{11}', 'e', 'h', s @ ..] => {
					super::push(out, 906);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', 'e', s @ ..] => {
					super::push(out, 907);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2748);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5728);
									match s {
										[] => true,
										['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8322);
											s.is_empty()
										}
										['r', 'e', 't', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 8323);
											s.is_empty()
										}
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
		['h', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['\u{11}', 't', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 909);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['\u{11}', 'u', '\u{1}', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 301);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 911);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 2750);
							match s {
								[] => true,
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 5729);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5730);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', s @ ..] => {
							super::push(out, 2751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 912);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2752);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2754);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', '\u{10}', s @ ..] => {
			super::push(out, 304);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							super::push(out, 2756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5731);
									s.is_empty()
								}
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
			super::push(out, 306);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					match s {
						[] => true,
						['b', 't', '\u{1}', s @ ..] => {
							super::push(out, 2757);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{11}', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['a', 'm', 'i', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', '\u{11}', 'c', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['\u{10}', '\u{1}', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				['\u{11}', 'e', '\u{1}', s @ ..] => {
					super::push(out, 919);
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
	match s {
		[] => true,
		['a', '\u{12}', 'm', s @ ..] => {
			super::push(out, 309);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 920);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 921);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2761);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2763);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5733);
									match s {
										[] => true,
										['e', 's', '\u{1}', s @ ..] => {
											super::push(out, 8324);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 8325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2764);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5734);
									s.is_empty()
								}
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
			super::push(out, 312);
			match s {
				[] => true,
				['\u{10}', 'p', '\u{1}', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2765);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5735);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 8326);
											s.is_empty()
										}
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

/// Words ending -r.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2766);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 5736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2768);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2769);
							s.is_empty()
						}
						['d', 'l', s @ ..] => {
							super::push(out, 2770);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 5737);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2771);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2772);
							s.is_empty()
						}
						['g', 'r', 'e', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2773);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2774);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5738);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5739);
									match s {
										[] => true,
										['a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8327);
											s.is_empty()
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
								['s', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5740);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5741);
									match s {
										[] => true,
										['y', '\u{1}', s @ ..] => {
											super::push(out, 8328);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2775);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									super::push(out, 5742);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2776);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5744);
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8329);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5745);
									match s {
										[] => true,
										['a', 's', '\u{1}', s @ ..] => {
											super::push(out, 8330);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2777);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5746);
									match s {
										[] => true,
										['i', 'q', '\u{1}', s @ ..] => {
											super::push(out, 8331);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 2778);
							s.is_empty()
						}
						['x', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2780);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2781);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2782);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5747);
									match s {
										[] => true,
										['d', 'o', 'c', 'r', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 8332);
											s.is_empty()
										}
										['g', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8333);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2783);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['\u{11}', 'l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n317(s, out),
		['g', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2819);
							s.is_empty()
						}
						['e', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						['v', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2822);
							match s {
								[] => true,
								['h', 't', 's', 'e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5791);
									s.is_empty()
								}
								['p', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 5792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2823);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5793);
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 8361);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2824);
							match s {
								[] => true,
								['n', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2826);
							match s {
								[] => true,
								['y', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2828);
							match s {
								[] => true,
								['f', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2829);
							match s {
								[] => true,
								['m', 'u', 'i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 5797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2830);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 2831);
							match s {
								[] => true,
								['a', 'h', s @ ..] => {
									super::push(out, 5798);
									match s {
										[] => true,
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8362);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2832);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t2::n321(s, out),
		['l', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['\u{11}', 'u', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t2::n323(s, out),
		['o', s @ ..] => super::t2::n324(s, out),
		['p', s @ ..] => super::t2::n325(s, out),
		['r', s @ ..] => super::t2::n326(s, out),
		['s', s @ ..] => super::t2::n327(s, out),
		['u', s @ ..] => super::t2::n328(s, out),
		['w', s @ ..] => super::t2::n329(s, out),
		['y', s @ ..] => {
			super::push(out, 329);
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
		['\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 330);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2869);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5822);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 5823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2870);
							s.is_empty()
						}
						['c', 't', '\u{1}', s @ ..] => {
							super::push(out, 2871);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2872);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5824);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2873);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 5826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2874);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 5827);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2876);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5828);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 5829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2878);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2879);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2880);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5830);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 5831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2882);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2885);
							match s {
								[] => true,
								['r', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5833);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5834);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8374);
											s.is_empty()
										}
										['l', '\u{1}', s @ ..] => {
											super::push(out, 8375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2886);
							match s {
								[] => true,
								['p', 'i', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2887);
							s.is_empty()
						}
						['g', s @ ..] => super::t2::n3103(s, out),
						['i', s @ ..] => super::t2::n3104(s, out),
						['l', s @ ..] => super::t2::n3105(s, out),
						['m', s @ ..] => super::t2::n3106(s, out),
						['n', s @ ..] => super::t2::n3107(s, out),
						['p', s @ ..] => super::t2::n3108(s, out),
						['r', s @ ..] => super::t2::n3109(s, out),
						['t', s @ ..] => super::t2::n3110(s, out),
						['w', 'h', 'k', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t2::n1023(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t2::n333(s, out),
		['c', s @ ..] => super::t2::n334(s, out),
		['d', s @ ..] => super::t2::n335(s, out),
		['e', s @ ..] => super::t2::n336(s, out),
		['f', s @ ..] => super::t2::n337(s, out),
		['g', s @ ..] => super::t2::n338(s, out),
		['h', s @ ..] => super::t2::n339(s, out),
		['i', s @ ..] => super::t2::n340(s, out),
		['j', '\u{10}', '\u{1}', s @ ..] => super::t2::n341(s, out),
		['k', s @ ..] => super::t2::n342(s, out),
		['l', s @ ..] => super::t2::n343(s, out),
		['m', s @ ..] => super::t2::n344(s, out),
		['n', s @ ..] => super::t2::n345(s, out),
		['o', s @ ..] => super::t2::n346(s, out),
		['p', s @ ..] => super::t2::n347(s, out),
		['q', s @ ..] => super::t2::n348(s, out),
		['r', s @ ..] => super::t2::n349(s, out),
		['s', s @ ..] => super::t2::n350(s, out),
		['t', s @ ..] => super::t2::n351(s, out),
		['u', s @ ..] => super::t2::n352(s, out),
		['v', s @ ..] => super::t2::n353(s, out),
		['w', s @ ..] => super::t2::n354(s, out),
		['x', s @ ..] => super::t2::n355(s, out),
		['y', s @ ..] => super::t2::n356(s, out),
		['z', s @ ..] => super::t2::n357(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3238);
							match s {
								[] => true,
								['r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 6367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'p', s @ ..] => {
							super::push(out, 3239);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6368);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 3240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3241);
							s.is_empty()
						}
						['e', 'n', s @ ..] => {
							super::push(out, 3242);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3243);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3244);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6370);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 8808);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3245);
							match s {
								[] => true,
								['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 6371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1018);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3247);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3248);
							match s {
								[] => true,
								['c', 'a', 'p', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3249);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3250);
							match s {
								[] => true,
								['a', 's', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3251);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6374);
									match s {
										[] => true,
										['e', 'd', 'u', 'j', '\u{1}', s @ ..] => {
											super::push(out, 8809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3252);
							match s {
								[] => true,
								['i', 'k', s @ ..] => {
									super::push(out, 6375);
									s.is_empty()
								}
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
			super::push(out, 358);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3253);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6376);
									s.is_empty()
								}
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
			super::push(out, 359);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3254);
							match s {
								[] => true,
								['c', 'o', 'h', s @ ..] => {
									super::push(out, 6377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3255);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 6378);
									match s {
										[] => true,
										['y', 'r', '\u{1}', s @ ..] => {
											super::push(out, 8810);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6379);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8811);
											match s {
												[] => true,
												['k', '\u{1}', s @ ..] => {
													super::push(out, 10231);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 8812);
											match s {
												[] => true,
												['o', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 10232);
													s.is_empty()
												}
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
							super::push(out, 3256);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 6380);
									match s {
										[] => true,
										['m', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8813);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 6381);
									match s {
										[] => true,
										['y', 'i', 'g', '\u{1}', s @ ..] => {
											super::push(out, 8814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', s @ ..] => {
							super::push(out, 3257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1022);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 3258);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6382);
									match s {
										[] => true,
										['l', 'h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3259);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6383);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 8816);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 8817);
											match s {
												[] => true,
												['l', 'c', 'u', '\u{1}', s @ ..] => {
													super::push(out, 10233);
													s.is_empty()
												}
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
							super::push(out, 3260);
							match s {
								[] => true,
								['a', 'g', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t2::n361(s, out),
		['g', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['\u{11}', 'a', s @ ..] => {
					super::push(out, 1025);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3270);
							s.is_empty()
						}
						['g', 'i', 'e', s @ ..] => {
							super::push(out, 3271);
							match s {
								[] => true,
								['w', 'd', s @ ..] => {
									super::push(out, 6391);
									match s {
										[] => true,
										['e', 'r', 'd', 'n', 'u', 'h', '\u{1}', s @ ..] => super::t2::n9652(s, out),
										['n', 'a', 's', 'u', 'o', 'h', 't', '\u{1}', s @ ..] => super::t2::n9653(s, out),
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
		['i', s @ ..] => super::t2::n364(s, out),
		['k', s @ ..] => super::t2::n365(s, out),
		['l', s @ ..] => super::t2::n366(s, out),
		['n', s @ ..] => super::t2::n367(s, out),
		['o', s @ ..] => super::t2::n368(s, out),
		['p', '\u{10}', s @ ..] => super::t2::n369(s, out),
		['r', s @ ..] => super::t2::n370(s, out),
		['s', s @ ..] => super::t2::n371(s, out),
		['t', s @ ..] => super::t2::n372(s, out),
		['u', s @ ..] => super::t2::n373(s, out),
		['w', '\u{10}', 'c', '\u{1}', s @ ..] => super::t2::n374(s, out),
		['x', s @ ..] => super::t2::n375(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 375);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3331);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 6438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3332);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 6439);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3333);
							match s {
								[] => true,
								['a', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 6440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3334);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3335);
							match s {
								[] => true,
								['b', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6441);
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 10251);
													s.is_empty()
												}
												['o', 'r', '\u{1}', s @ ..] => {
													super::push(out, 10252);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8861);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'b', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 6442);
									s.is_empty()
								}
								['n', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6443);
									s.is_empty()
								}
								['r', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6444);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													super::push(out, 10253);
													s.is_empty()
												}
												['g', '\u{1}', s @ ..] => {
													super::push(out, 10254);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8862);
											s.is_empty()
										}
										['t', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 8863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3336);
							s.is_empty()
						}
						['i', 'l', 'b', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3337);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3338);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6445);
									match s {
										[] => true,
										['a', 'h', 'w', '\u{1}', s @ ..] => {
											super::push(out, 8864);
											s.is_empty()
										}
										['h', 'w', '\u{1}', s @ ..] => {
											super::push(out, 8865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', 'e', s @ ..] => {
					super::push(out, 1057);
					match s {
						[] => true,
						['d', 'i', 'o', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3340);
							s.is_empty()
						}
						['l', 'e', 'n', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3341);
							s.is_empty()
						}
						['r', 'e', 'h', 'c', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3342);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'o', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6446);
									s.is_empty()
								}
								['n', 'a', 'm', 'e', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6447);
									s.is_empty()
								}
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
			super::push(out, 377);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 't', '\u{1}', s @ ..] => {
											super::push(out, 8866);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 8867);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6448);
									match s {
										[] => true,
										['h', 't', '\u{1}', s @ ..] => {
											super::push(out, 8868);
											s.is_empty()
										}
										['z', 'n', '\u{1}', s @ ..] => {
											super::push(out, 8869);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'a', 'g', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						['o', 'y', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 'c', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3345);
							s.is_empty()
						}
						['o', 'n', 'i', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 3346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['g', 'n', 'o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3347);
							s.is_empty()
						}
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['n', 'u', 'b', 'i', 'm', 'i', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3349);
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
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1060);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6449);
									s.is_empty()
								}
								['l', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6450);
									s.is_empty()
								}
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
			super::push(out, 381);
			match s {
				[] => true,
				['\u{12}', s @ ..] => super::t2::n1144(s, out),
				['\u{13}', 'u', 'k', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n383(s, out),
		['h', s @ ..] => super::t2::n384(s, out),
		['i', s @ ..] => super::t2::n385(s, out),
		['j', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['\u{12}', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t2::n387(s, out),
		['l', s @ ..] => super::t2::n388(s, out),
		['m', s @ ..] => super::t2::n389(s, out),
		['n', s @ ..] => super::t2::n390(s, out),
		['o', s @ ..] => super::t2::n391(s, out),
		['p', s @ ..] => super::t2::n392(s, out),
		['q', s @ ..] => super::t2::n393(s, out),
		['r', s @ ..] => super::t2::n394(s, out),
		['s', s @ ..] => super::t2::n395(s, out),
		['t', s @ ..] => super::t2::n396(s, out),
		['u', s @ ..] => super::t2::n397(s, out),
		['v', s @ ..] => super::t2::n398(s, out),
		['x', s @ ..] => super::t2::n399(s, out),
		['y', s @ ..] => super::t2::n400(s, out),
		['z', s @ ..] => {
			super::push(out, 398);
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
		['e', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3414);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3415);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', '\u{10}', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['g', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 401);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['\u{13}', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['t', 'o', s @ ..] => {
							super::push(out, 3417);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 6503);
									match s {
										[] => true,
										['s', 'g', 'n', 'u', 'n', 'n', 'e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 8892);
											s.is_empty()
										}
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
			super::push(out, 403);
			match s {
				[] => true,
				['\u{11}', 'y', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['r', 'i', 'e', '\u{1}', s @ ..] => {
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

/// Words ending -w.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1090);
					match s {
						[] => true,
						['l', 'm', 'a', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3419);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3420);
							match s {
								[] => true,
								['t', 'o', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', '\u{10}', 'w', '\u{1}', s @ ..] => {
			super::push(out, 405);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['\u{10}', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['\u{13}', 'h', 's', 'o', 'k', 'i', 't', 's', 'i', 'm', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['\u{12}', 'a', 'r', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1093);
					s.is_empty()
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['e', 'm', 'a', 'k', 'i', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3421);
							s.is_empty()
						}
						['o', 't', 'l', 'i', 'w', 'q', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1095);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3423);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3424);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 6505);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['\u{12}', 'a', 'r', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 412);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 413);
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
			super::push(out, 414);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3425);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 3426);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 3427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3428);
							match s {
								[] => true,
								['y', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6507);
									s.is_empty()
								}
								['r', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3429);
							match s {
								[] => true,
								['o', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 6509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3430);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 6510);
									match s {
										[] => true,
										['h', 'p', 'o', s @ ..] => {
											match s {
												[] => true,
												['m', 'o', 'n', '\u{1}', s @ ..] => {
													super::push(out, 10259);
													s.is_empty()
												}
												['t', 'a', 'm', 'o', 's', '\u{1}', s @ ..] => {
													super::push(out, 10260);
													s.is_empty()
												}
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
							super::push(out, 3431);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3432);
							match s {
								[] => true,
								['o', 'h', 't', 'o', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 8893);
											match s {
												[] => true,
												['y', s @ ..] => {
													super::push(out, 10261);
													match s {
														[] => true,
														['h', s @ ..] => super::t2::n12026(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', 'i', 'r', 'u', '\u{1}', s @ ..] => {
											super::push(out, 8894);
											s.is_empty()
										}
										['r', 'e', 't', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 8895);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				['\u{11}', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['\u{12}', 'e', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3434);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						['g', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3436);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3437);
							match s {
								[] => true,
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 6511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3439);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 6512);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8896);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 8897);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 6513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6514);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6515);
									s.is_empty()
								}
								['u', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3440);
							s.is_empty()
						}
						['l', s @ ..] => super::t2::n3686(s, out),
						['m', s @ ..] => super::t2::n3687(s, out),
						['n', s @ ..] => super::t2::n3688(s, out),
						['p', s @ ..] => super::t2::n3689(s, out),
						['r', 'b', 'm', 'i', '\u{1}', s @ ..] => super::t2::n3690(s, out),
						['t', s @ ..] => super::t2::n3691(s, out),
						_ => false,
					}
				}
				['\u{13}', s @ ..] => super::t2::n1205(s, out),
				_ => false,
			}
		}
		['f', '\u{10}', 'r', '\u{1}', s @ ..] => super::t2::n421(s, out),
		['h', '\u{10}', 'l', '\u{1}', s @ ..] => super::t2::n422(s, out),
		['i', s @ ..] => super::t2::n423(s, out),
		['j', '\u{11}', 'u', 'r', 'b', '\u{1}', s @ ..] => super::t2::n424(s, out),
		['l', s @ ..] => super::t2::n425(s, out),
		['m', s @ ..] => super::t2::n426(s, out),
		['n', s @ ..] => super::t2::n427(s, out),
		['o', s @ ..] => super::t2::n428(s, out),
		['r', s @ ..] => super::t2::n429(s, out),
		['t', s @ ..] => super::t2::n430(s, out),
		['u', s @ ..] => super::t2::n431(s, out),
		['x', s @ ..] => super::t2::n432(s, out),
		['y', s @ ..] => super::t2::n433(s, out),
		['z', '\u{12}', 'i', 't', 's', 'e', 'm', '\u{1}', s @ ..] => super::t2::n434(s, out),
		_ => false,
	}
}

/// Words ending -y.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1124);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['g', 'n', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3498);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['r', 'u', s @ ..] => {
							super::push(out, 3500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3501);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3503);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3504);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 6593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3505);
							match s {
								[] => true,
								['n', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 6594);
									s.is_empty()
								}
								['o', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', 'i', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 3506);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3507);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['a', 's', '\u{1}', s @ ..] => {
											super::push(out, 8925);
											s.is_empty()
										}
										['r', 't', '\u{1}', s @ ..] => {
											super::push(out, 8926);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', s @ ..] => {
							super::push(out, 3508);
							match s {
								[] => true,
								['y', '\u{1}', s @ ..] => {
									super::push(out, 6597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3509);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 6598);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 6599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3510);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['s', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6600);
									s.is_empty()
								}
								['t', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 6601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'o', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3511);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6602);
									s.is_empty()
								}
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3512);
							match s {
								[] => true,
								['k', 'c', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 6604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3513);
							match s {
								[] => true,
								['l', 't', 'r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6605);
									s.is_empty()
								}
								['t', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6606);
									s.is_empty()
								}
								['v', 'i', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3514);
							s.is_empty()
						}
						['s', 'e', 'd', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3515);
							s.is_empty()
						}
						['y', 'l', 'l', 'e', 'j', '\u{1}', s @ ..] => {
							super::push(out, 3516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['\u{11}', '\u{1}', s @ ..] => {
					super::push(out, 1129);
					s.is_empty()
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3517);
							s.is_empty()
						}
						['r', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', 'c', 'o', 'r', s @ ..] => {
									super::push(out, 6608);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8927);
											match s {
												[] => true,
												['l', 'l', 'o', 'd', '\u{1}', s @ ..] => {
													super::push(out, 10292);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6609);
									match s {
										[] => true,
										['s', 'a', 'g', 'i', 'g', '\u{1}', s @ ..] => {
											super::push(out, 8928);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3519);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3520);
							match s {
								[] => true,
								['a', 'd', 'i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6610);
									s.is_empty()
								}
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
			super::push(out, 436);
			match s {
				[] => true,
				['\u{12}', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6611);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6612);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 8929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 's', 'o', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1133);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3523);
							s.is_empty()
						}
						['o', 'b', 'e', 'm', s @ ..] => {
							super::push(out, 3524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t2::n440(s, out),
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3536);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 3537);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', 'f', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n442(s, out),
		['h', s @ ..] => super::t2::n443(s, out),
		['i', '\u{11}', 'd', '\u{1}', s @ ..] => {
			super::push(out, 440);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['\u{13}', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t2::n446(s, out),
		['l', s @ ..] => super::t2::n447(s, out),
		['m', s @ ..] => super::t2::n448(s, out),
		['n', s @ ..] => super::t2::n449(s, out),
		['o', s @ ..] => super::t2::n450(s, out),
		['p', s @ ..] => super::t2::n451(s, out),
		['r', s @ ..] => super::t2::n452(s, out),
		['s', s @ ..] => super::t2::n453(s, out),
		['t', s @ ..] => super::t2::n454(s, out),
		['u', s @ ..] => super::t2::n455(s, out),
		['v', s @ ..] => super::t2::n456(s, out),
		['w', s @ ..] => super::t2::n457(s, out),
		['x', s @ ..] => super::t2::n458(s, out),
		['z', s @ ..] => {
			super::push(out, 454);
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
		['\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 455);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3631);
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							super::push(out, 3632);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 3633);
							s.is_empty()
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 3634);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3635);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 3636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3637);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6695);
									s.is_empty()
								}
								['k', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['t', 'a', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', '\u{12}', 'a', 'k', 'h', 's', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 457);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3639);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3640);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3641);
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							super::push(out, 3642);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3643);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 3644);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3645);
							s.is_empty()
						}
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3646);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3647);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 6697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3648);
							s.is_empty()
						}
						['y', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3649);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 3650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 8970);
											s.is_empty()
										}
										['q', '\u{1}', s @ ..] => {
											super::push(out, 8971);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 6698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'r', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 3651);
							s.is_empty()
						}
						['h', 'c', 't', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3652);
							s.is_empty()
						}
						['l', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3653);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3654);
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 6699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 1180);
					match s {
						[] => true,
						['r', 'e', 'f', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3656);
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3657);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3658);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1184);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3660);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3661);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3662);
							match s {
								[] => true,
								['r', 'i', s @ ..] => {
									match s {
										[] => true,
										['h', 'k', '\u{1}', s @ ..] => {
											super::push(out, 8972);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 8973);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', 'g', s @ ..] => {
							super::push(out, 3663);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6700);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['h', 'k', '\u{1}', s @ ..] => {
													super::push(out, 10310);
													s.is_empty()
												}
												['k', '\u{1}', s @ ..] => {
													super::push(out, 10311);
													s.is_empty()
												}
												['q', '\u{1}', s @ ..] => {
													super::push(out, 10312);
													s.is_empty()
												}
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
				['\u{13}', 'u', 'q', 'o', 't', 'p', 'y', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', '\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['\u{10}', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 466);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 1187);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3665);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 3666);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3667);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 6701);
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 8974);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 8975);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3668);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n473(s, out),
		['s', s @ ..] => super::t2::n474(s, out),
		['t', s @ ..] => super::t2::n475(s, out),
		['u', s @ ..] => super::t2::n476(s, out),
		['w', '\u{10}', 'd', '\u{1}', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		['y', s @ ..] => super::t2::n478(s, out),
		['z', s @ ..] => super::t2::n479(s, out),
		_ => false,
	}
}

/// Words ending -heca (3 syllables).
pub(crate) fn n3967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3702);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', 'r', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 8978);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10934);
									s.is_empty()
								}
								['r', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 10935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'p', 'r', 'o', 'm', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 10313);
							s.is_empty()
						}
						['i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 10314);
							s.is_empty()
						}
						['l', 'u', 'c', 'i', 't', 'p', 'a', 'n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 10315);
							s.is_empty()
						}
						['n', 'o', 'c', 'i', '\u{1}', s @ ..] => {
							super::push(out, 10316);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10936);
									s.is_empty()
								}
								['e', 't', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 11254);
											s.is_empty()
										}
										['r', 'e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 11255);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 10938);
									s.is_empty()
								}
								['p', 'y', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 10939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 10317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plica (3 syllables).
pub(crate) fn n3970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3705);
	match s {
		[] => true,
		['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 6710);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dalmatica (3 syllables).
pub(crate) fn n3972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3707);
	s.is_empty()
}

/// Words ending -nca (3 syllables).
pub(crate) fn n1331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1218);
	s.is_empty()
}

/// Words ending -oca (3 syllables).
pub(crate) fn n1332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1219);
	s.is_empty()
}

/// Words ending -rca (3 syllables).
pub(crate) fn n1333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1220);
	match s {
		[] => true,
		['a', 's', 'i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3708);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3709);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6711);
					s.is_empty()
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6712);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sca (3 syllables).
pub(crate) fn n1334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1221);
	s.is_empty()
}

/// Words ending -uca (3 syllables).
pub(crate) fn n1335(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'i', 't', 'c', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 3710);
			s.is_empty()
		}
		['r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 3711);
			s.is_empty()
		}
		['t', 's', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 3712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -da.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 484);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1222);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['n', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3715);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 6713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1226);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 486);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3717);
							match s {
								[] => true,
								['g', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'a', s @ ..] => {
							super::push(out, 3718);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3719);
							match s {
								[] => true,
								['i', 't', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3720);
							match s {
								[] => true,
								['l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 6716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1228);
					match s {
						[] => true,
						['n', 'c', 'o', 't', 'p', 'e', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3721);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3722);
							match s {
								[] => true,
								['e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6718);
									s.is_empty()
								}
								['i', 'v', 'o', 'l', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 3723);
							match s {
								[] => true,
								['a', 'r', 'g', 'i', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'o', 'c', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 8979);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 8980);
											s.is_empty()
										}
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
					super::push(out, 1229);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3724);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 6720);
									match s {
										[] => true,
										['c', 'p', 'u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 8981);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3725);
							match s {
								[] => true,
								['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1231);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1232);
					match s {
						[] => true,
						['c', 'a', 'r', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3726);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3727);
							match s {
								[] => true,
								['a', 'n', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6722);
									s.is_empty()
								}
								['u', 'b', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 6723);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3728);
							s.is_empty()
						}
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
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['\u{11}', 'o', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3729);
							s.is_empty()
						}
						['r', 't', 's', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1237);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3732);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 6724);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 6725);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 6726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 6727);
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 6728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'z', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				['r', 't', 's', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				['v', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				['x', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
						['h', 'c', 'r', 'a', 'm', 'u', 'a', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 3733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3734);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6729);
									match s {
										[] => true,
										['r', 't', 'o', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
													super::push(out, 10318);
													s.is_empty()
												}
												['g', 'n', 'y', 'r', 'a', 'l', '\u{1}', s @ ..] => {
													super::push(out, 10319);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'r', 'o', 'n', 'e', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'l', 'e', 'c', 's', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3736);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3737);
							match s {
								[] => true,
								['g', 'u', 'b', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'h', 'r', 'r', 'o', 'n', 'e', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fa.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ga.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1248);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['h', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3740);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3741);
							match s {
								[] => true,
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 6731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1251);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 6732);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 6733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3742);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 6734);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 6735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 3743);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 3744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['i', 'l', 'i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1254);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3746);
							s.is_empty()
						}
						['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 3747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3748);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6736);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 8982);
											s.is_empty()
										}
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
					super::push(out, 1257);
					match s {
						[] => true,
						['a', 'm', 'i', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3749);
							s.is_empty()
						}
						['i', 't', 'a', s @ ..] => {
							super::push(out, 3750);
							match s {
								[] => true,
								['c', 'a', 'r', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6737);
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
					match s {
						[] => true,
						['o', 't', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3752);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 6738);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 8983);
											s.is_empty()
										}
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
					super::push(out, 1260);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3753);
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 6739);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6740);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 8984);
											match s {
												[] => true,
												['a', 's', '\u{1}', s @ ..] => {
													super::push(out, 10320);
													s.is_empty()
												}
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

/// Words ending -ha.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1261);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6741);
									s.is_empty()
								}
								['w', 'k', '\u{1}', s @ ..] => {
									super::push(out, 6742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'o', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 3754);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3755);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3756);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3757);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3758);
							s.is_empty()
						}
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3759);
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
							super::push(out, 3760);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3761);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3762);
							s.is_empty()
						}
						['m', 'y', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3763);
							s.is_empty()
						}
						['y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3765);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1265);
					match s {
						[] => true,
						['a', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3766);
							s.is_empty()
						}
						['i', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1266);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3768);
							match s {
								[] => true,
								['p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'y', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1268);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1269);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3769);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 6747);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8985);
											match s {
												[] => true,
												['h', '\u{1}', s @ ..] => {
													super::push(out, 10321);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 3770);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3771);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6748);
									match s {
										[] => true,
										['a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 8986);
											s.is_empty()
										}
										['o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 8987);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'o', 't', 'a', 'm', 'e', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6749);
									s.is_empty()
								}
								['r', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1270);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1271);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3772);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6751);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['a', 'j', '\u{1}', s @ ..] => {
													super::push(out, 10322);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 10323);
													match s {
														[] => true,
														['i', s @ ..] => super::t2::n12067(s, out),
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
							super::push(out, 3773);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 6752);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 8988);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'c', 'o', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'i', 'q', 'g', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1272);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1273);
					match s {
						[] => true,
						['e', 'b', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3774);
							s.is_empty()
						}
						['i', 'r', 'o', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3775);
							s.is_empty()
						}
						['n', 'a', 'd', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3776);
							s.is_empty()
						}
						['u', 'm', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['a', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3778);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3779);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 6754);
									match s {
										[] => true,
										['h', 'k', 'm', 'u', '\u{1}', s @ ..] => {
											super::push(out, 8989);
											s.is_empty()
										}
										['k', 'a', 'm', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8990);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'i', 'n', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 1275);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ia.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1276);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 3780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1277);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1279);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3781);
							match s {
								[] => true,
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 6755);
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
						['x', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1281);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1282);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3783);
							match s {
								[] => true,
								['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 6756);
									s.is_empty()
								}
								['n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6757);
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
						['e', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3784);
							s.is_empty()
						}
						['t', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1283);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1284);
					match s {
						[] => true,
						['n', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['m', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1286);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3788);
							s.is_empty()
						}
						['b', 'm', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3789);
							s.is_empty()
						}
						['i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3790);
							s.is_empty()
						}
						['o', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1287);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				['v', 'i', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1290);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 3793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1291);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3794);
							match s {
								[] => true,
								['l', 'h', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3795);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 6759);
									match s {
										[] => true,
										['d', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 8991);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 8992);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'n', 'u', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1292);
					s.is_empty()
				}
				['e', s @ ..] => super::t2::n1411(s, out),
				['g', s @ ..] => super::t2::n1412(s, out),
				['h', s @ ..] => super::t2::n1413(s, out),
				['i', s @ ..] => {
					super::push(out, 1296);
					match s {
						[] => true,
						['o', 'p', 'o', 'h', 't', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1297);
					s.is_empty()
				}
				['l', s @ ..] => super::t2::n1416(s, out),
				['m', s @ ..] => super::t2::n1417(s, out),
				['n', s @ ..] => super::t2::n1418(s, out),
				['o', s @ ..] => super::t2::n1419(s, out),
				['p', s @ ..] => super::t2::n1420(s, out),
				['r', s @ ..] => super::t2::n1421(s, out),
				['s', s @ ..] => super::t2::n1422(s, out),
				['t', s @ ..] => super::t2::n1423(s, out),
				['v', s @ ..] => super::t2::n1424(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ja.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['k', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1308);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3830);
							s.is_empty()
						}
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3831);
							s.is_empty()
						}
						['o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', 'n', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['a', 'l', 'i', 'b', 'm', 'i', 'a', 'r', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1310);
					s.is_empty()
				}
				['e', 'g', 'u', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1311);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ka.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3833);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3834);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 6796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				['g', 'n', 'a', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 1314);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1315);
					match s {
						[] => true,
						['s', 'y', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1316);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1317);
					s.is_empty()
				}
				['n', 'i', 'j', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1318);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3837);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 3838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1319);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1321);
					match s {
						[] => true,
						['a', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 3839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['d', 'e', 'z', 't', '\u{1}', s @ ..] => {
							super::push(out, 3840);
							s.is_empty()
						}
						['h', 'n', 'e', 'o', 'r', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3841);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3842);
							match s {
								[] => true,
								['p', 'p', 'o', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 6797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3843);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6798);
									s.is_empty()
								}
								['h', 's', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 6799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 't', 'a', 'r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1324);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', 'd', 'o', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6800);
									s.is_empty()
								}
								['o', 'l', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'y', 'n', 'l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 3844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1325);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3845);
							match s {
								[] => true,
								['k', 'e', 'i', 'p', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 6802);
									s.is_empty()
								}
								['s', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6803);
									s.is_empty()
								}
								['y', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'j', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3847);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 6805);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 9016);
											s.is_empty()
										}
										['j', '\u{1}', s @ ..] => {
											super::push(out, 9017);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'y', 'z', 'o', 'k', 's', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1328);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['i', 's', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 3848);
							s.is_empty()
						}
						['u', 'k', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 3849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1330);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 'w', '\u{1}', s @ ..] => {
									super::push(out, 6806);
									s.is_empty()
								}
								['m', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'i', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['o', 'j', 'i', 'm', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['e', 'l', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1333);
					match s {
						[] => true,
						['e', 'p', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 500);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3854);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 3855);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 3856);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1335);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['n', 'i', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1337);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3861);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 6808);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6809);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 6810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1343);
					match s {
						[] => true,
						['i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t2::n510(s, out),
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1353);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3895);
							s.is_empty()
						}
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3896);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3897);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3898);
							match s {
								[] => true,
								['q', 's', '\u{1}', s @ ..] => {
									super::push(out, 6877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3899);
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3900);
							s.is_empty()
						}
						['m', 't', '\u{1}', s @ ..] => {
							super::push(out, 3901);
							s.is_empty()
						}
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3902);
							s.is_empty()
						}
						['z', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 3904);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', 's', '\u{1}', s @ ..] => {
									super::push(out, 6878);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6879);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9095);
											s.is_empty()
										}
										['b', '\u{1}', s @ ..] => {
											super::push(out, 9096);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3905);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3906);
							s.is_empty()
						}
						['u', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3907);
							s.is_empty()
						}
						['y', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1356);
					match s {
						[] => true,
						['k', 'a', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1357);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3910);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3911);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1358);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1359);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6881);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 6882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 3915);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 3916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1360);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3917);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 6883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 6884);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 6885);
									s.is_empty()
								}
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 6886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1361);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1362);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3919);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 6887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1363);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 3921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t2::n513(s, out),
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 506);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3966);
							s.is_empty()
						}
						['o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1377);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1378);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 3968);
							s.is_empty()
						}
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3969);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 3970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1379);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3971);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1380);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3972);
							s.is_empty()
						}
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3974);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 6939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1383);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3975);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 6940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3976);
							s.is_empty()
						}
						['s', 't', '\u{1}', s @ ..] => {
							super::push(out, 3977);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3979);
							match s {
								[] => true,
								['a', 'r', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6941);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6942);
									match s {
										[] => true,
										['a', 't', 'n', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 9131);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 9132);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 10369);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', 'k', s @ ..] => {
							super::push(out, 3980);
							s.is_empty()
						}
						['k', 'a', s @ ..] => {
							super::push(out, 3981);
							match s {
								[] => true,
								['t', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 6943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'h', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3982);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3983);
							match s {
								[] => true,
								['b', 'm', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6944);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6945);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 9133);
											s.is_empty()
										}
										['o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9134);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3984);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6946);
									match s {
										[] => true,
										['g', 'k', '\u{1}', s @ ..] => {
											super::push(out, 9135);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 3985);
							match s {
								[] => true,
								['g', 'i', 'h', 'c', 'u', '\u{1}', s @ ..] => {
									super::push(out, 6947);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 6948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 3986);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6949);
									match s {
										[] => true,
										['z', 'o', 'n', '\u{1}', s @ ..] => super::t2::n9992(s, out),
										_ => false,
									}
								}
								['s', 't', 'o', 'm', '\u{1}', s @ ..] => super::t2::n7539(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t2::n1509(s, out),
				['g', s @ ..] => super::t2::n1510(s, out),
				['i', s @ ..] => super::t2::n1511(s, out),
				['l', s @ ..] => super::t2::n1512(s, out),
				['m', 'u', 'l', 'a', '\u{1}', s @ ..] => super::t2::n1513(s, out),
				['n', s @ ..] => super::t2::n1514(s, out),
				['o', s @ ..] => super::t2::n1515(s, out),
				['r', s @ ..] => super::t2::n1516(s, out),
				['u', s @ ..] => super::t2::n1517(s, out),
				['v', s @ ..] => super::t2::n1518(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oa.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1397);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1398);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1399);
					match s {
						[] => true,
						['a', 'w', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'm', 'a', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 511);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1401);
					match s {
						[] => true,
						['r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1402);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4022);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4023);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 6971);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 6972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1404);
					s.is_empty()
				}
				['o', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1405);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1406);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1408);
					s.is_empty()
				}
				['l', 'a', 'g', 'a', 't', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1410);
					match s {
						[] => true,
						['o', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1411);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4026);
							match s {
								[] => true,
								['e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6973);
									s.is_empty()
								}
								['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 6974);
									s.is_empty()
								}
								['o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
									super::push(out, 6975);
									s.is_empty()
								}
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
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	s.is_empty()
}

/// Words ending -ra.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1412);
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
		['\u{12}', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4028);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4029);
							match s {
								[] => true,
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 6976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 4032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['k', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4036);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4039);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6978);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 6979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4041);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 4042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4043);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4044);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4045);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6980);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9143);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4046);
							s.is_empty()
						}
						['p', s @ ..] => super::t2::n4349(s, out),
						['t', s @ ..] => {
							super::push(out, 4048);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4049);
							match s {
								[] => true,
								['h', 'c', 'u', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 6983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => super::t2::n4352(s, out),
						_ => false,
					}
				}
				['b', s @ ..] => super::t2::n1550(s, out),
				['d', s @ ..] => super::t2::n1551(s, out),
				['e', s @ ..] => super::t2::n1552(s, out),
				['h', s @ ..] => super::t2::n1553(s, out),
				['i', s @ ..] => super::t2::n1554(s, out),
				['o', s @ ..] => super::t2::n1555(s, out),
				['r', s @ ..] => super::t2::n1556(s, out),
				['t', s @ ..] => super::t2::n1557(s, out),
				['u', s @ ..] => super::t2::n1558(s, out),
				['y', 'h', 'p', 'e', '\u{1}', s @ ..] => super::t2::n1559(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4088);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4090);
							s.is_empty()
						}
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4093);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4094);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 7017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4095);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 7018);
									s.is_empty()
								}
								['l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4096);
							s.is_empty()
						}
						['r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4098);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['p', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7020);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7021);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9161);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 9162);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 10380);
													match s {
														[] => true,
														['v', s @ ..] => super::t2::n12091(s, out),
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
						['o', 'l', 'g', 'o', 't', 'c', 'i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4100);
							match s {
								[] => true,
								['e', 'm', 'o', s @ ..] => {
									match s {
										[] => true,
										['c', 'r', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 9163);
											s.is_empty()
										}
										['r', 'u', 'a', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 9164);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'h', 'p', 'o', 'h', 't', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 520);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4102);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'x', '\u{1}', s @ ..] => {
							super::push(out, 4103);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 4104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1452);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1453);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4105);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7023);
									s.is_empty()
								}
								['l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4106);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7025);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 7026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 4107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1456);
					match s {
						[] => true,
						['i', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4108);
							s.is_empty()
						}
						['u', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4110);
							s.is_empty()
						}
						['u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4111);
							s.is_empty()
						}
						['y', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1457);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1458);
					match s {
						[] => true,
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 4114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1459);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4115);
							match s {
								[] => true,
								['m', 'u', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 4116);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 7028);
									match s {
										[] => true,
										['s', 'x', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9165);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'b', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 4117);
							s.is_empty()
						}
						['g', 'i', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4118);
							s.is_empty()
						}
						['i', 'v', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 4119);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4120);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7029);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 9166);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'y', '\u{1}', s @ ..] => {
									super::push(out, 7030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4121);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7031);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9167);
											s.is_empty()
										}
										['k', 'o', 't', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 9168);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 's', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4122);
							match s {
								[] => true,
								['i', 'r', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4123);
							match s {
								[] => true,
								['e', 't', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 7034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4124);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'n', 'u', 'p', 'i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1461);
					match s {
						[] => true,
						['a', s @ ..] => super::t2::n4431(s, out),
						['l', s @ ..] => {
							super::push(out, 4127);
							match s {
								[] => true,
								['t', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t2::n4433(s, out),
						['w', s @ ..] => super::t2::n4434(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n1586(s, out),
				['l', 'u', s @ ..] => super::t2::n1587(s, out),
				['n', s @ ..] => super::t2::n1588(s, out),
				['o', s @ ..] => super::t2::n1589(s, out),
				['s', s @ ..] => super::t2::n1590(s, out),
				['t', s @ ..] => super::t2::n1591(s, out),
				['u', s @ ..] => super::t2::n1592(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['\u{11}', 'a', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', s @ ..] => {
					super::push(out, 1469);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4150);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 4151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1470);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 4152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7047);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 4154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1472);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4155);
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									super::push(out, 7048);
									match s {
										[] => true,
										['b', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9179);
											s.is_empty()
										}
										['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'e', 'p', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 524);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1477);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7049);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 7050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				['r', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				['z', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1481);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4156);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9181);
											s.is_empty()
										}
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9182);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4157);
							match s {
								[] => true,
								['s', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'g', 'n', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				['l', 'a', 'v', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1483);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1484);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 's', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7053);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7054);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['l', 'i', 'k', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9183);
											s.is_empty()
										}
										['r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9184);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4159);
							match s {
								[] => true,
								['h', 'd', 'n', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7056);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7057);
									s.is_empty()
								}
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

/// Words ending -wa.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['b', 'm', 'o', 'l', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4160);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7058);
									s.is_empty()
								}
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 7059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'c', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['k', 'u', 't', 'u', 'h', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7060);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4161);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 9185);
											s.is_empty()
										}
										['u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9186);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1491);
					match s {
						[] => true,
						['a', 's', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4162);
							s.is_empty()
						}
						['u', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1492);
					match s {
						[] => true,
						['a', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['k', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1494);
					match s {
						[] => true,
						['a', 't', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xa.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 528);
			s.is_empty()
		}
		['\u{12}', 'o', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['i', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1500);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4167);
							s.is_empty()
						}
						['d', 'o', 'n', 'o', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

