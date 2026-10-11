use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => super::t0::n1(s, out),
		['i', s @ ..] => super::t0::n2(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['r', s @ ..] => super::t0::n3(s, out),
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['s', s @ ..] => super::t0::n4(s, out),
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 19);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 98);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 20);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 21);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 99);
									match s {
										[] => true,
										['v', s @ ..] => {
											super::push(out, 426);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 100);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 101);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 102);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 103);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 104);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 22);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 23);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 25);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 26);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 105);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 27);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 28);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 29);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 30);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 108);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 31);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 32);
							match s {
								[] => true,
								['i', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 110);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 111);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t0::n6(s, out),
		_ => false,
	}
}

/// Words ending -si.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 89);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 377);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 378);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 379);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 380);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 919);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 920);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 921);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 381);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 90);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 91);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 92);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t0::n8(s, out),
		_ => false,
	}
}

/// Words ending -re (3 syllables).
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n13(s, out),
		['e', s @ ..] => super::t0::n14(s, out),
		['i', s @ ..] => super::t0::n15(s, out),
		['r', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 85);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 86);
					match s {
						[] => true,
						['v', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 87);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 376);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 88);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -si (3 syllables).
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['r', s @ ..] => super::t0::n18(s, out),
		_ => false,
	}
}

/// Words ending -are (3 syllables).
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 113);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1042);
									match s {
										[] => true,
										['p', 'a', s @ ..] => {
											super::push(out, 1645);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1043);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1646);
											s.is_empty()
										}
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
					super::push(out, 114);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 427);
							s.is_empty()
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
				['a', 'v', s @ ..] => {
					super::push(out, 115);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 116);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 428);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 429);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 430);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 431);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1044);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1647);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 1648);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 1649);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 1650);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 1651);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 1045);
									s.is_empty()
								}
								['i', 'f', s @ ..] => {
									super::push(out, 1046);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1652);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 1653);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 1654);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 1047);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1048);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 432);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1050);
									s.is_empty()
								}
								['l', 'l', s @ ..] => {
									super::push(out, 1051);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 1655);
											s.is_empty()
										}
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
					super::push(out, 117);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 118);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 433);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['i', 'v', s @ ..] => {
											super::push(out, 1656);
											match s {
												[] => true,
												['o', 'i', 'p', '\u{1}', s @ ..] => {
													super::push(out, 1998);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 1657);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'a', s @ ..] => {
							super::push(out, 434);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 435);
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									super::push(out, 1052);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 1658);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 1999);
													s.is_empty()
												}
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
							super::push(out, 436);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 437);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									super::push(out, 1053);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1659);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 438);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1054);
									match s {
										[] => true,
										['a', 'r', s @ ..] => {
											super::push(out, 1660);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 439);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 440);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 441);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1055);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2000);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 2001);
													s.is_empty()
												}
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
				['l', s @ ..] => super::t0::n120(s, out),
				['n', s @ ..] => {
					super::push(out, 119);
					s.is_empty()
				}
				['o', s @ ..] => super::t0::n122(s, out),
				['r', s @ ..] => super::t0::n123(s, out),
				['s', s @ ..] => super::t0::n124(s, out),
				['u', s @ ..] => super::t0::n125(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n36(s, out),
		['e', s @ ..] => super::t0::n37(s, out),
		['f', s @ ..] => super::t0::n38(s, out),
		['g', s @ ..] => super::t0::n39(s, out),
		['h', s @ ..] => super::t0::n40(s, out),
		['i', s @ ..] => super::t0::n41(s, out),
		['k', s @ ..] => super::t0::n42(s, out),
		['l', s @ ..] => super::t0::n43(s, out),
		['m', s @ ..] => super::t0::n44(s, out),
		['n', s @ ..] => super::t0::n45(s, out),
		['o', s @ ..] => super::t0::n46(s, out),
		['p', s @ ..] => super::t0::n47(s, out),
		['r', s @ ..] => super::t0::n48(s, out),
		['s', s @ ..] => super::t0::n49(s, out),
		['t', s @ ..] => super::t0::n50(s, out),
		['u', s @ ..] => super::t0::n51(s, out),
		['v', s @ ..] => super::t0::n52(s, out),
		['x', s @ ..] => super::t0::n53(s, out),
		['z', s @ ..] => super::t0::n54(s, out),
		_ => false,
	}
}

/// Words ending -ere (3 syllables).
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 54);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 228);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 230);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 231);
					match s {
						[] => true,
						['o', 'c', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1269);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 648);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1271);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1272);
									s.is_empty()
								}
								['p', 'm', s @ ..] => {
									super::push(out, 1273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 234);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 235);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1274);
									s.is_empty()
								}
								['e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1275);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1276);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 238);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1278);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 239);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 240);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 652);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1280);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 653);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1282);
									match s {
										[] => true,
										['u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1801);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 1283);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1284);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 241);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 655);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 656);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1285);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n58(s, out),
		['g', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				['g', s @ ..] => super::t0::n255(s, out),
				['i', s @ ..] => super::t0::n256(s, out),
				['l', s @ ..] => super::t0::n257(s, out),
				['n', s @ ..] => super::t0::n258(s, out),
				['o', s @ ..] => {
					super::push(out, 255);
					match s {
						[] => true,
						['r', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n260(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n60(s, out),
		['l', s @ ..] => super::t0::n61(s, out),
		['m', s @ ..] => super::t0::n62(s, out),
		['n', s @ ..] => super::t0::n63(s, out),
		['p', s @ ..] => super::t0::n64(s, out),
		['r', s @ ..] => super::t0::n65(s, out),
		['s', s @ ..] => super::t0::n66(s, out),
		['t', s @ ..] => super::t0::n67(s, out),
		['u', s @ ..] => super::t0::n68(s, out),
		['v', s @ ..] => super::t0::n69(s, out),
		_ => false,
	}
}

/// Words ending -ire (3 syllables).
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 292);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 293);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 294);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1416);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 746);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 747);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 748);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1419);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1886);
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1887);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 1888);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 749);
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
				['l', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							super::push(out, 750);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1420);
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
						['o', s @ ..] => {
							super::push(out, 751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 298);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1421);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 753);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 755);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 756);
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
				['a', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 758);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 759);
							match s {
								[] => true,
								['t', 'n', s @ ..] => {
									super::push(out, 1424);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1889);
											s.is_empty()
										}
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
					super::push(out, 302);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 761);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 1425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 303);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 763);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 764);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 765);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 766);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1426);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1428);
									s.is_empty()
								}
								['g', s @ ..] => super::t0::n1499(s, out),
								['p', s @ ..] => {
									super::push(out, 1430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n309(s, out),
				['l', s @ ..] => {
					super::push(out, 305);
					s.is_empty()
				}
				['n', s @ ..] => super::t0::n311(s, out),
				['r', s @ ..] => super::t0::n312(s, out),
				['s', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				['u', s @ ..] => super::t0::n314(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t0::n73(s, out),
		['g', s @ ..] => super::t0::n74(s, out),
		['h', s @ ..] => super::t0::n75(s, out),
		['l', s @ ..] => super::t0::n76(s, out),
		['m', s @ ..] => super::t0::n77(s, out),
		['n', s @ ..] => super::t0::n78(s, out),
		['p', s @ ..] => super::t0::n79(s, out),
		['r', s @ ..] => super::t0::n80(s, out),
		['s', s @ ..] => super::t0::n81(s, out),
		['t', s @ ..] => super::t0::n82(s, out),
		['u', s @ ..] => super::t0::n83(s, out),
		['v', s @ ..] => super::t0::n84(s, out),
		['z', s @ ..] => super::t0::n85(s, out),
		_ => false,
	}
}

/// Words ending -rsi (3 syllables).
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 383);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 384);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1561);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 1951);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 1952);
											s.is_empty()
										}
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
					super::push(out, 385);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 924);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1562);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 387);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 927);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 929);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1564);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1953);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 1954);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 1955);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 1565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 930);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 1566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 932);
							match s {
								[] => true,
								['o', 'c', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 389);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 936);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 937);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 938);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 939);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 940);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 941);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 942);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 943);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1568);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 944);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1570);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 945);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 946);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 948);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 949);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 950);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 952);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1571);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 392);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['r', s @ ..] => super::t0::n403(s, out),
				['s', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 961);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1579);
									match s {
										[] => true,
										['l', 'l', 'i', s @ ..] => {
											super::push(out, 1961);
											match s {
												[] => true,
												['r', 'g', 's', '\u{1}', s @ ..] => {
													super::push(out, 2089);
													s.is_empty()
												}
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
							super::push(out, 962);
							match s {
								[] => true,
								['p', 'i', s @ ..] => {
									super::push(out, 1580);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => super::t0::n2065(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t0::n406(s, out),
				['v', s @ ..] => super::t0::n407(s, out),
				['z', s @ ..] => super::t0::n408(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n95(s, out),
		['i', s @ ..] => super::t0::n96(s, out),
		['o', s @ ..] => super::t0::n97(s, out),
		['u', s @ ..] => super::t0::n98(s, out),
		_ => false,
	}
}

/// Words ending -lcare (3 syllables).
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['o', 'd', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ocare (3 syllables).
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1056);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcare (3 syllables).
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	s.is_empty()
}

/// Words ending -scare (3 syllables).
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1661);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 447);
			s.is_empty()
		}
		['u', 'r', 'b', 'b', 'a', 'r', s @ ..] => {
			super::push(out, 448);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucare (3 syllables).
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dare (3 syllables).
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					super::push(out, 451);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1062);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1063);
							match s {
								[] => true,
								['a', 's', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1064);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 127);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1068);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1069);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 1070);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1071);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 454);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 455);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							super::push(out, 1073);
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
				['o', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1074);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1075);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1076);
							match s {
								[] => true,
								['e', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 457);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1077);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['a', 'u', s @ ..] => {
					super::push(out, 458);
					match s {
						[] => true,
						['g', 's', s @ ..] => {
							super::push(out, 1079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eare (3 syllables).
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['n', 'r', s @ ..] => {
			super::push(out, 131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fare (3 syllables).
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1080);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 459);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1084);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1667);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1668);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1669);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 136);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 138);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1086);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1087);
							s.is_empty()
						}
						['t', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gare (3 syllables).
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['s', 's', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['l', 'b', 'b', 'u', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['v', 'e', s @ ..] => {
					super::push(out, 471);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['o', 'f', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1671);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1672);
									s.is_empty()
								}
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

/// Words ending -hare (3 syllables).
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iare (3 syllables).
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['b', 'a', 'r', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				['m', 'e', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 481);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1091);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1673);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1674);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2003);
											match s {
												[] => true,
												['r', '\u{1}', s @ ..] => {
													super::push(out, 2102);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'r', s @ ..] => {
							super::push(out, 1092);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1675);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1676);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 2004);
											match s {
												[] => true,
												['a', 'c', s @ ..] => {
													match s {
														[] => true,
														['c', s @ ..] => super::t0::n2232(s, out),
														['s', s @ ..] => super::t0::n2233(s, out),
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
									super::push(out, 1677);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1093);
							match s {
								[] => true,
								['b', 's', s @ ..] => {
									super::push(out, 1678);
									s.is_empty()
								}
								['r', 'p', s @ ..] => {
									super::push(out, 1679);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 2005);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 2006);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1094);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1680);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1681);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 2007);
											s.is_empty()
										}
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
					super::push(out, 482);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 483);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 484);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1095);
							match s {
								[] => true,
								['c', s @ ..] => {
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1096);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1097);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1684);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 1685);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 486);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 487);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1099);
							match s {
								[] => true,
								['b', 'm', s @ ..] => {
									super::push(out, 1686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1100);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1101);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1687);
									s.is_empty()
								}
								['l', 'i', s @ ..] => {
									super::push(out, 1688);
									s.is_empty()
								}
								['r', 't', 's', 'i', s @ ..] => {
									super::push(out, 1689);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'i', s @ ..] => {
							super::push(out, 1102);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1103);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1690);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1691);
									s.is_empty()
								}
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
			super::push(out, 151);
			match s {
				[] => true,
				['o', 'l', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', s @ ..] => {
							super::push(out, 1106);
							match s {
								[] => true,
								['p', 's', s @ ..] => {
									super::push(out, 1692);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => super::t0::n1158(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n156(s, out),
		['l', s @ ..] => super::t0::n157(s, out),
		['m', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['h', 'c', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n160(s, out),
		['p', s @ ..] => super::t0::n161(s, out),
		['r', s @ ..] => super::t0::n162(s, out),
		['s', s @ ..] => super::t0::n163(s, out),
		['t', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['u', s @ ..] => super::t0::n165(s, out),
		['v', s @ ..] => super::t0::n166(s, out),
		['z', s @ ..] => super::t0::n167(s, out),
		_ => false,
	}
}

/// Words ending -kare (3 syllables).
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	s.is_empty()
}

/// Words ending -lare (3 syllables).
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1126);
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 1719);
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['a', 'g', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1129);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1720);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 1721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'p', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1130);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 1132);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2020);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 2021);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1133);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1722);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1723);
									s.is_empty()
								}
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
			super::push(out, 170);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1135);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 1724);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 2022);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 2023);
											s.is_empty()
										}
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
					super::push(out, 530);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1136);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1137);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1725);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1726);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1138);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1139);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1728);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2024);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 2025);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 1729);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1730);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 2026);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 2027);
											s.is_empty()
										}
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
					super::push(out, 531);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1140);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1731);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 534);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1141);
							s.is_empty()
						}
						['c', 'c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1733);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1734);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 2028);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 2029);
											s.is_empty()
										}
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
					super::push(out, 535);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 536);
					match s {
						[] => true,
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 1143);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1144);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1145);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1146);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
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
				['z', s @ ..] => super::t0::n554(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n173(s, out),
		['u', s @ ..] => {
			super::push(out, 172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mare (3 syllables).
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 174);
			s.is_empty()
		}
		['l', 'e', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['m', 'o', 's', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		['o', 't', s @ ..] => {
			super::push(out, 177);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nare (3 syllables).
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1154);
							s.is_empty()
						}
						['u', 's', 'a', s @ ..] => {
							super::push(out, 1155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', 'a', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1156);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 553);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 1158);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				['i', 'v', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 558);
					match s {
						[] => true,
						['s', 'i', s @ ..] => {
							super::push(out, 1161);
							match s {
								[] => true,
								['b', 'i', s @ ..] => {
									super::push(out, 1739);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 2031);
											s.is_empty()
										}
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
			super::push(out, 183);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 559);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1162);
							match s {
								[] => true,
								['v', 'v', s @ ..] => {
									super::push(out, 1740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 560);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1163);
							match s {
								[] => true,
								['a', 'r', 'g', 'i', s @ ..] => {
									super::push(out, 1741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 561);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1165);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1166);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1742);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1167);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1168);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1744);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1169);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2032);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 2033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1170);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 2034);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 2111);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 2035);
											s.is_empty()
										}
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
					super::push(out, 564);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1173);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1174);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['o', 't', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1176);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1177);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1746);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1747);
									s.is_empty()
								}
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
			super::push(out, 188);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oare (3 syllables).
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['c', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pare (3 syllables).
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['e', 'r', s @ ..] => {
			super::push(out, 190);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
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
							super::push(out, 1179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1180);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1750);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1751);
									s.is_empty()
								}
								['s', 'i', s @ ..] => {
									super::push(out, 1752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['o', 't', 'n', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 1182);
							s.is_empty()
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
		['r', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rare (3 syllables).
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', 'p', 'i', s @ ..] => {
			super::push(out, 198);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 581);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1183);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1185);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1186);
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
				['b', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['u', 't', s @ ..] => {
							super::push(out, 1187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 586);
					match s {
						[] => true,
						['n', 'a', 'r', s @ ..] => {
							super::push(out, 1189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1190);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1191);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'p', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1193);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1194);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1197);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1756);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 'c', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1199);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'e', 'm', 's', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1200);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1759);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1760);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1202);
							match s {
								[] => true,
								['a', 'c', 'c', s @ ..] => {
									super::push(out, 1761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 1203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 601);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1762);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 1763);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1764);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1205);
							match s {
								[] => true,
								['i', 'g', s @ ..] => {
									super::push(out, 1766);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2036);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 1767);
									s.is_empty()
								}
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
			super::push(out, 207);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sare (3 syllables).
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['a', 'c', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['e', 'c', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tare (3 syllables).
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1207);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2037);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 2038);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2039);
											s.is_empty()
										}
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
					super::push(out, 615);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1208);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1210);
							s.is_empty()
						}
						['s', 'e', s @ ..] => {
							super::push(out, 1211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1212);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'e', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1215);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'v', 's', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', 'i', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1218);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1768);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1219);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1220);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1770);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'm', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1222);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1772);
									s.is_empty()
								}
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
			super::push(out, 215);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['o', 'p', 'm', s @ ..] => {
					super::push(out, 628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 631);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1229);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1230);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1774);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1231);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1232);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 633);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1234);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1235);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1236);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1238);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1239);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 1779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n655(s, out),
				['o', s @ ..] => super::t0::n656(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['p', 'i', s @ ..] => super::t0::n657(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uare (3 syllables).
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vare (3 syllables).
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 221);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['a', 'c', 'n', s @ ..] => {
					super::push(out, 643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xare (3 syllables).
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	s.is_empty()
}

/// Words ending -zare (3 syllables).
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1253);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1784);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 1785);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1786);
									s.is_empty()
								}
								['m', s @ ..] => {
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
						['c', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['a', 'v', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1256);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1258);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 1789);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 2040);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1259);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1260);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									super::push(out, 1791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1261);
							match s {
								[] => true,
								['g', 'n', s @ ..] => {
									super::push(out, 1792);
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
						['l', s @ ..] => {
							super::push(out, 1262);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['c', 'i', 't', 'i', s @ ..] => {
							super::push(out, 1264);
							match s {
								[] => true,
								['l', 'o', 'p', 'i', s @ ..] => {
									super::push(out, 1794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1265);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1795);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1796);
									match s {
										[] => true,
										['a', 't', s @ ..] => {
											super::push(out, 2041);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 2112);
													s.is_empty()
												}
												['s', 'i', 'r', 'c', 'i', s @ ..] => {
													super::push(out, 2113);
													s.is_empty()
												}
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
								['i', 'v', s @ ..] => {
									super::push(out, 1797);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1798);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 2042);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 2114);
													s.is_empty()
												}
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
							super::push(out, 1266);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1267);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1799);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2043);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 2044);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1268);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1800);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2045);
											s.is_empty()
										}
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

/// Words ending -dere (3 syllables).
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 661);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1291);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1802);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1803);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 1804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1292);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1805);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1806);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1294);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 1807);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1295);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
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
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 664);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1301);
							match s {
								[] => true,
								['r', 't', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2048);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2049);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1302);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1809);
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1304);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1305);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1812);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1813);
									s.is_empty()
								}
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
			super::push(out, 245);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 1306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1307);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1815);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1816);
									s.is_empty()
								}
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
			super::push(out, 246);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				['e', s @ ..] => super::t0::n696(s, out),
				['i', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 673);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1317);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1830);
									s.is_empty()
								}
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
			super::push(out, 247);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							super::push(out, 1319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n253(s, out),
		_ => false,
	}
}

/// Words ending -ggere (3 syllables).
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1839);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 2052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2053);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2055);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2056);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['f', 's', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -igere (3 syllables).
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1329);
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 690);
			match s {
				[] => true,
				['i', 'd', 'n', s @ ..] => {
					super::push(out, 1332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 691);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lgere (3 syllables).
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['f', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ngere (3 syllables).
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1338);
					s.is_empty()
				}
				['l', 'c', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1342);
					match s {
						[] => true,
						['n', 'i', 'r', s @ ..] => {
							super::push(out, 1844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1344);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1346);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 1845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1349);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'p', s @ ..] => {
											super::push(out, 2118);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 2119);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 2057);
									s.is_empty()
								}
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
				['m', s @ ..] => {
					super::push(out, 1350);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1351);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rgere (3 syllables).
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
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
				['\u{1}', s @ ..] => {
					super::push(out, 1352);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1353);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1354);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1356);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1357);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1358);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1359);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1360);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1362);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iere (3 syllables).
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['g', 'e', 'v', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1363);
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
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1852);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 1853);
									s.is_empty()
								}
								['i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lere (3 syllables).
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1369);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1370);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1855);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 2059);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1371);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1856);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2060);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 1857);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2061);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1858);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mere (3 syllables).
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 262);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['p', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 264);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nere (3 syllables).
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1382);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1860);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 1385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							super::push(out, 1386);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1389);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1390);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1392);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1863);
									s.is_empty()
								}
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

/// Words ending -pere (3 syllables).
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['o', 'r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1396);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1864);
									s.is_empty()
								}
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
			super::push(out, 273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rere (3 syllables).
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['p', 's', 'a', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['f', 'f', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['h', 'c', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['o', 'c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1865);
									s.is_empty()
								}
								['o', 's', s @ ..] => {
									super::push(out, 1866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1397);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1398);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
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
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1869);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1870);
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
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sere (3 syllables).
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1402);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1871);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1872);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 2062);
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2063);
											s.is_empty()
										}
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
					super::push(out, 724);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tere (3 syllables).
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1405);
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
				['c', s @ ..] => {
					super::push(out, 727);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['e', 'v', 'o', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1873);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1874);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1875);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1877);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1407);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1881);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1882);
									match s {
										[] => true,
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2064);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1408);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 733);
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

/// Words ending -uere (3 syllables).
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['n', 'i', 'l', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vere (3 syllables).
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 739);
					match s {
						[] => true,
						['c', 's', 'o', 'n', s @ ..] => {
							super::push(out, 1410);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1413);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2065);
											s.is_empty()
										}
										['a', '\u{1}', s @ ..] => {
											super::push(out, 2066);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1414);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1415);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1885);
									s.is_empty()
								}
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
			super::push(out, 290);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 741);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gredire (3 syllables).
pub(crate) fn n1499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1429);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1890);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1891);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1892);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -idire (3 syllables).
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'r', s @ ..] => {
			super::push(out, 770);
			s.is_empty()
		}
		['l', 'l', s @ ..] => {
			super::push(out, 771);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['u', 't', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 773);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 774);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1436);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['u', 'p', 'm', 'i', 'r', s @ ..] => {
							super::push(out, 1894);
							s.is_empty()
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
					super::push(out, 1438);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ndire (3 syllables).
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1440);
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
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['b', 'm', 'i', 'r', s @ ..] => {
							super::push(out, 1895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdire (3 syllables).
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1896);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udire (3 syllables).
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 781);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1446);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1447);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 783);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1452);
					s.is_empty()
				}
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

/// Words ending -fire (3 syllables).
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gire (3 syllables).
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1454);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1901);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1455);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hire (3 syllables).
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'l', s @ ..] => {
							super::push(out, 1457);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1903);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2067);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 2068);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 2069);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						['o', 'l', s @ ..] => {
							super::push(out, 1459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2070);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 2071);
											s.is_empty()
										}
										['v', 'l', 'e', 's', 'n', 'i', 'r', s @ ..] => {
											super::push(out, 2072);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 791);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lire (3 syllables).
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 798);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 800);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						['g', 'r', 'o', 'n', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1468);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1906);
									match s {
										[] => true,
										['a', 'i', s @ ..] => {
											super::push(out, 2073);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 1907);
									match s {
										[] => true,
										['i', 'r', s @ ..] => {
											super::push(out, 2074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1470);
							match s {
								[] => true,
								['p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1908);
									s.is_empty()
								}
								_ => false,
							}
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
						['b', s @ ..] => {
							super::push(out, 1471);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1472);
							match s {
								[] => true,
								['m', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['i', 'c', 'c', 'o', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 810);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1911);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['p', 'i', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mire (3 syllables).
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 814);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nire (3 syllables).
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1478);
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
				['i', s @ ..] => {
					super::push(out, 817);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1913);
									s.is_empty()
								}
								['a', 'r', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 819);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1482);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', 'r', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1484);
							match s {
								[] => true,
								['i', 'g', 'n', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1916);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1486);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 2075);
											s.is_empty()
										}
										['t', 'n', 'o', s @ ..] => {
											super::push(out, 2076);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1917);
									s.is_empty()
								}
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
			super::push(out, 323);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1487);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['f', 'a', s @ ..] => {
							super::push(out, 1489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 827);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['i', 't', '\u{1}', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 830);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1490);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 832);
					match s {
						[] => true,
						['c', 'n', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 833);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 837);
					match s {
						[] => true,
						['b', 'm', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pire (3 syllables).
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1500);
							match s {
								[] => true,
								['e', 'p', 'p', s @ ..] => {
									super::push(out, 1918);
									s.is_empty()
								}
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
			super::push(out, 331);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 841);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1503);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 1919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 334);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['c', 'n', 'o', s @ ..] => {
					super::push(out, 843);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rire (3 syllables).
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['r', 'a', 'b', 'm', 'i', 'r', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 845);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 846);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 847);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1505);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2077);
											s.is_empty()
										}
										['i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2078);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2079);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2120);
													s.is_empty()
												}
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
							super::push(out, 1506);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1507);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1920);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1509);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1922);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2080);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 853);
					match s {
						[] => true,
						['f', 'n', 'i', 'r', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 855);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 856);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1514);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1923);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1516);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 857);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 858);
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 340);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 859);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1925);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 861);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 862);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 863);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 864);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 865);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1520);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1927);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 866);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 867);
					match s {
						[] => true,
						['r', 'a', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1929);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 869);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 871);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 872);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['a', 'z', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['z', 'i', 'b', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1931);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 876);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 877);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1523);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1933);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1525);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sire (3 syllables).
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 348);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 350);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 881);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1527);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1529);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 884);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tire (3 syllables).
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 886);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 354);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 355);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['d', 'e', 'p', 'm', s @ ..] => {
							super::push(out, 1533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1534);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1535);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1537);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1538);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1936);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1937);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1938);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1539);
							s.is_empty()
						}
						['v', 'o', 'r', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 892);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1940);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 2081);
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											super::push(out, 2082);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 1941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 893);
					match s {
						[] => true,
						['v', 'o', s @ ..] => {
							super::push(out, 1541);
							match s {
								[] => true,
								['r', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1542);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1543);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1944);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1945);
									s.is_empty()
								}
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
			super::push(out, 359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1544);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1545);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1546);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1946);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				['i', 'f', 'n', s @ ..] => {
					super::push(out, 902);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 1548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 904);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uire (3 syllables).
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 362);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 363);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 906);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1948);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1949);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1950);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 2083);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 910);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1553);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
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
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['i', 'm', 'i', 'd', 'i', s @ ..] => {
					super::push(out, 913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 368);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vire (3 syllables).
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 370);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['e', 's', 's', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zire (3 syllables).
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 373);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				['e', 'm', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1557);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1558);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1559);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rarsi (3 syllables).
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 957);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1573);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1956);
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
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						['o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2084);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2085);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2086);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2087);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1958);
							s.is_empty()
						}
						['i', 'd', s @ ..] => {
							super::push(out, 1959);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word riputarsi (3 syllables).
pub(crate) fn n2065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1962);
	s.is_empty()
}

/// Words ending -uarsi (3 syllables).
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	s.is_empty()
}

/// Words ending -varsi (3 syllables).
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	s.is_empty()
}

/// Words ending -zarsi (3 syllables).
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	s.is_empty()
}

/// Words ending -ersi (3 syllables).
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 964);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 966);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1581);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1582);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 967);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 968);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1584);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1585);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1586);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1587);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1588);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 1963);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2090);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2121);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 2122);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 2091);
											s.is_empty()
										}
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
					super::push(out, 969);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1589);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1590);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1964);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2092);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1965);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1966);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1591);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1592);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1968);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 1969);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['g', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1596);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1971);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2093);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 1972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1599);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1973);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1974);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 978);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1602);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1977);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1978);
									s.is_empty()
								}
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
			super::push(out, 404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1604);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 985);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 408);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 409);
			s.is_empty()
		}
		['t', s @ ..] => super::t0::n419(s, out),
		['u', s @ ..] => {
			super::push(out, 411);
			s.is_empty()
		}
		['v', s @ ..] => super::t0::n421(s, out),
		_ => false,
	}
}

/// Words ending -irsi (3 syllables).
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['r', 'o', s @ ..] => {
					super::push(out, 1002);
					match s {
						[] => true,
						['s', 's', s @ ..] => {
							super::push(out, 1615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1004);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1616);
							s.is_empty()
						}
						['r', 'd', 's', '\u{1}', s @ ..] => {
							super::push(out, 1617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 1618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1619);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1620);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1984);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1014);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1015);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1621);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1986);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1987);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1988);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1016);
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
					super::push(out, 1017);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1623);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1624);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1019);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1626);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1627);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1628);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1629);
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
				['e', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1024);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1027);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1630);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1989);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1631);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1632);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1029);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1030);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1634);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1031);
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							super::push(out, 1636);
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
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1032);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1034);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1637);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1638);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1991);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1992);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2095);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2096);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 1993);
									match s {
										[] => true,
										['r', 'e', 's', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 2097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 1994);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2098);
											s.is_empty()
										}
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
					super::push(out, 1035);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1639);
							match s {
								[] => true,
								['p', 'i', 'd', s @ ..] => {
									super::push(out, 1995);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1640);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2100);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 2101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t0::n1083(s, out),
				['t', s @ ..] => {
					super::push(out, 1037);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n432(s, out),
		['v', s @ ..] => super::t0::n433(s, out),
		['z', s @ ..] => {
			super::push(out, 425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orsi (3 syllables).
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	s.is_empty()
}

/// Words ending -ursi (3 syllables).
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	s.is_empty()
}

/// Words ending -ccapricciare (3 syllables).
pub(crate) fn n2232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2123);
	s.is_empty()
}

/// Words ending -scapricciare (3 syllables).
pub(crate) fn n2233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2124);
	s.is_empty()
}

/// Words ending -eggiare (3 syllables).
pub(crate) fn n1158(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1693);
			s.is_empty()
		}
		['d', 'n', s @ ..] => {
			super::push(out, 1694);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					super::push(out, 2008);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'c', 'e', s @ ..] => {
			super::push(out, 1695);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', s @ ..] => {
			super::push(out, 1696);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1697);
			s.is_empty()
		}
		['r', 't', s @ ..] => {
			super::push(out, 1698);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hiare (3 syllables).
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1700);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1701);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 2011);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 2104);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 2105);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1702);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 2012);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 2106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1704);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2013);
									match s {
										[] => true,
										['c', 'a', s @ ..] => {
											super::push(out, 2107);
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 2125);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 2126);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 2014);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 2108);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 2109);
											s.is_empty()
										}
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
					super::push(out, 1108);
					match s {
						[] => true,
						['o', 't', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1109);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1110);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 1706);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2015);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2016);
									s.is_empty()
								}
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

/// Words ending -liare (3 syllables).
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1707);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1708);
							s.is_empty()
						}
						['v', 'o', 't', 't', 'a', s @ ..] => {
							super::push(out, 1709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1710);
							match s {
								[] => true,
								['o', 's', 's', 'i', 'r', s @ ..] => {
									super::push(out, 2017);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 2110);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'm', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'v', s @ ..] => {
			super::push(out, 494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oiare (3 syllables).
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 497);
			s.is_empty()
		}
		['m', 'i', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -piare (3 syllables).
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 499);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1711);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2018);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -riare (3 syllables).
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 504);
			s.is_empty()
		}
		['t', 'a', 'p', 's', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['f', 'n', 'i', 'r', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siare (3 syllables).
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uiare (3 syllables).
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['b', 'b', 'a', 'r', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viare (3 syllables).
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1713);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ziare (3 syllables).
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 519);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 520);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1715);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1716);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1717);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zolare (3 syllables).
pub(crate) fn n554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['z', 'a', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rlare (3 syllables).
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ettare (3 syllables).
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 636);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1241);
			match s {
				[] => true,
				['g', 'a', s @ ..] => {
					super::push(out, 1780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1242);
			s.is_empty()
		}
		['r', 'c', s @ ..] => {
			super::push(out, 1243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ottare (3 syllables).
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 637);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1244);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1245);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1246);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1247);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1248);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iputare (3 syllables).
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 638);
	match s {
		[] => true,
		['r', '\u{1}', s @ ..] => {
			super::push(out, 1249);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -endere (3 syllables).
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 671);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1818);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2115);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2127);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 2128);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2117);
									s.is_empty()
								}
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
			super::push(out, 1309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1820);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1822);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['p', 's', 'i', s @ ..] => {
					super::push(out, 1824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1826);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1827);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1828);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1312);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1313);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udere (3 syllables).
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['l', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1831);
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 681);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tersi (3 syllables).
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 992);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 993);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 994);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 995);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1609);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1610);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1979);
							match s {
								[] => true,
								['f', 'i', s @ ..] => {
									super::push(out, 2094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -versi (3 syllables).
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 997);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 998);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1612);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1001);
			match s {
				[] => true,
				['u', 'm', 'm', s @ ..] => {
					super::push(out, 1614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stirsi (3 syllables).
pub(crate) fn n1083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1036);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1641);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uirsi (3 syllables).
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1039);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1040);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -virsi (3 syllables).
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['r', 'e', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

