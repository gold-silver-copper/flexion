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
					super::push(out, 31);
					match s {
						[] => true,
						['a', 'l', 'o', 't', 'n', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 174);
							match s {
								[] => true,
								['l', 'b', 'a', 't', 'n', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 175);
							match s {
								[] => true,
								['o', 'm', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'y', s @ ..] => {
							super::push(out, 176);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 506);
									s.is_empty()
								}
								['e', 't', 'n', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 507);
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
						['i', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 508);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 509);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 510);
									match s {
										[] => true,
										['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 822);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 'r', 'e', 'm', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 177);
							s.is_empty()
						}
						['o', 'm', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 178);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 179);
							match s {
								[] => true,
								['o', 'b', 'a', 'r', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 32);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 33);
					match s {
						[] => true,
						['a', 'd', 'n', 'o', 'n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 181);
							s.is_empty()
						}
						['o', 't', 's', 'e', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 182);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 183);
							s.is_empty()
						}
						['u', 'y', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 34);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 513);
									s.is_empty()
								}
								['e', 's', 'o', 'c', 's', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 35);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 515);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['i', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 823);
											s.is_empty()
										}
										['r', 'e', 'f', 'n', 'o', 'c', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 824);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 825);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 826);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 's', 'a', 't', 'a', 'p', '\u{1}', s @ ..] => super::t0::n553(s, out),
								_ => false,
							}
						}
						['d', 'e', 'm', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => super::t0::n200(s, out),
						['h', 's', 'u', 'f', '\u{1}', s @ ..] => super::t0::n201(s, out),
						['j', 'u', 't', '\u{1}', s @ ..] => super::t0::n202(s, out),
						['p', s @ ..] => super::t0::n203(s, out),
						['r', s @ ..] => super::t0::n204(s, out),
						['s', 'c', 'u', 'f', '\u{1}', s @ ..] => super::t0::n205(s, out),
						['t', 's', 'e', 'b', s @ ..] => super::t0::n206(s, out),
						_ => false,
					}
				}
				['j', s @ ..] => super::t0::n38(s, out),
				['l', s @ ..] => super::t0::n39(s, out),
				['m', s @ ..] => super::t0::n40(s, out),
				['n', s @ ..] => super::t0::n41(s, out),
				['p', s @ ..] => super::t0::n42(s, out),
				['r', s @ ..] => super::t0::n43(s, out),
				['s', s @ ..] => super::t0::n44(s, out),
				['t', s @ ..] => super::t0::n45(s, out),
				['v', s @ ..] => super::t0::n46(s, out),
				['z', s @ ..] => super::t0::n47(s, out),
				['í', s @ ..] => super::t0::n48(s, out),
				_ => false,
			}
		}
		['b', 'u', 'l', 'c', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => super::t0::n2(s, out),
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
		['q', 'z', 'i', '\u{1}', s @ ..] => super::t0::n17(s, out),
		['r', s @ ..] => super::t0::n18(s, out),
		['s', s @ ..] => super::t0::n19(s, out),
		['t', s @ ..] => super::t0::n20(s, out),
		['u', s @ ..] => super::t0::n21(s, out),
		['v', 'l', 'd', '\u{1}', s @ ..] => super::t0::n22(s, out),
		['x', s @ ..] => super::t0::n23(s, out),
		['y', s @ ..] => super::t0::n24(s, out),
		['z', s @ ..] => super::t0::n25(s, out),
		['á', s @ ..] => super::t0::n26(s, out),
		['é', s @ ..] => super::t0::n27(s, out),
		['í', s @ ..] => super::t0::n28(s, out),
		['ó', s @ ..] => super::t0::n29(s, out),
		['ú', s @ ..] => super::t0::n30(s, out),
		_ => false,
	}
}

/// The word patasucia.
pub(crate) fn n553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 516);
	s.is_empty()
}

/// The word transmedia.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	s.is_empty()
}

/// The word fushia.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	s.is_empty()
}

/// The word tujia.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	s.is_empty()
}

/// Words ending -pia.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	s.is_empty()
}

/// Words ending -ria.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	s.is_empty()
}

/// The word fucsia.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	s.is_empty()
}

/// Words ending -bestia.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['a', 'f', 'l', 'a', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ja.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'a', 'r', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 191);
			s.is_empty()
		}
		['o', 'l', 'f', 'a', 'c', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -la.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'e', 'i', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['l', 'a', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 827);
							s.is_empty()
						}
						['n', 'a', 'p', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['l', 'é', 'c', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ma.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 198);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['r', 'c', 'l', 'u', 'z', 'a', '\u{1}', s @ ..] => {
					super::push(out, 522);
					s.is_empty()
				}
				['t', 's', 'i', 's', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 829);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['o', 'l', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'a', 'l', 'p', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1035);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 831);
							s.is_empty()
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
				['a', 'r', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				['l', 'p', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -na.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'l', 'e', 'v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['r', 'b', 'm', 'e', 'm', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				['t', 'n', 'e', 'v', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'b', 'r', 'u', 't', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 202);
			s.is_empty()
		}
		['o', 'z', 'a', 'r', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['a', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 204);
			s.is_empty()
		}
		['k', 'u', 'y', '\u{1}', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		['s', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'm', 'á', 'c', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['e', 'l', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['i', 'h', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['r', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				['e', 'u', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['i', 'v', 'i', 'd', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 210);
			s.is_empty()
		}
		['n', 'e', 'f', 'e', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		['o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['l', 'r', 'a', 'c', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 832);
							s.is_empty()
						}
						['c', 'ó', 'm', 'e', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						['v', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 's', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 536);
					match s {
						[] => true,
						['u', 'p', 's', 'e', 'r', 'r', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 837);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1037);
									match s {
										[] => true,
										['l', 'o', 'c', 'e', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1130);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 838);
							match s {
								[] => true,
								['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 839);
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
									super::push(out, 1039);
									match s {
										[] => true,
										['c', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1131);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 840);
							match s {
								[] => true,
								['m', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1040);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'a', 'l', s @ ..] => {
							super::push(out, 841);
							match s {
								[] => true,
								['a', 'g', 'n', 'i', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1041);
									s.is_empty()
								}
								['e', 'm', 'a', 'g', 'n', 'i', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1042);
									s.is_empty()
								}
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

/// Words ending -va.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', 'l', 'o', 't', 'n', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', 'l', 'p', 'i', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ía.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	s.is_empty()
}

/// The word interclub.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	s.is_empty()
}

/// Words ending -c.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 220);
					s.is_empty()
				}
				['t', 'e', 'h', 't', 's', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 221);
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
			super::push(out, 45);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 539);
							s.is_empty()
						}
						['i', 'l', 'a', 'i', 'c', 'e', 'p', 's', 'e', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'j', '\u{1}', s @ ..] => {
			super::push(out, 46);
			s.is_empty()
		}
		['g', 'c', '\u{1}', s @ ..] => {
			super::push(out, 47);
			s.is_empty()
		}
		['i', 'd', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 48);
			s.is_empty()
		}
		['r', 'a', 'd', 'n', 'á', 't', 's', 'e', '\u{1}', s @ ..] => {
			super::push(out, 49);
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
		['c', s @ ..] => {
			super::push(out, 50);
			match s {
				[] => true,
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 222);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 223);
					match s {
						[] => true,
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 'e', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 542);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 51);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 224);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 225);
					match s {
						[] => true,
						['i', 'm', 'u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 226);
					match s {
						[] => true,
						['a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'r', 'f', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 52);
			s.is_empty()
		}
		['h', 'c', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 53);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['c', 'i', 'f', 'r', 'e', 'p', 'u', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'i', 'l', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 546);
							s.is_empty()
						}
						['p', 'o', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 547);
							s.is_empty()
						}
						['z', 'i', 'n', 'u', 'l', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 229);
					s.is_empty()
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 231);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 549);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 842);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 1043);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'o', s @ ..] => {
									super::push(out, 843);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 1044);
											s.is_empty()
										}
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
					super::push(out, 232);
					match s {
						[] => true,
						['b', 'e', 'd', 'n', '\u{1}', s @ ..] => {
							super::push(out, 550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['a', 'v', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['g', 'n', 'ó', 'v', '\u{1}', s @ ..] => {
							super::push(out, 552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['i', 'n', 'á', '\u{1}', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				['r', 'i', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 238);
					match s {
						[] => true,
						['c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 554);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 239);
					match s {
						[] => true,
						['e', 'l', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['i', 'r', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 242);
					match s {
						[] => true,
						['w', 'm', 'o', 's', 'n', 'a', 'r', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 243);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 558);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', 'o', s @ ..] => {
					super::push(out, 244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 245);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 246);
					match s {
						[] => true,
						['p', 'm', 'é', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 248);
					match s {
						[] => true,
						['l', 'c', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 250);
					match s {
						[] => true,
						['d', 'r', 'o', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 562);
							s.is_empty()
						}
						['r', 'r', 'a', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 563);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 564);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 845);
									s.is_empty()
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
						['l', 'é', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 252);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 566);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 846);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 1045);
											match s {
												[] => true,
												['t', '\u{1}', s @ ..] => {
													super::push(out, 1132);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 847);
									match s {
										[] => true,
										['n', 'u', 'b', s @ ..] => super::t0::n1111(s, out),
										_ => false,
									}
								}
								['t', s @ ..] => super::t0::n911(s, out),
								_ => false,
							}
						}
						['e', s @ ..] => super::t0::n610(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n277(s, out),
				['r', s @ ..] => super::t0::n278(s, out),
				['s', s @ ..] => super::t0::n279(s, out),
				['u', s @ ..] => super::t0::n280(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n68(s, out),
		['v', s @ ..] => super::t0::n69(s, out),
		['x', s @ ..] => super::t0::n70(s, out),
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['f', 'o', '\u{1}', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 67);
			s.is_empty()
		}
		['í', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 68);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -g.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 69);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['c', 'a', 'n', 'a', 's', 'a', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 70);
			s.is_empty()
		}
		['k', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 71);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 72);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['a', 'g', 'h', 's', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 73);
			s.is_empty()
		}
		['b', '\u{1}', s @ ..] => {
			super::push(out, 74);
			s.is_empty()
		}
		['e', 'y', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 75);
			s.is_empty()
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				['c', 'e', '\u{1}', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 584);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'a', 'w', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 76);
			s.is_empty()
		}
		['k', 'u', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['b', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['n', 'a', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['l', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['g', 'a', 'n', 'a', 'v', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 263);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['r', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 266);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'b', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 81);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['r', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 269);
					s.is_empty()
				}
				['s', 'a', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['á', 'l', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 83);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -j.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 84);
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
		['i', s @ ..] => {
			super::push(out, 85);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 271);
					match s {
						[] => true,
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', s @ ..] => {
					super::push(out, 272);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 858);
									match s {
										[] => true,
										['e', 'a', 't', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 1051);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 859);
									match s {
										[] => true,
										['c', 'n', 'i', 'l', 'e', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1052);
											s.is_empty()
										}
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
					super::push(out, 273);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 274);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 589);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1053);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'i', s @ ..] => {
									super::push(out, 861);
									match s {
										[] => true,
										['r', 'c', 'i', 's', 'a', 'u', 'c', '\u{1}', s @ ..] => {
											super::push(out, 1054);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 862);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 863);
									match s {
										[] => true,
										['e', 'r', 'r', 'i', s @ ..] => {
											super::push(out, 1055);
											match s {
												[] => true,
												['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
													super::push(out, 1136);
													s.is_empty()
												}
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
					super::push(out, 275);
					s.is_empty()
				}
				['t', 'n', 'e', 'i', s @ ..] => {
					super::push(out, 276);
					match s {
						[] => true,
						['r', 'o', 'm', 'e', 'r', 't', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 277);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 278);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'ó', 'g', '\u{1}', s @ ..] => {
					super::push(out, 279);
					s.is_empty()
				}
				['v', 'i', 'n', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 280);
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
					super::push(out, 281);
					match s {
						[] => true,
						['á', 'h', s @ ..] => {
							super::push(out, 592);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 282);
					match s {
						[] => true,
						['á', s @ ..] => {
							super::push(out, 593);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['í', s @ ..] => {
							super::push(out, 594);
							match s {
								[] => true,
								['f', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', 'r', 'a', 'm', 'o', 'c', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 283);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 595);
							s.is_empty()
						}
						['á', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 285);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 286);
					match s {
						[] => true,
						['c', 'á', s @ ..] => {
							super::push(out, 597);
							match s {
								[] => true,
								['t', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 287);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['g', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 288);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 289);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 291);
					match s {
						[] => true,
						['e', 't', 's', 'e', 'l', 'o', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ñ', s @ ..] => {
					super::push(out, 292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 'u', 'h', 'á', 'n', '\u{1}', s @ ..] => {
			super::push(out, 90);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 293);
					s.is_empty()
				}
				['d', 'n', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 294);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 296);
					s.is_empty()
				}
				_ => false,
			}
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
			super::push(out, 92);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'd', 'n', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				['á', 'r', '\u{1}', s @ ..] => {
					super::push(out, 299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 't', 'd', 'h', '\u{1}', s @ ..] => {
			super::push(out, 93);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['c', 'i', 'x', 'e', 't', 'i', 'h', 'w', '\u{1}', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				['h', 'u', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', '\u{1}', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 303);
					s.is_empty()
				}
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 305);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 306);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 307);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 600);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 868);
									s.is_empty()
								}
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
			super::push(out, 97);
			match s {
				[] => true,
				['p', 'm', 'u', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 311);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'b', 'k', '\u{1}', s @ ..] => {
					super::push(out, 312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 99);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['á', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 314);
					match s {
						[] => true,
						['i', 'l', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'a', s @ ..] => {
					super::push(out, 315);
					match s {
						[] => true,
						['r', 'u', 'h', 'a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['h', 's', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['c', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				['s', 'i', 'o', 'h', 'k', '\u{1}', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				['t', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['t', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['í', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 323);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 324);
					s.is_empty()
				}
				['l', 'o', 'z', 'n', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 325);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 327);
					s.is_empty()
				}
				['s', 'e', 'm', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 328);
					s.is_empty()
				}
				['t', 'o', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 330);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 331);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 332);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 607);
							match s {
								[] => true,
								['i', 't', 'o', 'c', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 869);
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
						['w', '\u{1}', s @ ..] => {
							super::push(out, 608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 334);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 335);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 609);
							s.is_empty()
						}
						['d', 'o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 610);
							s.is_empty()
						}
						['l', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 611);
							s.is_empty()
						}
						['p', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 612);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 613);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 336);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 337);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 615);
							match s {
								[] => true,
								['e', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'm', s @ ..] => {
							super::push(out, 616);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 339);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 340);
					s.is_empty()
				}
				['ñ', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ú', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['d', 'l', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 617);
							s.is_empty()
						}
						['s', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'n', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['b', 'm', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['d', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				['g', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 619);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 621);
							s.is_empty()
						}
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 623);
							s.is_empty()
						}
						['a', 'r', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 624);
							s.is_empty()
						}
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['z', 'a', '\u{1}', s @ ..] => {
							super::push(out, 626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 627);
							s.is_empty()
						}
						['n', 'a', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'a', 'r', 'f', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 353);
					s.is_empty()
				}
				['g', 'i', '\u{1}', s @ ..] => {
					super::push(out, 354);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 355);
					match s {
						[] => true,
						['j', 'a', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 630);
							match s {
								[] => true,
								['v', 'o', '\u{1}', s @ ..] => {
									super::push(out, 872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 631);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 356);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 357);
					match s {
						[] => true,
						['u', 't', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 358);
					match s {
						[] => true,
						['t', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 359);
					match s {
						[] => true,
						['b', 'a', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 634);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 635);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 't', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 360);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 637);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 875);
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
						['f', s @ ..] => {
							super::push(out, 638);
							match s {
								[] => true,
								['í', 'l', s @ ..] => {
									super::push(out, 876);
									match s {
										[] => true,
										['o', 'r', 't', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1056);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 639);
							match s {
								[] => true,
								['á', 'p', s @ ..] => {
									super::push(out, 877);
									match s {
										[] => true,
										['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 640);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									super::push(out, 878);
									match s {
										[] => true,
										['c', 'e', 'd', 'a', 'x', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 1058);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 641);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 879);
									match s {
										[] => true,
										['ú', 'l', '\u{1}', s @ ..] => {
											super::push(out, 1059);
											s.is_empty()
										}
										_ => false,
									}
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
					super::push(out, 362);
					match s {
						[] => true,
						['e', 'u', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 363);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['a', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['a', 'c', 'o', 'n', 'o', 'm', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 646);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 647);
							match s {
								[] => true,
								['f', 'i', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 648);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 881);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n119(s, out),
		['e', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 372);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 670);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 913);
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
						['n', s @ ..] => {
							super::push(out, 671);
							match s {
								[] => true,
								['a', 'l', 'a', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['á', s @ ..] => {
							super::push(out, 672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 373);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 673);
							s.is_empty()
						}
						['c', 'ú', 'n', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				['r', 'é', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['t', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'g', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 378);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 379);
					match s {
						[] => true,
						['q', s @ ..] => {
							match s {
								[] => true,
								['n', 'a', 'l', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => super::t0::n978(s, out),
								['o', 'l', 'b', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => super::t0::n979(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t0::n121(s, out),
		['g', s @ ..] => super::t0::n122(s, out),
		['h', s @ ..] => super::t0::n123(s, out),
		['i', s @ ..] => super::t0::n124(s, out),
		['j', s @ ..] => super::t0::n125(s, out),
		['l', s @ ..] => super::t0::n126(s, out),
		['m', s @ ..] => super::t0::n127(s, out),
		['n', s @ ..] => super::t0::n128(s, out),
		['p', s @ ..] => super::t0::n129(s, out),
		['r', s @ ..] => super::t0::n130(s, out),
		['s', s @ ..] => super::t0::n131(s, out),
		['t', s @ ..] => super::t0::n132(s, out),
		['u', s @ ..] => super::t0::n133(s, out),
		['v', s @ ..] => super::t0::n134(s, out),
		['z', s @ ..] => super::t0::n135(s, out),
		['ñ', s @ ..] => super::t0::n136(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	s.is_empty()
}

/// The word izq.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	s.is_empty()
}

/// Words ending -r.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['a', 'r', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 454);
					match s {
						[] => true,
						['c', 'i', 'd', 'u', 'j', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 457);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				['k', 'o', 't', 'k', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['i', 'r', 'p', 'e', 'c', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['l', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'ó', 'p', '\u{1}', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				['v', 'í', 'd', '\u{1}', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['c', 'r', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 804);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1023);
									match s {
										[] => true,
										['c', 'i', s @ ..] => {
											super::push(out, 1127);
											match s {
												[] => true,
												['v', 'n', 'e', '\u{1}', s @ ..] => {
													super::push(out, 1155);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1024);
									match s {
										[] => true,
										['u', 't', s @ ..] => {
											super::push(out, 1128);
											match s {
												[] => true,
												['b', 'o', '\u{1}', s @ ..] => {
													super::push(out, 1156);
													s.is_empty()
												}
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
							super::push(out, 805);
							match s {
								[] => true,
								['í', '\u{1}', s @ ..] => {
									super::push(out, 1025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 467);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 468);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 806);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1026);
									s.is_empty()
								}
								['u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 1027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 469);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 470);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 471);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 472);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 810);
							match s {
								[] => true,
								['l', 'u', 'c', 's', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'a', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 811);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 812);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1129);
											match s {
												[] => true,
												['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
													super::push(out, 1157);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['c', 'i', 's', '\u{1}', s @ ..] => {
													super::push(out, 1158);
													s.is_empty()
												}
												['n', 'o', 'm', '\u{1}', s @ ..] => {
													super::push(out, 1159);
													s.is_empty()
												}
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
					super::push(out, 475);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['e', 't', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 477);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 479);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 480);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 815);
							match s {
								[] => true,
								['a', 't', 'e', 't', 'n', 'i', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1029);
									s.is_empty()
								}
								['e', 'i', 'c', 'o', 'h', 'c', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['á', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['m', 'a', 'd', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 482);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 483);
					match s {
						[] => true,
						['p', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 816);
							s.is_empty()
						}
						['t', 's', 'e', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 484);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 818);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 819);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['í', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['g', 'i', s @ ..] => {
							match s {
								[] => true,
								['l', 'l', 'e', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1032);
									s.is_empty()
								}
								['n', 'o', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1033);
									s.is_empty()
								}
								['r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'g', 'r', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ú', 'g', 'n', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['d', 'l', 'm', '\u{1}', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		['e', 'm', 'r', 'u', 'o', 'g', '\u{1}', s @ ..] => {
			super::push(out, 141);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', 'h', 'n', 'e', 'r', 'h', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 488);
					s.is_empty()
				}
				['x', 'e', 'r', 'b', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		['r', 'o', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 143);
			s.is_empty()
		}
		['u', 'k', 'o', 'y', '\u{1}', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	s.is_empty()
}

/// The word dlv.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	s.is_empty()
}

/// Words ending -x.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['d', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 145);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['p', 'a', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
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
			super::push(out, 148);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 820);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'n', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 149);
			s.is_empty()
		}
		['e', 'l', 'g', 'n', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 150);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['n', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['a', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -z.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['r', 't', 'o', 'm', 'i', 'd', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['l', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				['r', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['l', 'a', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -á.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		['m', 'r', 'o', 'n', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['p', 'a', 'c', 'u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['z', 't', 'i', '\u{1}', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -é.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 497);
					s.is_empty()
				}
				['o', 'm', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['g', 'a', 'g', 'n', 'e', '\u{1}', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['i', 's', 'a', 'm', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['m', 'r', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -í.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['r', 'o', 'd', 'i', 'n', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['e', 'm', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ó.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	s.is_empty()
}

/// Words ending -ú.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['c', 'u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 503);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bundante.
pub(crate) fn n1111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1046);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 1133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tante.
pub(crate) fn n911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 848);
	match s {
		[] => true,
		['t', 'e', 'l', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ente.
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 567);
	match s {
		[] => true,
		['d', 'u', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1049);
					match s {
						[] => true,
						['a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['c', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ote.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['c', 'r', 'e', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 568);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				['n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', 'a', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				['i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'o', 'g', 'u', 'h', '\u{1}', s @ ..] => {
			super::push(out, 569);
			s.is_empty()
		}
		['r', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 570);
			s.is_empty()
		}
		['s', 'o', 'h', 'c', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 571);
			s.is_empty()
		}
		['t', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rte.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['e', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 573);
			s.is_empty()
		}
		['o', 'c', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 574);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ste.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ute.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 576);
			s.is_empty()
		}
		['h', 'c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 577);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['h', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						['l', 'b', 'a', s @ ..] => {
							super::push(out, 857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xe.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -do.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 882);
							match s {
								[] => true,
								['e', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1060);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 883);
							match s {
								[] => true,
								['d', 'e', '\u{1}', s @ ..] => {
									super::push(out, 1061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'o', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['m', 'o', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 886);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'a', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['l', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 889);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 890);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1064);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
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
							super::push(out, 891);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1065);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 1138);
											s.is_empty()
										}
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
					super::push(out, 653);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 892);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1066);
									match s {
										[] => true,
										['o', 'r', 'd', 'i', 'h', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 893);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1067);
									match s {
										[] => true,
										['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1140);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 895);
							match s {
								[] => true,
								['r', 't', s @ ..] => {
									super::push(out, 1068);
									match s {
										[] => true,
										['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1141);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 896);
							match s {
								[] => true,
								['r', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', s @ ..] => {
							super::push(out, 897);
							match s {
								[] => true,
								['s', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 898);
							match s {
								[] => true,
								['l', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', s @ ..] => {
							super::push(out, 899);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 900);
							match s {
								[] => true,
								['o', 't', s @ ..] => {
									super::push(out, 1073);
									match s {
										[] => true,
										['o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1142);
											s.is_empty()
										}
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
			super::push(out, 367);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['á', 'r', s @ ..] => {
							super::push(out, 901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['e', 'u', s @ ..] => {
							super::push(out, 902);
							match s {
								[] => true,
								['q', '\u{1}', s @ ..] => {
									super::push(out, 1075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', s @ ..] => {
							super::push(out, 903);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 904);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1077);
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
													super::push(out, 1160);
													s.is_empty()
												}
												['s', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 1161);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 1078);
									match s {
										[] => true,
										['v', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1143);
											s.is_empty()
										}
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
			super::push(out, 368);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 661);
					match s {
						[] => true,
						['b', 'a', 'r', 't', 'n', 'o', 'c', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 905);
							s.is_empty()
						}
						['d', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['e', 'v', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				['o', 'h', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n401(s, out),
		['r', 'o', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n403(s, out),
		_ => false,
	}
}

/// The word antiblanqueo.
pub(crate) fn n978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 915);
	s.is_empty()
}

/// The word antibloqueo.
pub(crate) fn n979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 916);
	s.is_empty()
}

/// Words ending -fo.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['u', 'n', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['p', 'á', 'p', '\u{1}', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['i', 'd', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['n', 'í', '\u{1}', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 917);
							s.is_empty()
						}
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'g', 'n', 'i', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['e', 'i', 'r', 'r', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ho.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['a', 'v', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 385);
			s.is_empty()
		}
		['c', 'e', 'r', 'e', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['d', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -io.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'r', 'e', 'j', 'e', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 919);
							s.is_empty()
						}
						['u', 'j', 'a', 'r', 't', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['h', 'a', 's', 'e', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['e', 'm', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['c', 'i', 'u', 's', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'c', 'n', 'i', 'a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'o', 'm', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['i', 'r', 't', 'a', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['m', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['i', 'b', 'o', 'n', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 925);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1080);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 1144);
											s.is_empty()
										}
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
					super::push(out, 689);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', 'a', 'r', 'g', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['i', 'm', 'n', 'i', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['e', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jo.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['e', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['e', 'u', 'p', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['i', 'h', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 400);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['g', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 696);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 931);
							match s {
								[] => true,
								['n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 1081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 932);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['t', 'o', 'r', 'p', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
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
				['c', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['e', 'b', 'm', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 935);
							s.is_empty()
						}
						['í', 'd', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 700);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['l', 'ú', '\u{1}', s @ ..] => {
							super::push(out, 938);
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
				['h', '\u{1}', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 939);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 940);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 941);
							match s {
								[] => true,
								['u', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1083);
									s.is_empty()
								}
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

/// Words ending -no.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 942);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1084);
									match s {
										[] => true,
										['o', 'c', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1145);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 943);
							match s {
								[] => true,
								['s', 'i', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 944);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1086);
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['p', 'm', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 709);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 948);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 712);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 950);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 1088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'o', 'd', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 'g', '\u{1}', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['a', 'c', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
							super::push(out, 955);
							match s {
								[] => true,
								['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
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
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 411);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 956);
							match s {
								[] => true,
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'b', s @ ..] => {
							super::push(out, 957);
							match s {
								[] => true,
								['o', 'g', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1091);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 959);
							s.is_empty()
						}
						['t', 'e', 'r', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['e', 'i', 't', 'o', 'd', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 415);
			s.is_empty()
		}
		['u', 'r', 'g', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['á', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'c', '\u{1}', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 962);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1092);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1093);
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
						['e', 'p', 'r', 'o', 't', 'a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 728);
					match s {
						[] => true,
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 729);
					match s {
						[] => true,
						['i', 'l', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 730);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 966);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1094);
									match s {
										[] => true,
										['g', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 1146);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 1147);
											s.is_empty()
										}
										['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1148);
											s.is_empty()
										}
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
					super::push(out, 732);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 967);
							match s {
								[] => true,
								['g', 's', s @ ..] => {
									super::push(out, 1095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 968);
							match s {
								[] => true,
								['á', '\u{1}', s @ ..] => {
									super::push(out, 1096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 969);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 1097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 970);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 1098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['e', 't', 'n', 'í', '\u{1}', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['g', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 738);
					s.is_empty()
				}
				['t', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{1}', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 971);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 1099);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 's', 'e', 'd', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 972);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'd', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 743);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 'i', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['c', 's', 'o', '\u{1}', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 976);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['c', 'o', 'r', 't', 'e', 'r', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['u', 'p', s @ ..] => {
					super::push(out, 753);
					match s {
						[] => true,
						['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'e', 't', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 980);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 1100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 981);
							match s {
								[] => true,
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1101);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 982);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 1102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 983);
							match s {
								[] => true,
								['v', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 758);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1104);
									match s {
										[] => true,
										['r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 1149);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 1105);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1150);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'l', s @ ..] => {
									super::push(out, 1106);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 1151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'o', 'l', s @ ..] => {
							super::push(out, 985);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 1107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 986);
							match s {
								[] => true,
								['r', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'l', 'u', 'g', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				['m', 'r', s @ ..] => {
					super::push(out, 760);
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 988);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'm', s @ ..] => {
							super::push(out, 989);
							match s {
								[] => true,
								['u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', s @ ..] => {
							super::push(out, 990);
							match s {
								[] => true,
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['b', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 763);
					s.is_empty()
				}
				['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['m', 'r', 'o', 'f', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['i', 'u', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['q', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'p', 'm', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['f', 's', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['o', 'p', 'ó', 'r', 'p', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 994);
							s.is_empty()
						}
						['ó', 'p', 'o', s @ ..] => {
							super::push(out, 995);
							match s {
								[] => true,
								['r', 'p', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['q', 's', s @ ..] => {
							super::push(out, 996);
							match s {
								[] => true,
								['o', 'm', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['s', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 998);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 1114);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1115);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 1116);
									match s {
										[] => true,
										['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1152);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1117);
									s.is_empty()
								}
								['o', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1118);
									s.is_empty()
								}
								['u', 'p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', 'l', 'i', 'f', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 1001);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1002);
							match s {
								[] => true,
								['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1120);
									s.is_empty()
								}
								['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1121);
									s.is_empty()
								}
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
			super::push(out, 439);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1003);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 778);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1006);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1123);
									match s {
										[] => true,
										['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['h', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 1010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1011);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1012);
							match s {
								[] => true,
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 1124);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['p', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'j', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['b', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				['k', 'a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1016);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1017);
							s.is_empty()
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
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1018);
							match s {
								[] => true,
								['s', 'o', 'p', 's', s @ ..] => {
									super::push(out, 1125);
									match s {
										[] => true,
										['i', 'd', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
											super::push(out, 1154);
											s.is_empty()
										}
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
					super::push(out, 791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['o', 'p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['n', 'ó', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['h', 'c', 'e', 'r', 'r', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				['r', 'b', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 450);
			s.is_empty()
		}
		['o', 'm', 'n', 'e', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ño.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 796);
					match s {
						[] => true,
						['t', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1020);
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
					super::push(out, 797);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1021);
							match s {
								[] => true,
								['z', 'i', '\u{1}', s @ ..] => {
									super::push(out, 1126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'q', 'e', s @ ..] => {
					super::push(out, 798);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -odo.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udo.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['o', 't', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

