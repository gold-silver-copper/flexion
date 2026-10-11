use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['b', 'm', 'e', s @ ..] => {
					super::push(out, 27);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 314);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 29);
					s.is_empty()
				}
				['f', 'n', '\u{1}', s @ ..] => {
					super::push(out, 30);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 31);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 32);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 33);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 34);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 35);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 317);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 36);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 319);
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1247);
									s.is_empty()
								}
								['f', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 320);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 321);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1249);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1250);
									match s {
										[] => true,
										['o', 'g', '\u{1}', s @ ..] => {
											super::push(out, 2397);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 322);
							match s {
								[] => true,
								['o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 37);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 323);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 38);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 39);
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 325);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 40);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 41);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 42);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 43);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 44);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 45);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 326);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 327);
							s.is_empty()
						}
						['u', 'c', s @ ..] => {
							super::push(out, 328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 47);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 49);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 51);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 52);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 330);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 331);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 332);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 54);
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
					super::push(out, 55);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 334);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 335);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 336);
							match s {
								[] => true,
								['r', 'a', 't', 'e', s @ ..] => {
									super::push(out, 1253);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['h', 's', '\u{1}', s @ ..] => {
											super::push(out, 2398);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2399);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 337);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 338);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 339);
							match s {
								[] => true,
								['f', 'o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 340);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 341);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1255);
									s.is_empty()
								}
								['e', s @ ..] => super::t0::n1284(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => super::t0::n56(s, out),
				['d', s @ ..] => super::t0::n57(s, out),
				['e', s @ ..] => super::t0::n58(s, out),
				['f', s @ ..] => super::t0::n59(s, out),
				['h', '\u{1}', s @ ..] => super::t0::n60(s, out),
				['i', s @ ..] => super::t0::n61(s, out),
				['l', s @ ..] => super::t0::n62(s, out),
				['m', s @ ..] => super::t0::n63(s, out),
				['n', s @ ..] => super::t0::n64(s, out),
				['o', s @ ..] => super::t0::n65(s, out),
				['p', '\u{1}', s @ ..] => super::t0::n66(s, out),
				['r', s @ ..] => super::t0::n67(s, out),
				['s', s @ ..] => super::t0::n68(s, out),
				['t', s @ ..] => super::t0::n69(s, out),
				['v', s @ ..] => super::t0::n70(s, out),
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

/// Words ending -evac.
pub(crate) fn n1284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cc.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	s.is_empty()
}

/// Words ending -dc.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ec.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 344);
			s.is_empty()
		}
		['p', 's', 'n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 345);
			s.is_empty()
		}
		['x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fc.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	s.is_empty()
}

/// The word hc.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	s.is_empty()
}

/// Words ending -ic.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		['f', 'n', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 349);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1258);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				['i', 'm', 'o', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 352);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 353);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1261);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1262);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1264);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lc.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	s.is_empty()
}

/// Words ending -mc.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	s.is_empty()
}

/// Words ending -nc.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oc.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 360);
			s.is_empty()
		}
		['w', 't', '\u{1}', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word pc.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	s.is_empty()
}

/// Words ending -rc.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	s.is_empty()
}

/// Words ending -sc.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	s.is_empty()
}

/// Words ending -tc.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	s.is_empty()
}

/// Words ending -vc.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	s.is_empty()
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
				['b', s @ ..] => {
					super::push(out, 362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 364);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1273);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2402);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 2403);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3100);
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3325);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 3326);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 2404);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 2405);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 2406);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3101);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 3102);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2407);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2408);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 3103);
											s.is_empty()
										}
										['s', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3104);
											s.is_empty()
										}
										['u', '\u{1}', s @ ..] => {
											super::push(out, 3105);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2409);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2410);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						['r', s @ ..] => super::t0::n1303(s, out),
						['t', s @ ..] => {
							super::push(out, 1276);
							match s {
								[] => true,
								['s', 'e', 'b', s @ ..] => {
									super::push(out, 2425);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3119);
											s.is_empty()
										}
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
					super::push(out, 366);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 367);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 369);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2426);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2427);
									s.is_empty()
								}
								['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 371);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 372);
					match s {
						[] => true,
						['s', s @ ..] => super::t0::n1307(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n376(s, out),
				['s', s @ ..] => super::t0::n377(s, out),
				['u', s @ ..] => super::t0::n378(s, out),
				['w', s @ ..] => super::t0::n379(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n72(s, out),
		['e', s @ ..] => super::t0::n73(s, out),
		['i', s @ ..] => super::t0::n74(s, out),
		['l', s @ ..] => super::t0::n75(s, out),
		['n', s @ ..] => super::t0::n76(s, out),
		['o', s @ ..] => super::t0::n77(s, out),
		['r', s @ ..] => super::t0::n78(s, out),
		['u', s @ ..] => super::t0::n79(s, out),
		['w', s @ ..] => super::t0::n80(s, out),
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 81);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 440);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 441);
					match s {
						[] => true,
						['f', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 442);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 443);
					match s {
						[] => true,
						['a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 444);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 445);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1364);
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
						['u', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 449);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 450);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1366);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2491);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 2492);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 451);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1369);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1370);
							match s {
								[] => true,
								['r', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1371);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2495);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2496);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2497);
									s.is_empty()
								}
								['t', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 452);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1372);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2499);
									s.is_empty()
								}
								['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1375);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2501);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2502);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2503);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2504);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 2505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1376);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2506);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 2507);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2508);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2509);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 3139);
											match s {
												[] => true,
												['c', 'a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 3335);
													s.is_empty()
												}
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
							super::push(out, 1377);
							match s {
								[] => true,
								['b', s @ ..] => super::t0::n2600(s, out),
								['e', 'd', s @ ..] => super::t0::n2601(s, out),
								['g', '\u{1}', s @ ..] => super::t0::n2602(s, out),
								['k', s @ ..] => super::t0::n2603(s, out),
								['l', s @ ..] => super::t0::n2604(s, out),
								['p', s @ ..] => super::t0::n2605(s, out),
								['r', s @ ..] => super::t0::n2606(s, out),
								['t', s @ ..] => super::t0::n2607(s, out),
								_ => false,
							}
						}
						['s', s @ ..] => super::t0::n1411(s, out),
						['t', s @ ..] => super::t0::n1412(s, out),
						['u', s @ ..] => super::t0::n1413(s, out),
						['v', s @ ..] => super::t0::n1414(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n457(s, out),
				['n', s @ ..] => super::t0::n458(s, out),
				['o', s @ ..] => super::t0::n459(s, out),
				['r', s @ ..] => super::t0::n460(s, out),
				['u', s @ ..] => super::t0::n461(s, out),
				['y', s @ ..] => super::t0::n462(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n85(s, out),
		['f', s @ ..] => super::t0::n86(s, out),
		['g', s @ ..] => super::t0::n87(s, out),
		['h', s @ ..] => super::t0::n88(s, out),
		['i', s @ ..] => super::t0::n89(s, out),
		['k', s @ ..] => super::t0::n90(s, out),
		['l', s @ ..] => super::t0::n91(s, out),
		['m', s @ ..] => super::t0::n92(s, out),
		['n', s @ ..] => super::t0::n93(s, out),
		['o', s @ ..] => super::t0::n94(s, out),
		['p', s @ ..] => super::t0::n95(s, out),
		['r', s @ ..] => super::t0::n96(s, out),
		['s', s @ ..] => super::t0::n97(s, out),
		['t', s @ ..] => super::t0::n98(s, out),
		['u', s @ ..] => super::t0::n99(s, out),
		['v', s @ ..] => super::t0::n100(s, out),
		['w', s @ ..] => super::t0::n101(s, out),
		['x', s @ ..] => super::t0::n102(s, out),
		['y', s @ ..] => super::t0::n103(s, out),
		['z', s @ ..] => super::t0::n104(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['a', 'd', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1592);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2676);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1594);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 596);
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
			super::push(out, 113);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['l', 'u', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 't', 'u', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1597);
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
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 601);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 1599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 116);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				['f', 'n', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 118);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1601);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1603);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1604);
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2677);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2678);
									s.is_empty()
								}
								['r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2679);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 2680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1605);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1611);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1612);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1613);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2681);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2682);
									s.is_empty()
								}
								['r', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2683);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1614);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1615);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1616);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1617);
							s.is_empty()
						}
						['r', s @ ..] => super::t0::n1657(s, out),
						['s', s @ ..] => {
							super::push(out, 1619);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2696);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2697);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 3213);
											match s {
												[] => true,
												['u', '\u{1}', s @ ..] => {
													super::push(out, 3365);
													s.is_empty()
												}
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
							super::push(out, 1621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2698);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2699);
									s.is_empty()
								}
								['k', s @ ..] => super::t0::n2798(s, out),
								['p', s @ ..] => super::t0::n2799(s, out),
								['r', 'e', 'v', 'o', '\u{1}', s @ ..] => super::t0::n2800(s, out),
								['s', s @ ..] => super::t0::n2801(s, out),
								['t', s @ ..] => super::t0::n2802(s, out),
								_ => false,
							}
						}
						['y', '\u{1}', s @ ..] => super::t0::n1661(s, out),
						['z', s @ ..] => super::t0::n1662(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n623(s, out),
				['u', s @ ..] => super::t0::n624(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n120(s, out),
		['p', s @ ..] => super::t0::n121(s, out),
		['r', s @ ..] => super::t0::n122(s, out),
		['s', s @ ..] => super::t0::n123(s, out),
		['u', s @ ..] => super::t0::n124(s, out),
		['w', s @ ..] => super::t0::n125(s, out),
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 126);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1632);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2707);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2708);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2709);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2710);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 3215);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3216);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1633);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2712);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 2713);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1634);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1636);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2714);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2715);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1637);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2716);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1639);
							match s {
								[] => true,
								['r', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 2717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 631);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 1643);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1644);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1645);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2720);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1646);
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 2722);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 1649);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2723);
									match s {
										[] => true,
										['m', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 3218);
											s.is_empty()
										}
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
					super::push(out, 634);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1650);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2724);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 2725);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3219);
											match s {
												[] => true,
												['o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 3366);
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
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2727);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2728);
									s.is_empty()
								}
								['n', s @ ..] => {
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
								['u', s @ ..] => {
									super::push(out, 2732);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 2733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1651);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1652);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1653);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 635);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1655);
							match s {
								[] => true,
								['u', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2736);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 129);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 130);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['u', 'o', 'r', 'h', s @ ..] => {
					super::push(out, 637);
					match s {
						[] => true,
						['t', 'e', 'k', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n132(s, out),
		['n', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		['o', s @ ..] => super::t0::n134(s, out),
		['p', s @ ..] => super::t0::n135(s, out),
		['s', s @ ..] => super::t0::n136(s, out),
		['t', s @ ..] => super::t0::n137(s, out),
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
				['a', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 138);
			s.is_empty()
		}
		['d', 'i', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['b', 'i', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 648);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1662);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1663);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -j.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 150);
			s.is_empty()
		}
		['o', 'o', 'j', '\u{1}', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 152);
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
			super::push(out, 153);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1668);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1672);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2738);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2739);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1673);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2740);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3220);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 3221);
											match s {
												[] => true,
												['i', 'a', s @ ..] => {
													match s {
														[] => true,
														['j', s @ ..] => super::t0::n3548(s, out),
														['r', s @ ..] => super::t0::n3549(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', 'a', s @ ..] => {
											super::push(out, 3222);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													super::push(out, 3367);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 2741);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2742);
									s.is_empty()
								}
								['h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2743);
									s.is_empty()
								}
								['k', 's', '\u{1}', s @ ..] => {
									super::push(out, 2744);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2745);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 2746);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 3223);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1675);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 656);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1679);
							s.is_empty()
						}
						['a', 't', 'e', 'k', 'c', 'a', 'y', '\u{1}', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1681);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 2747);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1684);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1685);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1686);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2748);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2750);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3224);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 3368);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 3225);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3226);
											s.is_empty()
										}
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
					super::push(out, 668);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 2751);
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['l', 'p', 'p', s @ ..] => {
							super::push(out, 1687);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 2753);
									s.is_empty()
								}
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
			super::push(out, 155);
			match s {
				[] => true,
				['e', s @ ..] => super::t0::n680(s, out),
				['i', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['k', s @ ..] => super::t0::n682(s, out),
				['l', s @ ..] => super::t0::n683(s, out),
				['r', s @ ..] => super::t0::n684(s, out),
				['t', s @ ..] => super::t0::n685(s, out),
				['y', s @ ..] => super::t0::n686(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t0::n157(s, out),
		['i', s @ ..] => super::t0::n158(s, out),
		['l', s @ ..] => super::t0::n159(s, out),
		['n', s @ ..] => super::t0::n160(s, out),
		['o', s @ ..] => super::t0::n161(s, out),
		['r', s @ ..] => super::t0::n162(s, out),
		['s', s @ ..] => super::t0::n163(s, out),
		['u', s @ ..] => super::t0::n164(s, out),
		['w', s @ ..] => super::t0::n165(s, out),
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 1715);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1717);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1718);
							match s {
								[] => true,
								['e', 'r', 'o', s @ ..] => {
									super::push(out, 2771);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 3234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1719);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1720);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1721);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1722);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1723);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1724);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1725);
							match s {
								[] => true,
								['s', 'l', s @ ..] => {
									super::push(out, 2772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['t', 'r', s @ ..] => {
							super::push(out, 1728);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', 't', 'r', 'u', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 3235);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3236);
											s.is_empty()
										}
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
					super::push(out, 700);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2773);
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
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['y', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2774);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1733);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1734);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 3237);
											s.is_empty()
										}
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3369);
													s.is_empty()
												}
												['n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 3370);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3238);
											s.is_empty()
										}
										['n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3239);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 1735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 711);
					s.is_empty()
				}
				['g', s @ ..] => super::t0::n721(s, out),
				['i', s @ ..] => {
					super::push(out, 713);
					s.is_empty()
				}
				['n', s @ ..] => super::t0::n723(s, out),
				['p', s @ ..] => super::t0::n724(s, out),
				['s', s @ ..] => super::t0::n725(s, out),
				['t', s @ ..] => super::t0::n726(s, out),
				['v', s @ ..] => super::t0::n727(s, out),
				['w', s @ ..] => super::t0::n728(s, out),
				['y', '\u{1}', s @ ..] => super::t0::n729(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n169(s, out),
		['l', s @ ..] => super::t0::n170(s, out),
		['o', s @ ..] => super::t0::n171(s, out),
		['r', s @ ..] => super::t0::n172(s, out),
		['u', s @ ..] => super::t0::n173(s, out),
		['w', s @ ..] => super::t0::n174(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 744);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 746);
					match s {
						[] => true,
						['e', 'e', s @ ..] => {
							super::push(out, 1797);
							match s {
								[] => true,
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2802);
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
						['\u{1}', s @ ..] => {
							super::push(out, 1798);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1802);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2803);
									match s {
										[] => true,
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3252);
											s.is_empty()
										}
										['y', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 3253);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						['l', 'a', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 749);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1806);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2805);
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
						['s', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						['u', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 751);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 753);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['c', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2806);
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
						['\u{1}', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 755);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 756);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 757);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'i', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2807);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1815);
							s.is_empty()
						}
						['y', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 762);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1819);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 178);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 179);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1824);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2809);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['c', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 181);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 182);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				['l', 'g', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => super::t0::n795(s, out),
				['o', s @ ..] => super::t0::n796(s, out),
				['r', 'p', s @ ..] => super::t0::n797(s, out),
				['s', s @ ..] => super::t0::n798(s, out),
				['t', s @ ..] => super::t0::n799(s, out),
				['v', s @ ..] => super::t0::n800(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n185(s, out),
		['s', s @ ..] => super::t0::n186(s, out),
		['u', s @ ..] => super::t0::n187(s, out),
		['w', s @ ..] => super::t0::n188(s, out),
		['y', s @ ..] => super::t0::n189(s, out),
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 798);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['u', 't', '\u{1}', s @ ..] => {
									super::push(out, 2814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1853);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1855);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2815);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2816);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											super::push(out, 3254);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2817);
									match s {
										[] => true,
										['e', 'v', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3255);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2818);
									match s {
										[] => true,
										['u', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2819);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2820);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 3257);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2821);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1857);
							s.is_empty()
						}
						['i', 'l', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1861);
							match s {
								[] => true,
								['a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2824);
									s.is_empty()
								}
								['i', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'l', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 814);
					s.is_empty()
				}
				['y', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1871);
							s.is_empty()
						}
						['l', 'i', s @ ..] => {
							super::push(out, 1872);
							match s {
								[] => true,
								['m', 'n', s @ ..] => {
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
					super::push(out, 818);
					match s {
						[] => true,
						['r', 'c', 's', 'f', s @ ..] => {
							super::push(out, 1873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1874);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1876);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t0::n839(s, out),
				['p', s @ ..] => super::t0::n840(s, out),
				['r', s @ ..] => super::t0::n841(s, out),
				['s', s @ ..] => super::t0::n842(s, out),
				['t', s @ ..] => super::t0::n843(s, out),
				['v', s @ ..] => super::t0::n844(s, out),
				['y', s @ ..] => super::t0::n845(s, out),
				['z', s @ ..] => super::t0::n846(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t0::n192(s, out),
		['i', s @ ..] => super::t0::n193(s, out),
		['l', s @ ..] => super::t0::n194(s, out),
		['m', s @ ..] => super::t0::n195(s, out),
		['o', s @ ..] => super::t0::n196(s, out),
		['r', s @ ..] => super::t0::n197(s, out),
		['u', s @ ..] => super::t0::n198(s, out),
		['w', s @ ..] => super::t0::n199(s, out),
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['c', 'u', 't', 's', 'e', s @ ..] => {
					super::push(out, 872);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 873);
					s.is_empty()
				}
				['s', 'e', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['c', 'i', 'r', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1963);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1964);
							s.is_empty()
						}
						['n', 'i', s @ ..] => {
							super::push(out, 1965);
							s.is_empty()
						}
						['s', 'i', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1967);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1968);
							match s {
								[] => true,
								['m', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 2857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'o', 'r', 'w', '\u{1}', s @ ..] => {
					super::push(out, 877);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1972);
							match s {
								[] => true,
								['c', 's', 'e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2858);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 881);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 884);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1974);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 886);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 888);
					s.is_empty()
				}
				['k', 'c', 'u', 'd', 'k', 'c', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 889);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 890);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1978);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2859);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2860);
									s.is_empty()
								}
								['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2862);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2864);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 2865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1982);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2866);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 2867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1985);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2868);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2869);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2870);
									s.is_empty()
								}
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
			super::push(out, 206);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['h', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 901);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1987);
							match s {
								[] => true,
								['i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 2871);
									s.is_empty()
								}
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
			super::push(out, 209);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['l', 'l', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t0::n215(s, out),
		['t', s @ ..] => super::t0::n216(s, out),
		['v', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1995);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1996);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2872);
									s.is_empty()
								}
								['h', 't', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2873);
									s.is_empty()
								}
								['t', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 916);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 917);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 919);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				['y', 'd', 'd', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', '\u{1}', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 924);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1997);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1998);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1999);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2000);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 2001);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2002);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2875);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 3259);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 3260);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2003);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2876);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2005);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2006);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 2878);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2879);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2007);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2008);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2009);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2010);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 2881);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2882);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 3261);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['w', 's', '\u{1}', s @ ..] => {
							super::push(out, 2012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2013);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2014);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2883);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2884);
									match s {
										[] => true,
										['a', 'l', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 3262);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 3263);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 3264);
											s.is_empty()
										}
										['o', 'w', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3373);
													s.is_empty()
												}
												['r', s @ ..] => {
													match s {
														[] => true,
														['e', 'v', 'o', '\u{1}', s @ ..] => super::t0::n3550(s, out),
														['o', 'f', '\u{1}', s @ ..] => super::t0::n3551(s, out),
														_ => false,
													}
												}
												['s', 'i', s @ ..] => {
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t0::n3552(s, out),
														['m', '\u{1}', s @ ..] => super::t0::n3553(s, out),
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
							super::push(out, 2015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						['l', s @ ..] => super::t0::n2074(s, out),
						['s', s @ ..] => super::t0::n2075(s, out),
						_ => false,
					}
				}
				['n', 'r', s @ ..] => super::t0::n959(s, out),
				['r', s @ ..] => super::t0::n960(s, out),
				['s', s @ ..] => super::t0::n961(s, out),
				['t', s @ ..] => super::t0::n962(s, out),
				['u', s @ ..] => super::t0::n963(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n222(s, out),
		['m', s @ ..] => super::t0::n223(s, out),
		['o', s @ ..] => super::t0::n224(s, out),
		['p', s @ ..] => super::t0::n225(s, out),
		['r', s @ ..] => super::t0::n226(s, out),
		['s', s @ ..] => super::t0::n227(s, out),
		['t', s @ ..] => super::t0::n228(s, out),
		['u', s @ ..] => super::t0::n229(s, out),
		['v', s @ ..] => super::t0::n230(s, out),
		['w', s @ ..] => super::t0::n231(s, out),
		_ => false,
	}
}

/// Words ending -q.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
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
			super::push(out, 232);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2039);
							match s {
								[] => true,
								['o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 2889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2040);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['i', 'v', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2891);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2892);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 2893);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2894);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2044);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2046);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 2896);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2897);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2898);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2899);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2900);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3265);
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 3374);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 3375);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 3266);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3267);
											match s {
												[] => true,
												['t', '\u{1}', s @ ..] => {
													super::push(out, 3376);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2053);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 2054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['k', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2056);
							s.is_empty()
						}
						['m', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 985);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				['e', s @ ..] => super::t0::n1013(s, out),
				['f', s @ ..] => super::t0::n1014(s, out),
				['g', s @ ..] => super::t0::n1015(s, out),
				['h', s @ ..] => super::t0::n1016(s, out),
				['k', s @ ..] => super::t0::n1017(s, out),
				['m', s @ ..] => super::t0::n1018(s, out),
				['p', s @ ..] => super::t0::n1019(s, out),
				['t', s @ ..] => super::t0::n1020(s, out),
				['v', s @ ..] => super::t0::n1021(s, out),
				['w', s @ ..] => super::t0::n1022(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n235(s, out),
		['l', s @ ..] => super::t0::n236(s, out),
		['o', s @ ..] => super::t0::n237(s, out),
		['r', s @ ..] => super::t0::n238(s, out),
		['u', s @ ..] => super::t0::n239(s, out),
		['v', s @ ..] => super::t0::n240(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['r', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1019);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1020);
					match s {
						[] => true,
						['x', '\u{1}', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				['r', 'k', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1028);
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1029);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1030);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1031);
					match s {
						[] => true,
						['u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1032);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1033);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1035);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2096);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 's', 'o', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1037);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2913);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2914);
									s.is_empty()
								}
								['u', 'q', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2916);
									s.is_empty()
								}
								['u', 'l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1038);
					match s {
						[] => true,
						['o', 'f', 'o', s @ ..] => {
							match s {
								[] => true,
								['n', 'o', s @ ..] => {
									super::push(out, 2918);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1039);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2101);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2920);
									s.is_empty()
								}
								['n', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 2921);
									s.is_empty()
								}
								['r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 2922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1040);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1041);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 2102);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2923);
									s.is_empty()
								}
								['z', 'e', 'd', 'n', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1042);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						['o', 't', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1045);
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2105);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1046);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1047);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2107);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2108);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2925);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 3275);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													super::push(out, 3378);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 2926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', s @ ..] => {
							super::push(out, 2109);
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 2927);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2110);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2111);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2113);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2114);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2115);
							match s {
								[] => true,
								['e', 'e', 'l', s @ ..] => {
									super::push(out, 2928);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 3276);
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
								['e', s @ ..] => {
									super::push(out, 2929);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 2930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2117);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2931);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 3277);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2118);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2119);
							match s {
								[] => true,
								['s', 't', s @ ..] => {
									super::push(out, 2932);
									match s {
										[] => true,
										['u', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3278);
											s.is_empty()
										}
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
					super::push(out, 1048);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1050);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1051);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2122);
							match s {
								[] => true,
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 2933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1052);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1053);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2123);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1056);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1057);
					match s {
						[] => true,
						['s', 'o', 't', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1058);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1059);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 250);
			s.is_empty()
		}
		['e', s @ ..] => super::t0::n252(s, out),
		['f', s @ ..] => {
			super::push(out, 252);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['i', s @ ..] => super::t0::n2268(s, out),
						['u', s @ ..] => {
							super::push(out, 2194);
							match s {
								[] => true,
								['a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n255(s, out),
		['l', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['m', 'r', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n257(s, out),
		['o', s @ ..] => super::t1::n258(s, out),
		['p', s @ ..] => super::t1::n259(s, out),
		['r', s @ ..] => super::t1::n260(s, out),
		['s', s @ ..] => super::t1::n261(s, out),
		['t', s @ ..] => super::t1::n262(s, out),
		['u', s @ ..] => super::t1::n263(s, out),
		['v', s @ ..] => super::t1::n264(s, out),
		['w', s @ ..] => super::t1::n265(s, out),
		['x', s @ ..] => super::t1::n266(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	s.is_empty()
}

/// Words ending -v.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2317);
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
				['e', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'h', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				['t', 'o', 'l', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 268);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 270);
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
			super::push(out, 271);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2319);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2323);
							match s {
								[] => true,
								['s', 'r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1172);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3073);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'i', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 3075);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3076);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2328);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2329);
							s.is_empty()
						}
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 2330);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3077);
									s.is_empty()
								}
								['e', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3078);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							super::push(out, 2331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2332);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2333);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2334);
							match s {
								[] => true,
								['s', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2335);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2336);
							s.is_empty()
						}
						['r', 'u', s @ ..] => {
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3081);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 3082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1186);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1187);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2338);
							s.is_empty()
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2339);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2340);
							match s {
								[] => true,
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3083);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2342);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2343);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2344);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2347);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2348);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2350);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2351);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2352);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2353);
							s.is_empty()
						}
						['h', s @ ..] => super::t1::n2438(s, out),
						['r', s @ ..] => {
							super::push(out, 2355);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2356);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t1::n1219(s, out),
				['t', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['v', s @ ..] => super::t1::n1221(s, out),
				_ => false,
			}
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
			super::push(out, 274);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 276);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 278);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['m', 'n', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					super::push(out, 1201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['e', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 2360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 284);
			s.is_empty()
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
			super::push(out, 285);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2361);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2362);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3088);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3089);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3317);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3318);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3319);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3320);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2363);
							match s {
								[] => true,
								['f', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'c', 'i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2364);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2365);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2366);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3321);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2367);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2368);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3093);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2369);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2372);
							s.is_empty()
						}
						['e', 'l', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2374);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2375);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2377);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2378);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									super::push(out, 3095);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 3322);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2379);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2383);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3096);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 3323);
											s.is_empty()
										}
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
					super::push(out, 1214);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['n', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					super::push(out, 1218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1219);
					s.is_empty()
				}
				['k', 'n', 'o', s @ ..] => {
					super::push(out, 1220);
					s.is_empty()
				}
				['l', 'l', 'o', 'r', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2387);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3097);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 3324);
											match s {
												[] => true,
												['e', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 1224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2388);
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
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1226);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 2390);
							match s {
								[] => true,
								['w', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3098);
									s.is_empty()
								}
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
			super::push(out, 295);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['r', 'e', 't', s @ ..] => super::t1::n2476(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t1::n1255(s, out),
				['o', s @ ..] => super::t1::n1256(s, out),
				['p', s @ ..] => super::t1::n1257(s, out),
				['r', s @ ..] => super::t1::n1258(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t1::n299(s, out),
		['n', s @ ..] => super::t1::n300(s, out),
		['o', s @ ..] => super::t1::n301(s, out),
		['p', s @ ..] => super::t1::n302(s, out),
		['r', s @ ..] => super::t1::n303(s, out),
		['s', s @ ..] => super::t1::n304(s, out),
		['t', s @ ..] => super::t1::n305(s, out),
		['u', s @ ..] => super::t1::n306(s, out),
		['v', s @ ..] => super::t1::n307(s, out),
		['x', s @ ..] => super::t1::n308(s, out),
		['z', s @ ..] => super::t1::n309(s, out),
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['o', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		['u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -read.
pub(crate) fn n1303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1275);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2413);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2414);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3109);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3327);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3110);
					s.is_empty()
				}
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2415);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2416);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2417);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2418);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2419);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2420);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3329);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'd', s @ ..] => {
							super::push(out, 3330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3400);
									s.is_empty()
								}
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
			super::push(out, 2421);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2422);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2423);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3114);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 3331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'g', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3116);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -spad.
pub(crate) fn n1307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1279);
	s.is_empty()
}

/// Words ending -rad.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1280);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1281);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sad.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uad.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	s.is_empty()
}

/// Words ending -wad.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dd.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2429);
							s.is_empty()
						}
						['t', 'u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ed.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1285);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1286);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1287);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['b', 'e', 'e', s @ ..] => {
							super::push(out, 2431);
							match s {
								[] => true,
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1290);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1291);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['m', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2432);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3121);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 3122);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2433);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2434);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3124);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1293);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1295);
					match s {
						[] => true,
						['s', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 's', '\u{1}', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 383);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		['n', 's', '\u{1}', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['z', '\u{1}', s @ ..] => {
			super::push(out, 391);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -id.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1300);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 394);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 395);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				['l', 'a', 'p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 399);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 400);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ld.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1308);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['y', 'a', s @ ..] => {
							super::push(out, 2446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1310);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1311);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1312);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 406);
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
				['g', s @ ..] => {
					super::push(out, 1317);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2448);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2449);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1319);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2451);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3126);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nd.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1322);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1324);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1325);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1327);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', s @ ..] => {
									super::push(out, 3128);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 3401);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 3402);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 3129);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3130);
									s.is_empty()
								}
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
			super::push(out, 410);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2453);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2454);
							s.is_empty()
						}
						['k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2455);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2456);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1330);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1331);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1332);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1333);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2458);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2459);
							match s {
								[] => true,
								['e', 'v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1334);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2461);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2462);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2463);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2465);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2468);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3132);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3334);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3403);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3133);
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 3134);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3135);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2470);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2471);
							s.is_empty()
						}
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 2472);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2473);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2475);
							s.is_empty()
						}
						['s', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2476);
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
				['b', s @ ..] => {
					super::push(out, 1341);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1344);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1346);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2478);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2479);
							match s {
								[] => true,
								['e', 's', s @ ..] => {
									super::push(out, 3136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1350);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2481);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 3137);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2483);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2485);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 412);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['o', 's', 'n', s @ ..] => super::t1::n1385(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -od.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 415);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1353);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 419);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 420);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1356);
					match s {
						[] => true,
						['h', 't', 'u', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1357);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 424);
			s.is_empty()
		}
		['u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rd.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1359);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 428);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 430);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 431);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 435);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 437);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wd.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	s.is_empty()
}

/// Words ending -bride.
pub(crate) fn n2600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2510);
	s.is_empty()
}

/// Words ending -deride.
pub(crate) fn n2601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2511);
	s.is_empty()
}

/// The word gride.
pub(crate) fn n2602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2512);
	s.is_empty()
}

/// Words ending -kride.
pub(crate) fn n2603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2513);
	s.is_empty()
}

/// Words ending -lride.
pub(crate) fn n2604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2514);
	s.is_empty()
}

/// Words ending -pride.
pub(crate) fn n2605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2515);
	match s {
		[] => true,
		['u', '\u{1}', s @ ..] => {
			super::push(out, 3140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rride.
pub(crate) fn n2606(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 3141);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tride.
pub(crate) fn n2607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2516);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3143);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3336);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3337);
					s.is_empty()
				}
				['o', 'h', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3338);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3339);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -side.
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1378);
	s.is_empty()
}

/// Words ending -tide.
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1379);
	match s {
		[] => true,
		['e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uide.
pub(crate) fn n1413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1380);
	s.is_empty()
}

/// Words ending -vide.
pub(crate) fn n1414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1381);
	s.is_empty()
}

/// Words ending -lde.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['o', 'h', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nde.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1383);
			match s {
				[] => true,
				['h', 's', '\u{1}', s @ ..] => {
					super::push(out, 2518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1384);
			match s {
				[] => true,
				['w', 't', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2519);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ode.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	s.is_empty()
}

/// Words ending -rde.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	s.is_empty()
}

/// Words ending -ude.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yde.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1386);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 't', s @ ..] => {
			super::push(out, 1387);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ee.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 461);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 462);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 464);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1391);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 467);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 468);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fe.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	s.is_empty()
}

/// Words ending -ge.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1393);
					match s {
						[] => true,
						['b', s @ ..] => {
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
					super::push(out, 1394);
					s.is_empty()
				}
				['o', 'l', 'p', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['e', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1398);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1399);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1400);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2533);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3146);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3341);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 3147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2534);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2535);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3148);
									s.is_empty()
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
							super::push(out, 2536);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1402);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1403);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -he.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1406);
					s.is_empty()
				}
				['o', 'l', 'c', 'r', s @ ..] => {
					super::push(out, 1407);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ie.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1408);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['o', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				['u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'o', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['h', 's', '\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				['o', 'o', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2542);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2545);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3149);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						['e', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 's', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2548);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['t', 'u', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'y', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['o', 't', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 484);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 486);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2550);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'p', 'o', 'o', '\u{1}', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2552);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		['y', 'l', 'u', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ke.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 2554);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3150);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3151);
									s.is_empty()
								}
								['k', 'l', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3152);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3153);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3154);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 3155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2556);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2558);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2562);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3157);
									s.is_empty()
								}
								['s', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2564);
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
				['i', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['s', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2567);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3159);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3160);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 3404);
													s.is_empty()
												}
												['w', s @ ..] => {
													super::push(out, 3405);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 3342);
											match s {
												[] => true,
												['e', 'd', s @ ..] => {
													super::push(out, 3406);
													match s {
														[] => true,
														['n', 'u', 'h', 't', '\u{1}', s @ ..] => super::t1::n3560(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['y', s @ ..] => {
											super::push(out, 3343);
											s.is_empty()
										}
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
					super::push(out, 1447);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 2572);
							s.is_empty()
						}
						['r', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 2573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 499);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1451);
					s.is_empty()
				}
				['h', 'r', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1453);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2576);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3161);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2577);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1457);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['n', 'e', 'b', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1459);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2579);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3162);
									s.is_empty()
								}
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
			super::push(out, 511);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 512);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1461);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2582);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1462);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1463);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1464);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1465);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ne.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 515);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1468);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2585);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1469);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1470);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2588);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3163);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 3345);
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3407);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 3408);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3164);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3165);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3166);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3346);
											s.is_empty()
										}
										['u', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1471);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				['r', 't', 'a', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['o', 'r', s @ ..] => {
					super::push(out, 1474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oe.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 520);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['r', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pe.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1481);
					match s {
						[] => true,
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 2592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'c', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 526);
			s.is_empty()
		}
		['u', 't', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1487);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 2593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2595);
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 3167);
									s.is_empty()
								}
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
			super::push(out, 530);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 532);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 533);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 536);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1492);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1494);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1495);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1496);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1497);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2596);
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3168);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3169);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 3348);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 3170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3171);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2597);
							s.is_empty()
						}
						['p', 'u', s @ ..] => {
							super::push(out, 2598);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2599);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							super::push(out, 2600);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 539);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2602);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2603);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2604);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2605);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 3173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2606);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2607);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2608);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1504);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 1507);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 543);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3174);
									s.is_empty()
								}
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

/// Words ending -te.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1509);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2612);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 3175);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 3349);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 3350);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2613);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3176);
									s.is_empty()
								}
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
				['e', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1511);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1512);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							super::push(out, 2615);
							match s {
								[] => true,
								['e', 'd', 'o', s @ ..] => {
									super::push(out, 3178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1513);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2616);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 3179);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3351);
											s.is_empty()
										}
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
					super::push(out, 1514);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2617);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3180);
									s.is_empty()
								}
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
			super::push(out, 546);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1515);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1517);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2618);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3181);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 3182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 2620);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 3183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1522);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2622);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2623);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1525);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2625);
							s.is_empty()
						}
						['e', 'h', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2626);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2627);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2628);
							s.is_empty()
						}
						['t', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2629);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2630);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1529);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1532);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 550);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 1534);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 2633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1536);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3184);
									s.is_empty()
								}
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 1539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1545);
					match s {
						[] => true,
						['n', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2635);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'd', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 559);
			s.is_empty()
		}
		['q', 'i', 'l', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2640);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2641);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3186);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1555);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2643);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2644);
							match s {
								[] => true,
								['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2645);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3188);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 3189);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3352);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3353);
											match s {
												[] => true,
												['t', '\u{1}', s @ ..] => {
													super::push(out, 3409);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 3354);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3355);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3190);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 3410);
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 3411);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 3356);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2646);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 3192);
									s.is_empty()
								}
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 3193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 2647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1556);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2648);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3194);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1557);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1559);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 564);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1563);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1565);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1566);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1567);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2650);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3196);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2651);
							s.is_empty()
						}
						['e', 'b', s @ ..] => {
							super::push(out, 2652);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1568);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2654);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2655);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 3198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2656);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
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
						['o', s @ ..] => {
							super::push(out, 2657);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2658);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2659);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3202);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3203);
									match s {
										[] => true,
										['n', 'i', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 3357);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				['w', 's', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 566);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1573);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							super::push(out, 2662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1575);
					match s {
						[] => true,
						['c', 'r', 'o', s @ ..] => super::t1::n2761(s, out),
						['h', s @ ..] => {
							super::push(out, 2664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -we.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['a', 'r', 'd', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					super::push(out, 1577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xe.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	s.is_empty()
}

/// Words ending -ye.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['d', 'o', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		['e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 573);
			s.is_empty()
		}
		['h', '\u{1}', s @ ..] => {
			super::push(out, 574);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['r', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 's', '\u{1}', s @ ..] => {
			super::push(out, 576);
			s.is_empty()
		}
		['o', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 577);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 579);
			s.is_empty()
		}
		['v', '\u{1}', s @ ..] => {
			super::push(out, 580);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ze.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2669);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2670);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2671);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3206);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3207);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3358);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2672);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2674);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3208);
									s.is_empty()
								}
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

/// Words ending -ring.
pub(crate) fn n1657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1618);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2685);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2686);
			s.is_empty()
		}
		['d', '\u{1}', s @ ..] => {
			super::push(out, 2687);
			s.is_empty()
		}
		['e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2688);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2689);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2690);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3209);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 3210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2691);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3359);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3360);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2692);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2693);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2694);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3211);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3362);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3412);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3413);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3363);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2695);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kwing.
pub(crate) fn n2798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2700);
	s.is_empty()
}

/// Words ending -pwing.
pub(crate) fn n2799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2701);
	s.is_empty()
}

/// The word overwing.
pub(crate) fn n2800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2702);
	s.is_empty()
}

/// Words ending -swing.
pub(crate) fn n2801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2703);
	match s {
		[] => true,
		['r', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 3214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -twing.
pub(crate) fn n2802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2704);
	s.is_empty()
}

/// The word ying.
pub(crate) fn n1661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1622);
	s.is_empty()
}

/// Words ending -zing.
pub(crate) fn n1662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1623);
	s.is_empty()
}

/// Words ending -ong.
pub(crate) fn n623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	s.is_empty()
}

/// Words ending -ung.
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	s.is_empty()
}

/// Words ending -og.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 617);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 618);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2705);
							s.is_empty()
						}
						['m', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				['o', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 622);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pg.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	s.is_empty()
}

/// Words ending -rg.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	s.is_empty()
}

/// Words ending -sg.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	s.is_empty()
}

/// Words ending -ug.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 624);
			match s {
				[] => true,
				['h', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 1630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 625);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wg.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	s.is_empty()
}

/// Words ending -hh.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		['e', '\u{1}', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oh.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	s.is_empty()
}

/// Words ending -ph.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	s.is_empty()
}

/// Words ending -sh.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -th.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['i', 'r', s @ ..] => {
			super::push(out, 643);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 644);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1659);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 2737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jailbreak.
pub(crate) fn n3548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3415);
	s.is_empty()
}

/// Words ending -railbreak.
pub(crate) fn n3549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3416);
	s.is_empty()
}

/// Words ending -eek.
pub(crate) fn n680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 671);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1688);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1689);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1690);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1691);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1692);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1693);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kek.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 673);
	s.is_empty()
}

/// Words ending -lek.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1695);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rek.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	s.is_empty()
}

/// Words ending -tek.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1697);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yek.
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1698);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fk.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	s.is_empty()
}

/// Words ending -ik.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			super::push(out, 678);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lk.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['w', 'd', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nk.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 680);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1701);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2754);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2755);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2756);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2758);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3227);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3228);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2759);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2760);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2761);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1709);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2764);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2766);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2767);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 3230);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2768);
							match s {
								[] => true,
								['r', 'o', 'f', '\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 682);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ok.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		['r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 686);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rk.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							super::push(out, 2769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sk.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uk.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['n', '\u{1}', s @ ..] => {
			super::push(out, 690);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wk.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['o', 'b', s @ ..] => {
			super::push(out, 693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gel.
pub(crate) fn n721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 712);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1743);
			s.is_empty()
		}
		['a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1744);
			s.is_empty()
		}
		['d', 'u', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3240);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2777);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 2778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2779);
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nel.
pub(crate) fn n723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 714);
	s.is_empty()
}

/// Words ending -pel.
pub(crate) fn n724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1745);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1746);
			s.is_empty()
		}
		['p', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1747);
			s.is_empty()
		}
		['s', 'o', s @ ..] => {
			super::push(out, 1748);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1749);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sel.
pub(crate) fn n725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 716);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1750);
			s.is_empty()
		}
		['o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1751);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tel.
pub(crate) fn n726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 717);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1752);
			s.is_empty()
		}
		['n', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1753);
			s.is_empty()
		}
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1754);
			s.is_empty()
		}
		['r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1755);
			s.is_empty()
		}
		['s', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1756);
			s.is_empty()
		}
		['t', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1757);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vel.
pub(crate) fn n727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 718);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2784);
					s.is_empty()
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1759);
			s.is_empty()
		}
		['n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1760);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wel.
pub(crate) fn n728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 719);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1762);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word yel.
pub(crate) fn n729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 720);
	s.is_empty()
}

/// Words ending -il.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 722);
			s.is_empty()
		}
		['e', 'v', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2786);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 724);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1764);
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 725);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 727);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2788);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3242);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 3243);
									s.is_empty()
								}
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
			super::push(out, 728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ll.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1769);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1771);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2789);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2790);
							s.is_empty()
						}
						['r', 'e', 't', s @ ..] => {
							super::push(out, 2791);
							s.is_empty()
						}
						['t', 'i', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1772);
					match s {
						[] => true,
						['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 2793);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3244);
									s.is_empty()
								}
								['r', 'a', 'm', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1774);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1777);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1780);
					match s {
						[] => true,
						['s', 'e', 'r', 'o', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1782);
					match s {
						[] => true,
						['s', 's', 'i', 'm', s @ ..] => {
							super::push(out, 2795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1783);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1784);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2797);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3247);
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2798);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2799);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3249);
									s.is_empty()
								}
								['e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3371);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 3372);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3250);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1789);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
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

/// Words ending -ol.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['c', 'a', 'r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 734);
			s.is_empty()
		}
		['j', '\u{1}', s @ ..] => {
			super::push(out, 735);
			s.is_empty()
		}
		['k', 's', '\u{1}', s @ ..] => {
			super::push(out, 736);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 737);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1791);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1792);
					s.is_empty()
				}
				['r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1793);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['s', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1795);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rl.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	s.is_empty()
}

/// Words ending -ul.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 742);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wl.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	s.is_empty()
}

/// The word nom.
pub(crate) fn n795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 780);
	s.is_empty()
}

/// Words ending -oom.
pub(crate) fn n796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 781);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -prom.
pub(crate) fn n797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 782);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -som.
pub(crate) fn n798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 783);
	s.is_empty()
}

/// Words ending -tom.
pub(crate) fn n799(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1832);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1833);
			match s {
				[] => true,
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 2812);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vom.
pub(crate) fn n800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 784);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rm.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	s.is_empty()
}

/// Words ending -sm.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	s.is_empty()
}

/// Words ending -um.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 785);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 786);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1835);
					s.is_empty()
				}
				['e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1836);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1837);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 788);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 789);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 790);
			s.is_empty()
		}
		['i', 'd', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1839);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1841);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1842);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1843);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1844);
					s.is_empty()
				}
				['y', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 791);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 792);
			s.is_empty()
		}
		['o', 's', '\u{1}', s @ ..] => {
			super::push(out, 793);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 2813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wm.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	s.is_empty()
}

/// Words ending -ym.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	s.is_empty()
}

/// Words ending -ken.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1878);
			s.is_empty()
		}
		['a', '\u{1}', s @ ..] => {
			super::push(out, 1879);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1880);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2827);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2828);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'u', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2830);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1883);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1884);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2832);
					s.is_empty()
				}
				['i', 'm', s @ ..] => {
					super::push(out, 2833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pen.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1886);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1887);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 1888);
			s.is_empty()
		}
		['m', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1889);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1890);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1891);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2836);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1892);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1893);
			s.is_empty()
		}
		['s', 'i', 'm', s @ ..] => {
			super::push(out, 1894);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ren.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 822);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			super::push(out, 1897);
			match s {
				[] => true,
				['v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2841);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sen.
pub(crate) fn n842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 823);
	s.is_empty()
}

/// Words ending -ten.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 824);
	s.is_empty()
}

/// Words ending -ven.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	s.is_empty()
}

/// Words ending -yen.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 826);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zen.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 827);
	s.is_empty()
}

/// Words ending -gn.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	s.is_empty()
}

/// Words ending -in.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 829);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1900);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 832);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1906);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1908);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1909);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 2843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', s @ ..] => {
					super::push(out, 1910);
					match s {
						[] => true,
						['m', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1911);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 838);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 839);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1916);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1919);
					match s {
						[] => true,
						['e', 'h', s @ ..] => {
							super::push(out, 2845);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2846);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1920);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1921);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1922);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1923);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1925);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				['o', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1929);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2849);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 844);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 845);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1931);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1932);
					match s {
						[] => true,
						['u', 'o', '\u{1}', s @ ..] => {
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

/// Words ending -ln.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	s.is_empty()
}

/// Words ending -mn.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	s.is_empty()
}

/// Words ending -on.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1933);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1934);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1935);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1936);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1937);
					s.is_empty()
				}
				['r', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1938);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1939);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				['y', 'b', 'b', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1942);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1943);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1944);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 849);
			s.is_empty()
		}
		['l', 'l', s @ ..] => {
			super::push(out, 850);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 851);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rn.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1950);
					match s {
						[] => true,
						['l', 'e', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2852);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1952);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -un.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 860);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 861);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 862);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 863);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 864);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 865);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 866);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1960);
					match s {
						[] => true,
						['o', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 868);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wn.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['s', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 907);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 908);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['m', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 910);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 911);
			s.is_empty()
		}
		['s', 'a', s @ ..] => {
			super::push(out, 912);
			s.is_empty()
		}
		['t', 'e', 'h', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['c', 'n', 'a', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1993);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word overworship.
pub(crate) fn n3550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3417);
	s.is_empty()
}

/// The word forworship.
pub(crate) fn n3551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3418);
	s.is_empty()
}

/// The word disworship.
pub(crate) fn n3552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3419);
	s.is_empty()
}

/// The word misworship.
pub(crate) fn n3553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3420);
	s.is_empty()
}

/// Words ending -llip.
pub(crate) fn n2074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2017);
	s.is_empty()
}

/// Words ending -slip.
pub(crate) fn n2075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2018);
	s.is_empty()
}

/// Words ending -rnip.
pub(crate) fn n959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['u', 't', '\u{1}', s @ ..] => {
			super::push(out, 2019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rip.
pub(crate) fn n960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 937);
	match s {
		[] => true,
		['t', 's', 't', s @ ..] => {
			super::push(out, 2020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sip.
pub(crate) fn n961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 938);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2021);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tip.
pub(crate) fn n962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 939);
	s.is_empty()
}

/// Words ending -uip.
pub(crate) fn n963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 940);
	s.is_empty()
}

/// Words ending -lp.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	s.is_empty()
}

/// Words ending -mp.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 941);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -op.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 942);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 943);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 945);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['s', 'i', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 947);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2024);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2025);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2026);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2027);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2028);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 948);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 949);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 950);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 951);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 2032);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2885);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2033);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2887);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 954);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pp.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	s.is_empty()
}

/// Words ending -rp.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	s.is_empty()
}

/// Words ending -sp.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	s.is_empty()
}

/// Words ending -tp.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	s.is_empty()
}

/// Words ending -up.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['c', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		['e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 964);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2036);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vp.
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 229);
	s.is_empty()
}

/// Words ending -wp.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['o', 'l', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eer.
pub(crate) fn n1013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 990);
	s.is_empty()
}

/// Words ending -fer.
pub(crate) fn n1014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 991);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2065);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2066);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2067);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2068);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2069);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2070);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2071);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2072);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 2073);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n1015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 992);
	s.is_empty()
}

/// Words ending -her.
pub(crate) fn n1016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 993);
	s.is_empty()
}

/// Words ending -ker.
pub(crate) fn n1017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 994);
	s.is_empty()
}

/// Words ending -mer.
pub(crate) fn n1018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 995);
	s.is_empty()
}

/// Words ending -per.
pub(crate) fn n1019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 996);
	s.is_empty()
}

/// Words ending -ter.
pub(crate) fn n1020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 997);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2074);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2075);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2076);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2904);
					match s {
						[] => true,
						['s', 'i', 'd', s @ ..] => {
							super::push(out, 3268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3269);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3270);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3271);
							s.is_empty()
						}
						['s', 'i', s @ ..] => {
							super::push(out, 3272);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['i', 'd', s @ ..] => {
					super::push(out, 2907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ver.
pub(crate) fn n1021(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2079);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wer.
pub(crate) fn n1022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 998);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ir.
pub(crate) fn n235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 999);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1000);
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 1001);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		['u', 'q', 's', '\u{1}', s @ ..] => {
			super::push(out, 1003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lr.
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	s.is_empty()
}

/// Words ending -or.
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2082);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rr.
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	s.is_empty()
}

/// Words ending -ur.
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['b', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1007);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1008);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1009);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1011);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2088);
					s.is_empty()
				}
				['r', 'u', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1014);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 1016);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vr.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	s.is_empty()
}

/// Words ending -et.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1061);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2126);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2127);
					match s {
						[] => true,
						['h', 'p', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2128);
					s.is_empty()
				}
				['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2129);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2130);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1062);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2132);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2133);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2134);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2136);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 1063);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2139);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2140);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2141);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2142);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2143);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2938);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2939);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2145);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1066);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1068);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2943);
							s.is_empty()
						}
						['m', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2944);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2153);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2946);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2154);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2155);
					match s {
						[] => true,
						['a', 'g', s @ ..] => {
							super::push(out, 2948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2157);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2158);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2950);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1070);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 2160);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2953);
							s.is_empty()
						}
						['t', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2162);
					s.is_empty()
				}
				['o', 'y', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2163);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2955);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2164);
					s.is_empty()
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2165);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2167);
					match s {
						[] => true,
						['u', 'p', 't', s @ ..] => {
							super::push(out, 2957);
							match s {
								[] => true,
								['a', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3282);
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
							super::push(out, 2958);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1073);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2169);
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2960);
							s.is_empty()
						}
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2170);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2173);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2174);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2963);
							s.is_empty()
						}
						['f', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2964);
							s.is_empty()
						}
						['h', 'p', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n1099(s, out),
		['t', s @ ..] => {
			super::push(out, 1075);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1076);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				['q', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'p', '\u{1}', s @ ..] => super::t1::n3089(s, out),
						['i', 'r', 'b', '\u{1}', s @ ..] => super::t1::n3090(s, out),
						['o', s @ ..] => super::t1::n3091(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => super::t1::n1102(s, out),
		['w', s @ ..] => super::t1::n1103(s, out),
		['y', s @ ..] => super::t1::n1104(s, out),
		_ => false,
	}
}

/// Words ending -ight.
pub(crate) fn n2268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2193);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2993);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3288);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2995);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3290);
					match s {
						[] => true,
						['f', 'r', 'e', 'v', s @ ..] => {
							super::push(out, 3379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2996);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3292);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3294);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3295);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3380);
							s.is_empty()
						}
						['o', 'o', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3296);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3297);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3382);
							s.is_empty()
						}
						['s', 'a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 3383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'w', 't', '\u{1}', s @ ..] => {
					super::push(out, 3298);
					s.is_empty()
				}
				['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2999);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3000);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3001);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 3301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3002);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 3003);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3302);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -it.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1083);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2195);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2197);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1085);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1087);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1088);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2200);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3005);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2201);
					match s {
						[] => true,
						['o', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2202);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3007);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3009);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2204);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3012);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3013);
							match s {
								[] => true,
								['l', 'u', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3385);
											s.is_empty()
										}
										['t', 'u', 'o', '\u{1}', s @ ..] => {
											super::push(out, 3386);
											s.is_empty()
										}
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
		['k', s @ ..] => {
			super::push(out, 1091);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2213);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2214);
					match s {
						[] => true,
						['s', 't', s @ ..] => {
							super::push(out, 3015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2217);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2219);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2220);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3018);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3019);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3020);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3021);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 3022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2224);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3023);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 3305);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3307);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 3308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3024);
							s.is_empty()
						}
						['s', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3025);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3027);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3028);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3029);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3030);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3031);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2230);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2233);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2234);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2236);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2237);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2240);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['g', s @ ..] => super::t1::n2318(s, out),
				['h', 't', 'i', 'w', '\u{1}', s @ ..] => super::t1::n2319(s, out),
				['i', s @ ..] => super::t1::n2320(s, out),
				['n', s @ ..] => super::t1::n2321(s, out),
				['o', s @ ..] => super::t1::n2322(s, out),
				['r', s @ ..] => super::t1::n2323(s, out),
				['t', s @ ..] => super::t1::n2324(s, out),
				['y', s @ ..] => super::t1::n2325(s, out),
				_ => false,
			}
		}
		['t', '\u{1}', s @ ..] => super::t1::n1124(s, out),
		['u', s @ ..] => super::t1::n1125(s, out),
		['v', s @ ..] => super::t1::n1126(s, out),
		['w', s @ ..] => super::t1::n1127(s, out),
		['x', s @ ..] => super::t1::n1128(s, out),
		_ => false,
	}
}

/// Words ending -nt.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1105);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1106);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 2261);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1107);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2262);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

