use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 26);
					match s {
						[] => true,
						['o', 's', 'i', 'k', 'a', 'y', '\u{1}', s @ ..] => {
							super::push(out, 112);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 113);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 353);
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
						['c', s @ ..] => {
							super::push(out, 114);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 115);
							match s {
								[] => true,
								['r', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['a', 'd', 'o', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 117);
							s.is_empty()
						}
						['d', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 118);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 119);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 120);
							match s {
								[] => true,
								['a', 'b', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 356);
									s.is_empty()
								}
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 357);
									s.is_empty()
								}
								['o', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'm', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 29);
					match s {
						[] => true,
						['r', 'u', 'a', 'l', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 122);
							s.is_empty()
						}
						['s', 'u', 'a', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 30);
					match s {
						[] => true,
						['f', 'u', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 124);
							s.is_empty()
						}
						['o', 'i', '\u{1}', s @ ..] => {
							super::push(out, 125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 31);
					match s {
						[] => true,
						['e', 'i', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 126);
							s.is_empty()
						}
						['l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 127);
							s.is_empty()
						}
						['n', 'u', 'l', 'a', 'd', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 128);
							s.is_empty()
						}
						['o', 'r', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 32);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 130);
							match s {
								[] => true,
								['c', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 359);
									s.is_empty()
								}
								['t', 'o', 'r', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 131);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 132);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 133);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 361);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 803);
											s.is_empty()
										}
										['c', 'i', 'c', 's', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 804);
											s.is_empty()
										}
										['t', 'n', 'a', 'u', 'g', '\u{1}', s @ ..] => {
											super::push(out, 805);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 134);
							match s {
								[] => true,
								['i', 's', 'l', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 362);
									s.is_empty()
								}
								['l', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n33(s, out),
				['m', s @ ..] => super::t0::n34(s, out),
				['n', s @ ..] => super::t0::n35(s, out),
				['p', 'm', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 36);
					s.is_empty()
				}
				['r', s @ ..] => super::t0::n37(s, out),
				['s', s @ ..] => super::t0::n38(s, out),
				['t', s @ ..] => super::t0::n39(s, out),
				['u', s @ ..] => {
					super::push(out, 40);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 41);
					match s {
						[] => true,
						['i', s @ ..] => super::t0::n167(s, out),
						_ => false,
					}
				}
				['z', s @ ..] => super::t0::n42(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n2(s, out),
		['d', s @ ..] => super::t0::n3(s, out),
		['e', s @ ..] => super::t0::n4(s, out),
		['f', s @ ..] => super::t0::n5(s, out),
		['g', s @ ..] => super::t0::n6(s, out),
		['h', s @ ..] => super::t0::n7(s, out),
		['i', s @ ..] => super::t0::n8(s, out),
		['k', s @ ..] => super::t0::n9(s, out),
		['l', s @ ..] => super::t0::n10(s, out),
		['m', s @ ..] => super::t0::n11(s, out),
		['n', s @ ..] => super::t0::n12(s, out),
		['o', s @ ..] => super::t0::n13(s, out),
		['r', s @ ..] => super::t0::n14(s, out),
		['s', s @ ..] => super::t0::n15(s, out),
		['t', s @ ..] => super::t0::n16(s, out),
		['u', s @ ..] => super::t0::n17(s, out),
		['x', s @ ..] => super::t0::n18(s, out),
		['y', s @ ..] => super::t0::n19(s, out),
		['à', s @ ..] => super::t0::n20(s, out),
		['è', s @ ..] => super::t0::n21(s, out),
		['é', s @ ..] => super::t0::n22(s, out),
		['ì', s @ ..] => super::t0::n23(s, out),
		['ò', s @ ..] => super::t0::n24(s, out),
		['ù', s @ ..] => super::t0::n25(s, out),
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'b', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 136);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['n', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 806);
							s.is_empty()
						}
						['r', 'o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['i', 'a', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 366);
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
		['a', 'l', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 139);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'i', 'f', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
			super::push(out, 141);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'i', 'r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				['o', 'f', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 808);
							s.is_empty()
						}
						['n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
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
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 369);
					s.is_empty()
				}
				['t', 's', 'u', 'd', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['m', 'i', 'h', 'c', 'o', 's', 'i', '\u{1}', s @ ..] => {
					super::push(out, 371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['m', 'm', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 372);
					s.is_empty()
				}
				['r', 'a', 'm', 'o', 'e', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['d', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				['i', 'h', 'c', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 375);
					s.is_empty()
				}
				['r', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 810);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'i', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 149);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 379);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 153);
			s.is_empty()
		}
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'r', 'a', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 380);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 381);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['r', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 382);
					s.is_empty()
				}
				['t', 'i', 'r', 'a', 'm', 'l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', 'n', 'e', 'v', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 385);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 386);
					match s {
						[] => true,
						['a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['l', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 387);
					s.is_empty()
				}
				['r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 389);
					match s {
						[] => true,
						['l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 391);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 815);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1367);
									match s {
										[] => true,
										['r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 1780);
											s.is_empty()
										}
										['t', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1781);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 817);
							match s {
								[] => true,
								['s', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1368);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 818);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1369);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1782);
											match s {
												[] => true,
												['e', 'l', '-', 'a', 't', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 2015);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1370);
									match s {
										[] => true,
										['e', 'v', 'o', 'g', '\u{1}', s @ ..] => {
											super::push(out, 1783);
											s.is_empty()
										}
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
					super::push(out, 393);
					match s {
						[] => true,
						['g', 'a', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 819);
							s.is_empty()
						}
						['p', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'b', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['e', 'v', 'a', 'n', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				['o', 'c', 'a', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'd', 'a', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iva.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	s.is_empty()
}

/// Words ending -za.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', 't', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -c.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 167);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -d.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 44);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 168);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -e.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 169);
					match s {
						[] => true,
						['c', 'i', 'f', 'f', 'e', '\u{1}', s @ ..] => {
							super::push(out, 398);
							s.is_empty()
						}
						['f', '-', 'e', 'l', 'b', 'u', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 399);
							s.is_empty()
						}
						['g', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 400);
							s.is_empty()
						}
						['l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 401);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							super::push(out, 402);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 403);
							s.is_empty()
						}
						['v', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 170);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 822);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 823);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 1371);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1784);
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 1785);
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
										['m', s @ ..] => {
											super::push(out, 1372);
											match s {
												[] => true,
												['e', 's', '\u{1}', s @ ..] => {
													super::push(out, 1786);
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 1787);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 1373);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 405);
							s.is_empty()
						}
						['v', 'r', 'e', 's', '-', 'f', 'l', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 171);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 172);
					match s {
						[] => true,
						['c', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 407);
							s.is_empty()
						}
						['l', 'e', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 824);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 408);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 409);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 826);
									s.is_empty()
								}
								['t', 'a', '\u{1}', s @ ..] => {
									super::push(out, 827);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'a', 'v', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 46);
			match s {
				[] => true,
				['a', 'r', 'g', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 173);
					s.is_empty()
				}
				['e', 's', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 174);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 175);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 176);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 411);
							match s {
								[] => true,
								['a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 828);
									s.is_empty()
								}
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 177);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 178);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'k', 'n', 'a', 'y', '\u{1}', s @ ..] => {
			super::push(out, 47);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 48);
			match s {
				[] => true,
				['n', 'i', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 49);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 180);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 181);
					match s {
						[] => true,
						['l', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 413);
							s.is_empty()
						}
						['u', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 50);
			match s {
				[] => true,
				['c', 'i', 'f', 'r', 'e', 'p', 'u', 's', '-', 'e', 'i', 'c', 'i', 'f', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 182);
					s.is_empty()
				}
				['d', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 183);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 184);
					match s {
						[] => true,
						['a', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 415);
							s.is_empty()
						}
						['e', 's', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n51(s, out),
		['m', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 190);
					match s {
						[] => true,
						['r', 'c', 'a', 'l', 'a', 'p', 'p', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'i', 'f', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 191);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 192);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 193);
					match s {
						[] => true,
						['l', 'o', 'v', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n53(s, out),
		['r', s @ ..] => super::t0::n54(s, out),
		['s', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 209);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 482);
							match s {
								[] => true,
								['a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 934);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['l', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1447);
											s.is_empty()
										}
										['r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1448);
											s.is_empty()
										}
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
					super::push(out, 210);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 483);
							match s {
								[] => true,
								['g', 'r', 'o', s @ ..] => {
									super::push(out, 935);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => super::t0::n1579(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t0::n508(s, out),
						['p', 's', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => super::t0::n509(s, out),
						['t', s @ ..] => super::t0::n510(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n216(s, out),
				['o', s @ ..] => super::t0::n217(s, out),
				['r', 'o', 'b', 'a', 't', 'o', 'v', '\u{1}', s @ ..] => super::t0::n218(s, out),
				['s', s @ ..] => super::t0::n219(s, out),
				['t', '-', 'e', 's', 't', '\u{1}', s @ ..] => super::t0::n220(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t0::n56(s, out),
		['u', s @ ..] => super::t0::n57(s, out),
		['v', s @ ..] => super::t0::n58(s, out),
		['z', s @ ..] => super::t0::n59(s, out),
		['ù', 'i', 'p', '\u{1}', s @ ..] => super::t0::n60(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	s.is_empty()
}

/// Words ending -g.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 62);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['e', 'r', 'ë', 'b', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 232);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['u', 'c', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['t', 'a', 'n', 'e', 't', 'a', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 68);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 69);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 70);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	s.is_empty()
}

/// Words ending -n.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 71);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 72);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 73);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['o', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 238);
					match s {
						[] => true,
						['e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 515);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 516);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 967);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 968);
									match s {
										[] => true,
										['i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 1495);
											s.is_empty()
										}
										['s', 'i', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1496);
											s.is_empty()
										}
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
			super::push(out, 75);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 239);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 517);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 518);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 970);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 1497);
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t0::n2018(s, out),
												['b', 'u', '\u{1}', s @ ..] => super::t0::n2019(s, out),
												['m', 'i', '\u{1}', s @ ..] => super::t0::n2020(s, out),
												['u', '\u{1}', s @ ..] => super::t0::n2021(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => super::t0::n1059(s, out),
								_ => false,
							}
						}
						['m', s @ ..] => super::t0::n544(s, out),
						['p', s @ ..] => super::t0::n545(s, out),
						['r', 'b', 'i', 'r', 't', '\u{1}', s @ ..] => super::t0::n546(s, out),
						['t', 'a', 'm', '\u{1}', s @ ..] => super::t0::n547(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t0::n246(s, out),
				['e', s @ ..] => super::t0::n247(s, out),
				['i', s @ ..] => super::t0::n248(s, out),
				['l', s @ ..] => super::t0::n249(s, out),
				['n', s @ ..] => super::t0::n250(s, out),
				['o', s @ ..] => super::t0::n251(s, out),
				['r', s @ ..] => super::t0::n252(s, out),
				['s', s @ ..] => super::t0::n253(s, out),
				['u', s @ ..] => super::t0::n254(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n76(s, out),
		['e', s @ ..] => super::t0::n77(s, out),
		['f', s @ ..] => super::t0::n78(s, out),
		['g', s @ ..] => super::t0::n79(s, out),
		['h', 'c', 'a', 'm', '\u{1}', s @ ..] => super::t0::n80(s, out),
		['i', s @ ..] => super::t0::n81(s, out),
		['k', s @ ..] => super::t0::n82(s, out),
		['l', s @ ..] => super::t0::n83(s, out),
		['m', s @ ..] => super::t0::n84(s, out),
		['n', s @ ..] => super::t0::n85(s, out),
		['o', s @ ..] => super::t0::n86(s, out),
		['p', s @ ..] => super::t0::n87(s, out),
		['r', s @ ..] => super::t0::n88(s, out),
		['s', s @ ..] => super::t0::n89(s, out),
		['t', s @ ..] => super::t0::n90(s, out),
		['u', s @ ..] => super::t0::n91(s, out),
		['v', s @ ..] => super::t0::n92(s, out),
		['z', s @ ..] => super::t0::n93(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 94);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 99);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 101);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -y.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -à.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -è.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -é.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 106);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ì.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 108);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ò.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	s.is_empty()
}

/// Words ending -ù.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 109);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 110);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 417);
					match s {
						[] => true,
						['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 830);
							s.is_empty()
						}
						['d', 'r', s @ ..] => {
							super::push(out, 831);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									super::push(out, 1374);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1788);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 832);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 1375);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1789);
											match s {
												[] => true,
												['m', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
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
						['z', 'n', 'e', 's', s @ ..] => {
							super::push(out, 833);
							match s {
								[] => true,
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'r', s @ ..] => {
					super::push(out, 418);
					match s {
						[] => true,
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 835);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 836);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1378);
									match s {
										[] => true,
										['i', 'r', 'o', '\u{1}', s @ ..] => {
											super::push(out, 1790);
											s.is_empty()
										}
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1791);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1379);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1380);
									match s {
										[] => true,
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1792);
											s.is_empty()
										}
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
					super::push(out, 419);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							super::push(out, 837);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1381);
									match s {
										[] => true,
										['i', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1793);
											s.is_empty()
										}
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
						['t', 'n', s @ ..] => {
							super::push(out, 838);
							match s {
								[] => true,
								['e', 'c', s @ ..] => {
									super::push(out, 1382);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1794);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 't', 'a', s @ ..] => {
							super::push(out, 839);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 420);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 421);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 840);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1385);
									s.is_empty()
								}
								['t', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1386);
									s.is_empty()
								}
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
			super::push(out, 186);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 422);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 841);
							s.is_empty()
						}
						['u', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'n', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 187);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => super::t0::n439(s, out),
				['c', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						['i', 'f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 425);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1398);
									s.is_empty()
								}
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 426);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 427);
					match s {
						[] => true,
						['n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1400);
									match s {
										[] => true,
										['i', s @ ..] => super::t0::n1951(s, out),
										_ => false,
									}
								}
								['o', s @ ..] => super::t0::n1526(s, out),
								_ => false,
							}
						}
						['u', '\u{1}', s @ ..] => super::t0::n924(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t0::n444(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n193(s, out),
		['o', s @ ..] => super::t0::n194(s, out),
		_ => false,
	}
}

/// Words ending -ne.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 436);
					s.is_empty()
				}
				['v', 'o', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						['b', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'a', 'p', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['d', 'n', 'a', 'r', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 439);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 863);
							s.is_empty()
						}
						['n', 'o', 'c', 'e', 'r', 't', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['g', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 865);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 867);
							s.is_empty()
						}
						['l', 'o', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 868);
							s.is_empty()
						}
						['n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 870);
							s.is_empty()
						}
						['r', 'e', 't', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 'l', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 441);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 872);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', 'g', 's', '\u{1}', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						['r', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 443);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 875);
							match s {
								[] => true,
								['o', 's', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 876);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['t', 'a', 'p', 'm', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 1412);
									s.is_empty()
								}
								['z', 'z', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 877);
							s.is_empty()
						}
						['s', 'a', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 444);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 879);
							s.is_empty()
						}
						['i', 'm', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 880);
							s.is_empty()
						}
						['n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 445);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 446);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 882);
							s.is_empty()
						}
						['b', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 883);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 884);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1414);
									match s {
										[] => true,
										['a', 'c', 'r', 'o', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 1815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', 'c', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 885);
							s.is_empty()
						}
						['n', 'r', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 886);
							s.is_empty()
						}
						['p', 'm', 'a', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1415);
									s.is_empty()
								}
								['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 888);
							match s {
								[] => true,
								['i', 'v', 'o', 'p', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1417);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1816);
											s.is_empty()
										}
										['i', 'm', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1817);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 1418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 889);
							match s {
								[] => true,
								['a', s @ ..] => super::t0::n1546(s, out),
								['n', 'u', 'i', 'g', 'i', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 1419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n469(s, out),
				['m', s @ ..] => super::t0::n470(s, out),
				['n', s @ ..] => super::t0::n471(s, out),
				['p', s @ ..] => {
					super::push(out, 449);
					s.is_empty()
				}
				['r', s @ ..] => super::t0::n473(s, out),
				['s', s @ ..] => super::t0::n474(s, out),
				['t', s @ ..] => super::t0::n475(s, out),
				['z', 'z', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n205(s, out),
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['g', 'l', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 454);
					s.is_empty()
				}
				['i', 'l', 'i', 'm', s @ ..] => {
					super::push(out, 455);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 't', s @ ..] => {
									super::push(out, 1424);
									match s {
										[] => true,
										['r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1826);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 1425);
									match s {
										[] => true,
										['n', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 1827);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'o', s @ ..] => {
									super::push(out, 1426);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 1828);
											s.is_empty()
										}
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
					super::push(out, 457);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 907);
							match s {
								[] => true,
								['d', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1427);
									s.is_empty()
								}
								['r', 't', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'n', 'a', 'z', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['l', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 462);
					match s {
						[] => true,
						['l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['d', 'e', 'v', 'l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				['g', 'g', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 466);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 911);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1429);
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
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 468);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'l', 'o', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 'e', 'v', 'v', 'a', '\u{1}', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 470);
					match s {
						[] => true,
						['i', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 913);
							s.is_empty()
						}
						['o', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 471);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 915);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 472);
					match s {
						[] => true,
						['a', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 916);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 917);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1431);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 1432);
									match s {
										[] => true,
										['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
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
				['m', s @ ..] => {
					super::push(out, 473);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 918);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1433);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 919);
							match s {
								[] => true,
								['r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1434);
									s.is_empty()
								}
								['t', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'g', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				['p', 'a', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 475);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['a', 'v', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 920);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 921);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1436);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 922);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1437);
									match s {
										[] => true,
										['f', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1831);
											s.is_empty()
										}
										['s', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1832);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 924);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1439);
									match s {
										[] => true,
										['r', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1833);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t0::n501(s, out),
				['u', 'c', 'a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				['v', 'a', 't', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 480);
					match s {
						[] => true,
						['e', s @ ..] => super::t0::n1008(s, out),
						['n', 'o', 'm', '\u{1}', s @ ..] => super::t0::n1009(s, out),
						['u', 'l', s @ ..] => super::t0::n1010(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n213(s, out),
		_ => false,
	}
}

/// The word borghese.
pub(crate) fn n1579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1449);
	s.is_empty()
}

/// Words ending -nese.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 484);
	s.is_empty()
}

/// The word tagliaspese.
pub(crate) fn n509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	s.is_empty()
}

/// Words ending -tese.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nse.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	s.is_empty()
}

/// Words ending -ose.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['i', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 938);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word votaborse.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	s.is_empty()
}

/// Words ending -sse.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 488);
			s.is_empty()
		}
		['e', 'c', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 489);
			s.is_empty()
		}
		['o', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word tse-tse.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	s.is_empty()
}

/// Words ending -te.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['-', 'a', 'd', '-', 'i', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 491);
					s.is_empty()
				}
				['r', 'o', 'p', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 492);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 493);
					match s {
						[] => true,
						['a', 'p', 'a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 939);
							s.is_empty()
						}
						['s', 'e', '-', 'a', 'r', 'e', 'v', 'a', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['b', 'a', 'i', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['r', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['l', 'e', 't', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 941);
							s.is_empty()
						}
						['u', 'l', 'l', 'e', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'c', s @ ..] => {
					match s {
						[] => true,
						['-', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 943);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['c', 'c', 'i', s @ ..] => {
							super::push(out, 945);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1451);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1844);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'n', s @ ..] => {
							super::push(out, 946);
							match s {
								[] => true,
								['o', 'b', s @ ..] => {
									super::push(out, 1452);
									match s {
										[] => true,
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1845);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 947);
							match s {
								[] => true,
								['n', 'r', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 1453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'e', s @ ..] => {
							super::push(out, 948);
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 1454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1846);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'l', s @ ..] => {
							super::push(out, 949);
							match s {
								[] => true,
								['g', 'i', 'm', 'o', s @ ..] => {
									super::push(out, 1455);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 1847);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'l', 'i', s @ ..] => {
							super::push(out, 950);
							match s {
								[] => true,
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 952);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1457);
									match s {
										[] => true,
										['a', 'l', s @ ..] => {
											super::push(out, 1848);
											match s {
												[] => true,
												['f', 'e', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 2032);
													s.is_empty()
												}
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
							super::push(out, 953);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1458);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 1849);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'e', 'r', 'e', s @ ..] => {
									super::push(out, 1459);
									match s {
										[] => true,
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 1850);
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
								['r', s @ ..] => {
									super::push(out, 1460);
									match s {
										[] => true,
										['o', 'p', 'm', s @ ..] => {
											super::push(out, 1851);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 2033);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'i', s @ ..] => {
									super::push(out, 1461);
									match s {
										[] => true,
										['d', s @ ..] => super::t0::n1999(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => super::t0::n1034(s, out),
						['z', s @ ..] => super::t0::n1035(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n523(s, out),
				['o', s @ ..] => super::t0::n524(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n226(s, out),
		['s', s @ ..] => super::t0::n227(s, out),
		['t', 'e', 'u', 'l', 'b', '\u{1}', s @ ..] => super::t0::n228(s, out),
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['g', 'n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 964);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1492);
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									super::push(out, 1493);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', 'n', 'u', 'l', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				['o', 's', '\u{1}', s @ ..] => {
					super::push(out, 505);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 507);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['a', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 965);
							s.is_empty()
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				['t', 'o', 'm', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ze.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['n', 'e', 't', 'n', 'e', 's', 'a', 'z', 'z', 'a', 'm', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 229);
			s.is_empty()
		}
		['r', 'o', 'f', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 230);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word piùe.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	s.is_empty()
}

/// The word briaco.
pub(crate) fn n2018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1870);
	s.is_empty()
}

/// The word ubbriaco.
pub(crate) fn n2019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1871);
	s.is_empty()
}

/// The word imbriaco.
pub(crate) fn n2020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1872);
	s.is_empty()
}

/// The word ubriaco.
pub(crate) fn n2021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1873);
	s.is_empty()
}

/// Words ending -tiaco.
pub(crate) fn n1059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 971);
	match s {
		[] => true,
		['o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -maco.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 519);
	match s {
		[] => true,
		['r', 'a', 'f', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -paco.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word tribraco.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 521);
	s.is_empty()
}

/// The word mataco.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 522);
	s.is_empty()
}

/// Words ending -cco.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 524);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['g', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 527);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eco.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 528);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 529);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'l', 'o', '\u{1}', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 533);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ico.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1506);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1874);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 1875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 992);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1876);
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1877);
									s.is_empty()
								}
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
			super::push(out, 537);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 994);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1878);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 1879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['a', 'r', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1509);
							match s {
								[] => true,
								['p', '-', 'o', 'd', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['à', 'r', 'g', 'o', 'l', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['t', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', s @ ..] => {
							super::push(out, 1512);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 1882);
									s.is_empty()
								}
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
			super::push(out, 541);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1513);
							match s {
								[] => true,
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1883);
									s.is_empty()
								}
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
			super::push(out, 542);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1002);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1884);
									s.is_empty()
								}
								['c', 'a', 'r', 'v', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1885);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['s', 'a', s @ ..] => {
					super::push(out, 1004);
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1005);
					match s {
						[] => true,
						['p', 'm', s @ ..] => {
							super::push(out, 1516);
							match s {
								[] => true,
								['i', 's', '\u{1}', s @ ..] => {
									super::push(out, 1887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'c', 'o', s @ ..] => {
							super::push(out, 1517);
							match s {
								[] => true,
								['m', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1889);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1890);
									match s {
										[] => true,
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2045);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'm', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1519);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['l', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 2046);
											s.is_empty()
										}
										['n', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2047);
											s.is_empty()
										}
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

/// Words ending -lco.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 546);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1009);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1520);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1891);
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 1892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nco.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1014);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oco.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 551);
			s.is_empty()
		}
		['p', 'p', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 552);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'z', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 556);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rco.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1522);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sco.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1022);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 562);
			match s {
				[] => true,
				['d', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 563);
			s.is_empty()
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 564);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uco.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1024);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 566);
			s.is_empty()
		}
		['i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -do.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['e', 'r', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1026);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1027);
							match s {
								[] => true,
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1525);
									s.is_empty()
								}
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
			super::push(out, 251);
			match s {
				[] => true,
				['b', 'r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1028);
							match s {
								[] => true,
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1029);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				['g', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1031);
							s.is_empty()
						}
						['l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1032);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1034);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1035);
							s.is_empty()
						}
						['m', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1037);
							s.is_empty()
						}
						['q', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1039);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1040);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1529);
									s.is_empty()
								}
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'm', 's', '\u{1}', s @ ..] => {
							super::push(out, 1043);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1045);
							match s {
								[] => true,
								['o', 'c', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'r', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['e', 'v', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1047);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1532);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1048);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1533);
									s.is_empty()
								}
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
			super::push(out, 254);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['i', 's', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 1049);
							s.is_empty()
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1050);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1051);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1534);
									match s {
										[] => true,
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1893);
											s.is_empty()
										}
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
					super::push(out, 579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							super::push(out, 1052);
							match s {
								[] => true,
								['g', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 581);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1053);
							match s {
								[] => true,
								['i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1536);
									s.is_empty()
								}
								['o', 'r', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'c', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['s', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 256);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eo.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['c', 'u', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 259);
			s.is_empty()
		}
		['r', 'e', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fo.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 587);
					match s {
						[] => true,
						['i', 'n', 'i', 'l', 'l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1058);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1059);
							match s {
								[] => true,
								['l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 1538);
									s.is_empty()
								}
								['o', 'z', '\u{1}', s @ ..] => {
									super::push(out, 1539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1060);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['d', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 1894);
											s.is_empty()
										}
										['t', 't', 'e', 's', '\u{1}', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1061);
							s.is_empty()
						}
						['e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1063);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['i', 'v', 'a', 'n', 'o', 'e', '\u{1}', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				['p', 'o', 't', 'a', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['a', 'f', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 601);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1540);
									s.is_empty()
								}
								['o', 'c', 'c', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1541);
									s.is_empty()
								}
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
			super::push(out, 267);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['r', 'b', 'o', 'l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word macho.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	s.is_empty()
}

/// Words ending -io.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 270);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1067);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 1542);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1069);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1543);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 1897);
											match s {
												[] => true,
												['g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 2048);
													s.is_empty()
												}
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
							super::push(out, 1070);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1071);
							match s {
								[] => true,
								['t', 'n', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['f', 'i', 't', 'n', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1072);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1073);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1545);
									s.is_empty()
								}
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1074);
							match s {
								[] => true,
								['b', 's', '\u{1}', s @ ..] => {
									super::push(out, 1547);
									s.is_empty()
								}
								['f', 'i', 't', 'n', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1075);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1549);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
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
				['o', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1076);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1077);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1899);
											s.is_empty()
										}
										['e', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1900);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1078);
							match s {
								[] => true,
								['g', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 615);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1079);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1551);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['d', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1080);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1082);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1552);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1901);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1083);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1084);
							s.is_empty()
						}
						['i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['a', 'r', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1088);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1554);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1555);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 1902);
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
								['h', 'c', 'c', 'a', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1556);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1557);
									s.is_empty()
								}
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1558);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1090);
							s.is_empty()
						}
						['d', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1091);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['i', s @ ..] => super::t0::n1184(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n658(s, out),
				['u', 't', 't', 'a', 'm', '\u{1}', s @ ..] => super::t0::n659(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n283(s, out),
		['l', s @ ..] => super::t0::n284(s, out),
		['m', s @ ..] => super::t0::n285(s, out),
		['n', s @ ..] => super::t0::n286(s, out),
		['o', s @ ..] => super::t0::n287(s, out),
		['p', s @ ..] => super::t0::n288(s, out),
		['r', s @ ..] => super::t0::n289(s, out),
		['s', s @ ..] => super::t0::n290(s, out),
		['t', s @ ..] => super::t0::n291(s, out),
		['v', s @ ..] => super::t0::n292(s, out),
		['z', s @ ..] => super::t0::n293(s, out),
		_ => false,
	}
}

/// Words ending -ko.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	s.is_empty()
}

/// Words ending -lo.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['e', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 288);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['i', 'g', s @ ..] => {
							super::push(out, 1116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1570);
									s.is_empty()
								}
								['o', 'c', 'c', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['t', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['u', 'q', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['b', 'o', 'c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1119);
							s.is_empty()
						}
						['r', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1121);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['a', 'v', s @ ..] => {
							super::push(out, 1123);
							match s {
								[] => true,
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'c', 'u', 'r', 'd', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1124);
							s.is_empty()
						}
						['g', 'e', 'i', 'l', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', 's', 'e', 'l', 'o', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['t', 'a', 'c', 'o', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['c', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'a', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['é', 'm', '\u{1}', s @ ..] => {
			super::push(out, 293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1129);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1130);
							match s {
								[] => true,
								['i', 'n', s @ ..] => {
									super::push(out, 1575);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 1916);
											s.is_empty()
										}
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
					super::push(out, 658);
					match s {
						[] => true,
						['l', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1131);
							s.is_empty()
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
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'u', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1132);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['l', 'a', 'd', 'n', 'a', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						['r', 'o', 'r', 'r', 'e', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1135);
							s.is_empty()
						}
						['v', 'i', 's', 'u', 'b', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1136);
							s.is_empty()
						}
						['x', 'r', 'a', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 661);
					match s {
						[] => true,
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1138);
							s.is_empty()
						}
						['o', 'r', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1140);
							match s {
								[] => true,
								['o', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1578);
									match s {
										[] => true,
										['t', 'i', '\u{1}', s @ ..] => {
											super::push(out, 1917);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'a', 't', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1141);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1142);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1580);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1581);
									match s {
										[] => true,
										['r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1918);
											s.is_empty()
										}
										['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1143);
							match s {
								[] => true,
								['e', 'f', 'i', 'c', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', s @ ..] => {
							super::push(out, 1144);
							match s {
								[] => true,
								['i', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1145);
							match s {
								[] => true,
								['i', 't', '\u{1}', s @ ..] => {
									super::push(out, 1584);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1585);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 1920);
											s.is_empty()
										}
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
					super::push(out, 665);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1146);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1586);
									match s {
										[] => true,
										['o', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 1921);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 1147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['a', 'r', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['e', 'f', 'f', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1149);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1150);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1152);
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
				['g', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1154);
							match s {
								[] => true,
								['b', 'o', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'm', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 1155);
							s.is_empty()
						}
						['r', 'e', 't', 'o', 't', 't', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 1156);
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
			match s {
				[] => true,
				['a', 'b', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 't', s @ ..] => {
					super::push(out, 677);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1158);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 1590);
									match s {
										[] => true,
										['i', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 1922);
											s.is_empty()
										}
										['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1923);
											s.is_empty()
										}
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
			super::push(out, 302);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 1159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 680);
					match s {
						[] => true,
						['a', 'l', 'c', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 681);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'e', 'd', 'n', 'a', 'l', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 303);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['p', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'm', 'i', 'c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['t', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => super::t0::n1261(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n727(s, out),
				['o', s @ ..] => super::t0::n728(s, out),
				_ => false,
			}
		}
		['u', 'l', 'a', 'u', 'q', '\u{1}', s @ ..] => super::t0::n314(s, out),
		_ => false,
	}
}

/// Words ending -oo.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['d', 'o', 'o', 'v', '\u{1}', s @ ..] => {
			super::push(out, 307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 308);
			s.is_empty()
		}
		['i', 't', '\u{1}', s @ ..] => {
			super::push(out, 309);
			s.is_empty()
		}
		['l', 'o', 'c', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 310);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['c', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 'i', 'n', 'g', 'o', '\u{1}', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'i', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['c', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1174);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1595);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1924);
											s.is_empty()
										}
										['o', 'p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1925);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				['v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['n', 'a', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'i', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1176);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 1596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1177);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1597);
									match s {
										[] => true,
										['i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 1926);
											s.is_empty()
										}
										['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1927);
											s.is_empty()
										}
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
					super::push(out, 706);
					s.is_empty()
				}
				['g', 'g', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 1178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1180);
							match s {
								[] => true,
								['n', 'n', 'a', 'r', 'p', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1181);
							s.is_empty()
						}
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 1182);
							s.is_empty()
						}
						['i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['o', 'i', 'c', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 712);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1186);
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 1187);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1189);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1190);
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'f', 'f', 'a', 'z', '\u{1}', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 718);
					s.is_empty()
				}
				['e', 'd', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				['u', 't', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['a', 'e', 't', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 724);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1192);
							s.is_empty()
						}
						['v', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1194);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1600);
									match s {
										[] => true,
										['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1928);
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
								['p', s @ ..] => {
									super::push(out, 1601);
									match s {
										[] => true,
										['a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1929);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'q', 'e', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', 'r', '\u{1}', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1195);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1930);
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 1931);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1196);
							match s {
								[] => true,
								['o', 'p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 728);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['o', 'd', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1198);
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
				['c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1199);
							match s {
								[] => true,
								['m', 'r', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1604);
									s.is_empty()
								}
								['t', 't', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', 'r', 'v', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', 'r', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1201);
							s.is_empty()
						}
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1202);
							s.is_empty()
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'c', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1204);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				['u', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1206);
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1208);
							match s {
								[] => true,
								['l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'i', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'i', 'n', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1211);
							s.is_empty()
						}
						['d', 'i', 't', 's', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1212);
							s.is_empty()
						}
						['g', 'g', 'a', 't', s @ ..] => {
							super::push(out, 1213);
							match s {
								[] => true,
								['n', 'a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', s @ ..] => {
							super::push(out, 1214);
							match s {
								[] => true,
								['i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'g', 'o', 'g', s @ ..] => {
							super::push(out, 1215);
							match s {
								[] => true,
								['r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1216);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 1610);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1217);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1611);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1612);
									match s {
										[] => true,
										['b', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 1932);
											s.is_empty()
										}
										['l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 1933);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1218);
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1613);
									s.is_empty()
								}
								['e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1614);
									s.is_empty()
								}
								['i', 'b', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1615);
									s.is_empty()
								}
								['n', 'e', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 1616);
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
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1219);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1220);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'r', s @ ..] => {
									super::push(out, 1617);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1934);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 1618);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1935);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1619);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1621);
									s.is_empty()
								}
								['m', 'u', 'l', s @ ..] => super::t0::n1759(s, out),
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 1623);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'd', '\u{1}', s @ ..] => super::t0::n1319(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n785(s, out),
				['s', 's', 'a', 'p', 's', '\u{1}', s @ ..] => super::t0::n786(s, out),
				['t', s @ ..] => super::t0::n787(s, out),
				['u', s @ ..] => super::t0::n788(s, out),
				['v', 'r', s @ ..] => super::t0::n789(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n340(s, out),
		['s', s @ ..] => super::t0::n341(s, out),
		['u', s @ ..] => super::t0::n342(s, out),
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 751);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									super::push(out, 1643);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 1946);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t0::n1347(s, out),
						['r', s @ ..] => super::t0::n1348(s, out),
						['u', 'd', s @ ..] => super::t0::n1349(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t0::n802(s, out),
				['e', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['g', s @ ..] => super::t0::n804(s, out),
				['i', s @ ..] => super::t0::n805(s, out),
				['l', s @ ..] => super::t0::n806(s, out),
				['m', s @ ..] => super::t0::n807(s, out),
				['n', s @ ..] => super::t0::n808(s, out),
				['p', s @ ..] => super::t0::n809(s, out),
				['r', s @ ..] => super::t0::n810(s, out),
				['s', s @ ..] => super::t0::n811(s, out),
				['t', s @ ..] => super::t0::n812(s, out),
				['u', s @ ..] => super::t0::n813(s, out),
				['v', s @ ..] => super::t0::n814(s, out),
				['z', s @ ..] => super::t0::n815(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n344(s, out),
		['i', s @ ..] => super::t0::n345(s, out),
		['l', s @ ..] => super::t0::n346(s, out),
		['n', s @ ..] => super::t0::n347(s, out),
		['o', s @ ..] => super::t0::n348(s, out),
		['r', s @ ..] => super::t0::n349(s, out),
		['s', s @ ..] => super::t0::n350(s, out),
		['t', s @ ..] => super::t0::n351(s, out),
		['u', s @ ..] => super::t0::n352(s, out),
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 797);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['s', 'u', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['l', 'c', 's', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 1775);
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
						['a', 'g', s @ ..] => {
							super::push(out, 1362);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									super::push(out, 1776);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2014);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1363);
							match s {
								[] => true,
								['o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 1777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 1364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1778);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 1779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 801);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 347);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['e', 'p', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 802);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bile.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1387);
					s.is_empty()
				}
				['r', 'u', s @ ..] => {
					super::push(out, 1388);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1389);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1796);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2018);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2019);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 2074);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2080);
													s.is_empty()
												}
												['i', 'r', '\u{1}', s @ ..] => {
													super::push(out, 2081);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						['t', 'e', s @ ..] => {
							super::push(out, 1798);
							match s {
								[] => true,
								['p', 's', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2020);
									s.is_empty()
								}
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
			super::push(out, 844);
			match s {
				[] => true,
				['d', 'o', s @ ..] => {
					super::push(out, 1390);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1391);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1800);
							match s {
								[] => true,
								['e', 'm', 'm', 'o', 's', 'i', s @ ..] => {
									super::push(out, 2021);
									match s {
										[] => true,
										['t', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'e', s @ ..] => {
					super::push(out, 1392);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', s @ ..] => {
					super::push(out, 1393);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 1802);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1394);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['r', 'p', 'm', s @ ..] => {
									super::push(out, 2023);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 2076);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'o', s @ ..] => {
							super::push(out, 1803);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 2025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'u', s @ ..] => {
					super::push(out, 1395);
					match s {
						[] => true,
						['c', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1396);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 1397);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iettile.
pub(crate) fn n1951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1805);
	match s {
		[] => true,
		['o', 'r', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ottile.
pub(crate) fn n1526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1401);
	match s {
		[] => true,
		['s', '\u{1}', s @ ..] => {
			super::push(out, 1806);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word utile.
pub(crate) fn n924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 851);
	s.is_empty()
}

/// Words ending -vile.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1402);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lle.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['a', 'p', 's', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['e', 'p', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['i', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ole.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				['o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 858);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 1403);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'o', s @ ..] => {
							super::push(out, 1404);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 1808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'e', s @ ..] => {
							super::push(out, 1405);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1809);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1406);
							s.is_empty()
						}
						['n', 'o', 'i', 'g', s @ ..] => {
							super::push(out, 1407);
							match s {
								[] => true,
								['a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'l', s @ ..] => {
							super::push(out, 1408);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1409);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', 'i', 'c', 'e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1812);
									s.is_empty()
								}
								['o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1813);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1814);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 2027);
											match s {
												[] => true,
												['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 2077);
													s.is_empty()
												}
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
				['l', 'a', 'v', 'o', 'r', 't', 't', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -azione.
pub(crate) fn n1546(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', 'f', 'a', 'r', 't', 'n', 'o', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1818);
			s.is_empty()
		}
		['i', 'd', 'a', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1819);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', 'c', 'c', 'a', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2028);
					s.is_empty()
				}
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', s @ ..] => {
			super::push(out, 1820);
			s.is_empty()
		}
		['z', 'z', 'i', 'l', 'i', 'v', 'i', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1821);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lone.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 890);
			s.is_empty()
		}
		['i', 'c', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 891);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1822);
							s.is_empty()
						}
						['s', 's', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'e', 'h', 'c', 'o', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'n', 'n', 'e', 'm', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				['i', 'c', 'c', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mone.
pub(crate) fn n470(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 893);
			s.is_empty()
		}
		['l', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 894);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -none.
pub(crate) fn n471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 895);
			s.is_empty()
		}
		['n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rone.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['a', 'i', 'c', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 897);
			s.is_empty()
		}
		['e', 'i', 'h', 'c', 'c', 'a', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 898);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 899);
			s.is_empty()
		}
		['t', 'l', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sone.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['o', 't', 'i', 'r', 'i', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 901);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 902);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tone.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'a', 'b', 'i', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		['t', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -une.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tore.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 925);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1834);
							match s {
								[] => true,
								['i', 'g', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1835);
							match s {
								[] => true,
								['o', 't', 'n', 'a', 'p', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 926);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1836);
							s.is_empty()
						}
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						['o', 'l', 'u', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['a', 'e', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['r', 'i', 'r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 's', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['a', 'r', 't', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['a', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -estre.
pub(crate) fn n1008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 930);
	s.is_empty()
}

/// The word monstre.
pub(crate) fn n1009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 931);
	s.is_empty()
}

/// Words ending -lustre.
pub(crate) fn n1010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 932);
	match s {
		[] => true,
		['l', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1446);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ure.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -distante.
pub(crate) fn n1999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1852);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vante.
pub(crate) fn n1034(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1462);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 1853);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
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

/// Words ending -zante.
pub(crate) fn n1035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	s.is_empty()
}

/// Words ending -ente.
pub(crate) fn n523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1464);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1465);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1855);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1856);
							match s {
								[] => true,
								['n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'e', s @ ..] => {
							super::push(out, 1857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1468);
					s.is_empty()
				}
				['u', 'r', s @ ..] => {
					super::push(out, 1469);
					match s {
						[] => true,
						['p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1470);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1859);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2039);
									s.is_empty()
								}
								['l', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'f', 'f', 'e', s @ ..] => {
					super::push(out, 1473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'g', s @ ..] => {
					super::push(out, 1474);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1862);
							match s {
								[] => true,
								['c', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2041);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				['l', 'e', 'c', 'c', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				['o', 'd', s @ ..] => {
					super::push(out, 1477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'e', s @ ..] => {
			super::push(out, 956);
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1480);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 2042);
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2043);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1481);
					match s {
						[] => true,
						['p', 'm', 'o', 'c', 'i', s @ ..] => {
							super::push(out, 1864);
							match s {
								[] => true,
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2044);
									s.is_empty()
								}
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
				['e', 'd', s @ ..] => {
					super::push(out, 1482);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1483);
					match s {
						[] => true,
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1484);
					match s {
						[] => true,
						['o', 'c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'e', 'r', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					super::push(out, 1486);
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['e', 'v', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'q', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['o', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'r', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onte.
pub(crate) fn n524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['r', 'f', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rte.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				['p', 'o', 'r', 't', 't', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ste.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 501);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word bluette.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	s.is_empty()
}

/// Words ending -ingio.
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				['u', 't', '\u{1}', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', 'p', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ogio.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	s.is_empty()
}

/// The word mattugio.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	s.is_empty()
}

/// Words ending -hio.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 'c', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1094);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1095);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1561);
							match s {
								[] => true,
								['r', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1905);
									s.is_empty()
								}
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

/// Words ending -lio.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				['t', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mio.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	s.is_empty()
}

/// Words ending -nio.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['a', 'f', 'o', 't', 's', 'i', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['m', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 631);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1102);
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
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

/// Words ending -oio.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	s.is_empty()
}

/// Words ending -pio.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 632);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', 'c', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 634);
			s.is_empty()
		}
		['s', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rio.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1562);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2049);
											s.is_empty()
										}
										['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2050);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1563);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'e', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1564);
							match s {
								[] => true,
								['r', 'p', 'o', 'r', 'p', 'i', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 1907);
									s.is_empty()
								}
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
			super::push(out, 638);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['i', 'r', 'c', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1566);
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
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'o', s @ ..] => {
									super::push(out, 1908);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 2051);
											s.is_empty()
										}
										['v', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2052);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'n', s @ ..] => {
									super::push(out, 1909);
									match s {
										[] => true,
										['a', 'm', 'o', 'c', 'c', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2053);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'i', 't', 't', 'i', 'g', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1910);
									s.is_empty()
								}
								['n', 'o', 'i', 'h', 'c', 'n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1911);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1912);
									match s {
										[] => true,
										['r', 'u', 's', 'u', '\u{1}', s @ ..] => {
											super::push(out, 2054);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1567);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 1914);
									s.is_empty()
								}
								['u', 'd', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1915);
									s.is_empty()
								}
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

/// Words ending -sio.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['n', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['r', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tio.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 641);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['p', 'l', 'a', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1568);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 1569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vio.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zio.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['p', 's', 'a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 645);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word scarno.
pub(crate) fn n1261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1167);
	s.is_empty()
}

/// Words ending -erno.
pub(crate) fn n727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['d', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orno.
pub(crate) fn n728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 1169);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1593);
					s.is_empty()
				}
				['t', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word qualuno.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	s.is_empty()
}

/// Words ending -luminoso.
pub(crate) fn n1759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1622);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1936);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word dannoso.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1221);
	s.is_empty()
}

/// Words ending -roso.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['m', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1938);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 2055);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
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
					super::push(out, 1627);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1939);
							s.is_empty()
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word spassoso.
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	s.is_empty()
}

/// Words ending -toso.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1222);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1628);
					match s {
						[] => true,
						['g', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['i', 'c', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'o', s @ ..] => {
			super::push(out, 1223);
			match s {
				[] => true,
				['c', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'o', 'c', s @ ..] => {
			super::push(out, 1224);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', 'f', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1225);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uoso.
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 741);
	match s {
		[] => true,
		['s', 's', 'u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 1226);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['n', 'o', 's', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				['t', 'e', 'f', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rvoso.
pub(crate) fn n789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	match s {
		[] => true,
		['e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1227);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rso.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		['e', 'v', 'i', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sso.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1230);
					match s {
						[] => true,
						['s', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
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
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1232);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1638);
							match s {
								[] => true,
								['a', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'f', 'i', s @ ..] => {
					super::push(out, 1233);
					match s {
						[] => true,
						['r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1641);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 1945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1236);
					match s {
						[] => true,
						['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				['o', 'c', 'c', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'l', 'a', 'r', 't', 'x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uso.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['f', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 749);
			s.is_empty()
		}
		['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -icato.
pub(crate) fn n1347(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1645);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1647);
			s.is_empty()
		}
		['t', 's', s @ ..] => {
			super::push(out, 1648);
			match s {
				[] => true,
				['i', 'f', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rcato.
pub(crate) fn n1348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1243);
	match s {
		[] => true,
		['a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1649);
			s.is_empty()
		}
		['e', 'c', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 1650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ducato.
pub(crate) fn n1349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1244);
	match s {
		[] => true,
		['e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dato.
pub(crate) fn n802(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', s @ ..] => {
			super::push(out, 1245);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'l', s @ ..] => {
			super::push(out, 1246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gato.
pub(crate) fn n804(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1656);
					match s {
						[] => true,
						['r', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', s @ ..] => {
			super::push(out, 1247);
			match s {
				[] => true,
				['b', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1658);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iato.
pub(crate) fn n805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 753);
	match s {
		[] => true,
		['b', 'b', s @ ..] => {
			super::push(out, 1248);
			match s {
				[] => true,
				['a', 'r', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1249);
			match s {
				[] => true,
				['n', 'a', 'l', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1250);
			match s {
				[] => true,
				['i', 'v', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1251);
			match s {
				[] => true,
				['a', 'z', 'z', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'p', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'r', s @ ..] => {
					super::push(out, 1665);
					match s {
						[] => true,
						['a', 'l', 'o', 'c', 'i', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1252);
			s.is_empty()
		}
		['l', 'g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1955);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 't', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', s @ ..] => {
					super::push(out, 1666);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1957);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2078);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 2079);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'b', 'r', 'a', 'g', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1253);
			match s {
				[] => true,
				['e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', s @ ..] => {
			super::push(out, 1254);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1958);
							s.is_empty()
						}
						['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 1959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'a', 'v', 's', '\u{1}', s @ ..] => {
			super::push(out, 1255);
			s.is_empty()
		}
		['v', 'v', s @ ..] => {
			super::push(out, 1256);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'g', 's', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1672);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lato.
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 's', s @ ..] => {
			super::push(out, 1257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'f', s @ ..] => {
			super::push(out, 1258);
			match s {
				[] => true,
				['f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', 's', '\u{1}', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1677);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1961);
							s.is_empty()
						}
						['f', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 't', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1678);
					s.is_empty()
				}
				['p', 'o', s @ ..] => {
					super::push(out, 1679);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1964);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 1965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mato.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 1259);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1681);
					match s {
						[] => true,
						['l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nato.
pub(crate) fn n808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 754);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1260);
			s.is_empty()
		}
		['e', 'l', 'e', 'v', 'v', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1261);
			s.is_empty()
		}
		['g', 'e', 'p', 'm', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1262);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', 'f', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', s @ ..] => {
					super::push(out, 1684);
					match s {
						[] => true,
						['l', 'l', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'a', 'd', s @ ..] => {
			super::push(out, 1263);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1972);
							match s {
								[] => true,
								['a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 2058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'e', s @ ..] => {
							super::push(out, 1973);
							match s {
								[] => true,
								['f', 'f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2059);
									s.is_empty()
								}
								['l', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1689);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1974);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'o', 'i', 'g', 'g', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1264);
			s.is_empty()
		}
		['u', 't', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pato.
pub(crate) fn n809(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', 'u', 'l', s @ ..] => {
			super::push(out, 1266);
			match s {
				[] => true,
				['i', 'v', 's', '\u{1}', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1267);
			match s {
				[] => true,
				['c', 'c', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						['e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rato.
pub(crate) fn n810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 755);
	match s {
		[] => true,
		['a', 'p', 'e', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1268);
			s.is_empty()
		}
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1692);
					s.is_empty()
				}
				['i', 'l', 'i', 'u', 'q', 'e', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1978);
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
				['d', 'i', 's', 'e', s @ ..] => {
					super::push(out, 1694);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', s @ ..] => {
					super::push(out, 1695);
					match s {
						[] => true,
						['e', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', 'm', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1696);
					s.is_empty()
				}
				['p', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 1269);
			s.is_empty()
		}
		['i', 'p', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1698);
					s.is_empty()
				}
				['s', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					super::push(out, 1701);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1981);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', s @ ..] => {
					super::push(out, 1702);
					match s {
						[] => true,
						['n', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', s @ ..] => {
					super::push(out, 1703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 1271);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['n', 'e', 'c', 'n', s @ ..] => {
					super::push(out, 1704);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 1985);
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1986);
							match s {
								[] => true,
								['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2064);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sato.
pub(crate) fn n811(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'e', 's', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', 'r', 'e', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['t', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tato.
pub(crate) fn n812(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'i', 'f', 'f', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1275);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1276);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1277);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						['c', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				['m', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1993);
							s.is_empty()
						}
						['r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1994);
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
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'i', 'r', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1995);
							s.is_empty()
						}
						['u', 'q', 'e', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', 't', 'n', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uato.
pub(crate) fn n813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 756);
	s.is_empty()
}

/// Words ending -vato.
pub(crate) fn n814(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'l', 'e', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1281);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zato.
pub(crate) fn n815(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1282);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1997);
							s.is_empty()
						}
						['v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1283);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1999);
							match s {
								[] => true,
								['a', 'b', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2065);
									s.is_empty()
								}
								['r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 2066);
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
						['r', 'p', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eto.
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['a', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 758);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1719);
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ito.
pub(crate) fn n345(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'm', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'e', s @ ..] => {
			super::push(out, 761);
			match s {
				[] => true,
				['l', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 762);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1288);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							super::push(out, 1721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2001);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1722);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 2003);
									match s {
										[] => true,
										['i', 'r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 2067);
											s.is_empty()
										}
										['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 2068);
											s.is_empty()
										}
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
					super::push(out, 1290);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1291);
					s.is_empty()
				}
				['u', 'a', 'l', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 763);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1293);
					match s {
						[] => true,
						['a', 'u', 'g', 's', '\u{1}', s @ ..] => {
							super::push(out, 1723);
							s.is_empty()
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 1294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 764);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				['o', 'v', s @ ..] => {
					super::push(out, 1296);
					match s {
						[] => true,
						['a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['r', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					super::push(out, 1298);
					match s {
						[] => true,
						['o', 'r', 'p', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 766);
			s.is_empty()
		}
		['u', 'g', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lto.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1300);
					s.is_empty()
				}
				['b', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'v', 's', '\u{1}', s @ ..] => {
			super::push(out, 769);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nto.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', 'p', 'm', 'i', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1306);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1728);
							s.is_empty()
						}
						['e', 'p', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1729);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['g', 'r', 'a', 'o', 'r', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1308);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1732);
							s.is_empty()
						}
						['o', 'i', s @ ..] => {
							super::push(out, 1733);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 2004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1310);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1735);
							match s {
								[] => true,
								['i', 'h', 'c', 'c', 'e', 'v', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2005);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'g', 's', '\u{1}', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1311);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1737);
							s.is_empty()
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1312);
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 771);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 773);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1315);
					match s {
						[] => true,
						['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oto.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 774);
			match s {
				[] => true,
				['e', 'r', 'r', 'e', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1316);
					s.is_empty()
				}
				['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 775);
			s.is_empty()
		}
		['u', 'v', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 776);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1318);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rto.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['u', 'q', 'o', s @ ..] => {
					super::push(out, 1319);
					match s {
						[] => true,
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1744);
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sto.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 781);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['d', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1324);
					s.is_empty()
				}
				['i', 'h', 'c', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1325);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1327);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 783);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['c', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1329);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1746);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2007);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 2069);
											s.is_empty()
										}
										['o', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2070);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 2071);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1750);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1330);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['f', 'a', 'i', 'c', 'c', 'a', 'r', 'b', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1332);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1333);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2009);
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 2010);
									s.is_empty()
								}
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

/// Words ending -tto.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 's', 'i', 'd', 'd', s @ ..] => {
					super::push(out, 1335);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 1753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1336);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1337);
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 's', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1339);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 1756);
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
				['d', 'e', 'l', s @ ..] => {
					super::push(out, 1340);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'r', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1342);
					match s {
						[] => true,
						['c', 'c', 'o', 'b', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 1759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1345);
					match s {
						[] => true,
						['m', 'i', 'r', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1762);
							match s {
								[] => true,
								['i', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2011);
									s.is_empty()
								}
								['n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 2012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1346);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						['r', 'o', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1765);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 2013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2072);
											s.is_empty()
										}
										['i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2073);
											s.is_empty()
										}
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
			super::push(out, 785);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1767);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1350);
					s.is_empty()
				}
				['r', 'r', 'o', s @ ..] => {
					super::push(out, 1351);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1352);
					match s {
						[] => true,
						['a', 'i', 'c', 's', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1353);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1354);
					match s {
						[] => true,
						['a', s @ ..] => {
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

/// Words ending -uto.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 788);
			s.is_empty()
		}
		['d', 'n', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'c', 's', 'o', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 790);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 791);
			s.is_empty()
		}
		['m', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 792);
			s.is_empty()
		}
		['n', 'i', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['s', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1359);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

